use std::sync::Arc;
use twitch_rs::prelude::*;

mod command_handler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;

    let config = AuthConfig::from_env();
    let mut commands = command_handler::Commands::default();

    commands.add_command("hello", hello);

    Client::run(config, Arc::new(commands));

    Ok(())
}

fn hello(channel: &str, author: &str, content: &str) -> String {
    format!("Hello, {}", author)
}