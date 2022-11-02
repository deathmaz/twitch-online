use crate::stream::Stream;
use crate::utils;
use spinners::Spinner;
use std::{mem, thread};

use std::sync::mpsc;

#[derive(Debug)]
pub struct StreamList {
    pub inner: Vec<Stream>,
}

impl Default for StreamList {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamList {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn from(ids: Vec<String>) -> Self {
        let mut list = Self::new();
        list.create_from_ids(ids);
        list
    }

    pub fn add(&mut self, stream: Stream) {
        self.inner.push(stream)
    }

    pub fn create_from_ids(&mut self, ids: Vec<String>) {
        for (index, id) in ids.iter().enumerate() {
            self.add(Stream::new(id, index))
        }
    }

    pub fn fetch_all(&mut self) {
        let (tx, rx) = mpsc::channel();
        println!();
        let mut sp = Spinner::new(spinners::Spinners::Dots, "Fetching all streams".into());
        for stream in &mut self.inner {
            let tx_cloned = tx.clone();
            let url = String::from(&stream.url);
            thread::spawn(move || {
                utils::fetch(&url, tx_cloned);
            });
        }

        mem::drop(tx);

        for data in rx {
            if let Some(index) = self.inner.iter().position(|st| st.url == data.url) {
                self.inner[index].description = String::from(&data.description);
                self.inner[index].is_live = data.is_live;
            }
        }

        self.inner.sort_by(|a, b| a.index.cmp(&b.index));

        sp.stop_with_message("".into());
    }

    pub fn _fetch_all_and_show(&mut self) {
        self.fetch_all();
        self.show_only_live();
    }

    pub fn fetch_all_and_play(&mut self) {
        self.fetch_all();
        utils::play_stream(self);
    }

    pub fn show_all(&mut self) {
        println!();
        println!("Displaying all data:");
        self.inner.sort_by(|a, b| b.is_live.cmp(&a.is_live));
        for stream in &self.inner {
            stream.show();
        }
    }

    pub fn get_item(&self, index: u32) -> Option<&Stream> {
        self.inner
            .iter()
            .find(|&stream| stream.index == (index as usize))
    }

    pub fn show_only_live(&mut self) {
        let live_streams = self.get_live_streams();

        for stream in live_streams {
            stream.show()
        }
    }

    pub fn get_live_streams(&self) -> Vec<&Stream> {
        self.inner.iter().filter(|stream| stream.is_live).collect()
    }
}
