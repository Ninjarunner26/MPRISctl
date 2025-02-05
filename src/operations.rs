use std::env;
pub enum Operation {
    Play,
    Pause,
    PlayPause,
    Stop,
    Next,
    Previous,
    Seek(i64),
    Help,
}
impl Operation {
    pub fn from_args() -> Operation {
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
}