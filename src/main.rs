use std::path::PathBuf;

use twitch_online::{read_users, run};
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
    let path = PathBuf::from(format!("{}/.config/twitch-online/users", home));

    let stream_ids = read_users(&path.display().to_string()).expect("Error while reading users");
    run(stream_ids);
}
