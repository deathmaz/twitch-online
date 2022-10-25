use ansi_term::{
    ANSIGenericString,
    Colour::{Blue, Green, Purple, Red},
};
use scraper::{Html, Selector};
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io;
use std::process::{Command, Output};

pub fn fetch_page(url: &str) -> io::Result<Output> {
    Command::new("curl").arg("-s").arg("-S").arg(url).output()
}

#[derive(Debug)]
pub struct Stream {
    id: String,
    url: String,
    is_live: bool,
    description: String,
    index: usize,
}

impl Stream {
    pub fn new(id: &str, index: usize) -> Self {
        Self {
            id: id.to_owned(),
            url: format!("https://m.twitch.tv/{}", id),
            is_live: false,
            description: String::new(),
            index,
        }
    }

    pub fn fetch(&mut self) {
        println!("Please wait, trying to fetch {}", &self.url);

        let output = fetch_page(&self.url);
        match output {
            Ok(result) => {
                let page_output_string = String::from_utf8(result.stdout).unwrap();
                let document = Html::parse_document(&page_output_string);
                let selector = Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();
                match document.select(&selector).next() {
                    Some(script) => {
                        let v: Value = serde_json::from_str(script.inner_html().as_str()).unwrap();

                        self.is_live = v[0]["publication"]["isLiveBroadcast"]
                            .as_bool()
                            .unwrap_or_else(|| false);

                        self.description = v[0]["description"]
                            .as_str()
                            .unwrap_or_else(|| "No Description")
                            .to_string();
                    }
                    None => (),
                }
            }
            Err(e) => println!("Error, {}", e),
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
            Blue.underline().paint(&self.url),
            self.status_text()
        );
        println!("Streaming: {}", Purple.paint(self.description.to_string()));
        println!("");
    }
}

#[derive(Debug)]
pub struct StreamList {
    inner: Vec<Stream>,
}

impl StreamList {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
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
        println!("");
        println!("Fetching all streams");
        for stream in &mut self.inner {
            stream.fetch();
        }
    }

    pub fn fetch_all_and_show(&mut self) {
        self.fetch_all();
        self.show_all();
    }

    pub fn show_all(&mut self) {
        println!("");
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

    pub fn show_only_live(&self) {
        let live_streams: Vec<&Stream> =
            self.inner.iter().filter(|stream| stream.is_live).collect();

        for stream in live_streams {
            stream.show()
        }
    }
}

pub fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again.");
    }

    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

pub fn main_menu(stream_list: &mut StreamList) {
    fn show() {
        println!("");
        println!("== What would you like to do? ==");
        println!("1. View List");
        println!("2. Refetch data");
        println!("3. Play stream");
        println!("4. Show only live streams");
        println!("");
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
            "3" => play_stream(&stream_list),
            "4" => stream_list.show_only_live(),
            _ => break,
        }
    }
}

fn play_stream(stream_list: &StreamList) {
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
            match Command::new("streamlink")
                .arg("--player=mpv")
                .arg(&str.url)
                .output()
            {
                Ok(_) => println!("Done!"),
                Err(e) => println!("Error while running the stream {}", e),
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
        if &input == "" {
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
