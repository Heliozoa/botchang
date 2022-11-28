mod command;

use crate::command::{Command, CommandName};
use anyhow::Context;
use std::{env, time::Duration};
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
                tokio::task::spawn(handle_cmd(client.clone(), cmd));
            }
        }
    }
    Ok(())
}

async fn handle_cmd(client: Client, cmd: Command) {
    tracing::info!("Received cmd: {:#?}", cmd);
    let res = match cmd.command {
        CommandName::Hello => hello(client, cmd).await,
    };
    if let Err(err) = res {
        tracing::error!("{err}")
    }
}

async fn hello(client: Client, cmd: Command) -> anyhow::Result<()> {
    client
        .say(
            cmd.msg.channel_login.clone(),
            format!("<( Hello, {}! )", &cmd.msg.sender.name),
        )
        .await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    client
        .say(
            cmd.msg.channel_login.clone(),
            "<( How may I help you today? )".to_string(),
        )
        .await?;
    Ok(())
}
