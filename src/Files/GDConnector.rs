extern crate hyper;
extern crate google_drive3 as drive3;
use drive3::{Result, Error};
use drive3::{DriveHub, hyper_rustls, hyper_util, yup_oauth2};
use google_drive3::hyper_rustls::HttpsConnector;
use log::{debug, info};
use tokio::fs::File;

pub struct GDClient {
  hub: DriveHub<HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>>
}

impl GDClient {
  pub async fn new() ->  core::result::Result<GDClient, Error>{
    let secret = yup_oauth2::read_application_secret("secret.json")
      .await
      .expect("secret.json");

    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
      secret,
      yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
      .persist_tokens_to_disk("token_cache.json")
      .build()
      .await?;

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
      .build(
        hyper_rustls::HttpsConnectorBuilder::new()
          .with_native_roots()?
          .https_or_http()
          .enable_http1()
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
      .add_scope("https://www.googleapis.com/auth/drive")
      .doit()
      .await?;

    let fileList: Vec<google_drive3::api::File> = res.1.files.unwrap_or_default();

    info!("Got file list");
    debug!("GetFileList {:?}", fileList.iter().map(|f| f.name.clone().unwrap_or_default()).collect::<Vec<String>>());
    Ok(fileList)
  }


}

