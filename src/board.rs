use phf::{phf_map, phf_set};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hasher;
use std::{fmt, hash::Hash};

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

static CLEARS_ROW: phf::Set<char> = phf_set!('j', 'q', 'x', 'z');

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FoundWord {
    pub path: Vec<Position>,
    pub word: String,
    pub score: u32,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Board {
    pub id: u64,
    width: usize,
    height: usize,
    tiles: Vec<Vec<String>>,
    multipliers: Vec<Position>,
    cumulative_score: u32,
    searched: bool,
    words: Option<Vec<FoundWord>>,
    evolved_via: Option<FoundWord>,
    evolved_from: Option<u64>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id={}, came via = {}\n",
            self.id,
            self.evolved_via.as_ref().unwrap().word
        )?;
        for row in 0..=self.height {
            for col in 0..=self.width {
                let c = self.tiles.get(row).unwrap().get(col).unwrap();
                write!(f, "{}", c)?;
            }
            if row != self.height {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for FoundWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} for {} points via {:?}",
            self.word, self.score, self.path
        )
    }
}

impl Board {
    fn _hash_for(tiles: &Vec<Vec<String>>) -> u64 {
        let mut hasher = DefaultHasher::new();
        tiles.hash(&mut hasher);
        hasher.finish()
    }

    pub fn new_from(tiles: Vec<Vec<String>>, multipliers: Vec<(usize, usize)>) -> Self {
        let height = tiles.len() - 1;
        let width = tiles.get(0).unwrap().len() - 1;

        Self {
            id: Board::_hash_for(&tiles),
            width,
            height,
            tiles,
            multipliers: multipliers
                .iter()
                .map(|p| Position::new(p.0, p.1))
                .collect(),
            words: None,
            cumulative_score: 0,
            evolved_via: Some(FoundWord {
                path: vec![],
                word: "created by God".to_string(),
                score: 0,
            }),
            evolved_from: Some(0),
            searched: false,
        }
    }

    pub const BLOCK: &'static str = ".";
    pub const EMPTY: &'static str = " ";
    pub const DEBUG: &'static str = "*";

    pub fn get_score(&self) -> u32 {
        self.cumulative_score
    }

    pub fn searched(&self) -> bool {
        self.searched
    }

    pub fn words(&self) -> &Vec<FoundWord> {
        assert!(
            self.searched,
            "I haven't been searched yet! No words for you!"
        );
        self.words.as_ref().unwrap()
    }

    pub fn evolved_via(&self) -> FoundWord {
        self.evolved_via.to_owned().unwrap()
    }

    pub fn evolved_from(&self) -> u64 {
        self.evolved_from.unwrap()
    }

    pub fn get(&self, pos: &Position) -> &String {
        Board::_get(&self.tiles, pos)
    }

