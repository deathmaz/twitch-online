use ansi_term::{
    ANSIGenericString,
    Colour::{Blue, Green, Purple, Red},
};
use scraper::{Html, Selector};
use serde_json::Value;
use std::process::{Command, Output, Stdio};
use std::{error::Error, sync::mpsc::Sender};
use std::{fs, mem, thread};
use std::{io, sync::mpsc};

pub fn fetch_page(url: &str) -> io::Result<Output> {
    // TODO: is there a more `native` way to make http requests?
    Command::new("curl").arg("-s").arg("-S").arg(url).output()
}
#[derive(Debug)]
pub struct ChannelData {
    description: String,
    is_live: bool,
    url: String,
}

#[derive(Debug)]
pub struct Stream {
    url: String,
    is_live: bool,
    description: String,
    index: usize,
}

impl Stream {
    pub fn new(id: &str, index: usize) -> Self {
        Self {
            url: format!("https://m.twitch.tv/{}", id),
            is_live: false,
            description: String::new(),
            index,
        }
    }

    fn status_text(&self) -> ANSIGenericString<str> {
        match self.is_live {
            true => Green.bold().paint("live"),
            false => Red.bold().paint("offline"),
        }
    }

    pub fn show(&self) {
        println!(
            "{} {} is {}",
            Green.bold().paint(format!("{}.", self.index)),
            Blue.paint(&self.url),
            self.status_text()
        );
        println!("Streaming: {}", Purple.paint(self.description.to_string()));
        println!();
    }
}

pub fn fetch(url: &str, tx: Sender<ChannelData>) {
    let output = fetch_page(url);
    match output {
        Ok(result) => {
            // TODO: properly deal with possible errors
            let page_output_string = String::from_utf8(result.stdout).unwrap();
            let document = Html::parse_document(&page_output_string);
            let selector = Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();
            if let Some(script) = document.select(&selector).next() {
                let v: Value = serde_json::from_str(script.inner_html().as_str()).unwrap();

                let is_live = v[0]["publication"]["isLiveBroadcast"]
                    .as_bool()
                    .unwrap_or(false);

                let description = v[0]["description"]
                    .as_str()
                    .unwrap_or("No Description")
                    .to_string();

                tx.send(ChannelData {
                    description,
                    is_live,
                    url: url.to_string(),
                })
                .unwrap();
            }
        }
        Err(e) => println!("Error, {}", e),
    }
}

#[derive(Debug)]
pub struct StreamList {
    inner: Vec<Stream>,
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
        // ODO: add loading animation
        println!("Fetching all streams...");
        for stream in &mut self.inner {
            let tx_cloned = tx.clone();
            let url = String::from(&stream.url);
            thread::spawn(move || {
                fetch(&url, tx_cloned);
            });
        }

        mem::drop(tx);

        for data in rx {
            for stream in &mut self.inner {
                if stream.url == data.url {
                    stream.description = String::from(&data.description);
                    stream.is_live = data.is_live;
                }
            }
        }

        self.inner.sort_by(|a, b| a.index.cmp(&b.index));
    }

    pub fn fetch_all_and_show(&mut self) {
        self.fetch_all();
        self.show_only_live();
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

pub fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again.");
    }

    let input = buffer.trim().to_owned();
    if input.is_empty() {
        None
    } else {
        Some(input)
    }
}

pub fn main_menu(stream_list: &mut StreamList) {
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
        let input = match get_input() {
            Some(input) => input,
            None => return,
        };
        match input.as_str() {
            "1" => stream_list.show_all(),
            "2" => stream_list.fetch_all_and_show(),
            "3" => play_stream(stream_list),
            "4" => stream_list.show_only_live(),
            _ => continue,
        }
    }
}

pub fn play_stream(stream_list: &mut StreamList) {
    if stream_list.get_live_streams().is_empty() {
        println!();
        println!("No live streams avaliable.");
        return;
    }
    stream_list.show_only_live();
    println!("Type the number of the stream:");
    let number = match get_stream_number() {
        Some(num) => num,
        None => return,
    };

    let stream = stream_list.get_item(number);
    match stream {
        Some(str) => {
            println!("Starting stream: {}", &str.url);
            if let Err(e) = Command::new("streamlink")
                .arg(&str.url)
                .stdout(Stdio::null())
                .spawn()
            {
                println!("Error while running the stream {}", e);
            };
        }
        None => println!("The stream is missing"),
    }
}

fn get_stream_number() -> Option<u32> {
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if input.is_empty() {
            return None;
        }
        let parsed_input: Result<u32, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
}

pub fn read_users(path: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    let mut lines = vec![];
    for line in contents.lines() {
        lines.push(line.to_string());
    }
    Ok(lines)
}
