use crate::Files::GDConnector;
use chrono::{DateTime, Duration, Utc};
use google_drive3::api::File;
use log::{debug, error, info};
use serde::ser::StdError;
use std::cmp::min;
use std::{env, error::Error, fs, io};

pub struct GDController {
  gdClient: GDConnector::GDClient,
  clipsPath: String
}

impl GDController {
  pub async fn new() -> core::result::Result<GDController, Box<dyn Error>> {
    Ok(Self {
      gdClient: GDConnector::GDClient::new().await?,
      clipsPath: env::current_dir()?.parent().unwrap().parent().unwrap().join("Clips/").display().to_string()
    })
  }

  pub async fn backupNow(&self) -> Result<(), Box<dyn Error>> {
    self.checkClipFolderExistsAndFix().await?;
    self.uploadClipsAndClearLocal().await?;
    Ok(())
  }

  pub async fn checkClipFolderExistsAndFix(&self) -> Result<(), Box<dyn Error>>{
    let fileList:Vec<google_drive3::api::File> = self.gdClient.getFileList().await.unwrap();
    let containsFolder = fileList.iter().any(|f| f.name.clone().unwrap_or_default() == "ZeroCam Clips");
    if !containsFolder{
      self.gdClient.createClipsFolder().await.unwrap();
      info!("Clip folder created");
    }
    Ok(())
  }

  pub async fn uploadClipsAndClearLocal(&self) -> Result<(), Box<dyn Error>> {
    info!("Uploading local clips to GD");
    let mut gdFileList: Vec<google_drive3::api::File> = self.gdClient.getFileList().await.unwrap();
    let stringGDFileList: Vec<String> = gdFileList.iter().map(|f| f.name.clone().unwrap()).collect();
    let localFileList: Vec<String> = self.getLocalFilesOldestFirst()?;
    debug!("Local file list {:?}", localFileList);

    let localFileListInGD: Vec<String> = localFileList
      .iter()
      .cloned()
      .filter(|item| stringGDFileList.contains(item))
      .collect();
    debug!("Local files in GD: {:?}", localFileListInGD);

    for file in localFileListInGD {
      if let Err(e) = fs::remove_file(self.clipsPath.clone() + &file) {
        error!("Failed to delete '{}': {}", file, e);
      } else {
        info!("Deleted {} from local storage, already exists in GD", file);
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
      let fileSize: i64 = fs::metadata(self.clipsPath.clone() + &file).unwrap().len() as i64;

      let clipsFolderID = gdFileList.iter().find(|f| f.name.clone().unwrap() == "ZeroCam Clips").unwrap().id.clone().unwrap();
      let mut someOldestGDFile:Option<google_drive3::api::File> = self.getOldestGDFile(&clipsFolderID).await?;

      if fileSize < self.calculateSpaceAvailable(&clipsFolderID).await? {
        self.gdClient.uploadFile(self.clipsPath.clone() + file.clone().as_str(), file.clone(), clipsFolderID, ).await.unwrap();
        info!("Uploaded {} to GD", file);

        if let Err(e) = fs::remove_file(self.clipsPath.clone() + file.clone().as_str()) {
          error!("Failed to delete '{}': {}", file, e);
        } else {
          info!("Deleted {} from local storage", file);
        }
      }

      else if someOldestGDFile.is_some(){
        while fileSize > self.calculateSpaceAvailable(&clipsFolderID).await? && someOldestGDFile.is_some(){
          let oldestGDFile = someOldestGDFile.unwrap();
          if let Err(e) = self.gdClient.deleteFile(oldestGDFile).await {
            error!("Failed to delete '{}': {}", file, e);
          } else {
            info!("Successfully deleted {} from google drive", file);
          }
          someOldestGDFile = self.getOldestGDFile(&clipsFolderID).await?;
        }

        self.gdClient
          .uploadFile(self.clipsPath.clone() + file.clone().as_str(), file.clone(), clipsFolderID, )
          .await?;
        info!("Uploaded {} to GD", file);

        fs::remove_file(self.clipsPath.clone() + file.clone().as_str())?;
        info!("Deleted {} from local storage", file);
      }

      else{
        error!("File not uploaded: No space in GD");
        debug!("oldest file creation date: {}", someOldestGDFile.clone().unwrap().created_time.unwrap());
      }
    }
    Ok(())
  }

  async fn getOldestGDFile(&self, clipsFolderID: &String) -> Result<(Option<google_drive3::api::File>), Box<dyn Error>>{
    let gdFileList: Vec<google_drive3::api::File> = self.gdClient.getFileList().await?
      .iter()
      .filter(|f| f.parents.clone().unwrap().contains(&clipsFolderID))
      .cloned().collect();

    let oldestGDFile = gdFileList.iter()
      .min_by(|a, b| a.created_time.unwrap().cmp(&b.created_time.unwrap()))
      .cloned();
    Ok(oldestGDFile)
  }

  async fn calculateSpaceAvailable(&self, clipsFolderId: &String) -> Result<i64, Box<dyn Error>> {
    let gdClipsList: Vec<google_drive3::api::File> = self.gdClient.getFileList().await?
      .iter()
      .filter(|f| f.parents.clone().unwrap().contains(&clipsFolderId))
      .cloned()
      .collect();

    let storageQuota = self.gdClient.getAbout().await?.1.storage_quota.unwrap();
    let freeGDSpace = storageQuota.limit.unwrap() - storageQuota.usage.unwrap();
    let spaceAllowedByZeroCam: i64 = (1 * 1024 * 1024 * 1024) / 50; //1GB /50
    let freeZeroCamSpace = spaceAllowedByZeroCam - gdClipsList.iter().map(|f| f.size.unwrap()).sum::<i64>();
    let spaceAvailable = min(freeZeroCamSpace, freeGDSpace);

    debug!("GD Space Available: {}", freeGDSpace);
    debug!("ZeroCam Clips Folder Space Available: {}", freeZeroCamSpace);
    Ok(spaceAvailable)
  }

  fn getLocalFilesOldestFirst(&self) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files: Vec<_> = fs::read_dir(self.clipsPath.clone())?
      .filter_map(|e| {
        let entry = e.ok()?;
        let meta = entry.metadata().ok()?;
        let modified = meta.modified().ok()?;
        Some((modified, entry.file_name().into_string().unwrap()))
      })
      .collect();
    files.sort_by_key(|(time, _)| *time);
    Ok(files.into_iter().map(|(_, name)| name).collect())
  }
}
