use crate::identity_documents::Passport;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let mut passports: Vec<Passport> = vec![];
    let mut part1_valid_count = 0;
    let mut part2_valid_count = 0;
    for input_field in parse_input("input") {
        if let Some(passport) = Passport::new(input_field) {
            if passport.part1_valid() {
                part1_valid_count += 1;
            }
            if passport.part2_valid() {
                part2_valid_count += 1;
            }
            passports.push(passport);
        }
    }
    println!("part 1: {}", part1_valid_count);
    println!("part 2: {}", part2_valid_count);
}

fn parse_input(filename: &str) -> Vec<Vec<String>> {
    let mut password_entries: Vec<Vec<String>> = vec![];
    if let Ok(lines) = read_lines(filename) {
        let mut password_entry: Vec<String> = vec![];
        let mut pending_data = false;
        for line in lines {
            if let Ok(extracted_line) = line {
                if extracted_line.contains(":") {
                    pending_data = true;
                    let line_parts: Vec<&str> = extracted_line.split(' ').collect();
                    for line_part in line_parts {
                        password_entry.push(String::from(line_part));
                    }
                } else {
                    if pending_data {
                        password_entries.push(password_entry.clone());
                        pending_data = false;
                        password_entry = vec![];
                    }
                }
            }
        }
        if pending_data {
            password_entries.push(password_entry.clone());
        }
    }
    password_entries
}

#[test]
fn test_parse_input() {
    assert_eq!(parse_input("test_input").len(), 4);
    let mut valid_passports: Vec<Passport> = vec![];
    for input_field in parse_input("test_input") {
        if let Some(passport) = Passport::new(input_field) {
            if passport.part1_valid() {
                valid_passports.push(passport.clone());
            }
        }
    }
    assert_eq!(valid_passports.len(), 2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

mod identity_documents {
    use regex::Regex;

    #[derive(Debug, Clone)]
    pub struct Passport {
        birth_year: i16,
        issue_year: i16,
        expiration_year: i16,
        height: String,
        hair_colour: String,
        eye_colour: String,
        id: String,
        country_id: String,
    }

    impl Passport {
        pub fn new(fields: Vec<String>) -> Option<Passport> {
            let mut passport = Passport {
                birth_year: -1,
                issue_year: -1,
                expiration_year: -1,
                height: "".to_string(),
                hair_colour: "".to_string(),
                eye_colour: "".to_string(),
                id: "".to_string(),
                country_id: "".to_string(),
            };
            for field in fields {
                let parts: Vec<&str> = field.split(':').collect();
                if parts.len() != 2 {
                    continue;
                }
                match parts[0] {
                    "byr" => {
                        if let Ok(year) = parts[1].parse::<i16>() {
                            passport.birth_year = year;
                        }
                    }
                    "iyr" => {
                        if let Ok(year) = parts[1].parse::<i16>() {
                            passport.issue_year = year;
                        }
                    }
                    "eyr" => {
                        if let Ok(year) = parts[1].parse::<i16>() {
                            passport.expiration_year = year;
                        }
                    }
                    "hgt" => {
                        passport.height = String::from(parts[1]);
                    }
                    "hcl" => {
                        passport.hair_colour = String::from(parts[1]);
                    }
                    "ecl" => {
                        passport.eye_colour = String::from(parts[1]);
                    }
                    "pid" => passport.id = String::from(parts[1]),
                    "cid" => {
                        passport.country_id = String::from(parts[1]);
                    }
                    &_ => {
                        println!("unsupported id key: {:?}", parts[0]);
                    }
                }
            }
            Some(passport)
        }

        pub fn part1_valid(&self) -> bool {
            if self.height.is_empty() {
                return false;
            }
            if self.issue_year < 0 {
                return false;
            }
            if self.birth_year < 0 {
                return false;
            }
            if self.hair_colour.is_empty() {
                return false;
            }
            if self.expiration_year < 0 {
                return false;
            }
            if self.eye_colour.is_empty() {
                return false;
            }
            if self.id.is_empty() {
                return false;
            }
            return true;
        }

        pub fn part2_valid(&self) -> bool {
            if self.birth_year < 1920 || self.birth_year > 2002 {
                return false;
            }
            if self.issue_year < 2010 || self.issue_year > 2020 {
                return false;
            }

            if self.expiration_year < 2020 || self.expiration_year > 2030 {
                return false;
            }

            if self.height.contains("cm") {
                let height_number: i16 = self.height.replace("cm", "").parse().unwrap();
                if height_number < 150 || height_number > 193 {
                    return false;
                }
            } else if self.height.contains("in") {
                let height_number: i16 = self.height.replace("in", "").parse().unwrap();
                if height_number < 59 || height_number > 76 {
                    return false;
                }
            } else {
                return false;
            }
            let hair_colour_re = Regex::new(r"(^#[0-9a-f]{6})$").unwrap();
            if !hair_colour_re.is_match(&self.hair_colour) {
                return false;
            }
            let eye_colours: Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            let mut eye_colour_correct = false;
            for eye_colour in eye_colours {
                if self.eye_colour == eye_colour.to_string() {
                    eye_colour_correct = true;
                }
            }
            if !eye_colour_correct {
                return false;
            }

            let passport_id_re = Regex::new(r"(^[0-9]{9})$").unwrap();
            if !passport_id_re.is_match(&self.id) {
                return false;
            }

            return true;
        }
    }

    #[test]
    fn test_new_passport() {
        let fields: Vec<String> = vec![
            String::from("ecl:gry"),
            String::from("pid:860033327"),
            String::from("eyr:2020"),
            String::from("hcl:#fffffd"),
            String::from("byr:1937"),
            String::from("iyr:2017"),
            String::from("cid:147"),
            String::from("hgt:183cm"),
        ];
        if let Some(passport) = Passport::new(fields) {
            assert_eq!(passport.country_id, String::from("147"));
            assert_eq!(passport.id, String::from("860033327"));
            assert_eq!(passport.eye_colour, String::from("gry"));
            assert_eq!(passport.expiration_year, 2020);
            assert_eq!(passport.hair_colour, String::from("#fffffd"));
            assert_eq!(passport.birth_year, 1937);
            assert_eq!(passport.issue_year, 2017);
            assert_eq!(passport.height, String::from("183cm"));
        }
    }
}
