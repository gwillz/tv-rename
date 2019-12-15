
import os
from .parse import *

class Episode(object):
    path: str
    episode: int
    season: int
    name: str
    show_name: str
    extension: str
    
    def __init__(self, path: str, show_name: str, season: int):
        self.path = path
        self.show_name = show_name
        self.season = season
        self.episode = parseEpisode(path)
        self.extension = parseExtension(path)
        self.name = parseEpisodeName(path)
    
    
    def __str__(self):
        return f"{self.path} -> {self.result()}"
    
    
    def result(self):
        out = f"{self.show_name} S{self.season:02d}E{self.episode:02d}"
        if self.name: out += f" - {self.name}"
        out += f".{self.extension}"
        
        return out
    
    
    def move(self, target_path):
        source = os.path.join(target_path, self.path)
        target = os.path.join(target_path, self.result())
        
        os.rename(source, target)
    
    
    # @todo sorting methods