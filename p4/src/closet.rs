use std::collections::HashMap;
use chrono::prelude::*;
use regex::Regex;


lazy_static! {
    static ref SHIFTSTARTRE: Regex = Regex::new(r"^Guard #(\d+)\sbegins shift$").unwrap();
    static ref SLEEPSTARTRE: Regex = Regex::new(r"^falls asleep$").unwrap();
    static ref SLEEPENDRE: Regex = Regex::new(r"wakes up").unwrap();
}

#[derive(Debug)]
/// It seems the Elf guards are well known for sleeping on the job. Sadly this also seems to be in
/// a predictable way, thus tempting someone to do some research on their pattern.
struct Guard {
    id: String,
    sleep_count_per_minute: HashMap<u32, i32>,
    sleep_average_per_shift: i64,
    shifts: i64,
    total_sleep: i64,
}

impl Guard {
    fn new(id: String) -> Guard {
        Guard {
            id,
            sleep_average_per_shift: 0,
            sleep_count_per_minute: HashMap::new(),
            shifts: 0,
            total_sleep: 0,
        }
    }

    /// Record that a shift was started.
    fn report_shift(&mut self) {
        self.shifts += 1;
    }

    /// Record a sleep period within a shift. There may be more than one sleep seen during a shift.
    fn report_sleep(&mut self, sleep_start: DateTime<Utc>, sleep_end: DateTime<Utc>) {

        // Update the total sleep seen for this guard.
        self.total_sleep += sleep_end.signed_duration_since(sleep_start).num_minutes();

        // Update the sleep average.
        self.sleep_average_per_shift = self.total_sleep / self.shifts;


        for m in sleep_start.minute()..sleep_end.minute() {
            let sleep_slot = self.sleep_count_per_minute.entry(m).or_insert(0);
            *sleep_slot += 1;
        }
    }

    /// Find the minute the guard most frequently sleeps during.
    fn frequent_sleep_minute(&self) -> Option<u32> {
        let mut result: u32 = 0;
        let mut highest: i32 = 0;
        for (k, v) in self.sleep_count_per_minute.iter() {
            if v > &highest {
                highest = v.clone();
                result = k.clone();
            }
        }

        if highest == 0 {
            return None;
        }

        Some(result)
    }
}

#[test]
fn test_guard_record_shift() {
    let mut guard = Guard::new(String::from("1"));
    guard.report_shift();
    assert_eq!(guard.shifts, 1);
}

#[test]
fn test_guard_record_multiple_shifts() {
    let mut guard = Guard::new(String::from("1"));
    guard.report_shift();
    guard.report_shift();
    assert_eq!(guard.shifts, 2);
}

#[test]
fn test_guard_record_shift_and_one_sleep() {
    let mut guard = Guard::new(String::from("1"));
    guard.report_shift();
    let sleep_start = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let sleep_end = Utc.ymd(2014, 11, 28).and_hms(12, 3, 9);
    guard.report_sleep(sleep_start, sleep_end);
    assert_eq!(guard.shifts, 1);
    assert_eq!(guard.sleep_average_per_shift, 3);
    assert_eq!(guard.total_sleep, 3);
    assert_eq!(guard.sleep_count_per_minute.len(), 3);
    assert_eq!(*guard.sleep_count_per_minute.get(&0).unwrap(), 1);
    assert_eq!(*guard.sleep_count_per_minute.get(&1).unwrap(), 1);
    assert_eq!(*guard.sleep_count_per_minute.get(&2).unwrap(), 1);
}

#[test]
fn test_guard_record_shift_and_two_sleeps() {
    let mut guard = Guard::new(String::from("1"));
    guard.report_shift();
    let sleep_start = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let sleep_end = Utc.ymd(2014, 11, 28).and_hms(12, 3, 9);
    guard.report_sleep(sleep_start, sleep_end);
    let sleep_two_start = Utc.ymd(2014, 11, 28).and_hms(12, 12, 9);
    let sleep_two_end = Utc.ymd(2014, 11, 28).and_hms(12, 13, 9);
    guard.report_sleep(sleep_two_start, sleep_two_end);

    assert_eq!(guard.shifts, 1);
    assert_eq!(guard.sleep_average_per_shift, 4);
    assert_eq!(guard.total_sleep, 4);
    assert_eq!(guard.sleep_count_per_minute.len(), 4);
    assert_eq!(*guard.sleep_count_per_minute.get(&0).unwrap(), 1);
    assert_eq!(*guard.sleep_count_per_minute.get(&1).unwrap(), 1);
    assert_eq!(*guard.sleep_count_per_minute.get(&2).unwrap(), 1);
    assert_eq!(*guard.sleep_count_per_minute.get(&12).unwrap(), 1);
}

