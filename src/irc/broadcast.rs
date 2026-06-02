use crate::{
    broadcast::{BroadcastReceiver, MessageEvent, Source},
    config::GroupConfig,
};
use color_eyre::Result;
use serenity::async_trait;
use tracing::*;

use super::IrcBridge;

#[async_trait]
impl BroadcastReceiver for IrcBridge {
    #[instrument(skip_all)]
    async fn receive(&self, group: &GroupConfig, event: &MessageEvent) -> Result<()> {
        let irc_channel = match &group.irc_channel {
            Some(t) => t,
            None => return Ok(()),
        };

        match event {
            MessageEvent::Create(core_msg) => {
                let text = if let Some(reply_author) = &core_msg.reply_author {
                    format!(
                        "{} -> {}: {}",
                        core_msg.author.full_name(Some(0)),
                        reply_author.full_name(Some(0)),
                        core_msg.content
                    )
                } else {
                    format!(
                        "{}: {}",
                        core_msg.author.full_name(Some(0)),
                        core_msg.content
                    )
                };
                self.client.send_privmsg(irc_channel, text)?;
            }

            _ => todo!(),
        }

        Ok(())
    }
    fn get_receiver_source(&self) -> Source {
        Source::Irc
    }
}
