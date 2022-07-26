use std::sync::mpsc::Sender;

use crate::message::Message;

#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    uuid: String,
    listener: Sender<(String, Message)>,
}

impl User {
    pub fn new(name: &str, uuid: &str, sender: Sender<(String, Message)>) -> User {
        User {
            name: name.to_owned(),
            uuid: uuid.to_owned(),
            listener: sender,
        }
    }

    pub fn send_message(&self, message: Message) {
        self.listener.send((self.uuid.to_owned(), message)).expect("Failed to send message to relay thread. Shutting down.");
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
