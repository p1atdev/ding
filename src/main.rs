mod sender;

use anyhow::Result;
use clap::{Parser, Subcommand};
use sender::Sender;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None, allow_external_subcommands = true)]
struct Cli {
    /// The notification provider to send information to
    #[command(subcommand)]
    sender: SenderType,
}

#[derive(Debug, Clone, Subcommand)]
enum SenderType {
    Discord {
        #[arg(long)]
        webhook_url: String,

        commands: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    match args.sender {
        SenderType::Discord {
            webhook_url,
            commands,
        } => {
            let sender = sender::discord::DiscordSender::new(webhook_url, commands);
            sender.start().await?;
        } // _ => {}
    }

    Ok(())
}
