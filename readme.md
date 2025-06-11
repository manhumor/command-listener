## command-listener
### its a simple command listener without any specifics

#### default commands:
 
- exit -> stops the listener
- list -> lists all commands

### usage:

```rust
use command_listener::{Command, Listener};

fn main() {
    
    println!("Hello, command-listener!");

    // creating a command
    let test_command = Command::from(
        // name
        String::from("test"),
        // description
        String::from("this is a test command"),
        // execute function
        || {
            
            println!("This is a test command!");
        }
    );
    
    // creating a listener
    let mut listener = Listener::new();
    
    // adding the command to the listener
    listener.add_command(test_command);
    
    // listening for commands
    listener.listen();
}
```