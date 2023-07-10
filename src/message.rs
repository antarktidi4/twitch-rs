#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub tags: Vec<Tag>,
    pub prefix: Prefix,
    pub command: Command,
}

impl Message {
    pub fn from_string(mut message: String) -> Self {
        Self {
            tags: Self::parse_tags(&mut message),
            prefix: Self::parse_prefix(&mut message),
            command: Self::parse_command(&message),
        }
    }

    fn parse_tags(message: &mut String) -> Vec<Tag> {
        if !message.starts_with('@') { return Vec::new(); }
        let tags_end_idx = message.find(' ').unwrap();

        let tags = message[1..tags_end_idx]
            .split(';')
            .map(Tag::from_string)
            .collect();

        *message = message[tags_end_idx + 1..].to_string();

        tags
    }

    fn parse_prefix(message: &mut String) -> Prefix {
        let space_idx = message.find(' ').unwrap();
        let prefix = Prefix::from_string(&message[0..space_idx]);
        *message = message[space_idx + 1..].to_string();

        prefix
    }

    fn parse_command(message: &str) -> Command {
        Command::from_string(message)
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tags = self.tags.iter().map(Tag::to_string).collect::<Vec<String>>().join(";");
        if !tags.is_empty() {
            tags = format!("@{} ", tags);
        } 

        let prefix = self.prefix.to_string();
        let command = self.command.to_string();

        write!(f, "{}{} {}\r\n", tags, prefix, command)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    pub name: String,
    pub value: Option<String>
}

impl Tag {
    pub fn from_string(tag_string: &str) -> Self {
        let mut iter = tag_string.split('=');
        let name = iter.next().map(String::from).unwrap();
        let value = iter.next().map(String::from).filter(|v| !v.is_empty());
        
        Self {
            name,
            value,
        }
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.name, self.value.as_ref().unwrap_or(&String::new()))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prefix {
    pub nick: Option<String>,
    pub host: String,
}

impl Prefix {
    pub fn from_string(prefix_string: &str) -> Self {
        if prefix_string.contains('@') {
            let nick = prefix_string[1..prefix_string.find('!').unwrap_or(1)].to_string();
            let host = prefix_string[prefix_string.find('@').unwrap_or(1) + nick.len() + 2..].to_string();

            Self {
                nick: Some(nick),
                host
            }
        } else {
            Self {
                nick: None, 
                host: prefix_string[1..].to_string()
            }
        }
    }
}

impl std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // in irc docs prefix is <servername>!<nickname>@<host>
        // but twitch prefix is <nick?>!<nick?>@<nick?>.<host>
        // i dont fucking know if "nicks" can be different. but i'll left only one field...
        let nick_prefix = if let Some(nick) = self.nick.as_ref() {
            format!("{}!{}@{}.", nick, nick, nick)
        } else {
            String::new()
        };

