use crate::Config;
use crate::Config::ConfigFile;

use chrono::Utc;
use log::{info};
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs, thread, time};
use std::time::SystemTime;
use sysinfo::Disks;
use thread::sleep;

pub struct CameraController {
  recordingSegmentsPath : String,
  recordingPathsFilePath: String, // a file that stores the paths of files inside  LiveRecordings directory
  clipsPath             : String,
  config                : ConfigFile,
}

impl CameraController {
  pub async fn new() -> Result<CameraController, Box<dyn Error>> {
    Ok(Self{
      recordingSegmentsPath : "../lib/zerocam/LiveRecording/".to_string(),
      recordingPathsFilePath: "../lib/zerocam/recordingPaths.txt".to_string(),
      clipsPath             : "../lib/zerocam/Clips/".to_string(),
      config                : Config::getConfig().await?
    })
  }

  pub async fn clip(&self) -> Result<(), Box<dyn Error>> {
    const GB: i64 = 1024 * 1024 * 1024;
    fn getDiskSafeSpaceB(availableSpaceLimit: i64) -> i64 {
      Disks::new_with_refreshed_list()
        .list()[0]
        .available_space() as i64
        // - (GB * 319) // comment outside test case
        - availableSpaceLimit // never allow system to have less than 1GB available space for stability
    }

    info!("Clip scheduled, waiting for timer...");
    sleep(time::Duration::from_secs(self.config.camera_input.clip.timer_before_clip_sec));

    let outputSizeB:i64 = self.makePathsForWritingFileAndGetOutputSize().await?;
    info!("Clip outputSize: {:.0}MB", (outputSizeB as f64) / (1024.0 * 1024.0));

    //this assumes the dashcam is not running multiple drives and if it is the app is deployed on
    //the root file system
    let mut diskSafeSpaceB:i64 = getDiskSafeSpaceB(self.config.camera_input.clip.disk_full_buffer_gb.clone() * GB);
    info!("Safe disk space: {:.3}GB", (diskSafeSpaceB as f64) / (1024.0 * 1024.0 * 1024.0));
    while outputSizeB > diskSafeSpaceB {
      let oldestClipPath= self.getOldestLocalClip()?;
      fs::remove_file(&oldestClipPath)?;
      info!("Deleted oldest clip: {}", oldestClipPath.display());
      sleep(time::Duration::from_secs(1)); //allow kernel time to finish deleting
      diskSafeSpaceB = getDiskSafeSpaceB(self.config.camera_input.clip.disk_full_buffer_gb.clone() * GB);
    }

    let newFileName = self.clipsPath.clone() + &*Utc::now().to_string() + ".mp4";
    info!("Concatenating recordings to {}", &newFileName);
    Command::new("ffmpeg")
      .stdin(Stdio::null())
      .stdout(Stdio::null()) //peace
      .stderr(Stdio::null()) //and quiet :))
      .arg("-f"   ).arg("concat"                           ) //input existing files
      .arg("-safe").arg("0"                                ) //disables safety to allow full path use
      .arg("-i"   ).arg(self.recordingPathsFilePath.clone()) //input list of files to be concatenated
      .arg("-c").arg("copy")
      .arg(newFileName)
      .spawn().unwrap();
    Ok(())
  }

  async fn makePathsForWritingFileAndGetOutputSize(&self) -> Result<i64, Box<dyn Error>> {
    let mut outputs: Vec<(SystemTime, u64, String)> = fs::read_dir(self.recordingSegmentsPath.clone())?
      .filter_map(|e| {
        let entry = e.ok()?;
        let meta = &entry.metadata().ok()?;
        let modified = meta.modified().ok()?;
        let size = meta.len();
        Some((modified, size, entry.file_name().into_string().unwrap()))
      })
      .collect();

    outputs = outputs.clone().into_iter().filter(|(_, _, name)| name != ".gitkeep").collect();

    let outputSize: u64 = outputs.clone()
      .iter()
      .map(|(_, size, _)| size)
      .sum();

    outputs.sort_by_key(|(time, _, _)| *time);
    let sortedOutputs: Vec<String>              = outputs.into_iter().map(|(_, _, name)| name).collect();
    let sortedOutputsWithFullPaths: Vec<String> = sortedOutputs.iter().map({|output|
      "file '".to_owned()
        + "LiveRecording/"
        + &output
        + "'"
    }).collect();

    let pathsForWriting = sortedOutputsWithFullPaths.join("\n");
    fs::write(self.recordingPathsFilePath.clone(), pathsForWriting)?;

    Ok(outputSize as i64)
  }

  fn getOldestLocalClip(&self) -> Result<PathBuf, Box<dyn Error>> {
    let mut entries: Vec<fs::DirEntry> = fs::read_dir(self.clipsPath.clone()).unwrap().filter_map(Result::ok).collect();
    entries = entries.into_iter().filter(|e| e.file_name() != ".gitkeep").collect();

    if entries.is_empty() { panic!("Clips folder is empty!")}
    entries.sort_by_key({|e|
      e.metadata().unwrap()
        .modified().unwrap()
    });
    let oldest = entries.first().unwrap().path();
    Ok(oldest)
  }
}

