
pub struct Command {
    pub name: String,
    pub description: String,
    pub execute: fn(),
}

impl Command {

    // Command creating
    pub fn new() -> Self {

        Command {
            name: String::new(),
            description: String::new(),
            execute: || {
                println!("Execute is not set. Please configure it first.");
            },
        }
    }

    pub fn from(name: String, description: String, execute: fn()) -> Self {

        Command {
            name,
            description,
            execute,
        }
    }

    // setters
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_execute(&mut self, execute: fn()) {
        self.execute = execute;
    }
}