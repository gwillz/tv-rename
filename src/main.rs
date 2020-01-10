
use std::env;
use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;

use rustyline::error::ReadlineError;
use directories::ProjectDirs;

use input::Input;
use cleaner::Cleaner;
use guesser::Guesser;
use episode::EpisodeFactory;
use exclude_rules::write_exclude;

mod input;
mod parsers;
mod cleaner;
mod guesser;
mod episode;
mod exclude_rules;

fn main() {
    println!("TV Rename v1");
    println!("------------");
    
    let exclude_path = get_exclude_path()
        .unwrap_or_else(|e| quit(e));
    
    let cleaner = Cleaner::load(exclude_path)
        .unwrap_or_else(|_| quit("Failed to load config file."));
    
    let mut input = Input::new(input_errors);
    
    println!("Exclude DB loaded {} rules.", cleaner.size());
    
    // Get target path.
    let path = get_directory()
        .unwrap_or_else(|_| quit("Can't find that path!"));
    
    println!("Reading {}", path.display());
    
    // Read target directory.
    let files = read_directory(&path)
        .unwrap_or_else(|_| quit("Can't read the directory!"));
    
    println!("Loaded {} files.\n", files.len());
    
    for file in &files {
        println!("{:?}", file.file_name());
    }
    
    println!("");
    
    // The guesser object finds the most likely show/season.
    let guesser = Guesser::new(&files);
    
    // Guess show name.
    let show_name = guesser.get_show_name()
        .map(|name| cleaner.clean(&name));
    
    println!("{}",
        if show_name.is_some() { "I think this show is:" }
        else { "I don't know what this show is:" }
    );
    let show_name = input.text(show_name.unwrap_or(String::new()));
    println!("");
    
    // Guess the season number.
    let season_number = guesser.get_season_number().unwrap_or(1);
    
    println!("I think this season is:");
    let season_number = input.number(season_number);
    println!("");
    
    // Create episode objects.
    let mut factory = EpisodeFactory::new(&show_name, season_number, &cleaner);
    
    for file in files {
        factory.insert(file.path())
            .unwrap_or_else(|e| quit(e.as_ref()));
    }
    
    // Preview.
    println!("\nHow's this?\n");
    
    let episodes = factory.get_all();
    
    for ep in &episodes {
        println!("{}", ep);
    }
    
    println!("\nDo you want to rename these?");
    
    if input.confirm() {
        println!("\nWorking...");
        
        // Rename all the files.
        for (i, ep) in episodes.iter().enumerate() {
            println!("File: {}", i);
            ep.rename().unwrap_or_else(|_| quit("Failed to rename file."));
        }
        
        println!("All done!");
    }
    else {
        println!("\nOkay, I did nothing.");
    }
    
    // Wait for exit (good for working with midnight commander).
    input.pause();
}

/// Quit message. Kinda like panic, but prettier.
fn quit(message: &str) -> ! {
    println!("{}\nQuitting.", message);
    std::process::exit(1)
}

/// Handle errors from readline.
fn input_errors(err: ReadlineError) -> () {
    match err {
        ReadlineError::Interrupted => {
            quit("\nCtrl-C");
        }
        ReadlineError::Eof => {
            quit("\nCtrl-D");
        }
        err => {
            println!("\nError: {:?}", err);
            quit("uhh");
        }
    }
}

/// Get the directory from arg 1.
/// Or, if not provided, the current working directory.
fn get_directory() -> Result<PathBuf, io::Error> {
    match env::args().skip(1).next() {
        Some(path) => PathBuf::from(&path).canonicalize(),
        None => env::current_dir(),
    }
}

/// Read the directory as a vector of entries.
fn read_directory(path: &PathBuf) -> Result<Vec<DirEntry>, io::Error> {
    fs::read_dir(path).map(|dir| {
        dir.map(|entry| entry.unwrap())
        .collect()
    })
}

/// Get the config path.
fn get_exclude_path() -> Result<PathBuf, &'static str> {
    match ProjectDirs::from("com", "gwillz", "tv-rename") {
        // Good, now try writing the exclude file.
        Some(dirs) => {
            let path = dirs.config_dir().with_file_name("exclude.txt");
            
            match write_exclude(path.as_path()) {
                // Great, move on.
                Ok(_) => Ok(path),
                // Well..
                Err(err) => match err.kind() {
                    // This is okay.
                    io::ErrorKind::AlreadyExists => Ok(path),
                    // Something bad happened.
                    _ => Err("Failed to write config file."),
                }
            }
        }
        // Rare? I assume?
        None => Err("Failed to find config path.")
    }
}
