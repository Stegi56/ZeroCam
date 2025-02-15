use std::{fs, io};
use std::cmp::min;
use google_drive3::{DriveHub, Error};
use google_drive3::api::File;
use log::{debug, error, info};
use chrono::{DateTime, Duration, Utc};

use crate::Files::GDConnector;

pub struct GDController{
  gdClient: GDConnector::GDClient
}

impl GDController{
  pub async fn new() ->  core::result::Result<GDController, Error>{
    Ok(Self{
      gdClient: GDConnector::GDClient::new().await?
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
    let mut gdFileList: Vec<google_drive3::api::File> = self.gdClient.getFileList().await.unwrap();
    let stringGDFileList: Vec<String> = gdFileList.iter().map(|f| f.name.clone().unwrap()).collect();
    let localFileList: Vec<String> = std::fs::read_dir("Clips").unwrap()
      .map(|f| f.unwrap().file_name().into_string().unwrap())
      .collect();
    debug!("Local file list {:?}", localFileList);

    let localFileListInGD: Vec<String> = localFileList
      .iter()
      .cloned()
      .filter(|item| stringGDFileList.contains(item))
      .collect();
    debug!("Local files in GD: {:?}", localFileListInGD);

    for file in localFileListInGD {
      if let Err(e) = fs::remove_file("Clips/".to_string() + &file) {
        error!("Failed to delete '{}': {}", file, e);
      } else {
        info!("Deleted {} from local storage in 2nd catch pass", file);
      }
    }

    let localFileListNotInGD: Vec<String> = localFileList
      .iter()
      .cloned()
      .filter(|item| !stringGDFileList.contains(item))
      .collect();

    debug!("Local files not int GD: {:?}", localFileListNotInGD);
    debug!("Space Exists: {}", localFileListNotInGD.len());

    for file in localFileListNotInGD {
      let fileSize:i64 = fs::metadata("Clips/".to_string() + &file).unwrap().len() as i64;
      let fileCreationDate: DateTime<Utc> = fs::metadata("Clips/".to_string() + &file).unwrap().created().unwrap().into() ;

      let clipsFolderID = gdFileList.iter().find(|f| f.name.clone().unwrap() == "ZeroCam Clips").unwrap().id.clone().unwrap();
      let mut someOldestGDFile = self.getOldestGDFile(&clipsFolderID).await;

      if fileSize < self.calculateSpaceAvailable(&clipsFolderID).await {
        self.gdClient.uploadFile("Clips/".to_string() + file.clone().as_str(), file.clone(), clipsFolderID).await.unwrap_or_else(|e| panic!("Failed to upload file to GD: {}", e));
        info!("Uploaded {} to GD", file);

        if let Err(e) = fs::remove_file("Clips/".to_string() + file.clone().as_str()) {
          error!("Failed to delete '{}': {}", file, e);
        } else {
          info!("Deleted {} from local storage", file);
        }
      }

      else if (someOldestGDFile.is_some()) && (fileCreationDate > someOldestGDFile.clone().unwrap().created_time.unwrap()) {
        let mut oldestGDFile = someOldestGDFile.unwrap();
        while fileSize > self.calculateSpaceAvailable(&clipsFolderID).await {
          oldestGDFile = self.getOldestGDFile(&clipsFolderID).await.unwrap_or_else(|| panic!("No files to delete in GD and no space available!"));
          if let Err(e) = self.gdClient.deleteFile(oldestGDFile).await {
            error!("Failed to delete '{}': {}", file, e);
          } else {
            info!("Successfully deleted {} from google drive", file);
          }
        }

        self.gdClient.uploadFile("Clips/".to_string() + file.clone().as_str(), file.clone(), clipsFolderID).await.unwrap_or_else(|e| panic!("Failed to upload file to GD: {}", e));
        info!("Uploaded {} to GD", file);

        if let Err(e) = fs::remove_file("Clips/".to_string() + file.clone().as_str()) {
          error!("Failed to delete '{}': {}", file, e);
        } else {
          info!("Deleted {} from local storage", file);
        }
      }

      else{
        info!("File not uploaded: Either older than oldest file(testing error) or no space in GD");

        info!("file creation date: {}", fileCreationDate);
        info!("oldest file creation date: {}", someOldestGDFile.clone().unwrap().created_time.unwrap());
      }
    }
  }

  async fn getOldestGDFile(&self, clipsFolderID: &String) -> Option<google_drive3::api::File>{
    let gdClipsList: Vec<google_drive3::api::File> = self.gdClient.getFileList().await.unwrap()
      .iter()
      .filter(|f| f.parents.clone().unwrap().contains(&clipsFolderID))
      .cloned()
      .collect();

    gdClipsList.iter().min_by(|a, b| a.created_time.unwrap().cmp(&b.created_time.unwrap())).cloned()
  }

  async fn calculateSpaceAvailable(&self, clipsFolderId: &String) -> i64 {
    let gdClipsList: Vec<google_drive3::api::File> = self.gdClient.getFileList().await.unwrap()
      .iter()
      .filter(|f| f.parents.clone().unwrap().contains(&clipsFolderId))
      .cloned()
      .collect();

    let storageQuota = self.gdClient.getStorageQuota().await;
    let freeGDSpace = storageQuota.limit.unwrap() - storageQuota.usage.unwrap();
    let spaceAllowedByZeroCam: i64 = 14 * 1024 * 1024; //14GB
    let freeZeroCamSpace = spaceAllowedByZeroCam - gdClipsList.iter().map(|f| f.size.unwrap()).sum::<i64>();
    let spaceAvailable = min(freeZeroCamSpace, freeGDSpace);

    debug!("GD Space Available: {}", freeGDSpace);
    debug!("ZeroCam Clips Folder Space Available: {}", freeZeroCamSpace);
    spaceAvailable
  }
}