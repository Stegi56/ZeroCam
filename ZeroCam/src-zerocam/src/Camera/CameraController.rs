use std::{env, thread, time};
use std::error::Error;
use std::process::{Command, Stdio};
use log::info;

pub struct CameraController {}

impl CameraController {
  pub fn new() -> Result<CameraController, Box<dyn Error>> {
    Ok(Self{})
  }

  pub fn clip(&self) -> Result<(), Box<dyn Error>> {
    info!("Clipping...");
    thread::sleep(time::Duration::from_secs(5));
    info!("Clipped");
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
    .arg(format!("{}output%03d.ts", liveRecordingPath)) // output in numbered files
    .spawn().unwrap();
  Ok(())
}