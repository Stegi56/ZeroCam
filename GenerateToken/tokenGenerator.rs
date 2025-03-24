
#[tokio::main]
async fn main() {
  let secret = yup_oauth2::read_application_secret(env::current_dir()?.parent().unwrap().parent().unwrap().join("secret.json").display().to_string())
    .await?;

  let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
    secret,
    yup_oauth2::InstalledFlowReturnMethod::Interactive,
  )
    .persist_tokens_to_disk(env::current_dir()?.parent().unwrap().parent().unwrap().join("tokenCache.json").display().to_string())
    .build()
    .await?;

  let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
    .build(
      hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()?
        .https_or_http()
        .enable_http1()
        .build()
    );

  info!("Google Drive client created");

  hub: DriveHub::new(client, auth)
}