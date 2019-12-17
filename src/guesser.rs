
use std::fs::DirEntry;
use std::collections::HashMap;
use super::parsers::{parse_show_name, parse_season_number};

type Parser = fn(&String) -> Option<String>;

pub struct Guesser {
    files: Vec<String>,
}

impl Guesser {
    pub fn new(files: &Vec<DirEntry>) -> Guesser {
        let file_names = files.iter()
            .map(|entry| String::from(entry.file_name().to_str().unwrap()))
            .collect();
        
        Guesser {
            files: file_names,
        }
    }
    
    pub fn get_show_name(&self) -> Option<String> {
        self.guess(parse_show_name)
    }
    
    pub fn get_season_number(&self) -> Option<i32> {
        self.guess(parse_season_number).map(|num| num.parse::<i32>().unwrap())
    }
    
    fn guess(&self, parser: Parser) -> Option<String> {
        let mut guesses: HashMap<String, i32> = HashMap::new();
    
        for path in &self.files {
            let res = parser(&path);
            
            if res.is_some() {
                let key = res.unwrap();
                
                match guesses.get(&key) {
                    Some(t) => guesses.insert(key, t + 1),
                    None => guesses.insert(key, 1),
                };
            }
        }
        
        let mut largest = 0;
        let mut found: Option<String> = None;
    
        for (guess, count) in guesses {
            if count > largest {
                largest = count;
                found = Some(guess);
            }
        }
        
        return found;
    }
}