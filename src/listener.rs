use std::sync::atomic::spin_loop_hint;
use crate::command::Command;
pub struct Listener {
    
    pub listening: bool,
    pub commands: Vec<Command>,
}

impl Listener {

    // Listener creating
    pub fn new() -> Self {
        
        Listener {
            listening: false,
            commands: Vec::new(),
        }
    }
    
    pub fn from(commands: Vec<Command>) -> Self {
        
        Listener {
            listening: false,
            commands,
        }
    }
    
    // setters
    pub fn set_listening(&mut self, listening: bool) {
        self.listening = listening;
    }
    
    pub fn set_commands(&mut self, commands: Vec<Command>) {
        self.commands = commands;
    }
    
    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }
    
    pub fn add_commands(&mut self, commands: Vec<Command>) {
        self.commands.extend(commands);
    }
    
    pub fn listen(&mut self) {
        self.set_listening(true);
        println!("Listener is now listening for commands.");

        while self.listening {

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line")
            ;

            let input = input.trim();

            if input == "exit" {

                println!("Exiting listener.");
                self.listening = false;
                break;
            } else if input == "list" {

                println!("List of commands:");
                for command in &self.commands {
                    println!("| {}:", command.name);
                    println!("| {}", command.description);
                }
            } else {

                for command in &self.commands {
                    if command.name == input {
                        println!("Executing command: {}", command.name);
                        (command.execute)();
                        continue;
                    }
                }
            }
        }
    }
}