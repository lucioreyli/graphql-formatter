#[derive(Debug)]
pub struct Config {
    pub quiet_mode: bool,
}

impl Config {
    pub fn new() -> Config {
        Config { quiet_mode: false }
    }
}
