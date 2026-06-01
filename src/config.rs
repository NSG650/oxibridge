use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub shared: SharedConfig,
    pub groups: Vec<GroupConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedConfig {
    pub discord_token: Option<String>,
    pub telegram_token: Option<String>,
    pub r2: Option<R2Config>,
    pub irc: Option<IrcConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IrcConfig {
    pub server: String,
    pub port: u16,
    pub nickname: String,
    pub use_tls: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct R2Config {
    pub bucket_name: String,
    pub account_id: String,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupConfig {
    pub telegram_chat: Option<i64>,
    pub discord: Option<GroupDiscordConfig>,
    pub irc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupDiscordConfig {
    pub channel: u64,
    pub webhook: String,
}
