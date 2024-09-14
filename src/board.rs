use phf::{phf_map, phf_set};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hasher;
use std::{fmt, hash::Hash};

use crate::dictionary::Dictionary;

use deepsize::DeepSizeOf;

// Taken from https://en.wikipedia.org/wiki/Scrabble_letter_distributions but
// this is clearly not what's used in SpellTower.
static LETTER_SCORES: phf::Map<char, u32> = phf_map! {
    'a' => 1,
    'b' => 4,
    'c' => 4,
    'd' => 3,
    'e' => 1,
    'f' => 5,
    'g' => 3,
    'h' => 5,
    'i' => 1,
    'j' => 9,
    'k' => 6,
    'l' => 2,
    'm' => 4,
    'n' => 2,
    'o' => 1,
    'p' => 4,
    'q' => 12,
    'r' => 2,
    's' => 1,
    't' => 2,
    'u' => 1,
    'v' => 5,
    'w' => 5,
    'x' => 9,
    'y' => 5,
    'z' => 11,
};

static CLEARS_ROW: phf::Set<char> = phf_set!('j', 'q', 'x', 'z');

#[derive(Clone, Debug, Eq, Hash, PartialEq, DeepSizeOf)]
pub struct FoundWord {
    pub path: Vec<Position>,
    pub word: String,
    pub score: u32,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, DeepSizeOf)]
