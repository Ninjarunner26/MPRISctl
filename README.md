# MPRISctl
A simple tool for controlling MPRIS media players.
Like [playerctl](https://github.com/altdesktop/playerctl) but worse.

## Install
Make sure you've got rust and cargo installed.
Clone the repo and cd into the directory. Then run:
```
cargo build --release
```
Under target/release/ there will be a binary. Make it executable and stick it somewhere that's in your path.

## Usage
- **play** - Play a paused track
- **pause** - Pause a playing track
- **play-pause** - Toggle playing status of track
- **stop** - Stop the current track
- **next** -  Navigate to next track
- **previous** - Navigate to previous track
- **seek** [x] - Seek x seconds. Negative number to go backwards