use clap::Parser;
use serde_json;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(name = "Text Archiver")]
#[clap(author = "Jeffery D. Mitchell")]
#[clap(version = "0.1.0")]
#[clap(about = "Tool to record random bits of text")]
struct Args {
    #[clap(short, long, value_parser)]
    tag: String,
    #[clap(short, long, value_parser)]
    information: String,
}

fn main() {
    println!("Text Archiver");
    let args = Args::parse();
    let mut text_records: HashMap<String, String> = HashMap::new();
    text_records.insert(args.tag, args.information);
    for (key, value) in &text_records {
        println!("{}: {}", key, value);
    }
    let path = Path::new("archive.txt");
    let display = path.display();
    let file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(&path)
    {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", display, why),
    };
    match serde_json::to_writer(file, &text_records) {
        Ok(_) => println!("successfully wrote to {}", display),
        Err(why) => panic!("couldn't write to {}: {}", display, why),
    }
}
