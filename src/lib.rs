mod config;
mod main_menu;
mod stream;
mod stream_list;
mod thread_pool;
mod utils;
mod worker;

pub use crate::config::Config;
use crate::stream_list::StreamList;

pub fn run(config: Config) {
    let mut stream_list = StreamList::new(config);
    stream_list.fetch_all_and_play();

    main_menu::run(&mut stream_list);
}
