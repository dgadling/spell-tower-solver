use crate::position::Position;

use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hasher;
use std::{fmt, hash::Hash};

use crate::dictionary::Dictionary;

use deepsize::DeepSizeOf;

const LETTER_SCORES: &'static [u32] = &[
    1,  // a
    4,  // b
    4,  // c
    3,  // d
    1,  // e
    5,  // f
    3,  // g
    5,  // h
    1,  // i
    9,  // j
    6,  // k
    2,  // l
    4,  // m
    2,  // n
    1,  // o
    4,  // p
    12, // q
    2,  // r
    1,  // s
    2,  // t
    1,  // u
    5,  // v
    5,  // w
    9,  // x
    5,  // y
    11, // z
];

#[derive(Clone, Debug, Eq, Hash, PartialEq, DeepSizeOf)]
pub struct FoundWord {
    pub score: u32,
    pub path: Vec<Position>,
    pub word: String,
}

impl Ord for FoundWord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .cmp(&other.score)
            .reverse()
            .then(self.path.len().cmp(&other.path.len()))
            .then(self.word.cmp(&other.word))
    }
}

impl PartialOrd for FoundWord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, DeepSizeOf)]
pub struct Board {
    pub id: u64,
    width: usize,
    height: usize,
    min_word_length: usize,
    tiles: [[char; Board::WIDTH]; Board::HEIGHT],
    pub usable_tiles: usize,
    multipliers: Vec<Position>,
    cumulative_score: u32,
    searched: bool,
    words: Vec<FoundWord>,
    evolved_via: Option<FoundWord>,
    evolved_from: Option<u64>,
    cleaned: bool,
}

/*
Sort a Board by:
- cumulative score, higher wins
- usable tiles left, higher wins
- multipliers left, higher wins,
- words used, LOWER wins,
- evolved_via.word, LONGER wins
- evolved_from ; this is essentially a random number, but based on parents tiles so _could_ be the same
- id ; this is essentially random but based on tiles so _could_ be the same
- tiles ; idk how to compare these, so default sort

The remaining fields are identical for all boards that would be compared to each other:
- width
- height
- min_word_length
- searched
- cleaned
 */

impl Ord for Board {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cumulative_score
            .cmp(&other.cumulative_score)
            .reverse()
            .then(self.usable_tiles.cmp(&other.usable_tiles).reverse())
            .then(self.id.cmp(&other.id))
    }
}

impl PartialOrd for Board {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
    const MIN_WORD_LEN: usize = 3;
    pub const WIDTH: usize = 9;
    pub const HEIGHT: usize = 13;

    fn _hash_for(tiles: &[[char; Board::WIDTH]; Board::HEIGHT]) -> u64 {
        let mut hasher = DefaultHasher::new();
        tiles.hash(&mut hasher);
        hasher.finish()
    }

    fn get_usable_tiles(tiles: &[[char; Board::WIDTH]; Board::HEIGHT]) -> usize {
        tiles
            .iter()
            .map(|r| {
                r.iter()
                    .filter(|c| **c != Board::BLOCK && **c != Board::EMPTY)
                    .count()
            })
            .sum::<usize>()
    }

