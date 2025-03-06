use log::info;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
  pub telegram_key          : String,
  pub camera_input          : CameraInput,
  pub gui_stream_output     : GUIStreamOutput,
  pub internet_stream_output: InternetStreamOutput,
  pub g_cloud               : GCloud
}

#[derive(Debug, Deserialize)]
pub struct CameraInput {
  pub resolution: String,
  pub fps       : String,
  pub clip      : Clip
}

#[derive(Debug, Deserialize)]
pub struct Clip {
  pub segment_size_sec     : String,
  pub segments             : String,
  pub timer_before_clip_sec: u64,
  pub disk_full_buffer_gb  : i64
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

pub async fn getConfig() -> Result<ConfigFile, Box<dyn std::error::Error>> {
  let yaml_str = std::fs::read_to_string("../../config.yaml")?;
  let config: ConfigFile = serde_yaml::from_str(&yaml_str)?;
  Ok(config)
}

pub async fn showConfig() {
  info!("{:?}", getConfig().await.unwrap());
}