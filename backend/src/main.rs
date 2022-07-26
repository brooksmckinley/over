extern crate ws;

mod channel;
mod command;
mod errors;
mod message;
mod user;

use channel::Channel;
use message::Message;
use std::sync::mpsc;
use std::thread;

use ws::{listen, Sender};

use crate::user::User;

fn main() {
    println!("Hello, world!");

    let (command_sender, command_receiver) = mpsc::channel();

    // Main thread
    thread::spawn(move || {
        let mut main_channel = Channel::new();
        println!("Initializing channel thread...");
        main_channel.command_loop(command_receiver);
    });

    let (message_sender, message_receiver) = mpsc::channel();

    // Message relay thread
    // This thread takes messages from the chanel, translates them, and sends them to the appropriate socket connection
    thread::spawn(move || {
        loop {
            let message: Message = message_receiver.recv().expect("Unable to receive message from relay thread.");

            
        }
    });

    listen("0.0.0.0:23849", |sender| {
        move |msg| {
            sender.send("Hello world!")
        }
    })
    .expect("Failed to listen on port 23849.");
}

enum RelayMessage {
    NewConnection((Sender, String)),
    Message(Message),
}