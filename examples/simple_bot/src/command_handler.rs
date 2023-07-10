use std::collections::HashMap;
use twitch_rs::prelude::*;

type CommandHandler = fn(&str, &str, &str) -> String;
const PREFIX: &str = "!";


#[derive(Default)]
pub struct Commands {
    commands: HashMap<String, CommandHandler>
}

impl Commands {
    pub fn add_command(&mut self, command: &str, fun: CommandHandler) {
        self.commands.insert(command.to_string(), fun);
    }
}


impl MessageHandler for Commands {
    fn dispatch(&self, out: &Sender, message: Message) -> HandlerResult<()> {
        if message.command.command_type != CommandType::PRIVMSG { return Ok(()) }

        let content = message.command.content.as_ref().unwrap();
        let space_idx = content.find(' ').unwrap();

        let channel = content[1..space_idx].to_string();
        let author = message.prefix.nick.as_ref().unwrap(); 
        let content = content[space_idx + 2..].to_string();
        
        let space_idx = content.find(' ').unwrap_or(content.len() - 1);
        let command = content[1..space_idx + 1].to_string();

        if !content.starts_with(PREFIX) { return Ok(()) }

        let response = match self.commands.get(&command) {
            Some(fun) => fun(&channel, &author, &content),
            None => return Ok(())
        };

        out.send(format!("PRIVMSG #{} :{}", channel, response))?;
        
        Ok(())
    }
}