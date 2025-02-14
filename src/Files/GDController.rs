use google_drive3::{DriveHub, Error};
use log::{debug, info};
use crate::Files::GDConnector;

pub struct GDController{
  gdClient: GDConnector::GDClient
}

impl GDController{
  pub async fn new() ->  core::result::Result<GDController, Error>{
    Ok(Self{
      gdClient: GDConnector::GDClient::new().await.unwrap()
    })
  }

  pub async fn checkClipFolderExistsAndFix(&self){
    info!("Checking if clip folder exists in GD");
    let fileList:Vec<google_drive3::api::File> = self.gdClient.getFileList().await.unwrap();
    let containsFolder = fileList.iter().any(|f| f.name.clone().unwrap_or_default() == "ZeroCam Clips");
    debug!("Clip folder exists: {}", containsFolder);
    if !containsFolder{
      self.gdClient.createClipsFolder().await.unwrap();
      info!("Clip folder created");
    }
  }

  pub async fn uploadClipsAndClearLocal(&self){
    info!("Uploading local clips to GD");
    let gdFileList:Vec<String> = self.gdClient.getFileList()
      .await.unwrap()
      .iter()
      .map(|f| f.name.clone().unwrap_or_default())
      .collect();
    let localFileList: Vec<String> = std::fs::read_dir("Clips").unwrap()
      .map(|f| f.unwrap().file_name().into_string().unwrap())
      .collect();
    debug!("Local file list {:?}", localFileList);
    let localFileListNotInGD: Vec<String> = localFileList
      .iter()
      .cloned()
      .filter(|item| !gdFileList.contains(item))
      .collect();
    debug!("Local files not int GD: {:?}", localFileListNotInGD);
  }
}