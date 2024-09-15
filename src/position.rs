use deepsize::DeepSizeOf;
use std::{fmt, hash::Hash};

#[derive(Clone, Eq, Hash, PartialEq, DeepSizeOf, PartialOrd, Ord)]
pub struct Position {
    pub row: u8,
    pub col: u8,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position {
            row: row as u8,
            col: col as u8,
        }
    }

    pub fn at(row: u8, col: u8) -> Self {
        Position { row, col }
    }

    pub fn neighbors(&self, width: usize, height: usize) -> Vec<Position> {
        let mut neighbors = self.cardinal_neighbors(width, height);
        neighbors.reserve(4);

        if let Some(p) = self.north_west(width, height) {
            neighbors.push(p);
        }
        if let Some(p) = self.north_east(width, height) {
            neighbors.push(p);
        }
        if let Some(p) = self.south_west(width, height) {
            neighbors.push(p);
        }
        if let Some(p) = self.south_east(width, height) {
            neighbors.push(p);
        }

        neighbors
    }

    pub fn cardinal_neighbors(&self, width: usize, height: usize) -> Vec<Position> {
        let mut neighbors: Vec<Position> = Vec::with_capacity(4);

        if let Some(p) = self.north(width, height) {
            neighbors.push(p);
        }
        if let Some(p) = self.east(width, height) {
            neighbors.push(p);
        }
        if let Some(p) = self.west(width, height) {
            neighbors.push(p);
        }
        if let Some(p) = self.south(width, height) {
            neighbors.push(p);
        }

        neighbors
    }

    pub fn north_west(&self, _width: usize, _height: usize) -> Option<Position> {
        if self.row == 0 || self.col == 0 {
            return None;
        }
        Some(Position::at(self.row - 1, self.col - 1))
    }

    pub fn north(&self, _width: usize, _height: usize) -> Option<Position> {
        if self.row == 0 {
            return None;
        }
        Some(Position::at(self.row - 1, self.col))
    }

    pub fn north_east(&self, width: usize, _height: usize) -> Option<Position> {
        if self.row == 0 {
            return None;
        }
        let c = Position::at(self.row - 1, self.col + 1);
        if c.col as usize > width {
            return None;
        }
        Some(c)
    }

    pub fn west(&self, _width: usize, _height: usize) -> Option<Position> {
        if self.col == 0 {
            return None;
        }
        Some(Position::at(self.row, self.col - 1))
    }

    pub fn east(&self, width: usize, _height: usize) -> Option<Position> {
        let c = Position::at(self.row, self.col + 1);
        if c.col as usize > width {
            return None;
        }
        Some(c)
    }

    pub fn south_west(&self, _width: usize, height: usize) -> Option<Position> {
        if self.row as usize == height || self.col == 0 {
            return None;
        }
        Some(Position::at(self.row + 1, self.col - 1))
    }

    pub fn south(&self, _width: usize, height: usize) -> Option<Position> {
        if self.row as usize == height {
            return None;
        }
        Some(Position::at(self.row + 1, self.col))
    }

    pub fn south_east(&self, width: usize, height: usize) -> Option<Position> {
        let c = Position::at(self.row + 1, self.col + 1);

        if c.col as usize > width || c.row as usize > height {
            return None;
        }
        Some(c)
    }
}

#[cfg(test)]
mod position_tests {
    use super::*;

    // Turns 1+ tuples of (row, col) into a `Vec<Position>`
    macro_rules! to_path {
        ($($x:expr), *) => {
            {
                vec![
                    $(
                        Position::at($x.0, $x.1)
                    ), *
                ]
            }
        };
    }

    #[test]
    // A three letter word, other letters don't count
    pub fn basic_equality() {
        assert_eq!(Position::new(8, 2), Position::at(8, 2));
    }

