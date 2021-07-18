use anyhow::Context;
use std::env;
use std::time::Duration;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::{PrivmsgMessage, ServerMessage};
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

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

    let clone = client.clone();
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            if let ServerMessage::Privmsg(msg) = message {
                handle_priv(clone.clone(), msg).await;
            }
        }
    });

    client.join(channel);

    join_handle.await.unwrap();
    Ok(())
}

async fn handle_priv(client: Client, msg: PrivmsgMessage) {
    tracing::info!("Received message: {:#?}", msg);
    if msg.message_text.starts_with("!hello") {
        tokio::spawn(hello(client.clone(), msg));
    }
}

async fn hello(client: Client, msg: PrivmsgMessage) -> anyhow::Result<()> {
    client
        .say(
            msg.channel_login.clone(),
            format!("<( Hello, {}! )", &msg.sender.name),
        )
        .await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    client
        .say(
            msg.channel_login.clone(),
            "<( How may I help you today? )".to_string(),
        )
        .await?;
    Ok(())
}
