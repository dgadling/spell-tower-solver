use phf::{phf_map, phf_set};
use std::fmt;

use crate::dictionary::Dictionary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Vec<String>>,
    multipliers: Vec<(usize, usize)>,
}

impl Board {
    pub fn new_from(tiles: Vec<Vec<String>>, multipliers: Vec<(usize, usize)>) -> Self {
        let width = tiles.len();
        let height = tiles.get(0).unwrap().len();

        Self {
            width,
            height,
            tiles,
            multipliers,
        }
    }

    pub fn score_for(tiles: &Vec<Tile>) -> u32 {
        let base = tiles
            .iter()
            .map(|t| LETTER_SCORES.get(&t.letter).cloned().unwrap() * t.multiplier)
            .sum::<u32>();

        let multiplier = tiles.iter().map(|t| t.multiplier).product::<u32>();
        base * multiplier * tiles.len() as u32
    }

    pub fn find_words(&self, dict: &mut Dictionary) {
        let start = Position::new(0, 0);

        let mut path = Vec::new();

        let first_word = self._find_word(start, &mut path, "".to_string(), dict);
    }

    fn _find_word(
        &self,
        pos: Position,
        path: &mut Vec<Position>,
        path_str: String,
        dict: &mut Dictionary,
    ) -> String {
        let candidate_positions = pos.neighbors(self.width, self.height);
        let candidate_letters = candidate_positions
            .iter()
            .filter_map(|p| {
                let l = self.tiles.get(p.row).unwrap().get(p.col).unwrap();

                if l.eq("") || l.eq(".") {
                    None
                } else {
                    Some((p, l))
                }
            })
            .collect::<Vec<(&Position, &String)>>();
        println!(
            "At ({:?}, {}) with letters of {:?}",
            pos,
            self.tiles.get(pos.row).unwrap().get(pos.col).unwrap(),
            candidate_letters
        );

        "flub".to_string()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    row: usize,
    col: usize,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}
impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    pub fn neighbors(&self, width: usize, height: usize) -> Vec<Position> {
        vec![
            self.north_west(width, height),
            self.north(width, height),
            self.north_east(width, height),
            self.east(width, height),
            self.west(width, height),
            self.south_west(width, height),
            self.south(width, height),
            self.south_east(width, height),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    pub fn north_west(&self, _width: usize, _height: usize) -> Option<Position> {
        if self.row == 0 || self.col == 0 {
            return None;
        }
        Some(Position::new(self.row - 1, self.col - 1))
    }

    pub fn north(&self, _width: usize, _height: usize) -> Option<Position> {
        if self.row == 0 {
            return None;
        }
        Some(Position::new(self.row - 1, self.col))
    }

    pub fn north_east(&self, width: usize, _height: usize) -> Option<Position> {
        if self.row == 0 {
            return None;
        }
        let c = Position::new(self.row - 1, self.col + 1);
        if c.row > width {
            return None;
        }
        Some(c)
    }

    pub fn west(&self, _width: usize, _height: usize) -> Option<Position> {
        if self.col == 0 {
            return None;
        }
        Some(Position::new(self.row, self.col - 1))
    }

    pub fn east(&self, width: usize, _height: usize) -> Option<Position> {
        let c = Position::new(self.row, self.col + 1);
        if c.col > width {
            return None;
        }
        Some(c)
    }

    pub fn south_west(&self, _width: usize, height: usize) -> Option<Position> {
        if self.row == height || self.col == 0 {
            return None;
        }
        Some(Position::new(self.row + 1, self.col - 1))
    }

    pub fn south(&self, _width: usize, height: usize) -> Option<Position> {
        if self.row == height {
            return None;
        }
        Some(Position::new(self.row + 1, self.col))
    }

    pub fn south_east(&self, width: usize, height: usize) -> Option<Position> {
        let c = Position::new(self.row + 1, self.col + 1);

        if c.row > width || c.col > height {
            return None;
        }
        Some(c)
    }
}

pub struct Tile {
    letter: String,
    multiplier: u32,
}

impl Tile {
    pub fn new(letter: &str, multiplier: u32) -> Self {
        Self {
            letter: letter.to_string(),
            multiplier,
        }
    }
}

static CLEARS_ROW: phf::Set<&'static str> = phf_set!("j", "q", "x", "z");

// Taken from https://en.wikipedia.org/wiki/Scrabble_letter_distributions
static LETTER_SCORES: phf::Map<&'static str, u32> = phf_map! {
    "a" => 1,
    "b" => 3,
    "c" => 3,
    "d" => 2,
    "e" => 1,
    "f" => 4,
    "g" => 2,
    "h" => 4,
    "i" => 1,
    "j" => 8,
    "k" => 5,
    "l" => 1,
    "m" => 3,
    "n" => 1,
    "o" => 1,
    "p" => 3,
    "q" => 10,
    "r" => 1,
    "s" => 1,
    "t" => 1,
    "u" => 1,
    "v" => 4,
    "w" => 4,
    "x" => 8,
    "y" => 4,
    "z" => 10,
};
