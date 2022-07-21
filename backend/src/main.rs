extern crate ws;

use ws::listen;

fn main() {
    println!("Hello, world!");

    listen("0.0.0.0:23849", |sender| {

        move |msg| {
            sender.send("Hello world!")
        }
        
    }).expect("Failed to listen on port 23849.");

}
