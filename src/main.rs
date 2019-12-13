#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use twitchchat::commands;
use twitchchat::*;

const BOT_USERNAME_VAR: &str = "BOT_USERNAME";
const OAUTH_TOKEN_VAR: &str = "OAUTH_TOKEN";
const CHANNEL_NAME_VAR: &str = "CHANNEL_NAME";

lazy_static! {
    static ref RE: Regex = Regex::new(r"^!(\S+)(?: (.+))?").unwrap();
}

fn main() {
    dotenv().ok();

    let vars = env::vars().collect::<HashMap<String, String>>();
    let bot_username = vars
        .get(BOT_USERNAME_VAR)
        .expect("missing BOT_USERNAME environment variable");
    let oauth_token = vars
        .get(OAUTH_TOKEN_VAR)
        .expect("missing OAUTH_TOKEN environment variable");
    let channel_name = vars
        .get(CHANNEL_NAME_VAR)
        .expect("missing CHANNEL_NAME_VAR environment variable");

    let client = twitchchat::connect_easy(bot_username, oauth_token)
        .expect("failed to connect")
        .filter::<commands::PrivMsg>();

    let botchang = BotChang {
        channel: channel_name.to_string(),
        writer: client.writer(),
    };

    for event in client {
        match event {
            Event::Message(Message::PrivMsg(msg)) => {
                botchang.handle(msg);
            }
            Event::IrcReady(_) | Event::TwitchReady(_) => {
                println!("ready, joining {}", channel_name);
                botchang
                    .writer
                    .join(channel_name)
                    .expect("failed to join channel");
            }
            Event::Message(_message) => println!("ms {:?}", _message),
            Event::Error(_error) => println!("er {:?}", _error),
        }
    }
}

struct BotChang {
    channel: String,
    writer: Writer,
}

impl BotChang {
    fn handle(&self, msg: commands::PrivMsg) {
        let user = msg.user();
        let message = msg.message();
        let cap = RE.captures(message);
        if let Some(cap) = cap {
            let cmd = &cap[1];
            let args = &cap
                .get(2)
                .map(|args| args.as_str().split(" ").collect::<Vec<_>>())
                .unwrap_or(vec![]);
            match (cmd, args.as_slice()) {
                (_, _) => self.command(&cmd, &args),
            }
        } else {
            self.echo(&user, &message);
        }
    }

    fn command(&self, cmd: &str, args: &[&str]) {
        let msg = format!("command {} with {:?}", cmd, args);
        self.send(&msg);
    }

    fn echo(&self, user: &str, msg: &str) {
        let msg = format!("echo {}: {}", user, msg);
        self.send(&msg);
    }

    fn send(&self, msg: &str) {
        self.writer.send(&self.channel, msg).unwrap();
    }
}
