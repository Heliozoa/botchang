use std::time::Duration;

use crate::{
    command::{Command, CommandArgs, CommandName},
    Client,
};

pub async fn exec(client: Client, command: Command) {
    let res = match command.name {
        CommandName::Hello => hello(client, command.args).await,
    };
    if let Err(err) = res {
        tracing::error!("{err}")
    }
}

async fn hello(client: Client, args: CommandArgs) -> anyhow::Result<()> {
    client
        .say(
            args.msg.channel_login.clone(),
            format!("<( Hello, {}! )", &args.msg.sender.name),
        )
        .await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    client
        .say(
            args.msg.channel_login.clone(),
            "<( How may I help you today? )".to_string(),
        )
        .await?;
    Ok(())
}
