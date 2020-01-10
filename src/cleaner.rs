
use std::fs;
use std::io;
use std::path::Path;
use inflector::Inflector;

// @todo Could these rules be regex?
fn parse_rules<S: AsRef<str>>(contents: S) -> Vec<String> {
    contents.as_ref().to_lowercase().split("\n")
        .map(|s| String::from(s).trim())
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
    /// Load a set of rules.
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Cleaner, io::Error> {
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
