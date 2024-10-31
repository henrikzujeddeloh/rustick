// Import from standard library
use std::time::Duration;
use std::io::Write;

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
        clear: bool,
        #[clap(long, short, )]
        /// remove entry
        remove: Option<u32>,
    }
}

// create struct with entry parameters
#[derive(Serialize, Deserialize, PartialEq)]
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

    // parse arguments
    let args = Cli::parse();

    // define path to data.json
    let json_path = "data.json";

    // open file and read contents to string
    let mut file = utils::open_file(&json_path);
    let mut file_contents = utils::read_file(&mut file);
    // if file is empty, add empty log template
    match file_contents.is_empty() {
        true => file_contents = empty_log.to_string(),
        false => ()
    };
    // parse log from json
    let mut data_log: Log = match serde_json::from_str(&file_contents){
        Ok(data_log) => data_log,
        Err(err) => {
            panic!("error parsing json: {:?}", err)
        }
    };

    // run rustick command
    match args.command {
        Command::Start { task, duration }=> {
            println!("Running {} for {} minutes...", &task, &duration);
            let new_entry: Entry = run_task(&task, &duration);
            // add entry to the log
            data_log.log.push(new_entry);
            // write updated log as json to file
            utils::write_file(&log_to_json(&data_log), &json_path);
        },
        Command::Log { clear, remove } => {
            match remove {
                None => (),
                Some(index) => {
                    println!("Removing entry {}", index);
                    data_log.log.remove(index.try_into().unwrap());
                    utils::write_file(&log_to_json(&data_log), &json_path);
                },
            }
            if clear == true {
                println!("Clearing Log!");
                file_contents = empty_log.to_string();
                utils::write_file(&file_contents, &json_path);
            } else {
                println!("Listing entries");
                list_entries(&data_log);
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

// updating timer bar
fn update_bar(dur: &u32) {
    // get terminal width
    let term_width: u32 = utils::get_term_width();
    // calculate milliseconds per block of bar
    let millisec_per_block: u32 = (dur * 60000) / &term_width;
    // instantiate bar as empty string
    let mut bar = String::from("");
    // loop across terminal width
    for _block in 1..term_width{
        // add '#' to bar string 
        let _ = &bar.push('#');
        // print bar string to terminal
        print!("\r{}", &bar);
        let _ = std::io::stdout().flush();
        // enable reading raw keyboard input
        enable_raw_mode().unwrap();
        // check if keypress event occurs during wait time
        if poll(Duration::from_millis(millisec_per_block as u64)).unwrap() {
            let event = read().unwrap();
            println!("Event::{:?}\r", event);
            // checks if keypress is character 's'
            if event == Event::Key(KeyCode::Char('s').into()) {
                println!("Stopping timer");
                // exit timer bar loop
                break;
            }
        }
    }
}

// lists all entries and duration in log
fn list_entries(log: &Log) {
    for entry in log.log.iter().rev() {
        println!("{}. {} for {} mins.", &log.log.iter().position(|e| e == entry).unwrap()+1, entry.name, entry.duration);
    }
    
}

// convert log to json
fn log_to_json(log: &Log) -> String {
    let j: String = serde_json::to_string_pretty(&log).expect("cannot convert log to json");
    j
}
