use ansi_term::{
    ANSIGenericString,
    Colour::{Blue, Green, Purple, Red},
};

#[derive(Debug, Default)]
pub struct Stream {
    pub url: String,
    pub displayed_url: String,
    pub is_live: bool,
    pub description: String,
    pub index: usize,
}

impl Stream {
    pub fn new(id: &str, index: usize) -> Self {
        Self {
            url: format!("https://m.twitch.tv/{}", id),
            displayed_url: format!("https://twitch.tv/{}", id),
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
            Blue.paint(&self.displayed_url),
            self.status_text()
        );
        println!("Streaming: {}", Purple.paint(self.description.to_string()));
        println!();
    }
}
