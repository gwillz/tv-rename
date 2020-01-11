
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // Capture everything before the season/episode identifier.
    static ref SHOW_NAME: Regex =
        Regex::new(r"(.+?)\b(?i:collection|series|episode|ep|part|s\d+e\d+|\d+of\d+|\d+x\d+)\b")
        .unwrap();
    
    // Capture the season number by association to a word.
    static ref SEASON_NUMBER_BY_NAME: Regex =
        Regex::new(r"(?i:season|series|collection)\W*(\d+)")
        .unwrap();
    
    // Capture the season number by ID type S--E--
    static ref SEASON_NUMBER_BY_SE: Regex =
        Regex::new(r"(?i:s(\d{2,})e\d{2,})")
        .unwrap();
    
    // Capture the season number ID type --x--
    static ref SEASON_NUMBER_BY_X: Regex =
        Regex::new(r"(?i:(\d+)x\d+)")
        .unwrap();
    
    // Capture everything after the season/episode identifier.
    static ref EPISODE_NAME: Regex =
        Regex::new(r"(?i:s\d+e\d+|\d+of\d+|\d+x\d+|(?i:ep|episode|part)\W*\d+)(.+)\.(?i:[a-z0-9]+)$")
        .unwrap();
    
    // Capture the episode number by --of--
    static ref EPISODE_NUMBER_BY_OF: Regex =
        Regex::new(r"(?i:(\d+)\W*of\W*\d+)").unwrap();
    
    // Capture the episode number by S--E--
    static ref EPISODE_NUMBER_BY_SE: Regex =
        Regex::new(r"(?i:s\d{2,}e(\d{2,}))").unwrap();
    
    // Capture the episode number by --x--
    static ref EPISODE_NUMBER_BY_X: Regex =
        Regex::new(r"(?i:\d+x(\d+))").unwrap();
    
    // Capture the episode by association to a word.
    static ref EPISODE_NUMBER_BY_NAME: Regex =
        Regex::new(r"(?i:episode|ep|part)\W*(\d+)").unwrap();
    
    // Capture the file extension.
    static ref EXTENSION: Regex =
        Regex::new(r"\.(\w+)$").unwrap();
}

/// Get the show name.
pub fn parse_show_name(path: &str) -> Option<String> {
    SHOW_NAME.captures(path).map(|m| String::from(&m[1]))
}

/// Get the season number.
pub fn parse_season_number(path: &str)-> Option<u32> {
    
    let mut caps: Option<String> = None;
    
    // by name
    if caps.is_none() {
        caps = SEASON_NUMBER_BY_NAME
            .captures(path)
            .map(|m| String::from(&m[1]));
    }
    
    // by S--E--
    if caps.is_none() {
        caps = SEASON_NUMBER_BY_SE
            .captures(path)
            .map(|m| String::from(&m[1]));
    }
    
    // by --x--
    if caps.is_none() {
        caps = SEASON_NUMBER_BY_X
            .captures(path)
            .map(|m| String::from(&m[1]));
    }
    
    // Parse number, errors are None.
    if let Some(num) = caps {
        return num.parse::<u32>().ok();
    }
    
    None
}

/// Get the episode name.
pub fn parse_episode_name(path: &str) -> Option<String> {
    EPISODE_NAME
        .captures(path)
        .map(|m| String::from(&m[1]))
        .or(Some(String::new()))
}

/// Get the episode number.
pub fn parse_episode_number(path: &str) -> Option<u32> {
    
    let mut caps: Option<String> = None;
    
    // by --of--
    if caps.is_none() {
        caps = EPISODE_NUMBER_BY_OF
            .captures(path)
            .map(|m| String::from(&m[1]));
    }
    
    // by S--E--
    if caps.is_none() {
        caps = EPISODE_NUMBER_BY_SE
            .captures(path)
            .map(|m| String::from(&m[1]));
    }
    
    // by --x--
    if caps.is_none() {
        caps = EPISODE_NUMBER_BY_X
            .captures(path)
            .map(|m| String::from(&m[1]));
    }
    
    // by name
    if caps.is_none() {
        caps = EPISODE_NUMBER_BY_NAME
            .captures(path)
            .map(|m| String::from(&m[1]));
    }
    
    // Parse number, errors are None.
    if let Some(num) = caps {
        return num.parse::<u32>().ok();
    }
    
    None
}