#[test]
fn test_guard_record_two_shifts_and_two_sleeps() {
    let mut guard = Guard::new(String::from("1"));
    guard.report_shift();
    let sleep_start = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let sleep_end = Utc.ymd(2014, 11, 28).and_hms(12, 3, 9);
    guard.report_sleep(sleep_start, sleep_end);
    guard.report_shift();
    let sleep_two_start = Utc.ymd(2014, 11, 28).and_hms(12, 12, 9);
    let sleep_two_end = Utc.ymd(2014, 11, 28).and_hms(12, 13, 9);
    guard.report_sleep(sleep_two_start, sleep_two_end);

    assert_eq!(guard.shifts, 2);
    assert_eq!(guard.sleep_average_per_shift, 2);
    assert_eq!(guard.total_sleep, 4);
    assert_eq!(guard.sleep_count_per_minute.len(), 4);
    assert_eq!(*guard.sleep_count_per_minute.get(&0).unwrap(), 1);
    assert_eq!(*guard.sleep_count_per_minute.get(&1).unwrap(), 1);
    assert_eq!(*guard.sleep_count_per_minute.get(&2).unwrap(), 1);
    assert_eq!(*guard.sleep_count_per_minute.get(&12).unwrap(), 1);
}

#[test]
fn test_guard_record_two_shifts_and_two_sleeps_with_overlap() {
    let mut guard = Guard::new(String::from("1"));
    guard.report_shift();
    let sleep_start = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let sleep_end = Utc.ymd(2014, 11, 28).and_hms(12, 3, 9);
    guard.report_sleep(sleep_start, sleep_end);
    guard.report_shift();
    let sleep_two_start = Utc.ymd(2014, 11, 28).and_hms(12, 2, 9);
    let sleep_two_end = Utc.ymd(2014, 11, 28).and_hms(12, 4, 9);
    guard.report_sleep(sleep_two_start, sleep_two_end);

    assert_eq!(guard.shifts, 2);
    assert_eq!(guard.sleep_average_per_shift, 2);
    assert_eq!(guard.total_sleep, 5);
    assert_eq!(guard.sleep_count_per_minute.len(), 4);
    assert_eq!(*guard.sleep_count_per_minute.get(&0).unwrap(), 1);
    assert_eq!(*guard.sleep_count_per_minute.get(&1).unwrap(), 1);
    assert_eq!(*guard.sleep_count_per_minute.get(&2).unwrap(), 2);
    assert_eq!(*guard.sleep_count_per_minute.get(&3).unwrap(), 1);
}

#[test]
fn test_guard_frequent_minute() {
    let mut guard = Guard::new(String::from("1"));
    guard.report_shift();
    let sleep_start = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let sleep_end = Utc.ymd(2014, 11, 28).and_hms(12, 3, 9);
    guard.report_sleep(sleep_start, sleep_end);
    guard.report_shift();
    let sleep_two_start = Utc.ymd(2014, 11, 28).and_hms(12, 2, 9);
    let sleep_two_end = Utc.ymd(2014, 11, 28).and_hms(12, 4, 9);
    guard.report_sleep(sleep_two_start, sleep_two_end);

    assert_eq!(guard.shifts, 2);
    assert_eq!(guard.sleep_average_per_shift, 2);
    assert_eq!(guard.total_sleep, 5);
    assert_eq!(guard.sleep_count_per_minute.len(), 4);
    assert_eq!(*guard.sleep_count_per_minute.get(&0).unwrap(), 1);
    assert_eq!(*guard.sleep_count_per_minute.get(&1).unwrap(), 1);
    assert_eq!(*guard.sleep_count_per_minute.get(&2).unwrap(), 2);
    assert_eq!(*guard.sleep_count_per_minute.get(&3).unwrap(), 1);
    assert_eq!(guard.frequent_sleep_minute(), Some(2));
}

#[derive(Debug)]
/// Suit manufacturing lab where the extra special suit for the big boss takes the appropriate
/// shape. This is protected by a solitary Elf guard past which access is only possible when they
/// are asleep.
pub struct Lab {
    guards: HashMap<String, Guard>,
    journal: Journal,
}

impl Lab {
    pub fn new() -> Lab {
        Lab {
            guards: HashMap::new(),
            journal: Journal::new(),
        }
    }

    /// The sleep pattern input read from the wall in the supply closet lists all the shifts the
    /// guards perform during the midnight hour. The information logged includes the start and
    /// finish time of both the shift and any periods they slept during the shift. These are
    /// recorded as whole minutes. If all lines are successfully read this returns the number of
    /// entries recorded.
    //
    // The input looks like: -
    //
    // [1518-11-22 00:49] wakes up
    // [1518-05-18 00:01] Guard #1171 begins shift
    // [1518-11-20 00:28] wakes up
    // [1518-10-27 00:37] wakes up
    // [1518-08-14 00:39] falls asleep
    // [1518-09-08 00:51] falls asleep
    // [1518-07-27 00:57] wakes up
    // [1518-10-21 00:00] Guard #2699 begins shift
    pub fn read_input_into_journal(&mut self, input: String) -> Option<i32> {

        // Create our journal from the log found on the wall. Once created, we sort this
        // chronologically to make it easier to analyse.
        let mut journal = Journal::new();
        for line in input.lines() {
            match JournalEntry::from_input_line(line) {
                Some(this_entry) => {
                    journal.entries.push(this_entry);
                }
                None => {
                    return None;
                }
            }
        }

        // Explicitly sort by just the time field of the JournalEntry to correct the order of the
        // entries.
        journal.entries.sort_by(|a, b| a.time.cmp(&b.time));

        self.journal = journal;

        return Some(self.journal.entries.len() as i32);
    }

