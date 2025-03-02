use Camera::ClipScheduler::ClipScheduler;

use std::env;
use std::error::Error;
use std::sync::Arc;
use teloxide::{prelude::*, utils::command::BotCommands};
use crate::Camera;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
  #[command(description = "list of commands.")]
  Start,
  #[command(description = "clip.")]
  Clip,
}


pub async fn newBot() -> Result<(), Box<dyn Error>> {
  let botKeyPath = env::current_dir()?.parent().unwrap().parent().unwrap().join("telegramKey.txt").display().to_string();
  let bot = Bot::new(std::fs::read_to_string(botKeyPath)?);
  Command::repl(bot, answer).await;

  Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
  let clipScheduler = Arc::new(zerocam_lib::ClipScheduler::new());
  match cmd {
    Command::Start => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
    Command::Clip => {
      bot.send_message(msg.chat.id, "Attempting to make a clip...").await?;
      if clipScheduler.scheduleClip().await.is_ok() { bot.send_message(msg.chat.id, "Clip successful, should be visible in google drive soon...").await?
      } else { bot.send_message(msg.chat.id, "Failed to make clip").await?}
    }
  };

  Ok(())
}

