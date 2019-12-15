import re

EXTENSION_RE = r"\.([a-z0-9]+)$"

def parseExtension(path: str) -> str:
    m = re.search(EXTENSION_RE, path, re.IGNORECASE)
    if m: return m[1]
    
    raise Exception(f"Cannot parse extension [{path}]")


def parseShowName(path: str) -> str:
    m = re.search(r"^(.+?)(?:collection|series|episode|ep|s\d+e\d+|\d+of\d+|\d+x\d+)", path, re.IGNORECASE)
    
    if not m: raise Exception(f"Cannot parse show name [{path}]")
    
    return cleanText(m[1])


def parseEpisodeName(path: str) -> str:
    m = re.search(r"(?:s\d+e\d+|\d+of\d+|\d+x\d+)(.+)" + EXTENSION_RE, path, re.IGNORECASE)
    
    if not m: return ""
    
    return cleanText(m[1])


def parseEpisode(path: str) -> int:
    # ..of..
    m = re.search("(\d+)of\d+", path, re.IGNORECASE)
    if m: return int(m[1])
    
    # s..e..
    m = re.search("s\d\d+e(\d\d+)", path, re.IGNORECASE)
    if m: return int(m[1])
    
    # ..x..
    m = re.search("\d+x(\d+)", path, re.IGNORECASE)
    if m: return int(m[1])
    
    # episode x or part x
    m = re.search("(?:episode|ep|part)\W+(\d+)", path, re.IGNORECASE)
    if m: return int(m[1])
    
    raise Exception(f"Cannot parse episode [{path}]")


def parseSeason(path: str) -> str:
    # by name
    m = re.search("(?:season|series|collection)\W*(\d+)", path, re.IGNORECASE)
    if m: return int(m[1])
    
    # by S..E..
    m = re.search("s(\d\d+)e\d\d+", path, re.IGNORECASE)
    if m: return int(m[1])
    
    # by .x.
    m = re.search("(\d+)x\d+", path, re.IGNORECASE)
    if m: return int(m[1])
    
    return 0


EXCLUDE = [
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
]

def cleanText(text: str) -> str:
    parts = []
    index = 0
    
    text = text.lower()
    text = re.sub(r"\[[^\]]+\]", '', text)
    text = text.strip(".,-_ ")
    text += '.'
    
    for m in re.finditer("[ ._]", text):
        pos = m.start()
        
        part = text[index:pos]
        index = pos + 1
        
        if part not in EXCLUDE:
            parts.append(part)
    
    return " ".join(part.capitalize() for part in parts)

