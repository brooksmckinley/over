extern crate ws;

mod channel;
mod command;
mod errors;
mod message;
mod user;

use channel::Channel;
use std::sync::mpsc;
use std::thread;

use ws::listen;

fn main() {
    println!("Hello, world!");

    let (command_sender, command_receiver) = mpsc::channel();

    thread::spawn(move || {
        let mut main_channel = Channel::new();
        println!("Initializing channel thread...");
        main_channel.command_loop(command_receiver);
    });

    listen("0.0.0.0:23849", |sender| {
        move |msg| sender.send("Hello world!")
    })
    .expect("Failed to listen on port 23849.");
}
