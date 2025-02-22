extern crate google_drive3 as drive3;
extern crate hyper;

use std::env;
use chrono::{DateTime, Utc};
use drive3::{hyper_rustls, hyper_util, yup_oauth2, DriveHub};
use drive3::{Error, Result};
use google_drive3::api::{About, FileDeleteCall};
use google_drive3::common::Response;
use google_drive3::hyper_rustls::HttpsConnector;
use log::{debug, info};
use mime_guess::{from_path, Mime};
use std::io::{Bytes, Cursor};
use std::path::Path;

pub struct GDClient {
  hub           : DriveHub<HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>>
}

impl GDClient {
  pub async fn new() -> core::result::Result<GDClient, Error> {
    let secret = yup_oauth2::read_application_secret(env::current_dir()?.parent().unwrap().parent().unwrap().join("secret.json").display().to_string())
      .await?;

    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
      secret,
      yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
      .persist_tokens_to_disk(env::current_dir()?.parent().unwrap().parent().unwrap().join("tokenCache.json").display().to_string())
      .build()
      .await?;

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
      .build(
        hyper_rustls::HttpsConnectorBuilder::new()
          .with_native_roots()?
          .https_or_http()
          .enable_http2()
          .build(),
      );

    info!("Google Drive client created");

    Ok(Self{
      hub: DriveHub::new(client, auth)
    })
  }

  pub async fn getFileList(&self) -> Result<Vec<drive3::api::File>>{
    let res = self.hub.files()
      .list()
      .q("trashed = false")
      .param("fields", "files(id, name, createdTime, size, mimeType, parents)")
      .add_scope("https://www.googleapis.com/auth/drive")
      .doit()
      .await?;

    let fileList: Vec<google_drive3::api::File> = res.1.files.unwrap_or_default();
    Ok(fileList)
  }

  pub async fn createClipsFolder(&self) -> core::result::Result<Response, Error> {
    let mut newClipFolder = drive3::api::File::default();
    newClipFolder.name = Some("ZeroCam Clips".into());
    newClipFolder.mime_type = Some("application/vnd.google-apps.folder".into());
    let res = self.hub.files()
      .create(newClipFolder)
      .param("fields", "name, mimeType")
      .add_scope("https://www.googleapis.com/auth/drive")
      .upload(Cursor::new(vec![]), "application/vnd.google-apps.folder".parse().unwrap())
      .await?;
    Ok(res.0)
  }

  pub async fn getAbout(&self) -> Result<(Response, About)>{
    self.hub.about()
      .get()
      .param("fields", "storageQuota")
      .add_scope("https://www.googleapis.com/auth/drive")
      .doit()
      .await
  }

  pub async fn deleteFile(&self, file: drive3::api::File) -> Result<Response> {
    self.hub.files()
      .delete(file.id.unwrap().as_str())
      .add_scope("https://www.googleapis.com/auth/drive")
      .doit()
      .await
  }

  pub async fn uploadFile(&self, filePath: String, fileName:String, parentID: String) -> Result<Response> {
    let mimeType: Mime = from_path(filePath.clone()).first_or_octet_stream();

    let file = drive3::api::File {
      name: Some(fileName),
      mime_type: Some(mimeType.to_string()),
      parents: Some(vec![parentID]),
      ..Default::default()
    };

    let file_content = tokio::fs::read(filePath).await?;

    let res = self.hub.files()
      .create(file)
      .param("fields", "id, name, mimeType, parents")
      .add_scope("https://www.googleapis.com/auth/drive")
      .upload(Cursor::new(file_content), mimeType)
      .await?;
    Ok(res.0)
  }
}
