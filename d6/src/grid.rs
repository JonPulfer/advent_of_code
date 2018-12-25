use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
/// A location on the grid identified by a coordinate. The location can be described as finite if
/// it cannot be the closet location to an infinite number of locations.
struct Point {
    id: u16,
    x: i32,
    y: i32,
    finite: bool,
    allocations: usize,
}

impl Point {
    /// Return the Manhattan distance to the provided Point.
    fn distance_to_point(&self, p: Point) -> i32 {
        distance_between_points(self.clone(), p)
    }
}

#[derive(Debug)]
/// A collection of coordinates are provided as the puzzle input. It is not known whether any of
/// these are safe locations to provide the temporal device with as a destination. The challenge is
/// to find the location that is furthest away from other locations in the hope that this will
/// reduce the risk of being in or near a dangerous place.
pub struct Coordinates {
    /// This is a lookup map for all the points read from the input. The index key is just the
    /// numeric sequence number the point appears in the input list.
    points: HashMap<u16, Point>,
}

impl Coordinates {
    pub fn new() -> Coordinates {
        Coordinates {
            points: HashMap::new(),
        }
    }

    /// The input consists of coordinate pairs on each line. These points are stored and given a
    /// unique index.
    pub fn populate_from_input(&mut self, input: &str) {
        let mut count: u16 = 1;

        for line in input.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            let this_point = Point {
                id: count,
                x: parts[0].parse::<i32>().unwrap(),
                y: parts[1].trim_left().parse::<i32>().unwrap(),
                finite: false,
                allocations: 0,
            };
            self.points.insert(count, this_point);
            count += 1;
        }
    }

    /// Work through all the points and find the maximum x and y coordinates required.
    fn minimum_bounding_box_dimension(&self) -> (i32, i32) {
        let mut max_x: i32 = 0;
        let mut max_y: i32 = 0;

        for (_k, p) in self.points.iter() {
            if p.x > max_x {
                max_x = p.x.clone()
            }
            if p.y > max_y {
                max_y = p.y.clone()
            }
        }

        return (max_x, max_y);
    }
}

#[test]
fn test_minimum_bounding_box() {
    let mut g = Coordinates::new();
    g.points.insert(
        1,
        Point {
            id: 1,
            x: 3,
            y: 3,
            finite: false,
            allocations: 0,
        },
    );
    g.points.insert(
        2,
        Point {
            id: 2,
            x: 4,
            y: 2,
            finite: false,
            allocations: 0,
        },
    );
    g.points.insert(
        3,
        Point {
            id: 3,
            x: 2,
            y: 4,
            finite: false,
            allocations: 0,
        },
    );
    assert_eq!(g.minimum_bounding_box_dimension(), (4, 4));
}

#[derive(Debug)]
/// The 2D area that encloses all the coordinates. This is based on a minimum bounding box of all
/// the coordinates provided in the input for the puzzle.
pub struct Grid {
    matrix: Vec<Vec<Allocation>>,
    coords: Coordinates,
}

impl Grid {
    pub fn new(coords: Coordinates) -> Grid {
        let (max_x, max_y) = coords.minimum_bounding_box_dimension();
        let mut rows: Vec<Vec<Allocation>> = vec![];

        for y in 0..max_y {
            let mut row_allocations: Vec<Allocation> = vec![];
            for x in 0..max_x {
                let pos = Allocation::new(Point {
                    id: 0,
                    x,
                    y,
                    finite: false,
                    allocations: 0,
                });
                row_allocations.push(pos);
            }
            rows.push(row_allocations);
        }

        Grid {
            matrix: rows,
            coords,
        }
    }

