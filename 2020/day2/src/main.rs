use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let mut part1_count: i64 = 0;
    let mut part2_count: i64 = 0;

    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(extracted_line) = line {
                let line_parts: Vec<&str> = extracted_line.split(' ').collect();
                let policy_part1 = Part1PasswordPolicy::new(line_parts[0], line_parts[1]);
                if policy_part1.matches(line_parts[2]) {
                    part1_count += 1;
                }
                let policy_part2 = Part2PasswordPolicy::new(line_parts[0], line_parts[1]);
                if policy_part2.matches(line_parts[2]) {
                    part2_count += 1;
                }
            }
        }
    }

    println!("part1: {}", part1_count);
    println!("part2: {}", part2_count);
}

pub struct Part1PasswordPolicy<'a> {
    min: usize,
    max: usize,
    character: &'a str,
}

impl<'a> Part1PasswordPolicy<'a> {
    pub fn new(policy: &'a str, char: &'a str) -> Part1PasswordPolicy<'a> {
        let range: Vec<&str> = policy.split('-').collect();
        let character = char.strip_suffix(':').unwrap();

        let min: usize = range[0].parse::<usize>().unwrap();
        let max: usize = range[1].parse::<usize>().unwrap();

        Part1PasswordPolicy {
            min,
            max,
            character,
        }
    }

    pub fn matches(self, password: &'a str) -> bool {
        let matches: Vec<_> = password.match_indices(self.character).collect();

        matches.len() >= self.min && matches.len() <= self.max
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct Part2PasswordPolicy<'a> {
    first_position: usize,
    second_position: usize,
    character: &'a str,
}

impl<'a> Part2PasswordPolicy<'a> {
    pub fn new(policy: &'a str, char: &'a str) -> Part2PasswordPolicy<'a> {
        let range: Vec<&str> = policy.split('-').collect();
        let character = char.strip_suffix(':').unwrap();

        let first_position: usize = range[0].parse::<usize>().unwrap();
        let second_position: usize = range[1].parse::<usize>().unwrap();

        Part2PasswordPolicy {
            first_position,
            second_position,
            character,
        }
    }

    pub fn matches(self, password: &'a str) -> bool {
        let chars: Vec<char> = password.chars().collect();
        if chars.len() >= self.second_position {
            if chars[self.first_position - 1] == self.character.parse().unwrap()
                && chars[self.second_position - 1] != self.character.parse().unwrap()
            {
                return true;
            }
            if chars[self.first_position - 1] != self.character.parse().unwrap()
                && chars[self.second_position - 1] == self.character.parse().unwrap()
            {
                return true;
            }
        }
        false
    }
}
