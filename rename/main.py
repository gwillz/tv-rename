
import sys, os, re
from .episode import Episode
from .guess import *


def main():
    target_path = getArgs()
    
    print(f"Reading: {target_path}")
    
    files = os.listdir(target_path)
    print(f"Loaded {len(files)} files.")
    
    show_name = guessShow(files)
    season = guessSeason(files)
    
    if season == 0:
        print("Cannot guess season, assuming single season")
        season = 1
    
    # verify guesses
    while True:
        print("Guess season:", season)
        print("Guess show name:", show_name);
        question = input("Is this okay (y/n): ").lower()
        
        if question.startswith("y"): break
        
        season = int(input("New season: "))
        show_name = input("New show name: ")
    
    # parse episodes
    episodes = [Episode(path, show_name, season) for path in files]
    
    for episode in episodes:
        print(episode)
    
    # move files
    question = input("Is this okay (y/n): ").lower()
    
    if question.startswith("y"):
        for episode in episodes:
            episode.move(target_path)
        print("Done.")
    else:
        print("Cancelled.")
    

def getArgs() -> str:
    try:
        return sys.argv[1]
    except IndexError:
        return os.path.curdir
