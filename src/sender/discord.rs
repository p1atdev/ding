use std::{sync::Arc, time::Duration};

use crate::sender::{format_duration, Sender};

use anyhow::Result;
use chrono::Utc;

pub struct DiscordSender {
    client: reqwest::Client,
    webhook_url: String,
    commands: Vec<String>,
}

impl Sender for DiscordSender {
    fn new(webhook_url: String, commands: Vec<String>) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("ding"),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let builder = reqwest::ClientBuilder::new().default_headers(headers);
        let client = builder.build().unwrap();

        DiscordSender {
            client,
            webhook_url,
            commands,
        }
    }

    fn get_commands(&self) -> Vec<String> {
        self.commands.clone()
    }

    fn send(&self, message: &str) {
        println!("Sending message to Discord: {}", message);
    }

    async fn start(&self) -> Result<()> {
        let commands = Arc::new(self.commands.clone());

        let msg = self.create_start_message(&commands.join(" "));
        self.send_embed(&msg).await?;

        let start = std::time::Instant::now();

        match self.run_commands() {
            Ok(output) => {
                let elapsed = start.elapsed();

                let msg = self.create_finish_message(elapsed);
                self.send_embed(&msg).await?;
            }
            Err(e) => {
                let elapsed = start.elapsed();

                let msg = self.create_crash_message(&commands.join(" "), &e.to_string(), elapsed);
                self.send_embed(&msg).await?;
            }
        }

        Ok(())
    }
}

const START_MESSAGE_TEMPLATE: &str = r#"
{
    "content": "",
    "tts": false,
    "embeds": [
        {
            "title": "🚀 Process started",
            "description": "Process has started with the following command 🧨",
            "color": 3917055,
            "fields": [],
            "timestamp": "{TIMESTAMP}"
        },
        {
            "description": "```bash\n{COMMAND}\n```",
            "fields": []
        }
    ],
    "components": [],
    "actions": {},
    "username": "ding"
}
"#;

const CRASH_MESSAGE_TEMPLATE: &str = r#"
{
    "content": "",
    "tts": false,
    "embeds": [
        {
            "title": "💥 Process crashed",
            "description": "Process has crashed for the following reason 😭",
            "timestamp": "{TIMESTAMP}",
            "color": 14553618,
            "fields": [
                {
                    "name": "Elapsed time",
                    "value": "{ELAPSED_TIME}",
                    "inline": true
                }
            ]
        },
        {
            "title": "Crash log",
            "description": "```bash\n{CRASH_LOG}\n```",
            "fields": []
        }
    ],
    "components": [],
    "actions": {},
    "username": "ding"
}
"#;

const FINISH_MESSAGE_TEMPLATE: &str = r#"
{
    "content": "",
    "tts": false,
    "embeds": [
        {
            "title": "🎉 Process finished!",
            "description": "Process has finished successfully ✅",
            "timestamp": "{TIMESTAMP}",
            "color": 4452159,
            "fields": [
                {
                    "name": "Elapsed time",
                    "value": "{ELAPSED_TIME}",
                    "inline": true
                }
            ]
        }
    ],
    "components": [],
    "actions": {},
    "username": "ding"
}
"#;

impl DiscordSender {
    fn timestamp(&self) -> String {
        Utc::now().to_rfc3339()
    }

    fn create_start_message(&self, command: &str) -> String {
        let timestamp = self.timestamp();
        let message = &START_MESSAGE_TEMPLATE
            .replace("{TIMESTAMP}", &timestamp)
            .replace("{COMMAND}", command);

        message.to_string()
    }

    fn create_crash_message(&self, command: &str, log: &str, elapsed_time: Duration) -> String {
        let timestamp = self.timestamp();
        let message = &CRASH_MESSAGE_TEMPLATE
            .replace("{TIMESTAMP}", &timestamp)
            .replace("{COMMAND}", command)
            .replace("{ELAPSED_TIME}", &format_duration(elapsed_time))
            .replace("{CRASH_LOG}", log);

        message.to_string()
    }

    fn create_finish_message(&self, elapsed_time: Duration) -> String {
        let timestamp = self.timestamp();
        let message = &FINISH_MESSAGE_TEMPLATE
            .replace("{TIMESTAMP}", &timestamp)
            .replace("{ELAPSED_TIME}", &format_duration(elapsed_time));

        message.to_string()
    }

    async fn post(&self, body: &str) -> Result<()> {
        let req = self.client.post(&self.webhook_url).body(body.to_string());
        let res = req.send().await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to send message to Discord"))
        }
    }

    async fn send_embed(&self, embed: &str) -> Result<()> {
        self.post(&embed).await?;

        Ok(())
    }
}
