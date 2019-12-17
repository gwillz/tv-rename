
use std::io;
use std::fs;
use std::fmt;
use std::cmp::{self, Ordering};
use std::path::PathBuf;

use super::cleaner::Cleaner;
use super::parsers::{parse_episode_name, parse_episode_number, parse_extension};

pub struct EpisodeFactory<'c,> {
    season: i32,
    show_name: String,
    cleaner: &'c Cleaner,
}

impl<'c> EpisodeFactory<'c> {
    pub fn new(show_name: &String, season: i32, cleaner: &'c Cleaner) -> EpisodeFactory<'c> {
        EpisodeFactory {
            show_name: show_name.clone(),
            season: season,
            cleaner: cleaner,
        }
    }
    
    pub fn create(&self, path: PathBuf) -> Result<Episode, &str> {
        
        // I haven't seen this one fail yet.
        let file_name = match path.file_name() {
            Some(name) => String::from(name.to_str().unwrap()),
            None => return Err("Cannot get file name."),
        };
        
        // Episode numbers must exist.
        let episode_number = match parse_episode_number(&file_name) {
            Some(num) => num,
            None => return Err("Failed to parse episode number."),
        };
        
        // Extensions must exist.
        let extension = match parse_extension(&file_name) {
            Some(num) => num,
            None => return Err("Failed to parse file extension."),
        };
        
        // Episode names can be empty.
        let episode_name = match parse_episode_name(&file_name) {
            Some(name) => self.cleaner.clean(&name),
            None => String::new(),
        };
        
        Ok(Episode {
            path: path,
            season: self.season,
            show_name: self.show_name.clone(),
            episode: episode_number,
            extension: extension,
            name: episode_name,
        })
    }
}

pub struct Episode {
    pub(in crate) path: PathBuf,
    pub(in crate) episode: i32,
    pub(in crate) season: i32,
    pub(in crate) name: String,
    pub(in crate) show_name: String,
    pub(in crate) extension: String,
}

impl Episode {
    pub fn result(&self) -> String {
        if self.name.is_empty() {
            format!("{} S{:02}E{:02}.{}",
                self.show_name,
                self.season,
                self.episode,
                self.extension,
            )
        }
        else {
            format!("{} S{:02}E{:02} - {}.{}",
                self.show_name,
                self.season,
                self.episode,
                self.name,
                self.extension,
            )
        }
    }
    
    pub fn rename(&self) -> io::Result<()> {
        fs::rename(self.path.as_path(), self.path.with_file_name(self.result()))
    }
}

impl fmt::Display for Episode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let original = self.path.file_name().unwrap().to_str().unwrap();
        write!(f, "{} -> {}", original, self.result())
    }
}

impl cmp::Eq for Episode {}

impl cmp::PartialEq for Episode {
    fn eq(&self, other: &Self) -> bool {
        self.season == other.season &&
        self.episode == other.episode
    }
}

impl cmp::PartialOrd for Episode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Episode {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.season == other.season {
            if self.episode == other.episode {
                return Ordering::Equal;
            }
            else if self.episode > other.episode {
                return Ordering::Greater;
            }
            else {
                return Ordering::Less;
            }
        }
        else if self.season > other.season {
            return Ordering::Greater;
        }
        else {
            return Ordering::Less;
        }
    }
}