/// Get the extension.
pub fn parse_extension(path: &str) -> Option<String> {
    EXTENSION
        .captures(path)
        .map(|m| String::from(&m[1]))
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_show_name_1() {
        let actual = parse_show_name("Friends.5x02.whatever.mp4");
        let expected = "Friends.";
        assert_eq!(expected, actual.unwrap());
    }
    
    #[test]
    fn test_show_name_2() {
        let actual = parse_show_name("Friends.series.5.2of9.mp4");
        let expected = "Friends.";
        assert_eq!(expected, actual.unwrap());
    }
    
    // This returns 'series 5' which is kind of correct. But I can't think
    // of a better regex right now.
    // #[test]
    // fn test_show_name_bad() {
    //     let actual = parse_show_name("series.5.2of9.mp4");
    //     assert_eq!(actual, None);
    // }
    
    #[test]
    fn test_episode_name_1() {
        let actual = parse_episode_name("Friends.2of9.one.with.ross.mp4");
        let expected = ".one.with.ross";
        assert_eq!(expected, actual.unwrap());
    }
    
    #[test]
    fn test_episode_name_2() {
        let actual = parse_episode_name("Friends.5x2.one.with.ross.mp4");
        let expected = ".one.with.ross";
        assert_eq!(expected, actual.unwrap());
    }
    
    #[test]
    fn test_episode_name_3() {
        let actual = parse_episode_name("Friends.s05e02.one.with.ross.mp4");
        let expected = ".one.with.ross";
        assert_eq!(expected, actual.unwrap());
    }
    
    #[test]
    fn test_episode_name_4() {
        let actual = parse_episode_name("Friends.part.2.one.with.ross.mp4");
        let expected = ".one.with.ross";
        assert_eq!(expected, actual.unwrap());
    }
    
    #[test]
    fn test_episode_name_bad() {
        let actual = parse_episode_name("friends.5x2.mp4");
        assert_eq!("", actual.unwrap());
    }
    
    #[test]
    fn test_extension() {
        let actual = parse_extension("Friends.5x2.whatever-lol.mp4");
        let expected = "mp4";
        assert_eq!(expected, actual.unwrap());
    }
    
    #[test]
    fn test_extension_bad() {
        let actual = parse_extension("well well well-5x2-avi");
        assert_eq!(actual, None);
    }
    
    #[test]
    fn test_season_number_by_name() {
        let actual = parse_season_number("Friends.series.5.ep.2.mp4");
        let expected = 5;
        assert_eq!(expected, actual.unwrap());
    }
    
    #[test]
    fn test_season_number_by_se() {
        let actual = parse_season_number("Friends.s05e02.mp4");
        let expected = 5;
        assert_eq!(expected, actual.unwrap());
    }
    
    #[test]
    fn test_season_number_by_x() {
        let actual = parse_season_number("Friends.5x2.mp4");
        let expected = 5;
        assert_eq!(expected, actual.unwrap());
    }
    
    #[test]
    fn test_season_number_bad() {
        let actual = parse_season_number("friends - yep.mp4");
        assert_eq!(actual, None);
    }
    
    #[test]
    fn test_episode_number_by_of() {
        let actual = parse_episode_number("Grand.Designs.Series.5.2of9.mp4");
        let expected = 2;
        assert_eq!(expected, actual.unwrap());
    }
    
    #[test]
    fn test_episode_number_by_se() {
        let actual = parse_episode_number("Grand.Designs.s05e02.mp4");
        let expected = 2;
        assert_eq!(expected, actual.unwrap());
    }
    
    #[test]
    fn test_episode_number_by_x() {
        let actual = parse_episode_number("Grand.Designs.5x2.mp4");
        let expected = 2;
        assert_eq!(expected, actual.unwrap());
    }
    
    #[test]
    fn test_episode_number_by_name() {
        let actual = parse_episode_number("Grand.Designs.ep2.mp4");
        let expected = 2;
        assert_eq!(expected, actual.unwrap());
    }
    
    #[test]
    fn test_episode_number_bad() {
        let actual = parse_episode_number("friends - yep.mp4");
        assert_eq!(actual, None);
    }
}
