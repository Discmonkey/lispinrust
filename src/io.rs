use std::io;
use std::io::{Write};

pub struct UserIO {
    prefix: String
}

impl UserIO {

    pub fn new() -> Self {
        UserIO {prefix: "user> ".to_string()}
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = prefix;
    }

    pub fn read_line(&self) -> Option<String> {
        let mut input = String::new();

        let bytes = io::stdin().read_line(&mut input)
            .ok().expect("could not read line");

        if bytes == 0 {
            return None
        }

        // getting rid of newline
        input.pop();

        Some(input)
    }

    pub fn write_line(&self, line: &String) {
        println!("{}", &line);
    }

    pub fn write(&self, line: &String) {
        print!("{}", line);
        // we need to flush since we are not printing a new line,
        // rust buffers std::out output and only flushes on newlines from my understanding
        match io::stdout().flush() {
            _ => ()
        }
    }

    pub fn greet(&self) {
        self.write(&self.prefix)
    }



}