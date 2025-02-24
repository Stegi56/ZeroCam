use std::{env, fs, thread, time};
use std::error::Error;
use std::process::{Command, Stdio};
use chrono::Utc;
use log::{debug, info};

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
    info!("Clip scheduled, waiting for timer...");
    tokio::time::sleep(time::Duration::from_secs(5)).await;

    let mut outputs: Vec<_> = fs::read_dir(self.liveRecordingPath.clone())?
      .filter_map(|e| {
        let entry = e.ok()?;
        let meta = entry.metadata().ok()?;
        let modified = meta.modified().ok()?;
        Some((modified, entry.file_name().into_string().unwrap()))
      })
      .collect();
    outputs.sort_by_key(|(time, _)| *time);
    let sortedOutputs: Vec<String>              = outputs.into_iter().map(|(_, name)| name).collect();
    let sortedOutputsWithFullPaths: Vec<String> = sortedOutputs.iter().map({|output|
      "file '".to_owned()
      + self.liveRecordingPath.clone().as_str()
      + output
      + "'"
    }).collect();
    let pathsForWriting = sortedOutputsWithFullPaths.join("\n");

    fs::write(self.recordingPathsFilePath.clone(), pathsForWriting)?;

    let newFileName = self.clipsPath.clone() + &*Utc::now().to_string() + ".mp4";
    debug!("Concatenating recordings to {}", &newFileName);
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
}

pub async fn startRecording() -> Result<(), Box<dyn Error>> {
  let liveRecordingPath = env::current_dir()?.parent().unwrap().parent().unwrap().join("LiveRecording/").display().to_string();



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