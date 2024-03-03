use std::{fs, ptr::from_mut, sync::OnceLock};

use anyhow::Result;
use log::{error, info, warn};
use teloxide::prelude::*;

mod command;
use command::validate_enable_command;
mod config;
use config::Config;

static SUPER_USERS: OnceLock<Vec<u64>> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let Config {
        channel_id,
        bot_token,
        super_users,
    } = toml::from_str(&String::from_utf8_lossy(&fs::read("config.toml")?))?;
    let _ = SUPER_USERS.set(super_users);

    let bot = Bot::new(bot_token);
    teloxide::repl(bot, move |bot: Bot, msg: Message| async move {
        let from_chat = &msg.chat;
        info!("received msg from: {}", from_chat.id);
        // this message is not from source channel
        if from_chat.id.0 != channel_id {
            let super_users = SUPER_USERS.get().unwrap();
            if validate_enable_command(&bot, &msg, &from_chat, &super_users)
                .await
                .is_some()
            {
                println!("remember chat: {}", from_chat.id);
            }
            return Ok(());
        }

        let msg_id = msg.id;
        bot.forward_message(ChatId(1982546578), from_chat.id, msg_id)
            .await?;

        Ok(())
    })
    .await;

    Ok(())
}
