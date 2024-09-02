pub mod board;
pub mod dictionary;
mod game;

use board::Board;
use deepsize::DeepSizeOf;
use dictionary::Dictionary;

#[allow(dead_code)]
fn size_tests() {
    let dictionary_db_name = "dictionary.db";
    let boards = vec![
        vec![
            "i.ssbtpod".chars().map(|c| c.to_string()).collect(),
            "mcisneice".chars().map(|c| c.to_string()).collect(),
            "hcrqsovaa".chars().map(|c| c.to_string()).collect(),
            "ln.sgsnnr".chars().map(|c| c.to_string()).collect(),
            "eiusyijme".chars().map(|c| c.to_string()).collect(),
            "olmgapelf".chars().map(|c| c.to_string()).collect(),
            "tsaeeudhn".chars().map(|c| c.to_string()).collect(),
            "bsoenditr".chars().map(|c| c.to_string()).collect(),
            "cwoopteaf".chars().map(|c| c.to_string()).collect(),
            "itzoutner".chars().map(|c| c.to_string()).collect(),
            ".upriigal".chars().map(|c| c.to_string()).collect(),
            "tkayee.ld".chars().map(|c| c.to_string()).collect(),
            "xlihcrras".chars().map(|c| c.to_string()).collect(),
        ],
        vec![
            "         ".chars().map(|c| c.to_string()).collect(),
            "         ".chars().map(|c| c.to_string()).collect(),
            "         ".chars().map(|c| c.to_string()).collect(),
            "         ".chars().map(|c| c.to_string()).collect(),
            "eiusy    ".chars().map(|c| c.to_string()).collect(),
            "o.mga    ".chars().map(|c| c.to_string()).collect(),
            "ts.ee    ".chars().map(|c| c.to_string()).collect(),
            "bsoen    ".chars().map(|c| c.to_string()).collect(),
            "cwoop    ".chars().map(|c| c.to_string()).collect(),
            "itzoutn  ".chars().map(|c| c.to_string()).collect(),
            ".upriig  ".chars().map(|c| c.to_string()).collect(),
            "tkayee.l ".chars().map(|c| c.to_string()).collect(),
            "xlihcrra ".chars().map(|c| c.to_string()).collect(),
        ],
    ];

    let mult_locs: Vec<(usize, usize)> = vec![(0, 8), (1, 2), (9, 6)];

    let dict = Dictionary::new(&dictionary_db_name);

    for board in boards {
        println!("Board\n----------------------------------------");
        println!("  input board = {} bytes", &board.deep_size_of());
        let mut b = Board::new_from(board, mult_locs.clone());
        println!(" usable tiles = {}", b.usable_tiles());
        println!("board pre-pop = {} bytes", b.deep_size_of());
        let words = b.find_words(&dict, 10_000);
        println!(
            "        words = {} bytes / {} words",
            words.deep_size_of(),
            words.len()
        );
        let words_word_sizes = words.iter().map(|w| w.word.deep_size_of()).sum::<usize>();
        let words_path_sizes = words.iter().map(|w| w.path.deep_size_of()).sum::<usize>();
        let total_positions = words.iter().map(|w| w.path.len()).sum::<usize>();
        println!("words words = {} bytes", words_word_sizes);
        println!("words paths = {} bytes", words_path_sizes);
        println!("words paths total positions = {}", total_positions);

        let w1 = words[0].clone();
        println!(" w1 overall = {}", w1.deep_size_of());
        println!(
            "    w1.path = {} / {} path items",
            w1.path.deep_size_of(),
            w1.path.len()
        );
        println!("    w1.word = {}", w1.word.deep_size_of());
        println!("   w1.score = {}", w1.score.deep_size_of());

        b.set_words(words);
        println!("board ful-pop = {} bytes", b.deep_size_of());
    }
}

fn main() {
    let dictionary_db_name = "dictionary.db";
    Dictionary::init_from(dictionary_db_name, "nwl/nwl2020.txt", 3);

    let sample_board = vec![
        "i.ssbtpod".chars().map(|c| c.to_string()).collect(),
        "mcisneice".chars().map(|c| c.to_string()).collect(),
        "hcrqsovaa".chars().map(|c| c.to_string()).collect(),
        "ln.sgsnnr".chars().map(|c| c.to_string()).collect(),
        "eiusyijme".chars().map(|c| c.to_string()).collect(),
        "olmgapelf".chars().map(|c| c.to_string()).collect(),
        "tsaeeudhn".chars().map(|c| c.to_string()).collect(),
        "bsoenditr".chars().map(|c| c.to_string()).collect(),
        "cwoopteaf".chars().map(|c| c.to_string()).collect(),
        "itzoutner".chars().map(|c| c.to_string()).collect(),
        ".upriigal".chars().map(|c| c.to_string()).collect(),
        "tkayee.ld".chars().map(|c| c.to_string()).collect(),
        "xlihcrras".chars().map(|c| c.to_string()).collect(),
    ];

    let mult_locs: Vec<(usize, usize)> = vec![(0, 8), (1, 2), (9, 6)];
    game::play_game(dictionary_db_name, sample_board, mult_locs, 15_000);
}
