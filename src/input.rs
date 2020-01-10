
use rustyline::Editor;
use rustyline::error::ReadlineError;

type Handler = fn(e: ReadlineError) -> ();

pub struct Input {
    rl: Editor<()>,
    handler: Handler,
}

/// Wrapper for readline.
impl Input {
    
    /// A global error handler for all the methods.
    pub fn new(handler: Handler) -> Input {
        Input {
            rl: Editor::new(),
            handler: handler,
        }
    }
    
    /// Ask a question.
    pub fn confirm(&mut self) -> bool {
        loop {
            match self.rl.readline("yes? ") {
                Ok(line) => {
                    // Enforce a legitimate answer.
                    match line.as_ref() {
                        "y" | "yes" => {
                            return true;
                        }
                        "n" | "no" => {
                            return false;
                        }
                        _ => {
                            println!("Type 'yes' or 'no'.");
                        }
                    }
                },
                // Pass off to the global handler.
                Err(err) => (self.handler)(err),
            }
        }
    }
    
    /// Query for text. Must not be empty.
    pub fn text<S: AsRef<str>>(&mut self, prefill: S) -> String {
        loop {
            match self.rl.readline_with_initial(">> ", (prefill.as_ref(), "")) {
                Ok(line) => {
                    // Must not be empty.
                    if !line.is_empty() {
                        return line;
                    }
                    else {
                        println!("Type something?");
                        continue;
                    }
                },
                // Pass off to the global handler.
                Err(err) => (self.handler)(err),
            }
        }
    }
    
    /// Query for a number. Must be a positive integer.
    pub fn number(&mut self, num: u32) -> u32 {
        let prefill = num.to_string();
        
        loop {
            match self.rl.readline_with_initial(">> ", (prefill.as_ref(), "")) {
                Ok(line) => {
                    // Must be a number.
                    match line.parse::<u32>() {
                        Ok(num) => return num,
                        Err(_) => {
                            println!("Not a number, try again.");
                            continue;
                        }
                    }
                },
                Err(err) => (self.handler)(err),
            }
        }
    }
    
    /// Wait for input. Enter or an escape command - don't care.
    pub fn pause(&mut self) {
        println!("\nPress enter to exit.");
        match self.rl.readline("") {
            Ok(_) => (),
            Err(_) => (),
        }
    }
}