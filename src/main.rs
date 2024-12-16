use std::env;
use mpris::PlayerFinder;

fn main() {
    let args: Vec<String> = env::args().collect();
    let player = PlayerFinder::new().expect("Could not connect to DBus").find_active().expect("Couldn't find active media player");
    if args.len() > 1 {
        let op = args[1].as_str();
        let mut value: i64 = 0;
        if args.len() > 2 {
            value = args[2].parse::<i64>().expect("Given value was not valid number");
        }
        
        match op{ 
            "play" => player.play().expect("Failed to play"),
            "pause" => player.pause().expect("Failed to pause"),
            "play-pause" => player.play_pause().expect("Failed to play-pause"),
            "stop" => player.stop().expect("Failed to stop"),
            "next" => player.next().expect("Failed to go to next track"),
            "previous" => player.previous().expect("Failed to go to previous track"),
            "seek" => player.seek(value).expect("Failed to seek"),
            _ => println!("not test")
        }
    }
}
