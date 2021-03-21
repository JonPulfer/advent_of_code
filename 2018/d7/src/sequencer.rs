use std::collections::HashMap;

use petgraph::graph::NodeIndex;
use petgraph::visit::DfsPostOrder;
use petgraph::Graph;
use petgraph::Incoming;
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
/// From a raw list of directed pairs, a Sequencer is able to build a directed graph of the node
/// relationships. The instructions that form the puzzle input are performed in a strict sequence
/// that requires all child elements are completed before a parent can be completed. All child
/// elements are also explored in alphabetical order.
pub struct Sequencer {
    tree: Graph<u8, u8>,
}

impl Sequencer {
    /// Read the lines from the input of instruction sequences and build the dependency graph. This
    /// creates a sequencer that understands the relationship between each instruction step.
    pub fn new_from_input(input: &str) -> Sequencer {
        let mut deps = Graph::<u8, u8>::new();
        let mut node_indexes: HashMap<u8, NodeIndex<u32>> = HashMap::new();

        // Work through the lines in the input extracting the nodes and also the edges. These need
        // to be deduplicated using a hash map to make sure that regardless of which side of the
        // relation we see it, we only record it as one node.
        for line in input.lines() {
            let p = Pair::new_from_line(line);
            let pn: NodeIndex<u32>;
            let cn: NodeIndex<u32>;

            let parent_match = node_indexes.get(&convert_to_node_weight(&p.parent));
            match parent_match {
                Some(pn_found) => {
                    pn = *pn_found;
                }
                None => {
                    pn = deps.add_node(convert_to_node_weight(&p.parent));
                    node_indexes.insert(convert_to_node_weight(&p.parent), pn);
                }
            }

            let child_match = node_indexes.get(&convert_to_node_weight(&p.child));
            match child_match {
                Some(cn_found) => {
                    cn = *cn_found;
                }
                None => {
                    cn = deps.add_node(convert_to_node_weight(&p.child));
                    node_indexes.insert(convert_to_node_weight(&p.child), cn);
                }
            }

            deps.add_edge(pn, cn, 1);
        }

        Sequencer { tree: deps }
    }

    pub fn dfs(&self) -> String {
        let mut thing = self.tree.externals(Incoming);
        let mut d = DfsPostOrder::new(&self.tree, thing.next().unwrap());
        let mut nodes: Vec<String> = vec![];
        while let Some(c) = d.next(&self.tree) {
            nodes.push(convert_from_node_weight(self.tree[c]));
        }
        return nodes.join("");
    }
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
    let seq = Sequencer::new_from_input(input);
    assert_eq!("CABDFE", seq.dfs());
}

fn convert_to_node_weight(original: &String) -> u8 {
    let char = original.chars().next().unwrap();
    91 - char as u8
}

#[test]
fn test_convert_to_node_weight() {
    let letter_a = "A".to_string();
    assert_eq!(26, convert_to_node_weight(&letter_a));

    let letter_b = "B".to_string();
    assert_eq!(25, convert_to_node_weight(&letter_b));

    let letter_z = "Z".to_string();
    assert_eq!(1, convert_to_node_weight(&letter_z));
}

fn convert_from_node_weight(original: u8) -> String {
    let mut result: String = String::new();
    let calculated = 91 - original;
    result.push(calculated as char);
    return result;
}

#[test]
fn test_convert_from_node_weight() {
    let value_a = convert_to_node_weight(&"A".to_string());
    assert_eq!("A".to_string(), convert_from_node_weight(value_a));

    let value_b = convert_to_node_weight(&"B".to_string());
    assert_eq!("B".to_string(), convert_from_node_weight(value_b));
}
