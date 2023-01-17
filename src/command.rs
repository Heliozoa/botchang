use twitch_irc::message::PrivmsgMessage;

#[derive(Debug)]
pub struct Command {
    pub name: CommandName,
    pub args: CommandArgs,
}

impl Command {
    pub fn parse(msg: PrivmsgMessage) -> Option<Self> {
        let text = msg.message_text.as_str();
        let command_type = CommandType::parse(text)?;

        let cmd_and_args = text.get(1..)?;
        let (command, args_start) = if let Some(space) = cmd_and_args.find(' ') {
            (&cmd_and_args[..space], Some(space + 1))
        } else {
            (cmd_and_args, None)
        };
        let name = CommandName::parse(command)?;
        Some(Self {
            name,
            args: CommandArgs {
                command_type,
                msg,
                args_start: args_start.map(|s| s + 1),
            },
        })
    }
}

#[derive(Debug)]
pub enum CommandName {
    Hello,
    Echo,
}

impl CommandName {
    fn parse(s: &str) -> Option<Self> {
        let cmd = match s {
            "hello" => Self::Hello,
            "echo" => Self::Echo,
            _ => return None,
        };
        Some(cmd)
    }
}

#[derive(Debug)]
pub struct CommandArgs {
    pub command_type: CommandType,
    pub msg: PrivmsgMessage,
    args_start: Option<usize>,
}

impl CommandArgs {
    pub fn args(&self) -> &str {
        self.args_start
            .and_then(|a| self.msg.message_text.get(a..))
            .unwrap_or_default()
    }
}

#[derive(Debug)]
pub enum CommandType {
    Run,
}

impl CommandType {
    fn parse(s: &str) -> Option<Self> {
        let ct = match s.bytes().next()? {
            b'!' => Self::Run,
            _ => return None,
        };
        Some(ct)
    }
}
