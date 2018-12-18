use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

lazy_static! {
    static ref CLAIMRE: Regex = Regex::new(r"^#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)$").unwrap();
}

#[derive(Debug)]
/// A large, rectangular piece of fabric as discovered in a box lost in the mythical warehouse from
/// puzzle 2. This magical fabric is being fought over by the elves who are more than a little
/// fraught by a last minute requirement to make something (I can certainly sympathise with this
/// scenario!).
pub struct Fabric<'a> {
    grid: Vec<Vec<Allocation<'a>>>
}

impl<'a> Fabric<'a> {

    pub fn new() -> Fabric<'a> {
        let max_size = 1000;
        let grid: Vec<Vec<Allocation>> = vec!(vec!(Allocation {
            claims: vec!()
        }; max_size); max_size);

        return Fabric { grid };
    }

    /// Apply the claim to the fabric.
    pub fn allocate(&mut self, claim: Claim<'a>) {
        for coord in claim.coordinates() {
            self.grid[coord.from_top as usize][coord.from_left as usize]
                .claims.push(claim.clone());
        }
    }

    /// Work through all the grid squares and count those that are allocated more than once.
    pub fn count_over_allocated_squares(&self) -> i64 {
        let mut over_allocations: i64 = 0;
        for y in self.grid.iter() {
            for x in y {
                if x.claims.len() > 1 {
                    over_allocations += 1;
                }
            }
        }
        over_allocations
    }

    /// Search through the fabric allocations and record whether there are collisions for each claim
    /// seen. If there is a claim that has no collisions the claim id will be returned.
    pub fn find_claim_without_collisions(&self) -> String {
        let mut claim_collision_counts: HashMap<String, u32> = HashMap::new();
        let mut claim_seen: HashMap<String, bool> = HashMap::new();

        // Work through the allocated grid squares and record all claims seen in one map and in
        // another map, record colliding claims.
        for y in self.grid.iter() {
            for x in y {
                if x.claims.len() == 1 {
                    let seen = claim_seen.entry(x.claims[0].id.to_string())
                        .or_insert(false);
                    *seen = true;
                }
                if x.claims.len() > 1 {
                    for cl in x.claims.iter() {
                        let claim_id = String::from(cl.id);
                        let collided = claim_collision_counts.entry(claim_id)
                            .or_insert(0);
                        *collided += 1;
                    }
                }
            }
        }

        // Work through the two maps to see whether each seen claim appears in the map recording
        // those that collide with other claims.
        for (cl_id, _seen) in claim_seen {
            let possible_id = cl_id.clone();
            let collided = claim_collision_counts.entry(cl_id);
            match collided {
                Entry::Occupied(_) => {}
                Entry::Vacant(_) => { return possible_id }
            }
        }

        return "".to_string();
    }
}

#[derive(Clone,Debug)]
/// A rectangular claim to use a section of the fabric. The location is provided as the top left
/// coordinate. The dimension is provided as(width, height)
pub struct Claim<'a> {
    id: &'a str,
    top_left_position: Coordinate,
    size: Dimension,
}

impl<'a> Claim<'a> {
    /// Extract a claim from an input line.
    /// The input line looks like: -
    /// #1 @ 662,777: 18x27
    pub fn from_input_line(input: &'a str) -> Claim<'a> {
        let mut cl = Claim {
            id: "",
            top_left_position: Coordinate { from_top: 0, from_left: 0 },
            size: Dimension { width: 0, height: 0 },
        };
        let caps = CLAIMRE.captures(input);
        match caps {
            Some(line_caps) => {
                cl.id = line_caps.get(1).unwrap().as_str();
                let tlx = line_caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let tly = line_caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let top_left = Coordinate { from_left: tlx, from_top: tly };

                let dw = line_caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
                let dh = line_caps.get(5).unwrap().as_str().parse::<i32>().unwrap();
                let size = Dimension { width: dw, height: dh };

                cl.top_left_position = top_left;
                cl.size = size;

                return cl;
            }
            None => {}
        }
        return cl;
    }

    /// Provide a list of fabric coordinates required by this claim.
    fn coordinates(&self) -> Vec<Coordinate> {
        let mut results: Vec<Coordinate> = vec!();

        for y in self.top_left_position.from_top..(self.top_left_position.from_top +
            self.size.height) {
            for x in self.top_left_position.from_left..(self.top_left_position.from_left +
                self.size.width) {
                results.push(Coordinate { from_left: x, from_top: y });
            }
        }
        return results;
    }
}

#[test]
fn test_from_input_line() {
    let line = "#1 @ 662,777: 18x27";
    let cl = Claim::from_input_line(line);

    let target = Claim {
        id: "1",
        top_left_position: Coordinate { from_left: 662, from_top: 777 },
        size: Dimension { width: 18, height: 27 },
    };
    assert_eq!(cl.id, target.id);
    assert_eq!(cl.top_left_position.from_top, target.top_left_position.from_top);
    assert_eq!(cl.top_left_position.from_left, target.top_left_position.from_left);
    assert_eq!(cl.size.height, target.size.height);
    assert_eq!(cl.size.width, target.size.width);
}

#[test]
fn test_claim_coordinates() {
    let cl = Claim {
        id: "1",
        top_left_position: Coordinate { from_top: 2, from_left: 3 },
        size: Dimension { width: 3, height: 4 },
    };
    let stuff = cl.coordinates();
    assert_eq!(stuff.len(), 12);
    assert_eq!(stuff[0].from_left, Coordinate { from_top: 2, from_left: 3 }.from_left);
    assert_eq!(stuff[0].from_top, Coordinate { from_top: 2, from_left: 3 }.from_top);
    assert_eq!(stuff[11].from_top, Coordinate { from_top: 5, from_left: 5 }.from_top);
    assert_eq!(stuff[11].from_left, Coordinate { from_top: 5, from_left: 5 }.from_left);
}

#[derive(Clone,Debug)]
/// Location on a rectangular piece of fabric such as that fought over by the elves  tasked with
/// making a special suit for the big boss.
struct Coordinate {
    from_top: i32,
    from_left: i32,
}

#[derive(Clone,Debug)]
/// Size of a rectangular claim for a section of the fabric.
struct Dimension {
    width: i32,
    height: i32,
}

#[derive(Clone,Debug)]
/// For a given square inch on the fabric, this records each claim it falls within.
struct Allocation<'a> {
    claims: Vec<Claim<'a>>,
}