extern crate clap;
use clap::{App, Arg};

pub mod frequency;

fn main() {
    // set up the command line args
    let matches = App::new("p1")
        .version("1.0")
        .author("Jonathan Pulfer")
        .about("First puzzle solver for Advent Of Code 2018")
        .arg(
            Arg::with_name("start_frequency")
                .short("sf")
                .long("start_frequency")
                .value_name("START")
                .help("The initial frequency of the Chronal device. Defaults to 0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("adjustments")
                .short("adj")
                .long("adjustments")
                .value_name("ADJUSTMENTS")
                .help("Comma separated list of adjustments to make to the frequency")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("repeats")
                .short("r")
                .long("repeats")
                .value_name("REPEATS")
                .help("number of times to process adjustments to look for repeating frequency.")
                .takes_value(true),
        )
        .get_matches();

    // extract the starting frequency and convert to i64.
    let rcvd_freq: i64 = matches
        .value_of("start_frequency")
        .unwrap_or("0")
        .parse()
        .unwrap();

    // extract the number of repeats and convert to i64.
    let rcvd_repeats: i64 = matches.value_of("repeats").unwrap_or("0").parse().unwrap();

    // extract the adjustments.
    let adj = matches.value_of("adjustments").unwrap_or("0");

    // initialise the Frequency ready for adjustment.
    let mut freq: frequency::Frequency = frequency::Frequency::new(rcvd_freq);

    if rcvd_repeats > 0 {
        freq.process_adjustments_until_repeats(adj, rcvd_repeats);
    } else {
        freq.process_adjustments(adj);
    }
}
