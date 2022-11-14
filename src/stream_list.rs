use crate::stream::Stream;
use crate::thread_pool::ThreadPool;
use crate::utils::clear_screen;
use crate::{utils, Config};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::mpsc;
use std::{mem, thread};

#[derive(Debug)]
pub struct StreamList {
    pub inner: Vec<Stream>,
    pub thread_pool: Option<ThreadPool>,
    pub config: Config,
}

impl StreamList {
    pub fn new(config: Config) -> Self {
        let pool = config.threads_num.map(ThreadPool::new);

        let mut inner = Vec::with_capacity(config.streamers.len());
        for (index, id) in config.streamers.iter().enumerate() {
            inner.push(Stream::new(id, index));
        }

        Self {
            inner,
            thread_pool: pool,
            config,
        }
    }

    pub fn fetch_all(&mut self) {
        let (tx, rx) = mpsc::channel();
        let pb = ProgressBar::new(self.inner.len().try_into().unwrap());
        pb.set_style(
            ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar:.cyan/blue}]").unwrap(),
        );
        for stream in &mut self.inner {
            let url = String::from(&stream.url);
            let tx_cloned = tx.clone();
            match &self.thread_pool {
                Some(pool) => {
                    pool.execute(move || {
                        utils::fetch(&url, tx_cloned);
                    });
                }
                None => {
                    thread::spawn(move || {
                        utils::fetch(&url, tx_cloned);
                    });
                }
            }
        }

        mem::drop(tx);

        for data in rx {
            pb.inc(1);
            if let Some(index) = self.inner.iter().position(|st| st.url == data.url) {
                self.inner[index].description = String::from(&data.description);
                self.inner[index].is_live = data.is_live;
            }
        }

        self.inner.sort_by(|a, b| a.index.cmp(&b.index));
        pb.finish_and_clear();
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
        clear_screen();
        let live_streams = self.get_live_streams();

        for stream in live_streams {
            stream.show()
        }
    }

    pub fn get_live_streams(&self) -> Vec<&Stream> {
        self.inner.iter().filter(|stream| stream.is_live).collect()
    }
}
