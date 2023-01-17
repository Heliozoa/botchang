use crate::{
    command::{Command, CommandArgs, CommandName},
    Client,
};
use std::time::Duration;

pub async fn exec(client: Client, command: Command) {
    let res = match command.name {
        CommandName::Hello => hello(client, command.args).await,
        CommandName::Echo => echo(client, command.args).await,
    };
    if let Err(err) = res {
        tracing::error!("{err}")
    }
}

async fn hello(client: Client, args: CommandArgs) -> anyhow::Result<()> {
    client
        .say(
            args.msg.channel_login.clone(),
            format!("<( Hello, {}! )", args.msg.sender.name),
        )
        .await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    client
        .say(
            args.msg.channel_login,
            "<( How may I help you today? )".to_string(),
        )
        .await?;
    Ok(())
}

async fn echo(client: Client, args: CommandArgs) -> anyhow::Result<()> {
    let reply = args.args().to_string();
    client.say(args.msg.channel_login, reply).await?;
    Ok(())
}
