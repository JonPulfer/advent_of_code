extern crate regex;

#[macro_use]
extern crate lazy_static;

extern crate chrono;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod closet;

fn main() {
    let file_input = read_input("input");
    let mut secret_lab = closet::Lab::new();
    match secret_lab.read_input_into_journal(file_input) {
        Some(entry_count) => {
            println!("read in {} journal entries", entry_count);
            secret_lab.calculate_guard_sleep_patterns();
            println!("found {} guards", secret_lab.number_of_guards());
            let (sleepy_guard, sleep_minute) = secret_lab.target_guard_and_minute();
            println!(
                "guard and minute: guard {}, minute {}",
                sleepy_guard, sleep_minute
            );
            println!(
                "part 1: {}",
                sleepy_guard.parse::<u32>().unwrap() * sleep_minute
            );
            let (regular_sleepy_guard, regular_sleep_minute) =
                secret_lab.target_regular_sleeping_guard_and_minute();
            println!(
                "part 2: {}",
                regular_sleepy_guard.parse::<u32>().unwrap() * regular_sleep_minute
            );
        }
        None => {
            panic!("didn't read any entries");
        }
    }
}

/// This seems to be a common pattern for these puzzles.
fn read_input(file_name: &str) -> String {
    let path = Path::new(file_name);

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", file_name, why.description()),
        Ok(file) => file,
    };

    let contents: &mut String = &mut String::new();

    // Read the file contents into a string, returns `io::Result<usize>`
    match file.read_to_string(contents) {
        Err(why) => panic!("couldn't read {}: {}", file_name, why.description()),
        Ok(_) => println!("{} read", file_name),
    }

    return contents.clone();
}
