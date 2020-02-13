#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;
use log::{debug, info, trace, warn};
use regex::Regex;
use std::collections::HashMap;
use std::env;
use tokio::stream::StreamExt;
use twitchchat::client::Status;
use twitchchat::*;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^!(\S+)(?: (.+))?").unwrap();
}

#[tokio::main]
async fn main() {
    let _ = env_logger::init();
    let (bot_username, oauth_token, channel_name) = get_env_vars();
    let client = Client::new();
    client
        .writer()
        .join(&channel_name)
        .await
        .expect("failed to join");
    trace!("joined {}", channel_name);

    {
        let client = client.clone();
        tokio::task::spawn(async move {
            trace!("running handler");
            let mut privmsgs = client.dispatcher().await.subscribe::<events::Privmsg>();
            trace!("subscribed");
            while let Some(msg) = privmsgs.next().await {
                debug!("[{}] {}: {}", msg.channel, msg.name, msg.data);
                process(&client, &*msg).await;
            }
        });
    }

    let (read, write) = twitchchat::connect_easy(&bot_username, &oauth_token, Secure::UseTls)
        .await
        .expect("failed to connect");
    let rate_limit = rate_limit::RateLimit::full(20, std::time::Duration::from_secs(30));
    match client
        .run_with_user_rate_limit(read, write, rate_limit)
        .await
    {
        Ok(Status::Eof) => println!("EOF"),
        Ok(Status::Canceled) => println!("canceled"),
        Err(err) => eprintln!("error!: {}", err),
    };
}

fn get_env_vars() -> (String, String, String) {
    dotenv().ok();
    let mut vars = env::vars().collect::<HashMap<String, String>>();
    let bot_username = vars
        .remove("BOT_USERNAME")
        .expect("missing BOT_USERNAME environment variable");
    let oauth_token = vars
        .remove("OAUTH_TOKEN")
        .expect("missing OAUTH_TOKEN environment variable");
    let channel_name = vars
        .remove("CHANNEL_NAME")
        .expect("missing CHANNEL_NAME_VAR environment variable");
    (bot_username, oauth_token, channel_name)
}

async fn process(client: &Client, msg: &messages::Privmsg<'_>) {
    if let Some(caps) = RE.captures(&msg.data) {
        let command = &caps[1];
        let args = &caps
            .get(2)
            .map_or(vec![], |m| m.as_str().split_whitespace().collect());
        info!("{}: {}({:?})", msg.name, command, args);
        match (command, args.as_slice()) {
            ("test", []) => {
                client
                    .writer()
                    .privmsg(&msg.channel, "test")
                    .await
                    .expect("failed to send message");
            }
            _ => {
                warn!("no command matched");
            }
        }
    }
}
