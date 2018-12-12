extern crate regex;

#[macro_use]
extern crate lazy_static;

mod workshop;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let mut fabric = workshop::Fabric::new();

    let contents = read_input("input");
    for line in contents.lines() {
        fabric.allocate(workshop::Claim::from_input_line(line));
    }

    println!("over allocations: {}", fabric.count_over_allocated_squares());
    println!("claim without collisions: {}", fabric.find_claim_without_collisions());
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

    return  contents.clone();
}