
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


#[cfg(test)]
mod test {
    use super::*;
    use crate::cleaner::Cleaner;
    
    #[test]
    fn test_create() {
        let cleaner = Cleaner::new(vec!["lol", "ftw"]);
        let factory = EpisodeFactory::new("Friends", 1, &cleaner);
        
        let episode = factory.create(PathBuf::from("friends.1x01.the.one.[ftw]-LOL.mp4"));
        
        let actual = episode.unwrap().file_name();
        let expected = "Friends S01E01 - The One.mp4";
        
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn test_insert() {
        let cleaner = Cleaner::new(vec!["lol", "ftw"]);
        let mut factory = EpisodeFactory::new("friends", 1, &cleaner);
        
        factory.insert(PathBuf::from("friends.1x03.mp4")).unwrap();
        factory.insert(PathBuf::from("friends.1x01.mp4")).unwrap();
        factory.insert(PathBuf::from("friends.1x02.mp4")).unwrap();
        
        let sorted = factory.get_all();
        
        assert_eq!(sorted[0].identifier(), "S01E01");
        assert_eq!(sorted[1].identifier(), "S01E02");
        assert_eq!(sorted[2].identifier(), "S01E03");
    }
}
