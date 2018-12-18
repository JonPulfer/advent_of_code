#[derive(Debug)]
/// Raw component found in the special suit fabric. It's created from a raw string that lists all
/// of the units that make up the polymer. These, once triggered, react with each other to produce
/// the final polymer. Each unit indicates a polarity by it's case. Two units with the same type
/// (letter) but opposite polarity (upper and lower case) will cancel each other out and be removed
/// from the polymer string.
pub struct Polymer {
    raw: String,
    last: String,
    refined: bool,
    shortest: usize,
    shortest_last: String,
}

impl Polymer {
    pub fn new(input: &str) -> Polymer {
        Polymer {
            raw: input.to_string(),
            last: input.to_string(),
            refined: false,
            shortest: 0,
            shortest_last: String::new(),
        }
    }

    /// Refine the polymer by reacting all the units in the raw polymer. This reaction cascades
    /// until there are no reactions left.
    pub fn refine(&mut self) -> String {
        while self.refined == false {
            self.react();
        }
        return self.last.clone();
    }

    /// Perform a single round of reactions on the polymer.
    fn react(&mut self) {
        let mut result = String::new();
        let source_chars: Vec<char> = self.last.chars().collect();

        let mut skip = false;

        for i in 0..source_chars.len() {
            if skip {
                skip = false;
                continue;
            }
            if i == source_chars.len() - 1 {
                result.push(source_chars[i]);
                continue;
            }
            if !chars_react(source_chars[i], source_chars[i + 1]) {
                result.push(source_chars[i]);
            } else {
                skip = true;
            }
        }

        if result == self.last {
            self.refined = true;
            return;
        }
        self.last = result.clone();
    }

    /// By dropping one unit (of both polarities) it is possible to allow more reactions to occur
    /// and yield a shorter polymer.
    pub fn optimum_refine_by_dropping_a_unit(&mut self) -> String {
        self.shortest = self.last.len();

        for num in 97..97+26 as u8 {
            let lower = format!("{}", num as char);
            let upper = lower.to_uppercase();

            let new_raw = self.raw.replace(lower.as_str(), "")
                .replace(upper.as_str(), "");

            let mut new_p = Polymer::new(new_raw.as_str());
            let new_refined = new_p.refine();
            if new_refined.len() < self.shortest {
                self.shortest = new_refined.len();
                self.shortest_last = new_refined;
            }
        }

        return self.shortest_last.clone();
    }
}

#[test]
fn test_refine_one_pass() {
    let mut p = Polymer::new("abBcd");
    assert_eq!(p.refine(), String::from("acd"));
}

#[test]
fn test_refine_two_pass() {
    let mut p = Polymer::new("abBAd");
    assert_eq!(p.refine(), String::from("d"));
}

#[test]
fn test_using_larger_example() {
    let mut p = Polymer::new("dabAcCaCBAcCcaDA");
    assert_eq!(p.refine(), String::from("dabCBAcaDA"));
}

/// When subtracting the lower case char from the upper case, this is the result if the chars are
/// the same letter just a different case.
const CHAR_SPACE: u16 = 32;

#[allow(unused_assignments)]
/// Determine whether the two characters provided will react and cancel each other out.
fn chars_react(a: char, b: char) -> bool {
    let mut spacing = 0;
    let anum = a as u16;
    let bnum = b as u16;

    if anum > bnum {
        spacing = anum - bnum;
    } else {
        spacing = bnum - anum;
    }

    return spacing == CHAR_SPACE;
}

#[test]
fn test_chars_react_matches() {
    let lower = 'a';
    let upper = 'A';
    assert_eq!(chars_react(lower, upper), true);
    assert_eq!(chars_react(upper, lower), true);
}

#[test]
fn test_chars_react_pair_another_letter() {
    let lower = 'z';
    let upper = 'Z';
    assert_eq!(chars_react(lower, upper), true);
    assert_eq!(chars_react(upper, lower), true);
}

#[test]
fn test_chars_react_non_reacting_pair() {
    let lower = 'a';
    let upper = 'B';
    assert_eq!(chars_react(lower, upper), false);
    assert_eq!(chars_react(upper, lower), false);
}

#[test]
fn test_chars_react_non_reacting_pair_but_one() {
    let lower = 'a';
    let upper = 'C';
    assert_eq!(chars_react(lower, upper), false);
    assert_eq!(chars_react(upper, lower), false);
}

#[test]
fn test_chars_react_non_reacting_same_letter() {
    let lower = 'a';
    let upper = 'a';
    assert_eq!(chars_react(lower, upper), false);
    assert_eq!(chars_react(upper, lower), false);
}