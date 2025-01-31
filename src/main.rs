use std::env;
use mpris::PlayerFinder;

enum Operation {
    Play,
    Pause,
    PlayPause,
    Stop,
    Next,
    Previous,
    Seek(i64),
    Help,
}

fn get_operation() -> Operation {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(op) => match op.as_str() {
            "play" => Operation::Play,
            "pause" => Operation::Pause,
            "play-pause" => Operation::PlayPause,
            "stop" => Operation::Stop,
            "next" => Operation::Next,
            "previous" => Operation::Previous,
            "seek" => match args.get(2) {
                Some(val) => match val.parse::<i64>() {
                    Ok(num) => Operation::Seek(num),
                    Err(_) => Operation::Help,
                },
                None => Operation::Help,
            },
            _ => Operation::Help,
        },
        None => Operation::Help
    }
}

fn show_help_text() {
    println!("mprisctl:
        play - Play a paused track
        pause - Pause a playing track
        play-pause - Toggle playing status of track
        stop - Stop the current track
        next -  Navigate to next track
        previous - Navigate to previous track
        seek [x] - Seek x seconds. Negative number to go backwards.")
}

fn main() {
    let player = PlayerFinder::new().expect("Could not connect to DBus").find_active().expect("Couldn't find active media player. Make sure one is running.");
    let op = get_operation();
    const MICROSECONDS_IN_SECOND: i64 = 1000000;
    match op {
        Operation::Play => player.play().expect("failed to play"),
        Operation::Pause => player.pause().expect("failed to pause"),
        Operation::PlayPause => player.play_pause().expect("failed to toggle play/pause"),
        Operation::Stop => player.stop().expect("failed to stop"),
        Operation::Next => player.next().expect("failed to navigate to next track"),
        Operation::Previous => player.previous().expect("failed to navigate to previous track"),
        Operation::Seek(ms) => player.seek(ms * MICROSECONDS_IN_SECOND).expect("failed to seek"),
        Operation::Help => show_help_text(),
    }
}