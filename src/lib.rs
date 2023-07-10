pub mod message;
pub mod config;
pub mod irc;
pub mod message_handler;

pub mod prelude {
    pub use super::message::{Message, CommandType};
    pub use super::config::AuthConfig;
    pub use super::irc::Client;
    pub use super::message_handler::MessageHandler;
    pub use ws::Result as HandlerResult;
    pub use ws::Sender as Sender;
}