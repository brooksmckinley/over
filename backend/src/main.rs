extern crate ws;

mod channel;
mod errors;
mod user;
mod message;
mod command;

use std::thread;
use std::sync::mpsc;
use channel::Channel;

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

        move |msg| {
            sender.send("Hello world!")
        }

    }).expect("Failed to listen on port 23849.");

}
