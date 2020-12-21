mod grid;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let input = read_input("input");
    let mut coords = grid::Coordinates::new();
    coords.populate_from_input(input.as_str());
    let mut g = grid::Grid::new(coords);
    g.allocate_matrix_points();
    println!("part1 : {}", g.max_allocations_for_finite_point());
    println!("part2 : {}", g.qualifying_locations);
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
