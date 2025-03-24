#![allow(non_snake_case)]

extern crate google_drive3 as drive3;

use drive3::{hyper_rustls, hyper_util, yup_oauth2, DriveHub};
use std::env;

#[tokio::main]
async fn main() {

  let secret = yup_oauth2::read_application_secret(env::current_dir().unwrap().join("secret.json").display().to_string())
    .await.unwrap();

  let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
    secret,
    yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
  )
    .persist_tokens_to_disk(env::current_dir().unwrap().join("tokenCache.json").display().to_string())
    .build()
    .await.unwrap();

  let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
    .build(
      hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots().unwrap()
        .https_or_http()
        .enable_http2()
        .build()
    );


  let hub = DriveHub::new(client, auth);

  hub.about()
    .get()
    .param("fields", "storageQuota")
    .add_scope("https://www.googleapis.com/auth/drive")
    .doit()
    .await.unwrap();
}
