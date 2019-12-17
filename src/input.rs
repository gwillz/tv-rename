
use rustyline::Editor;
use rustyline::error::ReadlineError;

type Handler = fn(e: ReadlineError) -> ();

pub struct Input {
    rl: Editor<()>,
    handler: Handler,
}

impl Input {
    
    pub fn new(handler: Handler) -> Input {
        Input {
            rl: Editor::new(),
            handler: handler,
        }
    }
    
    pub fn confirm(&mut self) -> bool {
        loop {
            match self.rl.readline("yes? ") {
                Ok(line) => {
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
                Err(err) => (self.handler)(err),
            }
        }
    }
    
    pub fn text(&mut self, prefill: &str) -> String {
        loop {
            match self.rl.readline_with_initial(">> ", (prefill, "")) {
                Ok(line) => {
                    if !line.is_empty() {
                        return line;
                    }
                    else {
                        println!("Type something?");
                        continue;
                    }
                },
                Err(err) => (self.handler)(err),
            }
        }
    }
    
    pub fn number(&mut self, num: i32) -> i32 {
        let prefill = num.to_string();
        
        loop {
            match self.rl.readline_with_initial(">> ", (prefill.as_ref(), "")) {
                Ok(line) => {
                    match line.parse::<i32>() {
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

    pub fn pause(&mut self) {
        println!("\nPress enter to exit.");
        match self.rl.readline("") {
            Ok(_) => (),
            Err(_) => (),
        }
    }
}