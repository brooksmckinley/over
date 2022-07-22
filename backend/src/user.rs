use std::sync::mpsc::{Sender, SendError};

use crate::message::Message;

#[derive(Clone, Debug)]
pub struct User {
    name: String,
    listener: Sender<Message>,
}

impl User {
    pub fn new(name: &str, sender: Sender<Message>) -> User {
        User { 
            name: name.to_owned(), 
            listener: sender,
        }
    }

    pub fn sendMessage(&self, message: Message) -> Result<(), SendError<Message>>{
        self.listener.send(message)?;
        Ok(())
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
