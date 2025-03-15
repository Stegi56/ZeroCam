use crate::Net::NetworkConnector::{connectToNetwork, getAvailableNetworks, getCurrentConnectedNetworks, getKnownNetworks, getUUIDforSSID, setPriority};
use crate::Config::ConfigFile;
use crate::Camera::MotionListener::setParkedState;

use log::{debug, info};

pub async fn initialiseNetworkPriorities(config: &ConfigFile) -> Result<(), Box<dyn std::error::Error>> {
  let _ = getKnownNetworks().await?.iter().map(|ssid| {
    let uuid: String = getUUIDforSSID(&ssid).unwrap();
    if config.hotspot_networks.contains(&ssid) {
      setPriority(&uuid, 0).unwrap();
      info!("Set {} network to priority 0", &ssid);
    } else {
      setPriority(&uuid, 10).unwrap();
      info!("Set {} network to priority 10", &ssid);
    }
  }).collect::<Vec<_>>();

  Ok(())
}

pub async fn evaluateNetworkStateAndHandleChange(previousConnectionState: Vec<String>, config: &ConfigFile) -> Result<(Vec<String>), Box<dyn std::error::Error>> {
  let mut currentConnectionState: Vec<String> = getCurrentConnectedNetworks().await?;
  let knownNetworks = getKnownNetworks().await?;
  let availableKnownNonHotspotNetworks = getAvailableNetworks().await?.iter().filter(|ssid| {
    (!config.hotspot_networks.contains(&ssid))
    && knownNetworks.contains(&ssid)
  }).cloned().collect::<Vec<String>>();

  debug!("currentConnectionState: {:?}", currentConnectionState);
  debug!("knownNetworks: {:?}", knownNetworks);
  debug!("availableKnownNonHotspotNetworks: {:?}", availableKnownNonHotspotNetworks);

  if containsHotspotNetwork(currentConnectionState.clone(), &config.hotspot_networks)
    && !availableKnownNonHotspotNetworks.is_empty() {
    let targetNetwork = availableKnownNonHotspotNetworks.last().unwrap();
    info!("Transitioning away from hotspot to: {}", targetNetwork);
    connectToNetwork(targetNetwork).await?;
    currentConnectionState = getCurrentConnectedNetworks().await?;
  }

  if containsHotspotNetwork(previousConnectionState.clone(), &config.hotspot_networks)
    != containsHotspotNetwork(currentConnectionState.clone(), &config.hotspot_networks) { //if there was a transition
    let newParkedState: bool = !containsHotspotNetwork(currentConnectionState.clone(), &config.hotspot_networks);
    setParkedState(newParkedState);
    info!("Network state changed from: {:?}", previousConnectionState);
    info!("To: {:?}", &currentConnectionState);
  }

  info!("Completed network refresh");
  Ok((currentConnectionState))
}

fn containsHotspotNetwork(mut currentConnectionState: Vec<String>, hotspotNetworks: &Vec<String>) -> bool {
  currentConnectionState.iter().filter(|c| hotspotNetworks.contains(c))
    .cloned().collect::<Vec<String>>().len() > 0
}