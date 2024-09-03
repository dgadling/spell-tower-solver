pub mod board;
pub mod dictionary;
mod game;

use board::Board;
use clap::Parser;
use deepsize::DeepSizeOf;
use dictionary::Dictionary;

/// Figure out the optimial set of moves in a game of SpellTower
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path of the dictionary to use
    #[arg(long, default_value = "nwl/nwl2020.txt")]
    dict_path: String,

    /// Path of the dictionary database file to use
    #[arg(long, default_value = "dictionary.db")]
    db_path: String,

    /// Maximum number of children each board can spawn.
    #[arg(short = 'c', long, default_value_t = 9)]
    max_children: usize,

    /// Minimum length of a word we'll consider valid
    #[arg(short = 'w', long, default_value_t = 3)]
    min_word_length: usize,

    /// Show memory debugging info
    #[arg(long, default_value_t = false)]
    memory_debug: bool,

    /// Evolution batch size
    #[arg(long, default_value_t = 100)]
    evolution_batch_size: usize,
}

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
        let mut b = Board::new_from(board, mult_locs.clone(), 3);
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
    let args = Args::parse();
    Dictionary::init_from(&args.db_path, &args.dict_path, args.min_word_length);

    let sample_board = vec![
        "aesdnpchn".chars().map(|c| c.to_string()).collect(),
        "ilcsiroze".chars().map(|c| c.to_string()).collect(),
        "osltarmte".chars().map(|c| c.to_string()).collect(),
        "ste.uenfi".chars().map(|c| c.to_string()).collect(),
        "edyc.umih".chars().map(|c| c.to_string()).collect(),
        "btpryslrf".chars().map(|c| c.to_string()).collect(),
        "awqbhpxka".chars().map(|c| c.to_string()).collect(),
        "ag.sncopi".chars().map(|c| c.to_string()).collect(),
        "tadegtjne".chars().map(|c| c.to_string()).collect(),
        "reicsieeo".chars().map(|c| c.to_string()).collect(),
        "sigsiatmn".chars().map(|c| c.to_string()).collect(),
        ".srnuolor".chars().map(|c| c.to_string()).collect(),
        "evdoallui".chars().map(|c| c.to_string()).collect(),
    ];

    let mult_locs: Vec<(usize, usize)> = vec![(4, 1), (9, 1), (11, 5)];
    game::play_game(&args, sample_board, mult_locs);
}
