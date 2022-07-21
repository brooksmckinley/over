use std::sync::mpsc::Sender;

use crate::message::Message;

#[derive(Clone)]
pub struct User {
    name: String,
    uuid: String,
    listener: Sender<Message>,
}

impl User {
    pub fn new(name: &str, sender: Sender<Message>) -> User {
        User { 
            name: name.to_owned(), 
            uuid: "asdasd".to_owned(),
            listener: sender,
        }
    }

    pub fn sendMessage(&self, message: Message) {
        self.listener.send(message);
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