    pub fn calculate_guard_sleep_patterns(&mut self) {

        // find a start entry for a guard entry
        //    read subsequent lines and record any sleep periods exploding time period discovered
        //    by minute
        // end guard entry when shift end entry found
        let mut guard_id = String::new();
        let mut asleep = false;
        let mut sleep_start = Utc::now();

        for entry in self.journal.entries.iter() {
            match SHIFTSTARTRE.captures(entry.line.as_str()) {
                Some(shift_start_line) => {
                    guard_id = String::from(shift_start_line.get(1).map_or("", |m| m.as_str()));
                    let guard = self.guards.entry(guard_id.clone())
                        .or_insert(Guard::new(guard_id.clone()));
                    guard.report_shift();
                    continue;
                }
                None => {}
            }

            if SLEEPSTARTRE.is_match(entry.line.as_str()) {
                sleep_start = entry.time;
                asleep = true;
            }

            if asleep && SLEEPENDRE.is_match(entry.line.as_str()) {
                let guard = self.guards.entry(guard_id.clone())
                    .or_insert(Guard::new(guard_id.clone()));
                guard.report_sleep(sleep_start, entry.time);
                asleep = false;
            }
        }
    }

    /// count the number of guards seen in the journal.
    pub fn number_of_guards(&self) -> usize {
        self.guards.len()
    }

    /// Find the guard that sleeps the most and the offending minute.
    pub fn target_guard_and_minute(&self) -> (String, u32) {
        let mut guard_id = String::new();
        let mut most_sleep: i64 = 0;
        let mut peak_minute: u32 = 0;
        for (_k, v) in self.guards.iter() {
            if v.total_sleep > most_sleep {
                guard_id = v.id.clone();
                most_sleep = v.total_sleep;
                match v.frequent_sleep_minute() {
                    Some(target_minute) => {
                        peak_minute = target_minute;
                    }
                    None => {}
                }
            }
        }
        return (guard_id, peak_minute);
    }

    /// Find the guard that most regularly sleeps for a particular minute.
    pub fn target_regular_sleeping_guard_and_minute(&self) -> (String, u32) {
        let mut guard_id = String::new();
        let mut peak_per_minute: i32 = 0;
        let mut peak_minute: u32 = 0;
        for (_k, v) in self.guards.iter() {
            match v.frequent_sleep_minute() {
                Some(target_minute) => {
                    if v.sleep_count_per_minute[&target_minute] > peak_per_minute {
                        guard_id = v.id.clone();
                        peak_minute = target_minute;
                        peak_per_minute = v.sleep_count_per_minute[&target_minute];
                    }
                }
                None => {}
            }
        }
        return (guard_id, peak_minute);
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
/// An entry from the journal as read from the wall.
struct JournalEntry {
    time: DateTime<Utc>,
    line: String,
}

impl JournalEntry {
    fn new() -> JournalEntry {
        JournalEntry {
            time: Utc::now(),
            line: String::new(),
        }
    }

    /// create a JournalEntry from an individual line found in the input.
    fn from_input_line(line: &str) -> Option<JournalEntry> {
        let this_line = String::from(line);
        let line_parts: Vec<&str> = this_line.split(|c| c == ']' || c == '[').collect();

        let mut this_entry = JournalEntry::new();
        match Utc.datetime_from_str(line_parts[1], "%Y-%m-%d %H:%M") {
            Ok(the_time) => {
                this_entry.time = the_time;
            }
            Err(the_err) => {
                println!("failed to parse time: {}", the_err.to_string());
                return None;
            }
        }

        this_entry.line = line_parts[2].trim_left().to_string();

        return Some(this_entry);
    }
}

#[test]
fn test_journal_entry_from_input_line() {
    let input_line = "[1518-05-18 00:01] Guard #1171 begins shift";
    assert_eq!(JournalEntry::from_input_line(input_line), Some(
        JournalEntry {
            time: "1518-05-18T00:01:00Z".parse::<DateTime<Utc>>().unwrap(),
            line: String::from("Guard #1171 begins shift"),
        }
    ))
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
/// The complete journal as read from the wall. This enables sorting of the journal entries by time.
struct Journal {
    entries: Vec<JournalEntry>,
}

impl Journal {
    pub fn new() -> Journal {
        Journal {
            entries: vec!(),
        }
    }
}
