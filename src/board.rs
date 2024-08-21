use indicatif::{ProgressBar, ProgressStyle};
use phf::{phf_map, phf_set};
use std::fmt;
use std::time::SystemTime;

use crate::dictionary::Dictionary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundWord {
    path: Vec<Position>,
    word: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Vec<String>>,
    multipliers: Vec<(usize, usize)>,
}

impl Board {
    pub fn new_from(tiles: Vec<Vec<String>>, multipliers: Vec<(usize, usize)>) -> Self {
        let height = tiles.len() - 1;
        let width = tiles.get(0).unwrap().len() - 1;

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
        let now = SystemTime::now();
        let mut found_words = Vec::new();

        // for row in 0..self.height {
        for row in 0..1 {
            let bar = ProgressBar::new(self.width as u64);
            bar.set_prefix(format!("Row {:>2}/{:>02}", row, self.height));
            bar.set_style(ProgressStyle::with_template("{prefix} {wide_bar} {pos}/{len}").unwrap());
            // for col in 0..self.width {
            for col in 0..1 {
                let start = Position::new(row, col);
                let words = self.finds_words_starting_from(start, dict, &bar);
                if !words.is_empty() {
                    found_words.extend(words);
                }
                bar.inc(1);
            }
            bar.finish();
        }
        println!("Finished after {}ms", now.elapsed().unwrap().as_millis());

        println!("Found {} words! Here's the 15 longest", found_words.len());
        found_words.sort_by(|a, b| b.word.len().cmp(&a.word.len()));
        for fwd in &found_words[..2] {
            println!("  {} via {:?}", fwd.word, fwd.path);
        }

        println!("Finished after {}ms", now.elapsed().unwrap().as_millis());
    }

    fn finds_words_starting_from(&self, start: Position, dict: &mut Dictionary, bar: &ProgressBar) -> Vec<FoundWord> {
        let mut path = Vec::new();
        path.push(start.clone());

        let path_str = self.tiles.get(start.row).unwrap().get(start.col).unwrap();
        let words = self._find_word(&start, &mut path, &path_str, dict, bar);
        bar.inc(1);
        words
    }

    fn _find_word(
        &self,
        pos: &Position,
        path: &mut Vec<Position>,
        path_str: &String,
        dict: &mut Dictionary,
        bar: &ProgressBar,
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
            });
        }

        let candidate_positions = pos.neighbors(self.width, self.height);

        bar.inc_length(candidate_positions.len() as u64);
        for p in candidate_positions {
            // Can't cross our existing path
            if path.contains(&p) {
                bar.inc(1);
                continue;
            }

            let l = self.tiles.get(p.row).unwrap().get(p.col).unwrap();

            if l.eq("") || l.eq(".") {
                // This tile is a dead-end, no need to keep looking
                bar.inc(1);
                continue;
            }

            let fragment = path_str.clone() + l;
            if dict.has_path(&fragment) {
                let mut next_path = path.clone();
                next_path.push(p.clone());

                let found = self._find_word(&p, &mut next_path, &fragment, dict, bar);
                if !found.is_empty() {
                    found_words.extend(found);
                }
                bar.inc(1);
            }
        }

        bar.inc(1);
        found_words
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
