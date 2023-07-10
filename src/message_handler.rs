use crate::message::Message;

pub trait MessageHandler where Self: Send + Sync {
    fn dispatch(&self, out: &ws::Sender, message: Message) -> ws::Result<()>;
}