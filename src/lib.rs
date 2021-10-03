use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CronDiscordWebhook {
    pub url: String,
    pub schedule: String,
    pub webhook: DiscordWebhook,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DiscordWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
}

impl DiscordWebhook {
    pub fn new() -> Self {
        Self {
            username: None,
            content: None,
        }
    }

    pub fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    pub fn content(&mut self, content: String) -> &mut Self {
        self.content = Some(content);
        self
    }
}