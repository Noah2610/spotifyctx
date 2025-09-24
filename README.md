# `spotifyctx` and a couple wrappers

[`spotifyctx`] is a small wrapper script around [`spotify_player`]
for displaying spotify playback information.
Also uses [`playback-formatter`] rust cli wrapper for
[`cmus-status-line`] for its configurable progress-bar.

## Usage
See `spotifyctx --help` for full usage information.  
Examples:  
```
$ spotifyctx track     # Display currently playing track's artist and name
Chris Christodoulou - Once in a Lullaby

$ spotifyctx progress  # Display playback progress in percentage
42

$ spotifyctx bar       # Display playback status with progress-bar (configurable format)
Crystal Castles ◀■■■■———▶ Knights ▶
```

## License
Distributed under the terms of the [MIT License](./LICENSE)

[`spotifyctx`]: https://github.com/Noah2610/spotifyctx
[`spotify_player`]: https://github.com/aome510/spotify-player
[`playback-formatter`]: ./playback-formatter
[`cmus-status-line`]: https://github.com/Noah2610/cmus-status-line
