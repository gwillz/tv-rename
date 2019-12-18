
use std::io;
use std::fs;
use std::fmt;
use std::hash;
use std::clone::Clone;
use std::cmp::{self, Ordering};
use std::path::PathBuf;
use std::collections::HashSet;

use super::cleaner::Cleaner;
use super::parsers::{parse_episode_name, parse_episode_number, parse_extension};

/// Factory for creating episode objects.
pub struct EpisodeFactory<'c,> {
    season: i32,
    show_name: String,
    cleaner: &'c Cleaner,
    episodes: HashSet<Episode>,
}

impl<'c> EpisodeFactory<'c> {
    
    pub fn new(show_name: &String, season: i32, cleaner: &'c Cleaner) -> EpisodeFactory<'c> {
        EpisodeFactory {
            show_name: show_name.clone(),
            season: season,
            cleaner: cleaner,
            episodes: HashSet::new(),
        }
    }
    
    /// Create an episode.
    /// Parses the episode name, number and extension from the given path.
    pub fn create(&self, path: PathBuf) -> Result<Episode, String> {
        
        // I haven't seen this one fail yet.
        let file_name = match path.file_name() {
            Some(name) => String::from(name.to_str().unwrap()),
            None => return Err(String::from("Cannot get file name.")),
        };
        
        // Episode numbers must exist.
        let episode_number = match parse_episode_number(&file_name) {
            Some(num) => num,
            None => return Err(String::from("Failed to parse episode number.")),
        };
        
        // Extensions must exist.
        let extension = match parse_extension(&file_name) {
            Some(num) => num,
            None => return Err(String::from("Failed to parse file extension.")),
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
    
    /// Insert an episode.
    pub fn insert(&mut self, path: PathBuf) -> Result<(), String> {
        match self.create(path) {
            Ok(episode) => {
                if !self.episodes.insert(episode.clone()) {
                    Err(format!("Duplicate episode {}", episode.identifier()))
                }
                else {
                    Ok(())
                }
            },
            Err(err) => Err(err),
        }
    }
    
    /// Get a sorted collection of all episodes.
    pub fn get_all(&self) -> Vec<&Episode> {
        let mut episodes: Vec<&Episode> = self.episodes.iter().collect();
        episodes.sort();
        return episodes;
    }
}

/// This represents an old and new paths of an episode.
#[derive(Clone)]
pub struct Episode {
    pub(in crate) path: PathBuf,
    pub(in crate) episode: i32,
    pub(in crate) season: i32,
    pub(in crate) name: String,
    pub(in crate) show_name: String,
    pub(in crate) extension: String,
}

impl Episode {
    
    /// The unique identifier for an episode.
    pub fn identifier(&self) -> String {
        format!("S{:02}E{:02}", self.season, self.episode)
    }
    
    /// The new file name for an episode, created from parsed parts.
    pub fn file_name(&self) -> String {
        if self.name.is_empty() {
            format!("{} {}.{}", 
                self.show_name,
                self.identifier(),
                self.extension,
            )
        }
        else {
            format!("{} {}{}.{}",
            self.show_name,
            self.identifier(),
            self.name,
            self.extension,
            )
        }
    }
    
    /// Rename the episode file.
    pub fn rename(&self) -> io::Result<()> {
        fs::rename(self.path.as_path(), self.path.with_file_name(self.file_name()))
    }
}

impl fmt::Display for Episode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} -> \"{}\"",
            self.path.file_name().unwrap(),
            self.file_name(),
        )
    }
}

impl hash::Hash for Episode {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.season.hash(state);
        self.episode.hash(state);
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
        // @todo Would it better to just cmp() the identifier() ?
        if self.season == other.season {
            if self.episode == other.episode {
                Ordering::Equal
            }
            else if self.episode > other.episode {
                Ordering::Greater
            }
            else {
                Ordering::Less
            }
        }
        else if self.season > other.season {
            Ordering::Greater
        }
        else {
            Ordering::Less
        }
    }
}