    pub fn new_from(
        tiles: [[char; 9]; 13],
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

    pub const BLOCK: char = '.';
    pub const EMPTY: char = ' ';
    pub const DEBUG: char = '*';

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
            // self.tiles = []; -- can't "clean" an array ; it'll always be that many characters
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

    pub fn get(&self, pos: &Position) -> char {
        self.tiles[pos.row as usize][pos.col as usize]
    }

    fn _get(tiles: &[[char; Board::WIDTH]; Board::HEIGHT], pos: &Position) -> char {
        tiles[pos.row as usize][pos.col as usize]
    }

    fn find_path_of_destruction(&self, path: &Vec<Position>, word: &str) -> Vec<Position> {
        let mut path_of_destruction = Vec::with_capacity(word.len() * 3);
        path_of_destruction.extend(path.clone());

        // See if we're *directly* going over any of the row-clearing letters
        path_of_destruction.extend(
            word.char_indices()
                .filter_map(|(idx, c)| {
                    if c != 'j' && c != 'q' && c != 'x' && c != 'z' {
                        return None;
                    }

                    let p = path.get(idx).unwrap();
                    Some((0..=self.width).map(|c| Position {
                        row: p.row,
                        col: c as u8,
                    }))
                })
                .flatten(),
        );

        // Any blocks get destroyed if any block adjacent to them is destroyed
        path_of_destruction.extend(
            path.iter()
                .map(|p| {
                    p.cardinal_neighbors(self.width, self.height)
                        .into_iter()
                        .filter(|p| self.get(p) == Board::BLOCK)
                })
                .flatten(),
        );

        if path.len() >= 5 {
            path_of_destruction.extend(
                path.iter()
                    .map(|p| p.cardinal_neighbors(self.width, self.height))
                    .flatten(),
            );
        }

        HashSet::<Position>::from_iter(path_of_destruction.into_iter())
            .drain()
            .collect()
    }

    fn destroy_board(
        &self,
        path_of_destruction: &Vec<Position>,
    ) -> [[char; Board::WIDTH]; Board::HEIGHT] {
        let mut new_tiles = self.tiles.clone();

        for p in path_of_destruction {
            new_tiles[p.row as usize][p.col as usize] = Board::EMPTY;
        }

        new_tiles
    }

    fn apply_gravity(
        tiles: &mut [[char; Board::WIDTH]; Board::HEIGHT],
        path_of_destruction: &mut Vec<Position>,
    ) {
        // Reverse sort based on row so we start at the lowest row and work our way back up
        path_of_destruction.sort_by(|a, b| b.row.cmp(&a.row));

        // No need to check row 0, doesn't matter if it's got blanks
        // No need to start any lower than the first blown up row
        for r in (1..=path_of_destruction[0].row).rev() {
            for c in 0..tiles.get(0).unwrap().len() {
                if tiles[r as usize][c as usize] != Board::EMPTY {
                    continue;
                }

                for row in (0..=r - 1).rev() {
                    let above = Board::_get(&tiles, &Position { row, col: c as u8 });
                    if above == ' ' {
                        continue;
                    }

                    tiles[r as usize][c as usize] = above.clone();
                    tiles[row as usize][c as usize] = Board::EMPTY;
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

        if found_words.len() > top_n {
            //println!(
            //    "Have {} > {} words, gotta truncate",
            //    found_words.len(),
            //    top_n
            //);
            /*
            Since we have too many, we need to pick some. Sort & truncate.
            Sorting order is:
              - score ; highest wins
              - word length ; shortest wins
              - word alphabetically ; can't have a tie, no dupe words
            */
            found_words.sort_by(|a, b| {
                a.score
                    .cmp(&b.score)
                    .reverse()
                    .then(a.word.len().cmp(&b.word.len()).then(a.word.cmp(&b.word)))
            });

            found_words.truncate(top_n);
        }
        found_words
    }

    fn finds_words_in_starting_from(&self, dict: &Dictionary, start: Position) -> Vec<FoundWord> {
        let mut path = Vec::with_capacity(16);
        path.push(start.clone());

        let path_str = String::from(self.get(&Position {
            row: start.row,
            col: start.col,
        }));
        self._find_word(&start, &path, &path_str, dict)
    }

    fn _find_word(
        &self,
        pos: &Position,
        path: &Vec<Position>,
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

        if path_str.len() >= Board::MIN_WORD_LEN && dict.is_word(&path_str) {
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

            if l == Board::BLOCK || l == Board::EMPTY {
                // This tile is a dead-end, no need to keep looking
                continue;
            }

            let mut fragment = String::with_capacity(path_str.len() + 1);
            fragment.push_str(&path_str);
            fragment.push_str(&l.to_string());

            if dict.has_path(&fragment) {
                let mut next_path = Vec::with_capacity(path.len() + 1);
                next_path.clone_from(path);
                next_path.push(p.clone());

                let found = self._find_word(&p, &next_path, &fragment, dict);
                if !found.is_empty() {
                    found_words.extend(found);
                }
            }
        }

        found_words.shrink_to_fit();
        found_words
    }

    fn score_for(&self, word: &str, path: &Vec<Position>) -> u32 {
        // Base score is the sum of _all_ the letter values
        let base_score = self
            .find_path_of_destruction(path, word)
            .iter()
            .map(|p| {
                let our_letter = self.get(p);
                if our_letter == '.' || our_letter == ' ' {
                    return 0;
                }
                let our_letter_num = our_letter as usize;
                LETTER_SCORES[our_letter_num - 97]
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

/*
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
    */