    /// Work through the grid matrix and calculate the nearest coordinates for the location. Each
    /// location's allocation is updated to
    pub fn allocate_matrix_points(&mut self) {
        let (max_x, max_y) = self.coords.minimum_bounding_box_dimension();
        let mut allocated_on_edge: Vec<u16> = vec![];

        // Work through each location in the matrix and allocate closest points.
        for y in 0..max_y as usize {
            for x in 0..max_x as usize {
                // For each location in the matrix we check which points are the closest.
                for (_id, p) in self.coords.points.iter_mut() {
                    self.matrix[y][x].check_vicinity(p.clone());
                }

                // If only one point is the nearest we can count the allocation.
                if self.matrix[y][x].nearest.len() == 1 {
                    let p = self
                        .coords
                        .points
                        .get_mut(&self.matrix[y][x].nearest[0])
                        .unwrap();
                    p.allocations += 1;
                }

                // If this is a  perimeter location, exclude any points that are allocated as the
                // nearest points.
                if self.is_on_perimiter(x, y) {
                    for id in self.matrix[y][x].nearest.iter() {
                        allocated_on_edge.push(id.clone());
                    }
                }
            }
        }

        for (id, p) in self.coords.points.iter_mut() {
            let mut present = false;
            for infinite_point in allocated_on_edge.iter() {
                if *infinite_point == *id {
                    present = true;
                }
            }
            if !present {
                p.finite = true;
            }
        }
    }

    /// For the provided coordinate, calculate whether it is on the edge of the matrix. This will
    /// help identify whether any allocations for the location indicate that the coordinate is
    /// infinite.
    fn is_on_perimiter(&self, x: usize, y: usize) -> bool {
        let (max_x, max_y) = self.coords.minimum_bounding_box_dimension();

        if x == 0 || x == max_x as usize {
            return true;
        } else if y == 0 || y == max_y as usize {
            return true;
        }
        return false;
    }

    /// Once all the allocating has been performed, we can check what the largest finite allocation
    /// is.
    pub fn max_allocations_for_finite_point(&self) -> usize {
        let mut max_seen: usize = 0;

        for (_id, p) in self.coords.points.iter() {
            if p.allocations > max_seen && p.finite {
                max_seen = p.allocations
            }
        }

        return max_seen;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines: String = String::new();
        let (max_x, max_y) = self.coords.minimum_bounding_box_dimension();

        for y in 0..max_y as usize {
            let mut line = String::new();
            for x in 0..max_x as usize {
                line.push_str(self.matrix[y][x].placeholder.clone().as_str());
            }
            lines.push_str(format!("{}\n", line).as_str());
        }
        write!(f, "\n{}\n", lines)
    }
}

/// Using Manhattan distance (taxi cab distance), calculate the distance between point a and b.
fn distance_between_points(a: Point, b: Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

#[test]
fn test_distance_between_points() {
    let a1 = Point {
        id: 1,
        x: 1,
        y: 1,
        finite: false,
        allocations: 0,
    };
    let b1 = Point {
        id: 2,
        x: 2,
        y: 2,
        finite: false,
        allocations: 0,
    };
    assert_eq!(distance_between_points(a1, b1), 2);

    let a2 = Point {
        id: 3,
        x: 3,
        y: 5,
        finite: false,
        allocations: 0,
    };
    let b2 = Point {
        id: 4,
        x: 2,
        y: 7,
        finite: false,
        allocations: 0,
    };
    assert_eq!(distance_between_points(a2, b2), 3);
}

#[derive(Debug, Clone)]
/// For a point in a grid matrix, this holds the information about nearby points.
struct Allocation {
    location: Point,
    placeholder: String,
    lowest_distance: i32,
    nearest: Vec<u16>,
}

impl Allocation {
    fn new(location: Point) -> Allocation {
        Allocation {
            location,
            placeholder: " .".to_string(),
            lowest_distance: -1,
            nearest: vec![],
        }
    }

    /// Check whether the provided point is the nearest point the location of this allocation. This
    /// caters for multiple points being equally distant from the location.
    fn check_vicinity(&mut self, p: Point) {
        let vicinity = distance_between_points(self.location.clone(), p.clone());
        if self.lowest_distance < 0 || self.lowest_distance > vicinity {
            self.set_nearest_point(vicinity, p.clone());
        } else if self.lowest_distance == vicinity {
            self.add_nearest_point(p.clone());
        }
    }

    /// Set the absolute nearest individual point.
    fn set_nearest_point(&mut self, vicinity: i32, p: Point) {
        self.lowest_distance = vicinity;
        self.nearest = vec![];
        self.nearest.push(p.id);
        self.placeholder = format!("{}", p.id);
    }

    /// Extend the nearest points with the newly discovered equivalent one.
    fn add_nearest_point(&mut self, p: Point) {
        self.nearest.push(p.id);
        self.placeholder = " x".to_string();
    }
}
