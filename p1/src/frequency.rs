use std::error;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
/// Frequency chronal wrist device is operating at.
pub struct Frequency {
    pub value: i64,
    pub seen_values: HashMap<i64, i16>,
}

impl Frequency {
    pub fn new(value: i64) -> Frequency {
        Frequency {
            value,
            seen_values: HashMap::new(),
        }
    }

    /// adjust_frequency works through a list of drift values and adjusts accordingly.
    fn adjust_frequency(&mut self, adjustments: Vec<i32>) {
        let mut result: i64 = self.value;
        for i in adjustments {
            result += i as i64;
        }
        self.value = result;
    }

    /// adjust frequency recursively until the repeated frequency is found or limit_iterations
    /// reaches 0.
    fn adjust_frequency_until_repeats(&mut self, adjustments: &Vec<i32>, mut limit_iterations: i64) -> i64 {
        let mut result: i64 = self.value;
        for i in adjustments {
            result += *i as i64;
            let seen = self.seen_values.entry(result).or_insert(0);
            *seen += 1;
            if *seen > 1 {
                println!("repeat: {}", result);
                return 0;
            }
        }
        self.value = result;
        if limit_iterations > 0 {
            limit_iterations -= 1;
            limit_iterations = self.adjust_frequency_until_repeats(adjustments, limit_iterations);

        } else {
            println!("no repeat found");
        }
        limit_iterations
    }

    /// process_adjustments takes a comma separated list of adjustments as supplied via the
    /// command line argument and applies them to the Frequency.
    pub fn process_adjustments(&mut self, supplied_adjustments: &str) {
        match parse_frequency_adjustments(supplied_adjustments) {
            Ok(results) => {
                self.adjust_frequency(results);
                println!("frequency {}", self.value);
            }
            Err(fail) => {
                println!("{}", fail);
            }
        }
    }

    /// Process the adjustments repetitively until either the limit is reached or the repeat is
    /// found. This calls the recursive version of the frequency adjustment with depth limited to
    /// the user supplied value.
    pub fn process_adjustments_until_repeats(&mut self, supplied_adjustments: &str,
                                             limit_iterations: i64) {
        match parse_frequency_adjustments(supplied_adjustments) {
            Ok(results) => {
                self.adjust_frequency_until_repeats(&results,
                                                    limit_iterations);
                println!("frequency {}", self.value);
            }
            Err(fail) => {
                println!("{}", fail);
            }
        }
    }
}

#[test]
fn test_two_frequency_changes() {
    let inp: Vec<i32> = vec!(1, -2);
    let mut st: Frequency = Frequency::new(0);

    st.adjust_frequency(inp);
    assert_eq!(st.value, -1);
}

#[test]
fn test_three_frequency_changes() {
    let inp: Vec<i32> = vec!(500, -550, 50);
    let mut st: Frequency = Frequency::new(0);

    st.adjust_frequency(inp);
    assert_eq!(st.value, 0);
}

/// parse_frequency_adjustments from a comma separated string of values into an array..
fn parse_frequency_adjustments(input: &str) -> Result<Vec<i32>, AdjustmentError> {
    let mut results: Vec<i32> = vec!();
    let values: Vec<&str> = input.split(",").collect();

    for val in values {
        let v = val.parse::<i32>();
        if v.is_err() {
            return Err(AdjustmentError::new(val, "could not parse as i32"));
        }
        results.push(v.unwrap());
    }

    return Ok(results);
}

#[derive(Debug, Clone)]
/// A base error to return to identify that there has been a condition found that does not allow us
/// to continue processing.
struct AdjustmentError<'a> {
    source: &'a str,
    message: &'a str,
}

impl<'a> AdjustmentError<'a> {
    pub fn new(source: &'a str, message: &'a str) -> AdjustmentError<'a> {
        AdjustmentError {
            source,
            message,
        }
    }

    fn reason(&self) -> String {
        format!("{}: {}", self.message, self.source)
    }
}

impl<'a> fmt::Display for AdjustmentError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to process adjustment: {}", self.reason())
    }
}

impl<'a> error::Error for AdjustmentError<'a> {
    fn description(&self) -> &str {
        "failed to process adjustment"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}