use std::{env, fs, thread, time};
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use thread::sleep;
use sysinfo::Disks;
use chrono::Utc;
use log::{debug, info};
use tokio::fs::DirEntry;

pub struct CameraController {
  liveRecordingPath     : String,
  recordingPathsFilePath: String, // a file that stores the paths of files inside  LiveRecordings directory
  clipsPath             : String,
}

impl CameraController {
  pub fn new() -> Result<CameraController, Box<dyn Error>> {
    Ok(Self{
      liveRecordingPath     : env::current_dir()?.parent().unwrap().parent().unwrap().join("LiveRecording/").display().to_string(),
      recordingPathsFilePath: env::current_dir()?.parent().unwrap().parent().unwrap().join("recordingPaths.txt").display().to_string(),
      clipsPath             : env::current_dir()?.parent().unwrap().parent().unwrap().join("Clips/").display().to_string()
    })
  }

  pub async fn clip(&self) -> Result<(), Box<dyn Error>> {
    const GB: i64 = 1024 * 1024 * 1024;
    fn getDiskSafeSpaceB() -> i64 {
      Disks::new_with_refreshed_list()
        .list()[0]
        .available_space() as i64
        // - (GB * 319) // comment outside test case
        - GB         // never allow system to have less than 1GB available space for stability
    }

    info!("Clip scheduled, waiting for timer...");
    sleep(time::Duration::from_secs(10));

    let outputSizeB:i64 = self.makePathsForWritingFileAndGetOutputSize().await?;
    info!("Clip outputSize: {:.0}MB", (outputSizeB as f64) / (1024.0 * 1024.0));

    //we can take 0 item as this reads /proc/mounts which is in creation order
    //meaning 0 contains file system root.
    //this assumes the dashcam is not running multiple drives and if it is the app is deployed on
    //the root file system
    let mut diskSafeSpaceB:i64 = getDiskSafeSpaceB();
    info!("Safe disk space: {:.3}GB", (diskSafeSpaceB as f64) / (1024.0 * 1024.0 * 1024.0));
    while outputSizeB > diskSafeSpaceB {
      let oldestClipPath= self.getOldestLocalClip()?;
      fs::remove_file(&oldestClipPath)?;
      info!("Deleted oldest clip: {}", oldestClipPath.display());
      sleep(time::Duration::from_secs(2)); //allow kernel time to finish deleting
      diskSafeSpaceB = getDiskSafeSpaceB();
    }


    let newFileName = self.clipsPath.clone() + &*Utc::now().to_string() + ".mp4";
    info!("Concatenating recordings to {}", &newFileName);
    Command::new("ffmpeg")
      .stdin(Stdio::null())
      .arg("-f").arg("concat") //input existing files
      .arg("-safe").arg("0") //disables safety to allow full path use
      .arg("-i").arg(self.recordingPathsFilePath.clone()) //input list of files to be concatenated
      .arg("-c").arg("copy")
      .arg(newFileName)
      .spawn().unwrap();
    Ok(())
  }

  async fn makePathsForWritingFileAndGetOutputSize(&self) -> Result<(i64), Box<dyn Error>> {
    let mut outputs: Vec<_> = fs::read_dir(self.liveRecordingPath.clone())?
      .filter_map(|e| {
        let entry = e.ok()?;
        let meta = &entry.metadata().ok()?;
        let modified = meta.modified().ok()?;
        let size = meta.len();
        Some((modified, size, entry.file_name().into_string().unwrap()))
      })
      .collect();

    let outputSize: u64 = outputs.clone()
      .iter()
      .map(|(_, size, _)| size)
      .sum();

    outputs.sort_by_key(|(time, _, _)| *time);
    let sortedOutputs: Vec<String>              = outputs.into_iter().map(|(_, _, name)| name).collect();
    let sortedOutputsWithFullPaths: Vec<String> = sortedOutputs.iter().map({|output|
      "file '".to_owned()
        + self.liveRecordingPath.clone().as_str()
        + output
        + "'"
    }).collect();

    let pathsForWriting = sortedOutputsWithFullPaths.join("\n");
    fs::write(self.recordingPathsFilePath.clone(), pathsForWriting)?;

    Ok(outputSize as i64)
  }

  fn getOldestLocalClip(&self) -> Result<PathBuf, Box<dyn Error>> {
    let mut entries: Vec<fs::DirEntry> = fs::read_dir(self.clipsPath.clone()).unwrap().filter_map(Result::ok).collect();
    if entries.is_empty() { panic!("Clips folder is empty!")}
    entries.sort_by_key({|e|
      e.metadata().unwrap()
        .modified().unwrap()
    });
    let oldest = entries.first().unwrap().path();
    Ok(oldest)
  }
}

pub async fn startRecording() -> Result<(), Box<dyn Error>> {
  let liveRecordingPath = env::current_dir()?.parent().unwrap().parent().unwrap().join("LiveRecording/").display().to_string();

  fs::remove_dir_all(&liveRecordingPath)?;
  fs::create_dir_all(&liveRecordingPath)?; //wipe recordings from previous session to prevent corruption

  Command::new("ffmpeg")
    .stdout(Stdio::null()) //peace
    .stderr(Stdio::null()) //and quiet :)
    //Input
    .arg("-f")           .arg("v4l2"       ) //input format video 4 linux
    .arg("-input_format").arg("mjpeg"      ) //pixel format
    .arg("-framerate")   .arg("25"         )
    .arg("-video_size")  .arg("1920x1080"  )
    .arg("-i")           .arg("/dev/video0") //input source
    //Encoding
    .arg("-c:v")             .arg("libx264"               ) //h.264
    .arg("-preset")          .arg("ultrafast"             ) //compression algorithm speed (faster = lower quality)
    .arg("-crf")             .arg("17"                    ) //loss parameter (lower = less loss)
    .arg("-tune")            .arg("film"                  ) //optimise to lower deblocking
    .arg("-force_key_frames").arg("expr:gte(t,n_forced*5)") //force key frames every x seconds for splitting
    //Output
    .arg("-f")               .arg("segment"           ) //output in segments
    .arg("-reset_timestamps").arg("1"                 ) //prevent corruption of timestamps when loop recording
    .arg("-segment_time")    .arg("5"                 ) //x seconds per segment
    .arg("-segment_wrap")    .arg("10"                ) //loop after x segments
    .arg(format!("{}output%03d.ts", liveRecordingPath)) //output in numbered files
    .spawn().unwrap();
  Ok(())
}