        write!(f, ":{}{}", nick_prefix, self.host)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub command_type: CommandType,
    pub content: Option<String>,
}

impl Command {
    pub fn from_string(command_string: &str) -> Self {
        let command_type = CommandType::from_string(command_string);
        
        let content = if command_string.starts_with("CAP *") {
            command_string[10..].to_string()
 
        } else if command_string.starts_with("CAP R") {
            command_string[7..].to_string()
        } else {
            command_string[command_string.find(' ').unwrap_or(command_string.len() - 1) + 1..].to_string()
        };

        let content = if content.is_empty() {
            None
        } else {
            Some(content)
        };


        Self {
            command_type,
            content
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.command_type, self.content.as_ref().unwrap_or(&String::new()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommandType {
    JOIN,
    NICK,
    PART,
    PASS,
    PING,
    PONG,
    PRIVMSG,
    CLEARCHAT,
    CLEARMSG,
    GLOBALUSERSTATE,
    HOSTTARGET,
    NOTICE,
    RECONNECT,
    ROOMSTATE,
    USERNOTICE,
    USERSTATE,
    WHISPER,
    CAPREQ,
    CAPACK,
    CAPNAK,
    NUMERIC(String),
}

impl CommandType {
    fn from_string(command_string: &str) -> Self {
        // thats so fucking bad lmao
        match &command_string[..4] {
            "JOIN" => Self::JOIN,
            "NICK" => Self::NICK,
            "PART" => Self::PART,
            "PASS" => Self::PASS,
            "PING" => Self::PING,
            "PONG" => Self::PONG,
            "PRIV" => Self::PRIVMSG,
            "CLEA" => {
                if &command_string[..8] == "CLEARMSG" { Self::CLEARMSG }
                else { Self::CLEARCHAT }
            }
            "GLOB" => Self::GLOBALUSERSTATE,
            "HOST" => Self::HOSTTARGET,
            "NOTI" => Self::NOTICE,
            "RECO" => Self::RECONNECT,
            "ROOM" => Self::ROOMSTATE,
            "USER" => { 
                if &command_string[..9] == "USERSTATE" { Self::USERSTATE }
                else { Self::USERNOTICE }
            }
            "WHIS" => Self::WHISPER,
            "CAP " => {
                if &command_string[..7] == "CAP REQ" { Self::CAPREQ }
                else if &command_string[..7] == "CAP * A" { Self::CAPACK }
                else { Self::CAPNAK }
            }
            _ => Self::NUMERIC(command_string[..3].to_string())
        }
    }
}

impl std::fmt::Display for CommandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cmd = match self {
            CommandType::JOIN => "JOIN",
            CommandType::NICK => "NICK",
            CommandType::PART => "PART",
            CommandType::PASS => "PASS",
            CommandType::PING => "PING",
            CommandType::PONG => "PONG",
            CommandType::PRIVMSG => "PRIVMSG",
            CommandType::CLEARCHAT => "CLEARCHAT",
            CommandType::CLEARMSG => "CLEARMSG",
            CommandType::GLOBALUSERSTATE => "GLOBALUSERSTATE",
            CommandType::HOSTTARGET => "HOSTTARGET",
            CommandType::NOTICE => "NOTICE",
            CommandType::RECONNECT => "RECONNECT",
            CommandType::ROOMSTATE => "ROOMSTATE",
            CommandType::USERNOTICE => "USERNOTICE",
            CommandType::USERSTATE => "USERSTATE",
            CommandType::WHISPER => "WHISPER",
            CommandType::CAPREQ => "CAP REQ",
            CommandType::CAPACK => "CAP * ACK",
            CommandType::CAPNAK => "CAP * NAK",
            CommandType::NUMERIC(num) => num,
        };

        write!(f, "{}", cmd)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn roll_test() {
        let messages = vec![
            "@room-id=12345678;target-user-id=87654321;tmi-sent-ts=1642715756806 :tmi.twitch.tv CLEARCHAT #dallas :ronni\r\n",
            "@room-id=12345678;tmi-sent-ts=1642715695392 :tmi.twitch.tv CLEARCHAT #dallas\r\n",
            "@ban-duration=350;room-id=12345678;target-user-id=87654321;tmi-sent-ts=1642719320727 :tmi.twitch.tv CLEARCHAT #dallas :ronni\r\n",
            "@msg-id=delete_message_success :tmi.twitch.tv NOTICE #bar :The message from foo is now deleted.\r\n",
            "@badge-info=;badges=turbo/1;color=#0D4200;display-name=ronni;emotes=25:0-4,12-16/1902:6-10;id=b34ccfc7-4977-403a-8a94-33c6bac34fb8;mod=0;room-id=1337;subscriber=0;tmi-sent-ts=1507246572675;turbo=1;user-id=1337;user-type=global_mod :ronni!ronni@ronni.tmi.twitch.tv PRIVMSG #ronni :Kappa Keepo Kappa\r\n",
            "@emote-only=0;followers-only=0;r9k=0;slow=0;subs-only=0 :tmi.twitch.tv ROOMSTATE #dallas\r\n",
            "@badges=staff/1,bits-charity/1;color=#8A2BE2;display-name=PetsgomOO;emotes=;message-id=306;thread-id=12345678_87654321;turbo=0;user-id=87654321;user-type=staff :petsgomoo!petsgomoo@petsgomoo.tmi.twitch.tv WHISPER foo :hello\r\n",
        
            ":tmi.twitch.tv CLEARCHAT #dallas :ronni\r\n",
            ":tmi.twitch.tv CLEARCHAT #dallas\r\n",
            ":tmi.twitch.tv CLEARCHAT #dallas :ronni\r\n",
            ":petsgomoo!petsgomoo@petsgomoo.tmi.twitch.tv WHISPER foo :hello\r\n",
        ];
       
        for message in messages {
            let parsed = super::Message::from_string(message.trim().to_string()).to_string();
            assert_eq!(message, parsed);
        }
    }
}