pub struct Board {
    pub id: u64,
    width: usize,
    height: usize,
    min_word_length: usize,
    tiles: Vec<Vec<String>>,
    usable_tiles: usize,
    multipliers: Vec<Position>,
    cumulative_score: u32,
    searched: bool,
    words: Vec<FoundWord>,
    evolved_via: Option<FoundWord>,
    evolved_from: Option<u64>,
    cleaned: bool,
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
            "{} - {} pts via {}",
            self.word,
            self.score,
            self.path
                .iter()
                .map(|p| format!("{}", p))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Board {
    fn _hash_for(tiles: &Vec<Vec<String>>) -> u64 {
        let mut hasher = DefaultHasher::new();
        tiles.hash(&mut hasher);
        hasher.finish()
    }

    fn get_usable_tiles(tiles: &Vec<Vec<String>>) -> usize {
        tiles
            .iter()
            .map(|r| {
                r.iter()
                    .filter(|c| *c != Board::BLOCK && *c != Board::EMPTY)
                    .count()
            })
            .sum::<usize>()
    }

    pub fn new_from(
        tiles: Vec<Vec<String>>,
        multipliers: Vec<(usize, usize)>,
        min_word_length: usize,
    ) -> Self {
        let height = tiles.len() - 1;
        let width = tiles.get(0).unwrap().len() - 1;

        Self {
            id: Board::_hash_for(&tiles),
            width,
            height,
            min_word_length,
            usable_tiles: Self::get_usable_tiles(&tiles),
            tiles,
            multipliers: multipliers
                .iter()
                .map(|p| Position::new(p.0, p.1))
                .collect(),
            words: vec![],
            cumulative_score: 0,
            evolved_via: Some(FoundWord {
                path: vec![],
                word: "created by God".to_string(),
                score: 0,
            }),
            evolved_from: Some(0),
            searched: false,
            cleaned: false,
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

    pub fn usable_tiles(&self) -> usize {
        self.usable_tiles
    }

    pub fn words(&self) -> &Vec<FoundWord> {
        assert!(
            self.searched,
            "I haven't been searched yet! No words for you!"
        );
        self.words.as_ref()
    }

    pub fn set_words(&mut self, words: Vec<FoundWord>) {
        self.words = words;
        self.searched = true;
    }

    pub fn dirty(&self) -> bool {
        !self.cleaned
    }

    pub fn clean(&mut self) {
        if !self.cleaned {
            // Now that the board has been fully processed, free up some memory
            self.tiles = vec![];
            self.words = vec![];
            self.cleaned = true;
        }
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
        tiles
            .get(pos.row as usize)
            .unwrap()
            .get(pos.col as usize)
            .unwrap()
    }

    fn find_path_of_destruction(&self, path: &Vec<Position>, word: &str) -> Vec<Position> {
        let mut path_of_destruction: HashSet<Position> = HashSet::from_iter(path.clone());

        // See if we're *directly* going over any of the row-clearing letters
        path_of_destruction.extend(
            word.char_indices()
                .filter_map(|(idx, c)| {
                    if !CLEARS_ROW.contains(&c) {
                        return None;
                    }

                    let p = path.get(idx).unwrap();
                    Some((0..=self.width).map(|c| Position {
                        row: p.row,
                        col: c as u8,
                    }))
                })
                .flatten()
                .collect::<Vec<Position>>(),
        );

        // Any blocks get destroyed if any block adjacent to them is destroyed
        path_of_destruction.extend(
            path.iter()
                .map(|p| {
                    p.cardinal_neighbors(self.width, self.height)
                        .into_iter()
                        .filter(|p| self.get(p) == Board::BLOCK)
                })
                .flatten()
                .collect::<Vec<Position>>(),
        );

        if path.len() >= 5 {
            path_of_destruction.extend(
                path.iter()
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
            new_tiles[p.row as usize][p.col as usize] = Board::EMPTY.to_string();
        }

        new_tiles
    }

    fn apply_gravity(tiles: &mut Vec<Vec<String>>, path_of_destruction: &mut Vec<Position>) {
        // Reverse sort based on row so we start at the lowest row and work our way back up
        path_of_destruction.sort_by(|a, b| b.row.cmp(&a.row));

        // No need to check row 0, doesn't matter if it's got blanks
        // No need to start any lower than the first blown up row
        for r in (1..=path_of_destruction[0].row).rev() {
            for c in 0..tiles.get(0).unwrap().len() {
                if !tiles[r as usize][c as usize].eq(Board::EMPTY) {
                    continue;
                }

                for row in (0..=r - 1).rev() {
                    let above = Board::_get(&tiles, &Position { row, col: c as u8 });
                    if above.eq(" ") {
                        continue;
                    }

                    tiles.get_mut(r as usize).unwrap()[c as usize] = above.clone();
                    tiles.get_mut(row as usize).unwrap()[c as usize] = Board::EMPTY.to_string();
                    break;
                }
            }
        }
    }

    pub fn evolve_via(&self, found_word: FoundWord) -> Board {
        let mut path_of_destruction =
            self.find_path_of_destruction(&found_word.path, &found_word.word);
        let mut new_tiles = self.destroy_board(&path_of_destruction);
        Self::apply_gravity(&mut new_tiles, &mut path_of_destruction);

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
            min_word_length: self.min_word_length,
            usable_tiles: Self::get_usable_tiles(&new_tiles),
            tiles: new_tiles,
            multipliers: new_mults,
            cumulative_score: self.cumulative_score + found_word.score,
            words: vec![],
            evolved_via: Some(found_word),
            evolved_from: Some(self.id),
            searched: false,
            cleaned: false,
        }
    }

    pub fn is_terminal(&self) -> bool {
        assert!(self.searched, "idk if I'm terminal, nobody's looked!");
        self.words.len() == 0
    }

    pub fn find_words(&self, dict: &Dictionary, top_n: usize) -> Vec<FoundWord> {
        let mut found_words = Vec::new();
        for row in 0..self.height + 1 {
            for col in 0..self.width + 1 {
                let start = Position::new(row, col);
                if self.tiles[row][col] == Board::EMPTY || self.tiles[row][col] == Board::BLOCK {
                    // No words start with a space, or can start on a blocked tile. Skip them.
                    continue;
                }

                let found = self.finds_words_in_starting_from(dict, start);
                found_words.extend(found);
            }
        }

        found_words.sort_by(|a, b| b.score.cmp(&a.score));
        found_words
            .into_iter()
            .take(top_n)
            .collect::<Vec<FoundWord>>()
    }

    fn finds_words_in_starting_from(&self, dict: &Dictionary, start: Position) -> Vec<FoundWord> {
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
        dict: &Dictionary,
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

            let mut fragment = String::with_capacity(path_str.len() + 1);
            fragment.push_str(&path_str);
            fragment.push_str(l);

            if dict.has_path(&fragment) {
                let mut next_path = Vec::with_capacity(path.len() + 1);
                next_path.clone_from(path);
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
        // Base score is the sum of _all_ the letter values
        let base_score = self
            .find_path_of_destruction(path, word)
            .iter()
            .map(|p| {
                LETTER_SCORES
                    .get(&self.get(p).chars().next().unwrap())
                    .cloned()
                    .unwrap_or_default()
            })
            .sum::<u32>();

        /*
          Multipliers stack and in tower mode are only ever 2x. So if you use
          one of them the multiplier is 2. If you use two of them it's 4.
          If you don't use any of them your multiplier is 1, which does nothing
        */
        let multiplier = std::cmp::max(
            path.iter()
                .map(|p| if self.multipliers.contains(p) { 2 } else { 0 })
                .sum::<u32>(),
            1,
        );

        (base_score * word.len() as u32) * multiplier
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    /// Turns 1+ strings into a Vec<Vec<String>> suitable for passing to Board::new_from()
    macro_rules! to_board {
        ($($x:expr), *) => {
            {
                vec![
                    $(
                        $x.chars().map(|c| c.to_string()).collect()
                    ), *
                ]
            }
        };
    }

    /// Turns 1+ tuples of (row, col) into a Vec<Position>
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
    /// A three letter word, other letters don't count
    fn simple_three() {
        let b = Board::new_from(to_board!("vis", "asd"), vec![], 3);
        let path = to_path![(0, 0), (0, 1), (0, 2)];
        assert_eq!(b.score_for("vis", &path), 21);
    }

    #[test]
    /// The word "zoo" is present and the "z" is going to clear the whole line
    /// its on. The score should reflect that and not include any of the other
    /// letters
    fn clearing_three() {
        let b = Board::new_from(to_board!("seezahbep", "fnsoobksl"), vec![], 3);
        let path = to_path![(0, 3), (1, 3), (1, 4)];
        assert_eq!(b.score_for("zoo", &path), 93);
    }

    #[test]
    /// A four letter word, other letters don't count
    fn simple_four() {
        let b = Board::new_from(to_board!("sign", "asdf"), vec![], 3);
        let path = to_path![(0, 0), (0, 1), (0, 2), (0, 3)];
        assert_eq!(b.score_for("sign", &path), 28);
    }

    #[test]
    /// A five letter word with three adjacent letters, a space and a block
    fn simple_five() {
        let b = Board::new_from(to_board!("lho .", "nodes"), vec![], 3);
        let path = to_path![(1, 0), (1, 1), (1, 2), (1, 3), (1, 4)];
        assert_eq!(b.score_for("nodes", &path), 80);
    }

    #[test]
    /// Here we have a word where there's a line-clearing letter getting
    /// destructed. Make sure that line-clearing `j` doesn't cause the `v` to
    /// get consumed as well. It shouldn't since it's not cardinally-adjacent
    /// to any of the tiles in the word.
    fn bonus_letters_dont_clear() {
        let b = Board::new_from(
            to_board!(".ulsalidc.", "oprincess.", "..bm..j..v"),
            vec![],
            3,
        );
        let path = to_path![
            (1, 1),
            (1, 2),
            (1, 3),
            (1, 4),
            (1, 5),
            (1, 6),
            (1, 7),
            (1, 8)
        ];
        assert_eq!(b.score_for("princess", &path), 392);
    }

    #[test]
    /// Test that board "evolution" and "gravity" work as expected. Given an
    /// input board and some `FoundWord`s to iterate over, make sure we get the
    /// expected board at the end.
    fn evolution_test() {
        let input_board = to_board!(
            "i.ssbtpod",
            "mcisneice",
            "hcrqsovaa",
            "ln.sgsnnr",
            "eiusyijme"
        );

        let output_board: Vec<Vec<String>> = to_board!(
            "    bt   ",
            "   snepod",
            "   qsovaa",
            "  .sgsnnr",
            "  usyijme"
        );

        let word_pickings = vec![
            FoundWord {
                score: 1,
                word: "ice".to_string(),
                path: to_path![(1, 6), (1, 7), (1, 8)],
            },
            FoundWord {
                score: 1,
                word: "icicle".to_string(),
                path: to_path![(0, 0), (1, 1), (1, 2), (2, 1), (3, 0), (4, 0)],
            },
        ];

        let mut b = Board::new_from(input_board, vec![], 3);

        for findings in word_pickings {
            b = b.evolve_via(findings);
        }

        assert_eq!(b.tiles, output_board);
    }

    #[test]
    /// Test that a `Board`s `id` is based solely on the content of the tiles.
    /// We do this by making two `Board`s that take the same tiles, but the
    /// other input is different. We then verify that their `id`s are the same
    /// while they're not pointing at the same object in memory.
    ///
    /// Note that if they're given the same parameters the compiler makes sure
    /// they **do** point at the same object in memory!
    fn id_test() {
        let sample_b1 = vec![
            "i.ssbtpod".chars().map(|c| c.to_string()).collect(),
            "mcisneice".chars().map(|c| c.to_string()).collect(),
            "hcrqsovaa".chars().map(|c| c.to_string()).collect(),
            "ln.sgsnnr".chars().map(|c| c.to_string()).collect(),
            "eiusyijme".chars().map(|c| c.to_string()).collect(),
        ];
        let sample_b2 = vec![
            "i.ssbtpod".chars().map(|c| c.to_string()).collect(),
            "mcisneice".chars().map(|c| c.to_string()).collect(),
            "hcrqsovaa".chars().map(|c| c.to_string()).collect(),
            "ln.sgsnnr".chars().map(|c| c.to_string()).collect(),
            "eiusyijme".chars().map(|c| c.to_string()).collect(),
        ];

        let b1 = Board::new_from(sample_b1, vec![], 3);
        let b2 = Board::new_from(sample_b2, vec![(0, 0), (1, 1), (2, 2)], 4);

        assert_eq!(b1.id, b2.id);
    }
}

#[derive(Clone, Eq, Hash, PartialEq, DeepSizeOf)]
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

    fn at(row: u8, col: u8) -> Self {
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
