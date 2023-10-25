#![allow(unused)]

// Import from standard library
use std::time::Duration;
use std::thread::sleep;
use std::io::Read;
use std::io::Write;
use std::io::ErrorKind;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;

// Import from third parties
use clap::Parser;
use clap::error::ContextKind;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use chrono::Local;

// import own modules
mod utils;

// create struct with CLI arguments
#[derive(Parser)]
struct Cli {
    task: String,
    duration: u32,
}

// create struct with entry parameters
#[derive(Serialize, Deserialize)]
struct Entry {
    name: String,
    duration: u32,
    tag: String,
    start: String,
    finish: String

}

// create log struct
#[derive(Serialize, Deserialize)]
struct Log {
    log: Vec<Entry>
}


fn main() {
    let args = Cli::parse();
    let file_data = read_file("data.json");
    // TODO: handle initial json parsing errors (add template if empty?)
    let mut data_log: Log = serde_json::from_str(&file_data).expect("parsing json file");

    println!("Running {} for {} minutes...", &args.task, &args.duration);
    let new_entry: Entry = run_task(&args);
    println!("New entry: {}", entry_to_json(&new_entry));
    data_log.log.push(new_entry);
    write_file(&log_to_json(&data_log));
}

fn run_task(input: &Cli) -> Entry{
    let start_time = chrono::offset::Local::now();
    update_bar(&input.duration);
    let end_time = chrono::offset::Local::now();
    let entry = Entry {name: input.task.to_string(),
                        duration: input.duration,
                        tag: "Default".to_owned(),
                        start: start_time.to_string(),
                        finish: end_time.to_string()
                        };
    entry
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


fn read_file(file_path: &str) -> String {
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_path) {
                Ok(fc) => { 
                    init_json_file();
                    fc
                }
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    contents
}

fn write_file(new_entry: &String) {
    fs::write("data.json", &new_entry).expect("Unable to write file");
}

fn init_json_file() {
    let contents: &str = r#"
{
    "log": []
}"#;
    fs::write("data.json", &contents).expect("Unable to write file");

}

fn entry_to_json(entry: &Entry) -> String {
    let j: String = serde_json::to_string_pretty(&entry).expect("cannot convert entry to json");
    j
}

fn log_to_json(full_log: &Log) -> String {
    let j: String = serde_json::to_string_pretty(&full_log).expect("cannot convert log to json");
    j
}
