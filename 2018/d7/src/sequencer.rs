use regex::Regex;

lazy_static! {
    static ref PAIRRE: Regex =
        Regex::new(r"^Step\s(?P<child>\w)\s.*\sstep\s(?P<parent>\w)\s.*$").unwrap();
}

#[derive(Debug, Clone)]
/// Related pairing of a parent and child defining the directed link between two nodes.
struct Pair {
    parent: String,
    child: String,
}

impl Pair {
    /// Create a directed relationship from a line taken from the problem input.
    ///
    /// The line looks like: -
    ///
    /// `Step F must be finished before step N can begin.`
    pub fn new_from_line(line: &str) -> Pair {
        let caps = PAIRRE.captures(line).unwrap();
        let parent = caps.name("parent").unwrap().as_str();
        let child = caps.name("child").unwrap().as_str();

        Pair {
            parent: parent.to_string(),
            child: child.to_string(),
        }
    }
}

#[test]
fn test_new_pair_from_line() {
    let p = Pair::new_from_line("Step F must be finished before step N can begin.");
    assert_eq!("N".to_string(), p.parent);
    assert_eq!("F".to_string(), p.child);
}

#[derive(Debug)]
/// Sequencer performs the following basic algorithm
///
/// Extract pairs of steps from puzzle input to obtain a list of step dependencies
/// While there are uncompleted steps:
/// Find the next available step from the list of step dependencies
///     Remove step from uncompleted set
///     Append step to a list of completed steps
///     Remove all step dependencies which require the completed step
/// Return list of completed steps
pub struct Sequencer {
    uncompleted_steps: Vec<Pair>,
    completed_steps: Vec<String>,
}

impl Sequencer {
    /// Read the lines from the input of instruction sequences and build the dependency graph. This
    /// creates a sequencer that understands the relationship between each instruction step.
    pub fn new_from_input(input: &str) -> Sequencer {
        let mut uncompleted_steps: Vec<Pair> = vec![];
        let completed_steps: Vec<String> = vec![];
        for line in input.lines() {
            let uncompleted_step = Pair::new_from_line(&line);
            uncompleted_steps.push(uncompleted_step);
        }

        Sequencer {
            uncompleted_steps,
            completed_steps,
        }
    }

    pub fn sequence(&mut self) -> String {
        while !self.uncompleted_steps.is_empty() {
            let next_steps = self.available_steps();
            if next_steps.len() > 0 {
                self.completed_steps.push(String::from(&next_steps[0]));
                let mut still_uncompleted_steps: Vec<Pair> = vec![];
                for incomplete_step in &self.uncompleted_steps {
                    if !incomplete_step.child.eq(&next_steps[0]) {
                        still_uncompleted_steps.push(Pair{
                            parent: String::from(&incomplete_step.parent),
                            child: String::from(&incomplete_step.child)
                        })
                    }
                }
                if self.uncompleted_steps.len() == 1 {
                    self.completed_steps.push(String::from(&self.uncompleted_steps[0].parent))
                }
                self.uncompleted_steps = still_uncompleted_steps
            }
        }
        self.completed_steps.join("")
    }

    fn available_steps(&self) -> Vec<String> {
        let mut results: Vec<String> = vec![];

        let mut child_steps: Vec<String> = vec![];
        let mut parent_steps: Vec<String> = vec![];
        for pair in &self.uncompleted_steps {
            if self.completed_steps.contains(&pair.child) {
                results.push(String::from(&pair.parent));
                continue;
            }

            child_steps.push(String::from(&pair.child));
            parent_steps.push(String::from(&pair.parent));
        }

        for child_step in child_steps {
            if !parent_steps.contains(&child_step) && !results.contains(&child_step) {
                results.push(String::from(child_step))
            }
        }
        results.sort();
        results
    }
}

#[test]
fn first_available_steps_from_example() {
    let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
    let seq = Sequencer::new_from_input(input);
    let expected_result: Vec<String> = vec![String::from("C")];
    assert_eq!(expected_result, seq.available_steps());
}

#[test]
fn test_sequencer_using_example() {
    let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
    let mut seq = Sequencer::new_from_input(input);
    assert_eq!("CABDFE", seq.sequence());
}
