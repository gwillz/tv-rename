# TV Show Renaming

Bulk automatic renaming of TV show files.

Something like: `show.name.Series6.Part4.THE.CLOSER[ettv].hdtv-EVO.mp4`
is renamed to `Show Name S06E04 - The Closer.mp4`.

## Install

```sh
cargo install --path .

tv_rename .
```

## Exclude tag rules

On first run `tv-rename` will create a rules file at `~/.config/exclude.txt`.

These are set of common tags that are straight up ugly (ettv, hdtv, etc) and
can't easily be identified apart from normal words. 

## Identifier rules

### By Name

Any number following these words:

`Series, Collection, Season, Episode, Part, Ep`

### By `S--E--`

This identifier is also the exported version.

Eg. `S01E10, S100E01`

### By `-x-`

Eg. `1x4, 2x10, 30x4, 50x60`

### By 'of'

This method can only identify the episode number.

Eg. `1of9, 2 of 10, 6.of.6`

