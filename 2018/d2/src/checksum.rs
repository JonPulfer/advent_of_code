use std::collections::HashMap;

/// Produce a simple checksum from the input.
///
/// The input is expected to contain lines of strings of chars that represent box references in
/// a warehouse. The format of the strings look like: -
///
/// pnebjqralgdgckzfifvtxywomu
/// pnebjqsalrdgcqzfihotxhwomu
/// pneajqsalrdgckzfihytxywoml
/// ...
///
/// The checksum is generated by multiplying the result of two counts.
///     1. count that have exactly two of any letter
///     2. count that have exactly three of any letter
///
/// The resulting total is returned as the checksum.
pub fn simple_checksum(input: &str) -> i64 {
    let mut two_count = 0;
    let mut three_count = 0;

    for line in input.lines() {
        let results = check_string(line);
        if results.two_letter {
            two_count += 1;
        }
        if results.three_letter {
            three_count += 1;
        }
    }

    return two_count * three_count;
}

/// Results of which counts the string matches.
struct CheckResults {
    two_letter: bool,
    three_letter: bool,
}

/// Review the input string and look to see whether it should be included in the counts used for
/// calculating a checksum.
fn check_string(input: &str) -> CheckResults {
    let mut letter_counts: HashMap<char, i64> = HashMap::new();

    // count each char repetition.
    for c in input.chars() {
        let letter = letter_counts.entry(c).or_insert(0);
        *letter += 1;
    }

    let mut found_two = false;
    let mut found_three = false;

    // review whether there are matches for either of the count criteria.
    for (_letter, count) in letter_counts {
        if count == 2 {
            found_two = true;
        }
        if count == 3 {
            found_three = true;
        }
    }

    return CheckResults {
        two_letter: found_two,
        three_letter: found_three,
    };
}

#[test]
fn test_should_match_two_count() {
    let inp: &str = "aabcd";
    let results = check_string(inp);
    assert_eq!(results.two_letter, true);
    assert_eq!(results.three_letter, false);
}

#[test]
fn test_should_match_three_count() {
    let inp: &str = "abafad";
    let results = check_string(inp);
    assert_eq!(results.two_letter, false);
    assert_eq!(results.three_letter, true);
}

#[test]
fn test_should_match_two_and_three_count() {
    let inp: &str = "ababad";
    let results = check_string(inp);
    assert_eq!(results.two_letter, true);
    assert_eq!(results.three_letter, true);
}