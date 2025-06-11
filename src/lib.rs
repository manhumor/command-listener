mod command;
mod listener;

pub use crate::{command::Command, listener::Listener};

pub fn credits() {
    println!("This is a simple command listener written in Rust.");
    println!("It allows you to create commands and listen for them.");
    println!("You can add commands, set their names, descriptions, and execute functions.");
    println!("Commands can be executed by typing their name in the listener.");
    println!("Type 'exit' to stop the listener.");
    println!("Type 'list' to see the list of commands.");
    println!();
    println!("Author: man_humor");
}