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

  let sensitivity: f64 = 40f64;

  let mut cap = VideoCapture::new(2, CAP_ANY).unwrap();
  cap.set(CAP_PROP_BUFFERSIZE, 1.0).unwrap();

  let mut startFrame = Mat::default();
  cap.read(&mut startFrame).unwrap();

  let mut startFrameGray = Mat::default();
  cvt_color(&mut startFrame, &mut startFrameGray, COLOR_BGR2GRAY.into(),0).unwrap();

  let mut frame = Mat::default();
  loop{
    sleep(Duration::from_millis(100));
    cap.read(&mut frame).expect("error reading");

    let mut frameGray = Mat::default();
    cvt_color(&mut frame, &mut frameGray, COLOR_BGR2GRAY.into(),0).unwrap();

    let mut frameBlurred = Mat::default();
    gaussian_blur(&mut frameGray, &mut frameBlurred, Size::new(15, 15), 0., 0., 0.into()).expect("error bluring");

    let mut claheImg = Mat::default();
    let clahe = create_clahe(15f64, Size::new(1, 1));
    clahe.expect("error creating clahe").apply(&mut frameBlurred, &mut  claheImg).expect("error appyling clahe");

    let mut difference = Mat::default();
    absdiff(&mut claheImg, &mut startFrameGray, &mut difference).expect("error getting difference");

    let mut differenceBinned = Mat::default();
    threshold(&mut difference, &mut differenceBinned, sensitivity, 255f64, THRESH_BINARY.into()).expect("error binning difference");

    let differenceTotal: f64 = sum_elems(&mut differenceBinned.clone()).expect("error summing elems").iter().sum();
    info!("Difference Total: {}", differenceTotal);

    imshow("input"              , &frame).unwrap();
    imshow("gray"               , &frameGray).unwrap();
    imshow("gaussian blur"      , &frameBlurred).unwrap();
    imshow("clahe normalisation", &claheImg).unwrap();
    imshow("difference"         , &difference).unwrap();
    imshow("binned"             , &differenceBinned).unwrap();

    startFrameGray = claheImg;

    if wait_key(10 as i32).unwrap() == 27 {
      highgui::destroy_all_windows().unwrap();
      break;
    }

  }
}
