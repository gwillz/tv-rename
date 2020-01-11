
use std::fs;
use std::io;
use std::path::Path;
use inflector::Inflector;

// @todo Could these rules be regex?
fn parse_rules<S: AsRef<str>>(contents: S) -> Vec<String> {
    contents.as_ref().to_lowercase().split("\n")
        .map(|s| String::from(s.trim()))
        .collect()
}

/// Clean strings.
/// - Removes strings provided by an 'exclude' file.
/// - Replaces separators with white space.
/// - Formats in 'Title Case'.
pub struct Cleaner {
    pub(in crate) rules: Vec<String>,
}

impl Cleaner {
    /// For testing.
    pub fn new<S: ToString>(rules: Vec<S>) -> Cleaner {
        Cleaner {
            rules: rules.iter().map(|rule| rule.to_string()).collect()
        }
    }
    
    /// Load a set of rules.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Cleaner, io::Error> {
        fs::read_to_string(path).map(|contents| {
            Cleaner {
                rules: parse_rules(contents)
            }
        })
    }
    
    /// Clean this text.
    pub fn clean<T: AsRef<str>>(&self, text: T) -> String {
        let mut working = text.as_ref().to_lowercase();
        
        for rule in &self.rules {
            working = working.replace(rule.as_str(), "");
        }
        
        // @todo How could we do non-caps for 'with' 'to' 'the'?
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
    fn test_clean() {
        let cleaner = Cleaner::new(vec!["lol", "ac3", "h264", "hdtv"]);
        
        assert_eq!(4, cleaner.size());
        
        let actual = cleaner.clean("yep.okay.SURe[H264][AC3]-LOL");
        let expected = "Yep Okay Sure";
        
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn test_load() {
        let cleaner = Cleaner::load(Path::new("test/exclude.txt")).unwrap();
        
        assert_eq!(27, cleaner.size());
    }
}
