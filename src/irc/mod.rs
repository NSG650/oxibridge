use crate::{broadcast::Broadcaster, Config};
use color_eyre::Result;
use irc::client::{prelude::Client, ClientStream};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::*;

mod broadcast;
use futures::prelude::*;

pub struct IrcBridge {
    client: Client,
    stream: Arc<Mutex<ClientStream>>,
    broadcaster: Arc<Mutex<Broadcaster>>,
}

impl IrcBridge {
    #[instrument(skip_all)]
    pub async fn new(config: Arc<Config>, broadcaster: Arc<Mutex<Broadcaster>>) -> Result<Self> {
        debug!("Creating IRC bot");

        let irc_config = match &config.shared.irc {
            Some(t) => t,
            None => {
                return Err(color_eyre::eyre::eyre!(
                    "IRC is not configured in shared config"
                ))
            }
        };

        let mut channels = Vec::new();
        for group in &config.groups {
            if let Some(channel) = group.irc_channel.clone() {
                channels.push(channel.to_owned());
            }
        }

        let config = irc::client::prelude::Config {
            nickname: Some(irc_config.nickname.to_owned()),
            server: Some(irc_config.server.to_owned()),
            channels,
            use_tls: Some(irc_config.use_tls),
            port: Some(irc_config.port),
            ..irc::client::data::Config::default()
        };

        let mut client = Client::from_config(config).await?;
        client.identify()?;
        let stream = Arc::new(Mutex::new(client.stream()?));

        Ok(Self {
            client,
            stream,
            broadcaster,
        })
    }

    pub async fn start(&self) {
        while let Ok(Some(message)) = self.stream.lock().await.next().await.transpose() {
            debug!("{}", message);
        }
    }
}
