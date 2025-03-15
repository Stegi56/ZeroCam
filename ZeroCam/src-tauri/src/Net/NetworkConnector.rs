use std::error::Error;
use std::process::Command;
use log::debug;

pub async fn ping_google() -> bool {
  match tokio::net::TcpStream::connect("8.8.8.8:53").await {
    Ok(_) => true,
    Err(_) => false
  }
}

pub async fn getKnownNetworks() -> Result<Vec<String>, Box<dyn Error>> {
  let output = Command::new("sh").arg("-c")
    .arg("nmcli -f NAME,TYPE connection show | awk '$NF==\"wifi\" {print}'")
    .output()
    .unwrap();
  Ok(
    String::from_utf8(output.stdout).unwrap().lines().map(|s| {
      let mut words = s.split_whitespace().collect::<Vec<&str>>();
      words.pop(); //remove "wifi"
      words.join(" ")
    }).collect()
  )
}

pub async fn getCurrentConnectedNetworks() -> Result<Vec<String>, Box<dyn Error>> {
  let output = Command::new("sh").arg("-c")
    .arg("nmcli -f CONNECTION,STATE dev | awk '$NF==\"connected\" {print}'")
    .output().unwrap();
  Ok(
    String::from_utf8(output.stdout).unwrap().lines().map(|s| {
      let mut words = s.split_whitespace().collect::<Vec<&str>>();
      words.pop(); //remove "connected"
      words.join(" ")
    }).collect()
  )
}

pub async fn getAvailableNetworks() -> Result<Vec<String>, Box<dyn Error>> {
  let output = Command::new("sh").arg("-c")
    .arg("nmcli -f SSID dev wifi | awk '$1!=\"--\"' | awk '$1!=\"SSID\"'")
    .output().unwrap();

  Ok(
    String::from_utf8(output.stdout).unwrap().lines().map(|s| s.trim()).map(String::from).collect()
  )
}

pub async fn connectToNetwork(ssid: &String) -> Result<(), Box<dyn Error>> {
  let output = Command::new("sh")
    .arg("-c")
    .arg(format!("nmcli device wifi connect '{}'", ssid))
    .output().unwrap();
  let res = String::from_utf8(output.stdout).unwrap();
  if res.contains("successfully activated") {Ok(())}
  else {Err("Failed to change connection".into())}
}

pub fn getUUIDforSSID(ssid: &String) -> Result<String, Box<dyn Error>> {
  let output = Command::new("sh").arg("-c")
    .arg(format!("nmcli -t -f uuid,name connection show | grep '{}' | cut -d: -f1", ssid))
    .output()
    .unwrap();

  Ok(
    String::from_utf8(output.stdout).unwrap()
      .trim().to_string()
  )
}

//priority 0-8, 10 = highest
pub fn setPriority(uuid: &String, priority: i8) -> Result<(), Box<dyn Error>> {
  Command::new("sh").arg("-c")
    .arg(format!("nmcli con modify {} connection.autoconnect-priority {}", uuid, priority))
    .spawn().unwrap();
  debug!("set priority {} to {}", priority, uuid);
  Ok(())
}
