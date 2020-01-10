
use std::fs;
use std::io;
use std::path::Path;
use inflector::Inflector;

/// Clean strings.
/// - Removes strings provided by an 'exclude' file.
/// - Replaces separators with white space.
/// - Formats in 'Title Case'.
pub struct Cleaner {
    rules: Vec<String>,
}

impl Cleaner {
    
    fn new() -> Cleaner {
        Cleaner {
            rules: Vec::new(),
        }
    }
    
    fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<(), io::Error> {
        fs::read_to_string(path)
            .map(|contents| {
                self.rules = contents.split("\n")
                    .map(|s| String::from(s))
                    .collect();
            })
    }
    
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Cleaner, io::Error> {
        let mut cleaner = Cleaner::new();
        cleaner.load(path).map(|_| cleaner)
    }
    
    /// Clean this text.
    pub fn clean(&self, text: &String) -> String {
        let mut working = text.to_lowercase();
        
        for rule in &self.rules {
            working = working.replace(rule.as_str(), "");
        }
        
        return working.to_title_case();
    }
    
    pub fn size(&self) -> usize {
        self.rules.len()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test() {
        let cleaner = Cleaner {
            rules: vec![
                String::from("fov"),
                String::from("mtb"),
                String::from("h264"),
                String::from("lol"),
                String::from("hdtv"),
                String::from("ac3"),
            ]
        };
        
        let actual = cleaner.clean("yep.okay.SURe[H264][AC3]-LOL");
        let expected = "Yep Okay Sure";
        
        assert_eq!(expected, actual);
    }
}
