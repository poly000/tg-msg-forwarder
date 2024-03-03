use log::error;
use teloxide::{prelude::*, types::Chat, utils::command::BotCommands};

#[derive(BotCommands, PartialEq, Debug)]
#[command(rename_rule = "lowercase")]
enum AdminCommand {
    Enable,
}

pub async fn validate_enable_command(
    bot: &Bot,
    msg: &Message,
    from_chat: &Chat,
    super_users: &[u64],
) -> Option<()> {
    if !from_chat.id.is_group() {
        return None;
    }
    let Some(msg_text) = msg.text() else {
        return None;
    };

    // is the command authorized?
    if !msg.from().is_some_and(|u| super_users.contains(&u.id.0)) {
        return None;
    }

    let bot_me = match bot.get_me().await {
        Ok(me) => me,
        Err(e) => {
            error!("failed to get bot username: {e}");
            return None;
        }
    };
    let bot_username = bot_me.username();
    // is it a enable command?
    let command = AdminCommand::parse(msg_text, bot_username).ok()?;

    if command != AdminCommand::Enable {
        return None;
    }

    Some(())
}
