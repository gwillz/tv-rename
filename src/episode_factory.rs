
use std::path::{Path, PathBuf};
use std::collections::HashSet;

use super::cleaner::Cleaner;
use super::parsers::{parse_episode_name, parse_episode_number, parse_extension};

use crate::episode::Episode;

/// Factory for creating episode objects.
pub struct EpisodeFactory<'c> {
    season: u32,
    show_name: String,
    cleaner: &'c Cleaner,
    episodes: HashSet<Episode>,
}

impl<'c> EpisodeFactory<'c> {
    
    pub fn new<S: ToString>(show_name: S, season: u32, cleaner: &'c Cleaner) -> EpisodeFactory<'c> {
        EpisodeFactory {
            show_name: show_name.to_string(),
            season: season,
            cleaner: cleaner,
            episodes: HashSet::new(),
        }
    }
    
    /// Create an episode.
    /// Parses the episode name, number and extension from the given path.
    pub fn create<P: AsRef<Path>>(&self, path: P) -> Result<Episode, String> {
        let path = PathBuf::from(path.as_ref());
        
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
    pub fn insert<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
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
        episodes.sort_unstable();
        return episodes;
    }
}

