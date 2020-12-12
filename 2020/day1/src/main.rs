use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::borrow::Borrow;

fn main() {
    let mut expenses :Vec<i64> = Vec::new();

    if let Ok(lines) = read_lines("part1_input") {
        for line in lines {
            if let Ok(expense) = line {
                expenses.push(expense.parse::<i64>().unwrap());
            }
        }
    }

    expenses.sort();
    expenses.reverse();

    let mut part1_found :bool = false;
    let mut part2_found :bool = false;
    for expense in expenses.clone() {
        if remainder(expense.clone()) > 0 {
            let required = remainder(expense.clone());
            for potential in expenses.clone() {
                if potential == required {
                    if !part1_found {
                        println!("result (part 1): {}", expense * required);
                        part1_found = true;
                    }
                }
                if remainder(expense.clone() + potential.clone()) > 0 {
                    for final_part in expenses.clone() {
                        if final_part == remainder(expense.clone() + potential.clone()) {
                            if !part2_found {
                                println!("result (part2): {}", expense * potential * final_part);
                                part2_found = true;
                                if part1_found {
                                    return
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// Problem statement:
// Find the two numbers from the part1_input which added together result in 2020

fn remainder(origin :i64) -> i64 {
    2020 - origin
}

#[test]
fn test_remainder() {
    assert_eq!(remainder(20), 101);
    assert_eq!(remainder(4040), 0);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}