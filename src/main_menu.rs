use crate::{
    stream_list::StreamList,
    utils::{self, clear_screen},
};

pub fn run(stream_list: &mut StreamList) {
    fn show() {
        println!();
        println!("== What would you like to do? ==");
        println!("1. View List");
        println!("2. Refetch data");
        println!("3. Play stream");
        println!("4. Show only live streams");
        println!();
        println!("Enter selection:")
    }

    loop {
        show();
        let input = match utils::get_input() {
            Some(input) => input,
            None => return,
        };
        clear_screen();
        match input.as_str() {
            "1" => stream_list.show_all(),
            "2" => stream_list.fetch_all_and_play(),
            "3" => utils::play_stream(stream_list),
            "4" => stream_list.show_only_live(),
            _ => continue,
        }
    }
}
