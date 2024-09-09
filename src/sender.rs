pub mod discord;

use anyhow::{Context, Result};
use std::process::{Command, Output};

pub trait Sender {
    fn new(webhook_url: String, commands: Vec<String>) -> Self;

    fn send(&self, message: &str);

    fn get_commands(&self) -> Vec<String>;

    fn run_commands(&self) -> Result<Output> {
        let commands = self.get_commands();
        let command = commands
            .first()
            .context("At least one command must be provided")?;
        let args = commands.iter().skip(1).collect::<Vec<&String>>();

        Ok(Command::new(command).args(args).output()?)
    }

    async fn start(&self) -> Result<()>;
}

pub fn format_duration(duration: std::time::Duration) -> String {
    let total_seconds = duration.as_secs();
    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}
