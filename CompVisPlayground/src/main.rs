#![allow(non_snake_case)]

use std::thread::sleep;
use std::time::Duration;
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
  imshow
}};
use log::{info, warn};

#[tokio::main]
async fn main() {
  env_logger::init();
  rustls::crypto::ring::default_provider().install_default().unwrap();

  let sensitivity       : f64 = 50f64;

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
    sleep(Duration::from_millis(100));
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
    threshold(&mut difference, &mut differenceBinned, sensitivity, 255f64, THRESH_BINARY.into()).unwrap();

    imshow("video", &frame).unwrap();
    imshow("video2", &frameGray).unwrap();
    imshow("video3", &frameBlurred).unwrap();
    imshow("video4", &frameEqualized).unwrap();
    imshow("video5", &difference).unwrap();
    imshow("video5", &differenceBinned).unwrap();

    let differenceTotal: f64 = sum_elems(&mut differenceBinned.clone()).unwrap().iter().sum();
    info!("Difference Total: {}", differenceTotal);

    startFrameEqualized = frameEqualized;

    if wait_key(10 as i32).unwrap() == 27 {
      highgui::destroy_all_windows().unwrap();
      break;
    }

  }
}
