// Import from standard library
use std::time::Duration;
use std::io::Write;
use std::fs;

// Import from third parties
use clap::Parser;
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use crossterm::terminal::enable_raw_mode;
use crossterm::event::{poll, read, Event, KeyCode};

// import own modules
mod utils;

// create struct with rustick commands
#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

// commands with their arguments
#[derive(Subcommand, Debug)]
enum Command {
    /// Start a new task time
    Start {
        /// description of task
        task: String,
        /// duration of task timer
        duration: u32
    },
    /// View and/or edit task entry log
    Log {
        #[clap(long, short, action)]
        /// clear log
        clear: bool
    }
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
    // define json template for newly-created data.json
    let empty_log: &str = 
    r#"{
        "log": []
    }"#;

    let args = Cli::parse();

    // define path to data.json
    let json_path = "data.json";
    let mut file = utils::open_file(&json_path);
    let mut file_contents = utils::read_file(&mut file);
    match file_contents.is_empty() {
        true => file_contents = empty_log.to_string(),
        false => ()
    };
    let mut data_log: Log = match serde_json::from_str(&file_contents){
        Ok(data_log) => data_log,
        Err(err) => {
            panic!("error parsing json: {:?}", err)
        }
    };
    match args.command {
        Command::Start { task, duration }=> {
            println!("Running {} for {} minutes...", &task, &duration);
            let new_entry: Entry = run_task(&task, &duration);
            //println!("New entry: {}", entry_to_json(&new_entry));
            // add entry to the log
            data_log.log.push(new_entry);
            // write updated log as json to file
            write_file(&log_to_json(&data_log));
        },
        Command::Log { clear } => {
            if clear == true {
                println!("Clearing Log!");
                file_contents = empty_log.to_string();
                write_file(&file_contents);
            } else {
                println!("Listing entries");
            }
        },
    }
}

fn run_task(task: &String, duration: &u32) -> Entry{
    let start_time = chrono::offset::Local::now();
    update_bar(duration);
    let end_time = chrono::offset::Local::now();
    let elapsed = (end_time - start_time).num_minutes();
    
    let entry = Entry {name: task.to_string(),
                        duration: elapsed as u32,
                        tag: "Default".to_owned(),
                        start: start_time.to_string(),
                        finish: end_time.to_string()
                        };
    entry
}

fn update_bar(dur: &u32) {
    let term_width: u32 = utils::get_term_width();
    let millisec_per_block: u32 = (dur * 60000) / &term_width;
    let mut bar = String::from("");
    for _block in 1..term_width{
        let _ = &bar.push('#');
        print!("\r{}", &bar);
        let _ = std::io::stdout().flush();
        enable_raw_mode().unwrap();
        if poll(Duration::from_millis(millisec_per_block as u64)).unwrap() {
            let event = read().unwrap();
            println!("Event::{:?}\r", event);
            if event == Event::Key(KeyCode::Char('s').into()) {
                println!("Stopping timer");
                break;
            }

        } else {
            // do nothing
        }
    }
}


// write string to file
fn write_file(contents: &String) {
    fs::write("data.json", &contents).expect("write contents file");
}

// convert single entry to json
fn entry_to_json(entry: &Entry) -> String {
    let j: String = serde_json::to_string_pretty(&entry).expect("cannot convert entry to json");
    j
}

// convert log to json
fn log_to_json(log: &Log) -> String {
    let j: String = serde_json::to_string_pretty(&log).expect("cannot convert log to json");
    j
}