    // Make a function that tests the cardinal neighbors of $position are equal
    // to $expected
    macro_rules! make_cardinal_neighbor_testcase {
        ($test_name:ident, $position:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                let c = $position;

                let mut expected = $expected;
                expected.sort();

                let mut result = c.cardinal_neighbors(2, 2);
                result.sort();
                assert_eq!(result, expected);
            }
        };
    }

    make_cardinal_neighbor_testcase!(
        test_cardinal_neighbors_top_left,
        Position::at(0, 0),
        to_path![(1, 0), (0, 1)]
    );

    make_cardinal_neighbor_testcase!(
        test_cardinal_neighbors_top_center,
        Position::at(0, 1),
        to_path![(0, 0), (0, 2), (1, 1)]
    );

    make_cardinal_neighbor_testcase!(
        test_cardinal_neighbors_top_right,
        Position::at(0, 2),
        to_path![(0, 1), (1, 2)]
    );
    make_cardinal_neighbor_testcase!(
        test_cardinal_neighbors_mid_left,
        Position::at(1, 0),
        to_path![(0, 0), (1, 1), (2, 0)]
    );

    make_cardinal_neighbor_testcase!(
        test_cardinal_neighbors_mid_center,
        Position::at(1, 1),
        to_path![(0, 1), (1, 0), (1, 2), (2, 1)]
    );

    make_cardinal_neighbor_testcase!(
        test_cardinal_neighbors_mid_right,
        Position::at(1, 2),
        to_path![(0, 2), (1, 1), (2, 2)]
    );

    make_cardinal_neighbor_testcase!(
        test_cardinal_neighbors_bot_left,
        Position::at(2, 0),
        to_path![(1, 0), (2, 1)]
    );

    make_cardinal_neighbor_testcase!(
        test_cardinal_neighbors_bot_center,
        Position::at(2, 1),
        to_path![(2, 0), (1, 1), (2, 2)]
    );

    make_cardinal_neighbor_testcase!(
        test_cardinal_neighbors_bot_right,
        Position::at(2, 2),
        to_path![(1, 2), (2, 1)]
    );

    // Make a function that tests the cardinal neighbors of $position are equal
    // to $expected
    macro_rules! make_neighbor_testcase {
        ($test_name:ident, $position:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                let c = $position;

                let mut expected = $expected;
                expected.sort();

                let mut result = c.neighbors(2, 2);
                result.sort();
                assert_eq!(result, expected);
            }
        };
    }

    make_neighbor_testcase!(
        test_neighbors_top_left,
        Position::at(0, 0),
        to_path![(1, 0), (0, 1), (1, 1)]
    );

    make_neighbor_testcase!(
        test_neighbors_top_center,
        Position::at(0, 1),
        to_path![(0, 0), (0, 2), (1, 0), (1, 1), (1, 2)]
    );

    make_neighbor_testcase!(
        test_neighbors_top_right,
        Position::at(0, 2),
        to_path![(0, 1), (1, 1), (1, 2)]
    );
    make_neighbor_testcase!(
        test_neighbors_mid_left,
        Position::at(1, 0),
        to_path![(0, 0), (0, 1), (1, 1), (2, 0), (2, 1)]
    );

    make_neighbor_testcase!(
        test_neighbors_mid_center,
        Position::at(1, 1),
        to_path![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2)
        ]
    );

    make_neighbor_testcase!(
        test_neighbors_mid_right,
        Position::at(1, 2),
        to_path![(0, 1), (0, 2), (1, 1), (2, 1), (2, 2)]
    );

    make_neighbor_testcase!(
        test_neighbors_bot_left,
        Position::at(2, 0),
        to_path![(1, 0), (1, 1), (2, 1)]
    );

    make_neighbor_testcase!(
        test_neighbors_bot_center,
        Position::at(2, 1),
        to_path![(1, 0), (1, 1), (1, 2), (2, 0), (2, 2)]
    );

    make_neighbor_testcase!(
        test_neighbors_bot_right,
        Position::at(2, 2),
        to_path![(1, 1), (1, 2), (2, 1)]
    );
}