    fn _get<'a, 'b>(tiles: &'a Vec<Vec<String>>, pos: &'b Position) -> &'a String {
        tiles.get(pos.row).unwrap().get(pos.col).unwrap()
    }

    fn find_path_of_destruction(&self, found_word: &FoundWord) -> Vec<Position> {
        let mut path_of_destruction: HashSet<Position> =
            HashSet::from_iter(found_word.path.clone());

        // See if we're *directly* going over any of the row-clearing letters
        path_of_destruction.extend(
            found_word
                .word
                .char_indices()
                .filter_map(|(idx, c)| {
                    if !CLEARS_ROW.contains(&c) {
                        return None;
                    }

                    let p = found_word.path.get(idx).unwrap();
                    Some((0..=self.width).map(|c| Position { row: p.row, col: c }))
                })
                .flatten()
                .collect::<Vec<Position>>(),
        );

        // Any blocks get destroyed if any block adjacent to them is destroyed
        path_of_destruction.extend(
            found_word
                .path
                .iter()
                .map(|p| {
                    p.cardinal_neighbors(self.width, self.height)
                        .into_iter()
                        .filter(|p| self.get(p) == Board::BLOCK)
                })
                .flatten()
                .collect::<Vec<Position>>(),
        );

        if found_word.path.len() >= 5 {
            path_of_destruction.extend(
                found_word
                    .path
                    .iter()
                    .map(|p| p.cardinal_neighbors(self.width, self.height))
                    .flatten()
                    .collect::<Vec<Position>>(),
            );
        }

        path_of_destruction.into_iter().collect()
    }

    fn destroy_board(&self, path_of_destruction: &Vec<Position>) -> Vec<Vec<String>> {
        let mut new_tiles = self.tiles.clone();

        for p in path_of_destruction {
            new_tiles[p.row][p.col] = Board::EMPTY.to_string();
        }

        new_tiles
    }

    fn apply_gravity(mut tiles: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // NOTE: No need to check row 0, doesn't matter if it's got blanks
        for r in (1..tiles.len()).rev() {
            for c in 0..tiles.get(0).unwrap().len() {
                if !tiles.get(r).unwrap().get(c).unwrap().eq(" ") {
                    continue;
                }

                for row in (0..=r - 1).rev() {
                    let above = Board::_get(&tiles, &Position { row, col: c });
                    if above.eq(" ") {
                        continue;
                    }

                    tiles.get_mut(r).unwrap()[c] = above.clone();
                    tiles.get_mut(row).unwrap()[c] = Board::EMPTY.to_string();
                    break;
                }
            }
        }

        tiles
    }

    pub fn evolve_via(&self, found_word: FoundWord) -> Board {
        let path_of_destruction = self.find_path_of_destruction(&found_word);
        let new_tiles = Self::apply_gravity(self.destroy_board(&path_of_destruction));

        let new_mults = self
            .multipliers
            .iter()
            .filter(|p| !path_of_destruction.contains(p))
            .cloned()
            .collect();

        Board {
            id: Board::_hash_for(&new_tiles),
            width: self.width,
            height: self.height,
            tiles: new_tiles,
            multipliers: new_mults,
            cumulative_score: self.cumulative_score + found_word.score,
            words: None,
            evolved_via: Some(found_word),
            evolved_from: Some(self.id),
            searched: false,
        }
    }

    pub fn is_terminal(&self) -> bool {
        assert!(self.searched, "idk if I'm terminal, nobody's looked!");
        self.words.as_ref().unwrap().len() == 0
    }

    pub fn find_words(&mut self, dict: &mut Dictionary) {
        let mut found_words = Vec::new();
        for row in 0..self.height + 1 {
            for col in 0..self.width + 1 {
                let start = Position::new(row, col);
                let found = self.finds_words_in_starting_from(dict, start);
                found_words.extend(found);
            }
        }

        found_words.sort_by(|a, b| b.score.cmp(&a.score));
        self.words = Some(found_words);
        self.searched = true;
    }

    fn finds_words_in_starting_from(
        &self,
        dict: &mut Dictionary,
        start: Position,
    ) -> Vec<FoundWord> {
        let mut path = Vec::new();
        path.push(start.clone());

        let path_str = self.get(&Position {
            row: start.row,
            col: start.col,
        });
        self._find_word(&start, &mut path, &path_str, dict)
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

        for p in candidate_positions.iter() {
            // Can't cross our existing path
            if path.contains(p) {
                continue;
            }

            let l = self.get(p);

            if l.eq(Board::BLOCK) || l.eq(Board::EMPTY) {
                // This tile is a dead-end, no need to keep looking
                continue;
            }

            let fragment = path_str.clone() + &l;
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

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
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
        let mut all_neighbors = self.cardinal_neighbors(width, height);

        all_neighbors.extend(
            vec![
                self.north_west(width, height),
                self.north_east(width, height),
                self.south_west(width, height),
                self.south_east(width, height),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<Position>>(),
        );

        all_neighbors
    }

    pub fn cardinal_neighbors(&self, width: usize, height: usize) -> Vec<Position> {
        vec![
            self.north(width, height),
            self.east(width, height),
            self.west(width, height),
            self.south(width, height),
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
