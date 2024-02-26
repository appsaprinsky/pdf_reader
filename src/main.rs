use lopdf::Document;
use std::env;
use std::path::PathBuf;

use std::fs::File;
use std::io::{BufWriter, Write};
// use gtts::Client;


fn main() {

    let args: Vec<String> = env::args().collect();

    // let pattern = args[1].clone();
    let path = PathBuf::from(args[1].clone());

    // let file = "sample.pdf";
    let binding = path.into_os_string();
    let file = binding.to_str().unwrap_or("");

    match Document::load(file) {
        Ok(document) => {
            let pages = document.get_pages();
            let mut texts = Vec::new();

            for (i, _) in pages.iter().enumerate() {
                let page_number = (i + 1) as u32;
                let text = document.extract_text(&[page_number]);
                texts.push(text.unwrap_or_default());
            }

            // let client = Client::new("en");
            // let mut speech = client.get_speech(texts);
            // let mut file = BufWriter::new(File::create("output.mp3").unwrap());
            // speech.write_to(&mut file).unwrap();

            for (i, text) in texts.iter().enumerate() {
                println!("Text on page {}: {:?}", i + 1, text);
            }
        }
        Err(err) => {
            eprintln!("{err}")
        }
    }


}

