use crate::Config::ConfigFile;
use crate::Config;

use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI8, Ordering};
use std::thread;
use std::time::Duration;
use log::{debug, error, info};
use opencv::{prelude::*, videoio::{
  CAP_PROP_BUFFERSIZE,
  CAP_PROP_FPS,
  CAP_ANY,
  VideoCapture
}, core::{
  Size,
  sum_elems,
  absdiff,
  Rect,
  CV_8UC4
}, imgproc::{
  ColorConversionCodes::COLOR_BGR2GRAY,
  cvt_color,
  equalize_hist,
  gaussian_blur,
  ThresholdTypes::THRESH_BINARY,
  threshold
}, highgui::{
  wait_key,
}};
use tokio::time::sleep;
use crate::Camera::ClipScheduler::ClipScheduler;

static WATCHING : AtomicBool = AtomicBool::new(true);
static TRIGGERED: AtomicBool = AtomicBool::new(false);
static DURATION : AtomicI8   = AtomicI8::new(0);

pub struct MotionListener {
  clipScheduler : Arc<ClipScheduler>,
  config        : ConfigFile,
  streamUrl     : String
}

impl MotionListener {
  pub async fn new(clipScheduler: Arc<ClipScheduler>) -> Result<MotionListener, Box<dyn Error>> {
    Ok(Self{
      clipScheduler : clipScheduler,
      config        : Config::getConfig().await?,
      streamUrl     : String::from("http://localhost:8888/stream1/index.m3u8")
    })
  }

  pub async fn run(self) {
    let mut cap = VideoCapture::new(3, CAP_ANY).unwrap();
    cap.set(CAP_PROP_BUFFERSIZE, 1.0).unwrap();

    let mut startFrame = Mat::default();
    cap.read(&mut startFrame).unwrap();

    let mut startFrameGray = Mat::default();
    cvt_color(&mut startFrame, &mut startFrameGray, COLOR_BGR2GRAY.into(),0).unwrap();

    let mut startFrameBlurred = Mat::default();
    gaussian_blur(&mut startFrameGray, &mut startFrameBlurred, Size::new(5, 5), 0., 0., 0.into()).unwrap();

    let mut startFrameEqualized = Mat::default();
    equalize_hist(&mut startFrameBlurred, &mut startFrameEqualized).unwrap();


    let mut frame = Mat::default();
    loop{
      sleep(Duration::from_millis(400)).await;
      if WATCHING.load(Ordering::SeqCst) {
        sleep(Duration::from_millis(self.config.motion_listener.frame_delay_millisec));
        cap.read(&mut frame).unwrap();

        let mut frameGray = Mat::default();
        cvt_color(&mut frame, &mut frameGray, COLOR_BGR2GRAY.into(),0).unwrap();

        let mut frameBlurred = Mat::default();
        gaussian_blur(&mut frameGray, &mut frameBlurred, Size::new(3, 3), 0., 0., 0.into()).unwrap();

        let mut frameEqualized = Mat::default();
        equalize_hist(&mut frameBlurred, &mut frameEqualized).unwrap();

        let mut difference = Mat::default();
        absdiff(&mut frameEqualized, &mut startFrameEqualized, &mut difference).unwrap();

        let mut differenceBinned = Mat::default();
        threshold(&mut difference, &mut differenceBinned, self.config.motion_listener.sensitivity_inverse, 255f64, THRESH_BINARY.into()).unwrap();

        let differenceTotal: f64 = sum_elems(&mut differenceBinned.clone()).unwrap().iter().sum();
        debug!("Motion in frame total: {}", differenceTotal);

        startFrameEqualized = frameEqualized;

        if differenceTotal < (self.config.motion_listener.threshold_sum_kilo * 1000.0){
          if DURATION.load(Ordering::Relaxed) > 0{
            DURATION.store(DURATION.load(Ordering::Relaxed) - 1, Ordering::Relaxed);
            if TRIGGERED.load(Ordering::Relaxed){
              TRIGGERED.store(false, Ordering::Relaxed);
              debug!("Motion sensor: RELAXED");
            }
          }
        }else if !TRIGGERED.load(Ordering::Relaxed){
          if DURATION.load(Ordering::Relaxed) < self.config.motion_listener.trigger_duration{
            DURATION.store(DURATION.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
          }
          if DURATION.load(Ordering::Relaxed) == self.config.motion_listener.trigger_duration{
            if !TRIGGERED.load(Ordering::Relaxed){
              TRIGGERED.store(true, Ordering::Relaxed);
              info!("Motion sensor: TRIGGERED");
              self.clipScheduler.scheduleClip().await.unwrap();
            }
          }
        }
        debug!("Motion Duration: {}", DURATION.load(Ordering::Relaxed));
      }
    }
  }
}

pub fn startWatching() {
  WATCHING.store(true, Ordering::Relaxed);
}

pub fn stopWatching() {
  WATCHING.store(false, Ordering::Relaxed);
  TRIGGERED.store(false, Ordering::Relaxed);
  DURATION.store(0, Ordering::Relaxed);
}