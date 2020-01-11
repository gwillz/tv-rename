
use std::fs;
use std::io;
use std::fmt;
use std::hash;
use std::path::PathBuf;
use std::cmp::{self, Ordering};


/// This represents an old and new paths of an episode.
#[derive(Clone)]
pub struct Episode {
    pub path: PathBuf,
    pub episode: u32,
    pub season: u32,
    pub name: String,
    pub show_name: String,
    pub extension: String,
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
            format!("{} {} - {}.{}",
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


#[cfg(test)]
mod test {
    use super::*;
    
    fn create_episode(episode: u32, season: u32) -> Episode {
        Episode {
            path: PathBuf::from("one/two/three.mp4"),
            episode: episode,
            season: season,
            name: String::from("The One With The Baby Shower"),
            show_name: String::from("Friends"),
            extension: String::from("mp4"),
        }
    }
    
    #[test]
    fn test_episode_filename() {
        let episode = create_episode(20, 8);
        
        let actual = episode.file_name();
        let expected = "Friends S08E20 - The One With The Baby Shower.mp4";
        
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn test_episode_compare() {
        let mut episodes = vec![
            create_episode(20, 8),
            create_episode(10, 8),
            create_episode(30, 4),
        ];
        
        episodes.sort();
        
        assert_eq!(episodes[0].identifier(), "S04E30");
        assert_eq!(episodes[1].identifier(), "S08E10");
        assert_eq!(episodes[2].identifier(), "S08E20");
    }
}