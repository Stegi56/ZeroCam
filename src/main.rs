#![allow(non_snake_case)]
mod Files;

use env_logger;

#[tokio::main]
async fn main(){
  env_logger::init();

  let gdController = Files::GDController::GDController::new().await.unwrap();
  gdController.uploadClipsAndClearLocal().await;
}
