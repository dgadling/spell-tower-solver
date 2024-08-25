use phf::phf_map;
use std::fmt;

use crate::dictionary::Dictionary;

// Taken from https://en.wikipedia.org/wiki/Scrabble_letter_distributions but
// this is clearly not what's used in SpellTower.
static LETTER_SCORES: phf::Map<char, u32> = phf_map! {
    'a' => 1,
    'b' => 3,
    'c' => 3,
    'd' => 2,
    'e' => 1,
    'f' => 4,
    'g' => 2,
    'h' => 4,
    'i' => 1,
    'j' => 8,
    'k' => 5,
    'l' => 1,
    'm' => 3,
    'n' => 1,
    'o' => 1,
    'p' => 3,
    'q' => 10,
    'r' => 1,
    's' => 1,
    't' => 1,
    'u' => 1,
    'v' => 4,
    'w' => 4,
    'x' => 8,
    'y' => 4,
    'z' => 10,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundWord {
    pub path: Vec<Position>,
    pub word: String,
    pub score: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Vec<String>>,
    multipliers: Vec<Position>,
}

impl Board {
    pub fn new_from(tiles: Vec<Vec<String>>, multipliers: Vec<(usize, usize)>) -> Self {
        let height = tiles.len() - 1;
        let width = tiles.get(0).unwrap().len() - 1;

        Self {
            width,
            height,
            tiles,
            multipliers: multipliers
                .iter()
                .map(|p| Position::new(p.0, p.1))
                .collect(),
        }
    }

    pub fn find_words(&self, dict_path: &str) -> Vec<FoundWord> {
        let mut found_words = Vec::new();
        for row in 0..self.height + 1 {
            for col in 0..self.width + 1 {
                let start = Position::new(row, col);
                found_words.extend(self.finds_words_in_starting_from(&dict_path, start));
            }
        }

        found_words
    }

    fn finds_words_in_starting_from(&self, dict_path: &str, start: Position) -> Vec<FoundWord> {
        let mut dict = Dictionary::new(dict_path);
        let mut path = Vec::new();
        path.push(start.clone());

        let path_str = self.tiles.get(start.row).unwrap().get(start.col).unwrap();
        self._find_word(&start, &mut path, &path_str, &mut dict)
    }

    fn _find_word(
        &self,
        pos: &Position,
        path: &mut Vec<Position>,
        path_str: &String,
        dict: &mut Dictionary,
    ) -> Vec<FoundWord> {
        /*
        We have arrived at pos. From here we need to
            1. Figure out if path_str counts as a complete word, add path + word to our list of results
            2. Find candidate positions (not in our path, and not blocked)
            3. Filter out candidate positions where path_str + their value is not part of a word
            4. For each candidate: recurse and add their list of words to ours
            5. Return a flattened version of our list of results
        */
        let mut found_words: Vec<FoundWord> = Vec::new();

        if path_str.len() >= 3 && dict.is_word(&path_str) {
            found_words.push(FoundWord {
                path: path.clone(),
                word: path_str.clone(),
                score: self.score_for(path_str, path),
            });
        }

        let candidate_positions = pos.neighbors(self.width, self.height);

        for p in candidate_positions {
            // Can't cross our existing path
            if path.contains(&p) {
                continue;
            }

            let l = self.tiles.get(p.row).unwrap().get(p.col).unwrap();

            if l.eq("") || l.eq(".") {
                // This tile is a dead-end, no need to keep looking
                continue;
            }

            let fragment = path_str.clone() + l;
            if dict.has_path(&fragment) {
                let mut next_path = path.clone();
                next_path.push(p.clone());

                let found = self._find_word(&p, &mut next_path, &fragment, dict);
                if !found.is_empty() {
                    found_words.extend(found);
                }
            }
        }

        found_words
    }

    fn score_for(&self, word: &str, path: &Vec<Position>) -> u32 {
        // Base score is the sum of all of the letter values
        let base_score = word
            .chars()
            .into_iter()
            .map(|c| LETTER_SCORES.get(&c).unwrap())
            .sum::<u32>();

        /*
          Multipliers stack and in tower mode are only ever 2x. So if you use
          one of them the multiplier is 2. If you use two of them it's 4.
          If you don't use any of them your multiplier is 1, which does nothing
        */
        let multiplier = path
            .iter()
            .map(|p| if self.multipliers.contains(p) { 2 } else { 1 })
            .product::<u32>();

        base_score * multiplier * word.len() as u32
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
        if c.col > width {
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

        if c.col > width || c.row > height {
            return None;
        }
        Some(c)
    }
}