pub async fn startCameraAndStream() -> Result<(), Box<dyn Error>> {
  let config = Config::getConfig().await?;
  let liveRecordingPath = env::current_dir()?.parent().unwrap().join("lib/zerocam/LiveRecording/").display().to_string();

  fs::remove_dir_all(&liveRecordingPath)?;
  fs::create_dir_all(&liveRecordingPath)?; //wipe recordings from previous session to prevent corruption

  let mediamtxPath             = env::current_dir()?.parent().unwrap().join("lib/zerocam/MediaMTX/mediamtx").display().to_string();
  let mediamtxLocalConfPath    = env::current_dir()?.parent().unwrap().join("lib/zerocam/MediaMTX/mediamtx-local.yml").display().to_string();
  let mediamtxInternetConfPath = env::current_dir()?.parent().unwrap().join("lib/zerocam/MediaMTX/mediamtx-internet.yml").display().to_string();

  Command::new(&mediamtxPath) //start localhost stream
    .stdout(Stdio::null()) //peace
    .stderr(Stdio::null()) //and quiet :)
    .arg(mediamtxLocalConfPath)
    .spawn()?;

  Command::new(&mediamtxPath) //start internet stream with security features
    .stdout(Stdio::null()) //peace
    .stderr(Stdio::null()) //and quiet :)
    .arg(mediamtxInternetConfPath)
    .spawn()?;

  Command::new("sudo")
    .stdout(Stdio::null())//peace
    .stderr(Stdio::null())//and quiet :)
    .arg("modprobe")
    .arg("v4l2loopback")
    .arg("devices=1") //create virtual devices for loopback
    .spawn()?;

  Command::new("ffmpeg")
    .stdout(Stdio::null()) //peace
    .stderr(Stdio::null()) //and quiet :))
    .arg("-threads")       .arg("4"                            )
    .arg("-f")             .arg("v4l2"                         ) // demuxer format v4l2
    .arg("-input_format")  .arg("mjpeg"                        )
    .arg("-framerate")     .arg(config.camera_input.fps        )
    .arg("-video_size")    .arg(&config.camera_input.resolution)

    //input
    .arg("-i")             .arg("/dev/video0") // read original source

    //output for storage
    .arg("-vf")              .arg("format=gray"                             )
    .arg("-pix_fmt")         .arg("yuv420p"                                 )
    .arg("-c:v")             .arg(&config.camera_input.encoder              )
    .arg("-b:v")             .arg("20M"                                     ) //bitrate
    .arg("-f")               .arg("segment"                                 ) //output in segments
    .arg("-force_key_frames").arg(format!("expr:gte(t,n_forced*{})"
                                , config.camera_input.clip.segment_size_sec)) //force key frames every x seconds for splitting
    .arg("-reset_timestamps").arg("1"                                       ) //prevent corruption of timestamps when loop recording
    .arg("-segment_time")    .arg(config.camera_input.clip.segment_size_sec ) //x seconds per segment
    .arg("-segment_wrap")    .arg(config.camera_input.clip.segments         ) //loop after x segments
    .arg(format!("{}output%03d.ts", liveRecordingPath)                      ) //output in numbered files

    //Output for local stream to Internet
    .arg("-f")        .arg("rtsp"                                  ) // RTSP container
    .arg("-pix_fmt")  .arg("yuv420p"                               )
    .arg("-c:v")      .arg("libx264"                               ) // h.264 encoder with gpu
    .arg("-preset")   .arg("ultrafast"                             ) // Keep latency low
    .arg("-s")        .arg(config.internet_stream_output.resolution)
    .arg("-b:v")      .arg(config.internet_stream_output.bit_rate  )
    .arg("-r")        .arg(config.internet_stream_output.fps       )
    .arg("rtsp://localhost:8555/stream1"                           ) // RTMP stream to local MediaMTX

    //Output for local stream to GUI
    .arg("-f")          .arg("rtsp"                             ) // RTSP container
    .arg("-pix_fmt")    .arg("yuv420p"                          )
    .arg("-c:v")        .arg("libx264"                          ) // h.264 encoder
    .arg("-preset")     .arg("ultrafast"                        ) // Keep latency low
    .arg("-s")          .arg(config.gui_stream_output.resolution)
    .arg("-b:v")        .arg(config.gui_stream_output.bit_rate  )
    .arg("-r")          .arg(config.gui_stream_output.fps       )
    .arg("rtsp://localhost:8554/stream1"                        ) // RTMP stream to local MediaMTX

    //output for opencv
    .arg("-pix_fmt")       .arg("yuv420p"                        )
    .arg("-s")             .arg(config.motion_listener.resolution)
    .arg("-r")             .arg(config.motion_listener.fps       )
    .arg("-b:v")           .arg(&config.motion_listener.bit_rate )
    .arg("-f").arg("v4l2") .arg("/dev/video2"                    )
    .spawn()?;
  Ok(())
}
