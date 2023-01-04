use std::path::PathBuf;

use twitch_online::{run, Config};
extern crate directories;
use directories::UserDirs;

fn main() {
    let mut home = String::new();
    if let Some(user_dirs) = UserDirs::new() {
        match user_dirs.home_dir().to_str() {
            Some(path) => home = path.to_string(),
            None => panic!("Can't find home dir!"),
        }
    }
    let path = PathBuf::from(format!("{}/.config/twitch-online/config.toml", home));
    let config = Config::from(&path.display().to_string());
    match config {
        Ok(config) => run(config),
        Err(error) => println!(
            "Something went wrong while reading config.toml file:\n{:#}",
            error
        ),
    }
}
