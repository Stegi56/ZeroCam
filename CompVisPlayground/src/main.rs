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
}, highgui};
use log::{info, warn};
use opencv::imgproc::create_clahe;
use opencv::videoio::{CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH};

#[tokio::main]
async fn main() {
  env_logger::init();
  rustls::crypto::ring::default_provider().install_default().unwrap();

  let sensitivity       : f64 = 40f64;

  let mut cap = VideoCapture::new(2, CAP_ANY).unwrap();
  cap.set(CAP_PROP_BUFFERSIZE, 1.0).unwrap();

  let mut startFrame = Mat::default();
  cap.read(&mut startFrame).unwrap();

  let mut frame = Mat::default();
  loop{
    sleep(Duration::from_millis(100));
    cap.read(&mut frame).unwrap();

    let mut frameBlurred1 = Mat::default();
    gaussian_blur(&mut frame, &mut frameBlurred1, Size::new(15, 15), 0., 0., 0.into()).unwrap();

    let mut claheImg = Mat::default();
    let clahe = create_clahe(15f64, Size::new(1, 1));
    clahe.unwrap().apply(&mut frameBlurred1, &mut  claheImg).unwrap();

    let mut difference = Mat::default();
    absdiff(&mut claheImg, &mut startFrame, &mut difference).unwrap();

    let mut differenceBinned = Mat::default();
    threshold(&mut difference, &mut differenceBinned, sensitivity, 255f64, THRESH_BINARY.into()).unwrap();

    imshow("video", &frame).unwrap();
    imshow("video2", &frameBlurred1).unwrap();
    imshow("video3", &claheImg).unwrap();
    imshow("video4", &difference).unwrap();
    imshow("video5", &differenceBinned).unwrap();

    let differenceTotal: f64 = sum_elems(&mut differenceBinned.clone()).unwrap().iter().sum();
    info!("Difference Total: {}", differenceTotal);

    startFrame = claheImg;

    if wait_key(10 as i32).unwrap() == 27 {
      highgui::destroy_all_windows().unwrap();
      break;
    }

  }
}
