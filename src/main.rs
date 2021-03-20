use twitch_online::StreamList;
use twitch_online::{main_menu, read_users};
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
    let path = format!("{}/.config/twitch_online/users", home);
    let stream_ids = read_users(&path).expect("Error while reading users");

    let mut stream_list = StreamList::new();
    stream_list.create_from_ids(stream_ids);
    stream_list.fetch_all_and_show();

    main_menu(&mut stream_list);
}
