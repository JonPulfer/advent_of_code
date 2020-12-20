use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input") {
        let mut source_lines: Vec<String> = vec![];
        for line in lines {
            if let Ok(extracted_line) = line {
                source_lines.push(extracted_line);
            }
        }
        let toboggan_map = toboggan_corp::TobogganMap::new(source_lines);
        let toboggans: Vec<toboggan_corp::Toboggan> = vec![
            toboggan_corp::Toboggan::new((1, 1)),
            toboggan_corp::Toboggan::new((3, 1)),
            toboggan_corp::Toboggan::new((5, 1)),
            toboggan_corp::Toboggan::new((7, 1)),
            toboggan_corp::Toboggan::new((1, 2)),
        ];

        let mut tree_counts: Vec<usize> = vec![];
        for mut toboggan in toboggans {
            let mut tree_count: usize = 0;
            let mut check_position = toboggan.next();
            while toboggan_map.still_on_map(check_position) {
                if toboggan_map.tree_at_location(check_position) {
                    tree_count += 1;
                }
                check_position = toboggan.next()
            }
            tree_counts.push(tree_count);
        }
        println!("part 1: {}", tree_counts[1]);
        let mut result: usize = 1;
        for count in &tree_counts {
            result = count * result;
        }
        println!("part 2: {}", result);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub mod toboggan_corp {
    pub struct TobogganMap {
        cells: Vec<Vec<char>>,
    }

    impl TobogganMap {
        pub fn new(rows: Vec<String>) -> TobogganMap {
            let mut cells: Vec<Vec<char>> = vec![];
            for row in rows {
                let mut map_row: Vec<char> = vec![];
                let mut chars = row.chars();
                while let Some(c) = chars.next() {
                    map_row.push(c);
                }
                cells.push(map_row);
            }

            TobogganMap { cells }
        }

        pub fn tree_at_location(&self, location: (usize, usize)) -> bool {
            self.cells[location.1][location.0 % self.cells[location.1].len()] == '#'
        }

        pub fn still_on_map(&self, location: (usize, usize)) -> bool {
            location.1 < self.cells.len()
        }
    }

    #[test]
    fn test_create_toboggan_map() {
        let tm: TobogganMap = TobogganMap::new(vec![
            String::from("..##......."),
            String::from("#...#...#.."),
        ]);
        assert_eq!(2, tm.cells.len());
        for row in tm.cells {
            assert_eq!(11, row.len());
        }
    }

    #[test]
    fn test_tree_at_location() {
        let tm: TobogganMap = TobogganMap::new(vec![
            String::from("..##......."),
            String::from("#...#...#.."),
        ]);
        assert_eq!(tm.tree_at_location((0, 0)), false);
        assert_eq!(tm.tree_at_location((11, 1)), true);
        assert_eq!(tm.tree_at_location((15, 1)), true);
        assert_eq!(tm.tree_at_location((16, 1)), false);
    }

    pub struct Toboggan {
        current_position: (usize, usize),
        moves: (usize, usize),
    }

    impl Toboggan {
        pub fn new(moves: (usize, usize)) -> Toboggan {
            Toboggan {
                current_position: (0, 0),
                moves,
            }
        }

        // calculate the next coordinates that the toboggan will slide to.
        pub fn next(&mut self) -> (usize, usize) {
            self.current_position = (
                self.current_position.0 + self.moves.0,
                self.current_position.1 + self.moves.1,
            );
            self.current_position
        }
    }

    #[test]
    fn test_toboggan_next() {
        let mut t = Toboggan::new((3, 1));
        assert_eq!(t.next(), (3, 1));
        assert_eq!(t.next(), (6, 2));
    }
}
