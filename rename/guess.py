
from typing import List, Callable
from .parse import *

def guessShow(files: List[str]) -> str:
    name = guess(files, parseShowName)
    if not name: raise Exception("Cannot guess show name")
    
    return name


def guessSeason(files: List[str]) -> int:
    season = guess(files, parseSeason)
    if not season:
        print("Cannot guess season, assuming single season.")
        # raise Exception("Cannot guess season")
        return 1
    
    return season
    

def guess(files: List[str], fn):
    guess_map = {}
    
    # loopy
    for path in files:
        guess = fn(path)
        if not guess: continue
        
        try:
            guess_map[guess] += 1
        except KeyError:
            guess_map[guess] = 1
    
    # validate
    if len(guess_map) == 0:
        return None
    
    largest = 0
    final = None
    
    # sort
    for guess, count in guess_map.items():
        if count > largest:
            final = guess
    
    return final
