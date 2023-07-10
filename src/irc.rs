use std::sync::Arc;

use crate::config::AuthConfig;
use crate::message_handler::MessageHandler;
use crate::message::Message;

const TWITCH_WS_URL: &str = "ws://irc-ws.chat.twitch.tv:80";

pub struct Client {
    pub out: ws::Sender,
    config: AuthConfig,
    message_handler: Arc<dyn MessageHandler>,
}


impl Client {
    pub fn run(config: AuthConfig, message_handler: Arc<dyn MessageHandler>) {
        std::thread::Builder::new()
            .spawn(move || {
                ws::connect(TWITCH_WS_URL, move |out| Self {
                    out,
                    config: config.clone(),
                    message_handler: Arc::clone(&message_handler),
                }).expect("Connection error: ")
            }).expect("Can not spawn thread.")
            .join().expect("Can not join to main thread.");
    }
}

impl ws::Handler for Client {
    fn on_open(&mut self, _shake: ws::Handshake) -> ws::Result<()> {
        self.out.send(format!("CAP REQ :{}", self.config.capabilities))?;
        self.out.send(format!("PASS {}", self.config.token))?;
        self.out.send(format!("NICK {}", self.config.username))?;
        self.out.send(format!("JOIN {}", self.config.broadcaster))?;

        Ok(())
    }

    fn on_message(&mut self, message: ws::Message) -> ws::Result<()> {
        let message = message.as_text()?.replace('\u{e0000}', "");

        for msg in message.split("\r\n") {
            let message = msg.trim();

            if message.is_empty() {
                return Ok(());
            }

            if message.starts_with("PING") {
                self.out.send("PONG :tmi.twitch.tv")?;
                return Ok(());
            }

            let message = Message::from_string(message.to_string());
            self.message_handler.dispatch(&self.out, message)?;
        }

        Ok(())
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        // tbh idk how this should work, but i hope it works...
        self.out.send(format!("PART {}", self.config.broadcaster)).expect("Can not send a part message.");
    }
}