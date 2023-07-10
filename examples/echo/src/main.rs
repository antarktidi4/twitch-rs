use std::sync::Arc;

use twitch_rs::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;

    let config = AuthConfig::from_env();
    let echo = Echo::default();

    Client::run(config, Arc::new(echo));

    Ok(())
}

#[derive(Default)]
struct Echo;

impl MessageHandler for Echo {
    fn dispatch(&self, out: &Sender, message: Message) -> HandlerResult<()> {
        if message.command.command_type != CommandType::PRIVMSG { return Ok(()) }

        let content = message.command.content.as_ref().unwrap();
        let space_idx = content.find(' ').unwrap();

        let channel = content[1..space_idx].to_string();
        let author = message.prefix.nick.as_ref().unwrap(); 
        let content = content[space_idx + 2..].to_string();
        
        let answer = format!("PRIVMSG #{} :{} sayed {}", channel, author, content);

        out.send(answer)?;
        
        Ok(())
    }
}
