use crate::Config;
use crate::Config::ConfigFile;
use crate::GDFiles::GDConnector;

use log::{debug, info};
use std::cmp::{min, Reverse};
use std::{env, error::Error, fs};

pub struct GDController {
  gdClient  : GDConnector::GDClient,
  clipsPath : String,
  configFile: ConfigFile,
}

impl GDController {
  pub async fn new() -> Result<GDController, Box<dyn Error>> {
    Ok(Self {
      gdClient  : GDConnector::GDClient::new().await?,
      clipsPath : env::current_dir()?.parent().unwrap().parent().unwrap().join("Clips/").display().to_string(),
      configFile: Config::getConfig().await?,
    })
  }

  pub async fn backupNow(&self) -> Result<(), Box<dyn Error>> {
    self.checkClipFolderExistsAndFix().await?;
    self.uploadClips().await?;
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

  pub async fn uploadClips(&self) -> Result<(), Box<dyn Error>> {
    info!("Uploading local clips to GD");
    let mut gdFileListDescending            : Vec<google_drive3::api::File> = self.gdClient.getFileList().await.unwrap();
    let clipsFolderID                       : String                        = gdFileListDescending.iter().find(|f| f.name.clone().unwrap() == "ZeroCam Clips").unwrap().id.clone().unwrap();
    let mut gdClipsFileListDescending       : Vec<google_drive3::api::File> = gdFileListDescending.clone().iter().filter(|f| f.parents.clone().unwrap().contains(&clipsFolderID)).cloned().collect();
    let mut stringGDClipsFileListDescending : Vec<String>                   = gdClipsFileListDescending.clone().iter().map(|f| f.name.clone().unwrap()).collect();
    let mut localFileListDescending         : Vec<String>                   = self.getLocalFilesDescending()?;
    let mut localFileListNotInGDDescending  : Vec<String>                   = localFileListDescending.clone().iter().filter(|f| !stringGDClipsFileListDescending.contains(f)).cloned().collect();

    debug!("Local file list {:?}", &localFileListDescending);
    debug!("Local file list not in GD{:?}", &localFileListNotInGDDescending);

    while (
      (!localFileListNotInGDDescending.is_empty())
      && (
        localFileListNotInGDDescending.first().cloned().unwrap_or_else(|| "".to_string())
          > stringGDClipsFileListDescending.last().cloned().unwrap_or_else(|| "".to_string())
      )
    ){
      let localFile: &String = localFileListNotInGDDescending.first().unwrap();
      let localFileSize: i64 = fs::metadata(self.clipsPath.clone() + &localFile).unwrap().len() as i64;

      while localFileSize > self.calculateSpaceAvailable(&clipsFolderID).await? {
        let oldestGDFile = gdClipsFileListDescending.last().expect("No files left to delete to make space for file in GD!");
        self.gdClient.deleteFile(oldestGDFile.clone()).await.expect(format!("Error deleting oldest gd file: {}", oldestGDFile.clone().name.unwrap()).as_str());

        info!("Deleted: {} from google drive to make space for : {}", oldestGDFile.clone().name.unwrap(), &localFile);

        gdFileListDescending      = self.gdClient.getFileList().await.unwrap();
        gdClipsFileListDescending = gdFileListDescending.clone().iter().filter(|f| f.parents.clone().unwrap().contains(&clipsFolderID)).cloned().collect();
      }

      self.gdClient
        .uploadFile(self.clipsPath.clone() + localFile.clone().as_str(), localFile.clone(), clipsFolderID.clone() )
        .await?;

      info!("Successfully uploaded to googled drive: {}", localFile.clone().as_str());

      gdFileListDescending            = self.gdClient.getFileList().await.unwrap();
      gdClipsFileListDescending       = gdFileListDescending.clone().iter().filter(|f| f.parents.clone().unwrap().contains(&clipsFolderID)).cloned().collect();
      stringGDClipsFileListDescending = gdClipsFileListDescending.clone().iter().map(|f| f.name.clone().unwrap()).collect();
      localFileListDescending         = self.getLocalFilesDescending()?;
      localFileListNotInGDDescending  = localFileListDescending.clone().iter().filter(|f| !stringGDClipsFileListDescending.contains(f)).cloned().collect();
    }

    info!("Finished backing up all files");

    Ok(())
  }

  async fn calculateSpaceAvailable(&self, clipsFolderId: &String) -> Result<i64, Box<dyn Error>> {
    let gdClipsList: Vec<google_drive3::api::File> = self.gdClient.getFileList().await?
      .iter()
      .filter(|f| f.parents.clone().unwrap().contains(&clipsFolderId))
      .cloned()
      .collect();

    let storageQuota = self.gdClient.getAbout().await?.1.storage_quota.unwrap();
    let freeGDSpace = storageQuota.limit.unwrap() - storageQuota.usage.unwrap();
    let GB:i64 = 1024 * 1024 * 1024;
    let spaceAllowedByZeroCam: i64 = self.configFile.g_cloud.limit_gb * GB;
    let freeZeroCamSpace = spaceAllowedByZeroCam - gdClipsList.iter().map(|f| f.size.unwrap()).sum::<i64>();
    let spaceAvailable = min(freeZeroCamSpace, freeGDSpace);

    debug!("GD Space Available: {:.3}GB", (freeGDSpace as f64) / ((GB) as f64));
    debug!("GD ZeroCam Clips Folder Space Available: {:.3}GB", ((freeZeroCamSpace as f64) / ((GB) as f64)));
    Ok(spaceAvailable)
  }

  fn getLocalFilesDescending(&self) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files: Vec<_> = fs::read_dir(self.clipsPath.clone())?
      .filter_map(|e| {
        let entry = e.ok()?;
        let meta = entry.metadata().ok()?;
        let modified = meta.modified().ok()?;
        Some((modified, entry.file_name().into_string().unwrap()))
      })
      .collect();
    files.sort_by_key(|(time, _)| Reverse(*time));
    Ok(files.into_iter().map(|(_, name)| name).collect())
  }
}
