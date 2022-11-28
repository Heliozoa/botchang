use twitch_irc::message::PrivmsgMessage;

#[derive(Debug)]
pub struct Command {
    pub name: CommandName,
    pub args: CommandArgs,
}

impl Command {
    pub fn parse(msg: PrivmsgMessage) -> Option<Self> {
        let text = msg.message_text.as_str();
        let byte = text.bytes().next()?;
        let command_type = CommandType::parse(byte)?;
        let args_start = text.find(' ');
        let command = if let Some(args_start) = args_start {
            &text[1..args_start]
        } else {
            text
        };
        let name = CommandName::parse(command)?;
        Some(Self {
            name,
            args: CommandArgs {
                command_type,
                msg,
                args_start,
            },
        })
    }
}

#[derive(Debug)]
pub enum CommandName {
    Hello,
}

impl CommandName {
    fn parse(s: &str) -> Option<Self> {
        let cmd = match s {
            "hello" => Self::Hello,
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
        if let Some(args_start) = self.args_start {
            &self.msg.message_text[args_start..]
        } else {
            ""
        }
    }
}

#[derive(Debug)]
pub enum CommandType {
    Run,
}

impl CommandType {
    fn parse(s: u8) -> Option<Self> {
        let ct = match s {
            b'!' => Self::Run,
            _ => return None,
        };
        Some(ct)
    }
}
