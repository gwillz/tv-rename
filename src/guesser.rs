
use std::fs::DirEntry;
use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

use super::parsers::{parse_show_name, parse_season_number};

type Parser<R> = fn(&String) -> Option<R>;

/// Show/season guesser.
/// This parses the season/show from each file and chooses the most frequent.
pub struct Guesser {
    files: Vec<String>,
}

impl Guesser {
    
    pub fn new(files: &Vec<DirEntry>) -> Guesser {
        // Get just the file names.
        let file_names = files.iter()
            .map(|entry| String::from(entry.file_name().to_str().unwrap()))
            .collect();
        
        Guesser {
            files: file_names,
        }
    }
    
    /// Get the most likely show name.
    pub fn get_show_name(&self) -> Option<String> {
        self.guess(parse_show_name)
    }
    
    /// Get the most likely season number.
    pub fn get_season_number(&self) -> Option<i32> {
        self.guess(parse_season_number)
    }
    
    /// Internal guesser loop.
    fn guess<R: Hash + Eq>(&self, parser: Parser<R>) -> Option<R> {
        let mut guesses: HashMap<R, i32> = HashMap::new();
    
        // Gather up all the possible values.
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
        let mut found: Option<R> = None;
        
        // Find the most common and return that.
        for (guess, count) in guesses {
            if count > largest {
                largest = count;
                found = Some(guess);
            }
        }
        
        return found;
    }
}