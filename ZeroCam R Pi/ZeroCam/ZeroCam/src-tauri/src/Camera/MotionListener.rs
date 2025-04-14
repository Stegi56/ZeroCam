use crate::Camera::ClipScheduler::ClipScheduler;
use crate::Config;
use crate::Config::ConfigFile;

use log::{debug, info, warn};
use opencv::{
  core::{absdiff, sum_elems, Size, AlgorithmHint::ALGO_HINT_DEFAULT},
  highgui::wait_key,
  imgproc::{
    cvt_color, equalize_hist, gaussian_blur, threshold, ColorConversionCodes::COLOR_BGR2GRAY,
    ThresholdTypes::THRESH_BINARY,
  },
  prelude::*,
  videoio::{VideoCapture, CAP_ANY, CAP_PROP_BUFFERSIZE},
};
use std::error::Error;
use std::sync::atomic::{AtomicBool, AtomicI8, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use thread::sleep;
use opencv::imgproc::create_clahe;

static WATCHING : AtomicBool = AtomicBool::new(true);
static TRIGGERED: AtomicBool = AtomicBool::new(false);
static DURATION : AtomicI8   = AtomicI8::new(0);

pub struct MotionListener {
  clipScheduler: Arc<ClipScheduler>,
  config       : ConfigFile
}

impl MotionListener {
  pub async fn new(clipScheduler: Arc<ClipScheduler>) -> Result<MotionListener, Box<dyn Error>> {
    Ok(Self{
      clipScheduler: clipScheduler,
      config       : Config::getConfig().await?
    })
  }

  pub async fn run(self) {

    let mut cap = VideoCapture::new(2, CAP_ANY).unwrap();
    cap.set(CAP_PROP_BUFFERSIZE, 1.0).unwrap();
    debug!("made cap");

    let mut startFrame = Mat::default();
    cap.read(&mut startFrame).unwrap();

    let mut startFrameGray = Mat::default();
    cvt_color(&mut startFrame, &mut startFrameGray, COLOR_BGR2GRAY.into(),0, ALGO_HINT_DEFAULT).unwrap();

    let mut frame = Mat::default();
    loop{
      if WATCHING.load(Ordering::SeqCst) {
        sleep(Duration::from_millis(self.config.motion_listener.frame_delay_millisec));
        cap.read(&mut frame).expect("error reading");

        let mut frameGray = Mat::default();
        cvt_color(&mut frame, &mut frameGray, COLOR_BGR2GRAY.into(),0, ALGO_HINT_DEFAULT).unwrap();

        let mut frameBlurred1 = Mat::default();
        gaussian_blur(&mut frameGray, &mut frameBlurred1, Size::new(15, 15), 0., 0., 0.into(), ALGO_HINT_DEFAULT).expect("error bluring");

        let mut claheImg = Mat::default();
        let clahe = create_clahe(15f64, Size::new(1, 1));
        clahe.expect("error creating clahe").apply(&mut frameBlurred1, &mut  claheImg).expect("error appyling clahe");

        let mut difference = Mat::default();
        absdiff(&mut claheImg, &mut startFrameGray, &mut difference).expect("error getting difference");

        let mut differenceBinned = Mat::default();
        threshold(&mut difference, &mut differenceBinned, self.config.motion_listener.sensitivity_inverse, 255f64, THRESH_BINARY.into()).expect("error binning difference");

        let differenceTotal: f64 = sum_elems(&mut differenceBinned.clone()).expect("error summing elems").iter().sum();
        debug!("Difference Total: {}", differenceTotal);

        startFrameGray = claheImg;


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
              if self.clipScheduler.scheduleClip().await.is_err(){warn!{"Motion sensor clip cooldown overlap!"}}
            }
          }
        }
        debug!("Motion Duration: {}", DURATION.load(Ordering::Relaxed));
      }else{
        sleep(Duration::from_secs(5));
      }
    }
  }
}

pub fn setParkedState(b: bool) {
  WATCHING.store(b, Ordering::Relaxed);
  info!("Motion Listener State: {}", b.to_string())
}

pub fn getParkedState() -> bool{
  WATCHING.load(Ordering::Relaxed)
}
