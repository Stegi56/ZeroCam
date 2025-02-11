#![allow(non_snake_case)]
mod Files;

use env_logger;

#[tokio::main]
async fn main(){
  env_logger::init();

  let gdClient = Files::GDConnector::GDClient::new().await.unwrap();
  let fileList = gdClient.getFileList().await.unwrap();
  println!("GD FileList {:?}", fileList.iter().map(|f| f.name.clone().unwrap_or_default()).collect::<Vec<String>>())
}