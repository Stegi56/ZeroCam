use std::env;
use std::error::Error;
use std::sync::Arc;
use teloxide::{prelude::*, utils::command::BotCommands};
use crate::Config::ConfigFile;
use crate::Config;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
  #[command(description = "list of commands.")]
  Start,
  #[command(description = "clip.")]
  Clip,
  #[command(description = "get stream url.")]
  Stream,
}


pub async fn newBot() -> Result<(), Box<dyn Error>> {
  let key = Config::getConfig().await?.telegram_key;
  let bot = Bot::new(key);
  Command::repl(bot, answer).await;
  Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
  let clipScheduler = Arc::new(zerocam_lib::ClipScheduler::new().await);
  match cmd {
    Command::Start => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
    Command::Clip => {
      bot.send_message(msg.chat.id, "Attempting to make a clip...").await?;
      if clipScheduler.scheduleClip().await
        .is_ok() { bot.send_message(msg.chat.id, "Clip successful, should be visible in google drive soon...").await?
        } else { bot.send_message(msg.chat.id, "Already in progress - try again later").await? }
    },
    Command::Stream => {
      let config:ConfigFile = Config::getConfig().await.unwrap();
      let streamUrl = config.internet_stream_output.url;
      let username = config.internet_stream_output.username;
      bot.send_message(msg.chat.id, format!("Click link to view stream: {} \nUsername: {}", streamUrl, username)).await?
    },
  };

  Ok(())
}

