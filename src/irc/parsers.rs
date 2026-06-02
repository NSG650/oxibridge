use crate::{
    broadcast::Source,
    core::{self},
};
use color_eyre::eyre::Result;
use irc::client::prelude::Message;

pub async fn to_core_message(message: &Message) -> Result<core::Message> {
    let author = to_core_author(message)?;

    match &message.command {
        irc::client::prelude::Command::PRIVMSG(_, content) => {
            return Ok(core::Message::new(author, content.clone(), Vec::new(), None, None).await);
        }
        irc::client::prelude::Command::NOTICE(channel, content) => {
            let server_username = format!("Server_notice_for_{}", channel);
            let server_author = core::Author {
                username: server_username.clone(),
                display_name: Some(server_username.clone()),
                avatar: None,
                source: Source::Irc,
            };

            return Ok(
                core::Message::new(server_author, content.clone(), Vec::new(), None, None).await,
            );
        }
        _ => {
            return Err(color_eyre::eyre::eyre!("Failed to parse IRC message"));
        }
    }
}

pub fn to_core_author(message: &irc::client::prelude::Message) -> Result<core::Author> {
    let nickname = message.source_nickname().unwrap_or("Unknown");
    Ok(core::Author {
        username: nickname.to_owned(),
        display_name: Some(nickname.to_owned()),
        avatar: None,
        source: Source::Irc,
    })
}
