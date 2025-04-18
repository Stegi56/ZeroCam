use log::info;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
  pub telegram_key          : String,
  pub camera_input          : CameraInput,
  pub motion_listener       : MotionListener,
  pub gui_stream_output     : GUIStreamOutput,
  pub internet_stream_output: InternetStreamOutput,
  pub g_cloud               : GCloud,
  pub hotspot_networks      : Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct CameraInput {
  pub resolution: String,
  pub fps       : String,
  pub encoder   : String,
  pub clip      : Clip
}

#[derive(Debug, Deserialize)]
pub struct Clip {
  pub segment_size_sec     : String,
  pub segments             : String,
  pub timer_before_clip_sec: u64,
  pub cooldown_sec         : i64,
  pub disk_full_buffer_gb  : i64
}

#[derive(Debug, Deserialize)]
pub struct MotionListener {
  pub sensitivity_inverse : f64,
  pub threshold_sum_kilo  : f64,
  pub frame_delay_millisec: u64,
  pub trigger_duration    : i8,
  pub resolution          : String,
  pub bit_rate            : String,
  pub fps                 : String
}

#[derive(Debug, Deserialize)]
pub struct GUIStreamOutput {
  pub resolution: String,
  pub bit_rate  : String,
  pub fps       : String
}

#[derive(Debug, Deserialize)]
pub struct InternetStreamOutput {
  pub url       : String,
  pub username  : String,
  pub password  : String,
  pub resolution: String,
  pub bit_rate  : String,
  pub fps       : String
}

#[derive(Debug, Deserialize)]
pub struct GCloud {
  pub limit_gb                    : i64,
  pub backup_scheduler_timeout_sec: u64,
}

pub async fn getConfig() -> Result<ConfigFile, Box<dyn Error>> {
  let yaml_str = std::fs::read_to_string("../lib/zerocam/config.yaml")?;
  let config: ConfigFile = serde_yaml::from_str(&yaml_str)?;
  Ok(config)
}

pub async fn getConfigAsString() -> Result<String, Box<dyn Error>> {
  let yaml_str = std::fs::read_to_string("../lib/zerocam/config.yaml")?;
  Ok(yaml_str)
}

pub fn setConfigFromString(configString: String) -> Result<(), Box<dyn Error>> {
  std::fs::write("../lib/zerocam/config.yaml", configString)?;
  info!("Updated config");
  Ok(())
}

pub async fn showConfig() {
  info!("{:?}", getConfig().await.unwrap());
}
