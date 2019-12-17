
use std::fs::File;
use std::path::Path;
use std::io::{self, Write};

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

pub fn write_exclude<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
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
