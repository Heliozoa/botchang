mod command;
mod commands;

use crate::command::Command;
use anyhow::Context;
use std::env;
use twitch_irc::{
    login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport,
    TwitchIRCClient,
};

type Client = TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().init();

    let bot_username =
        env::var("BOT_USERNAME").context("missing BOT_USERNAME environment variable")?;
    let oauth_token =
        env::var("OAUTH_TOKEN").context("missing OAUTH_TOKEN environment variable")?;
    let channel = env::var("CHANNEL_NAME").context("missing CHANNEL_NAME environment variable")?;

    let config =
        ClientConfig::new_simple(StaticLoginCredentials::new(bot_username, Some(oauth_token)));
    let (mut incoming_messages, client) = Client::new(config);
    client.join(channel).unwrap();

    while let Some(message) = incoming_messages.recv().await {
        if let ServerMessage::Privmsg(msg) = message {
            if let Some(cmd) = Command::parse(msg) {
                tokio::task::spawn(commands::exec(client.clone(), cmd));
            }
        }
    }
    Ok(())
}
