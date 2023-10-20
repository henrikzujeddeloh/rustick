#![allow(unused)]

use std::time::Duration;
use std::thread::sleep;
use std::io::Write;

use clap::Parser;

mod utils;

#[derive(Parser)]
struct Cli {
    task: String,
    duration: u16,
}


fn main() {
    let args = Cli::parse();
    println!("Running {} for {}", &args.task, &args.duration);
    run_task(&args)
}

fn run_task(input: &Cli) {
    let term_width = utils::get_term_width();
    update_bar(input.duration, term_width);
}

fn update_bar(dur: u16, cols: u16) {
    let millisec_per_block: f32 = (dur as f32 * 60000.0) / cols as f32;
    let mut bar = String::from("");
    for block in 1..cols {
        bar.push('#');
        print!("\r{}", bar);
        std::io::stdout().flush();
        sleep(Duration::from_millis(millisec_per_block as u64));
    }
}

