mod main_menu;
mod stream;
mod stream_list;
mod utils;

use crate::stream_list::StreamList;
pub use crate::utils::read_users;

pub fn run(stream_ids: Vec<String>) {
    let mut stream_list = StreamList::from(stream_ids);
    stream_list.fetch_all_and_play();

    main_menu::run(&mut stream_list);
}
