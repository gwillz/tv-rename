
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{self, Write};

use directories::ProjectDirs;

/// Default in-built exclude rules.
pub const EXCLUDE_RULES: [&'static str; 26] = [
    "aac",
    "ac3",
    "hdtv",
    "org",
    "net",
    "com",
    "webrip",
    "480p",
    "576p",
    "720p",
    "1080p",
    "x264",
    "x265",
    "h264",
    "h265",
    "xvid",
    "mvgroup",
    "yify",
    "yts",
    "eztv",
    "mp4",
    "mp3",
    "mkv",
    "dvdrip",
    "bdrip",
    "hd",
];

/// Write the default exclude list to file.
pub fn write_rules<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    match File::create(path) {
        Ok(file) => {
            for rule in &EXCLUDE_RULES {
                if let Err(err) = write!(&file, "{}\n", rule) {
                    return Err(err);
                }
            }
            Ok(())
        },
        Err(err) => Err(err),
    }
}

/// Get the config path.
pub fn get_rules_path() -> Result<PathBuf, &'static str> {
    // @todo Should these be const somewhere?
    match ProjectDirs::from("com", "gwillz", "tv-rename") {
        Some(dirs) => {
            let path = dirs.config_dir().with_file_name("exclude.txt");
            
            // Write a fresh file if it doesn't already exist.
            if !path.exists() {
                if write_rules(&path).is_err() {
                    return Err("Failed to write config file.");
                }
            }
            
            return Ok(path);
        },
        // Rare? I assume?
        None => Err("Failed to find config.")
    }
}
