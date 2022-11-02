use scraper::{Html, Selector};
use serde_json::Value;
use std::error::Error;
use std::process::{Command, Output, Stdio};
use std::sync::mpsc::Sender;
use std::{fs, io};

use crate::stream_list::StreamList;

#[derive(Debug)]
pub struct ChannelData {
    pub description: String,
    pub is_live: bool,
    pub url: String,
}

pub fn fetch_page(url: &str) -> io::Result<Output> {
    // TODO: is there a more `native` way to make http requests?
    Command::new("curl").arg("-s").arg("-S").arg(url).output()
}

pub fn fetch(url: &str, tx: Sender<ChannelData>) {
    let output = fetch_page(url);
    match output {
        Ok(result) => {
            // TODO: properly deal with possible errors
            let page_output_string = String::from_utf8(result.stdout).unwrap();
            let document = Html::parse_document(&page_output_string);
            let selector = Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();
            let mut is_live = false;
            let mut description = String::from("No description");
            if let Some(script) = document.select(&selector).next() {
                let v: Value = serde_json::from_str(script.inner_html().as_str()).unwrap();

                is_live = v[0]["publication"]["isLiveBroadcast"]
                    .as_bool()
                    .unwrap_or(false);

                description = v[0]["description"]
                    .as_str()
                    .unwrap_or("No Description")
                    .to_string();
            }
            tx.send(ChannelData {
                description,
                is_live,
                url: url.to_string(),
            })
            .unwrap();
        }
        Err(e) => println!("Error, {}", e),
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

pub fn read_users(path: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    let mut lines = vec![];
    for line in contents.lines() {
        lines.push(line.to_string());
    }
    Ok(lines)
}
