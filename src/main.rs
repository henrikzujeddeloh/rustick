#![allow(unused)]

use std::time::Duration;
use std::thread::sleep;
use std::io::Read;
use std::io::Write;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::prelude::*;


use clap::Parser;
use clap::error::ContextKind;
use serde::{Deserialize, Serialize};
use serde_json::Result;


mod utils;

#[derive(Parser)]
struct Cli {
    task: String,
    duration: u32,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    name: String,
    duration: u32,
    tag: String
}

#[derive(Serialize, Deserialize)]
struct Log {
    log: Vec<Entry>
}


fn main() {
    let args = Cli::parse();
    let file_data = read_file();
    let mut data_log: Log = serde_json::from_str(&file_data).unwrap();
    println!("Data in data.json: {}", &file_data);
    println!("Running {} for {} minutes...", &args.task, &args.duration);
    let new_entry: Entry = run_task(&args);
    println!("New entry: {}", entry_to_json(&new_entry));
    data_log.log.push(new_entry);
    write_file(&log_to_json(&data_log));
}

fn run_task(input: &Cli) -> Entry{
    update_bar(&input.duration);
    let e = Entry { name: input.task.to_string(), duration: input.duration, tag: "Default".to_owned()};
    e
}

fn update_bar(dur: &u32) {
    let term_width: u32 = utils::get_term_width().into();
    let millisec_per_block: u32 = (dur * 60000) / &term_width;
    let mut bar = String::from("");
    for block in 1..term_width{
        &bar.push('#');
        print!("\r{}", &bar);
        std::io::stdout().flush();
        sleep(Duration::from_millis(millisec_per_block as u64));
    }
}


fn read_file() -> String {
    // TODO: check file exists, and if not, create it
    let mut file = File::open("data.json").expect("File cannot be opened");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File cannot be converted to string");
    contents
}

fn write_file(new_entry: &String) {
    fs::write("data.json", &new_entry).expect("Unable to write file");
}

fn entry_to_json(entry: &Entry) -> String {
    let j: String = serde_json::to_string_pretty(&entry).expect("cannot convert entry to json");
    j
}

fn log_to_json(full_log: &Log) -> String {
    let j: String = serde_json::to_string_pretty(&full_log).expect("cannot convert log to json");
    j
}
