pub mod board;
pub mod dictionary;
mod game;

use std::str::FromStr;

use board::Board;
use clap::Parser;
use deepsize::DeepSizeOf;
use dictionary::Dictionary;
use indicatif::HumanDuration;

/// Figure out the optimial set of moves in a game of SpellTower
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path of the dictionary to use
    #[arg(long, default_value = "nwl/nwl2023.txt")]
    dict_path: String,

    /// Path of the dictionary database file to use
    #[arg(long, default_value = "dictionary.db")]
    db_path: String,

    /// Starting max number of children each board can spawn. Setting this forces quiet mode!
    #[arg(short = 's', long)]
    start_max_children: Option<usize>,

    /// Maximum number of children each board can spawn.
    #[arg(short = 'c', long, default_value_t = 5)]
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

    /// Quiet - don't show any output: overrides --memory-debug
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

#[allow(dead_code)]
fn size_tests() {
    let args = Args {
        db_path: String::from_str("dictionary.db").unwrap(),
        start_max_children: None,
        max_children: 0,
        memory_debug: false,
        min_word_length: 3,
        evolution_batch_size: 0,
        quiet: false,
        dict_path: "dictionary.db".to_string(),
    };

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

    let dict = Dictionary::new(&args);

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
        println!("words word = {} bytes", words_word_sizes);
        println!("words path = {} bytes", words_path_sizes);
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
        b.clean();
        println!("board cleaned = {} bytes", b.deep_size_of());
    }
}

fn main() {
    let mut args = Args::parse();
    Dictionary::init_from(&args);

    let sample_board = vec![
        /*
         012345678
        */
        "esekaleen".chars().map(|c| c.to_string()).collect(), //  0
        "duisehunm".chars().map(|c| c.to_string()).collect(), //  1
        "ridco.set".chars().map(|c| c.to_string()).collect(), //  2
        "dorson..n".chars().map(|c| c.to_string()).collect(), //  3
        "riagawtni".chars().map(|c| c.to_string()).collect(), //  4
        "sphiuulbe".chars().map(|c| c.to_string()).collect(), //  5
        "hrsnccelt".chars().map(|c| c.to_string()).collect(), //  6
        "epasiqrot".chars().map(|c| c.to_string()).collect(), //  7
        "searycope".chars().map(|c| c.to_string()).collect(), //  8
        "sis.vgild".chars().map(|c| c.to_string()).collect(), //  9
        "afbroyizm".chars().map(|c| c.to_string()).collect(), // 10
        "jmtatatlp".chars().map(|c| c.to_string()).collect(), // 11
        "flgxcneoi".chars().map(|c| c.to_string()).collect(), // 12
    ];

    let mult_locs: Vec<(usize, usize)> = vec![(4, 1), (9, 1), (11, 5)];

    let game_run_time = std::time::Instant::now();
    if let Some(start) = args.start_max_children {
        for child_count in start..=args.max_children {
            args.max_children = child_count;
            game::play_game(&args, sample_board.clone(), mult_locs.clone());
        }
    } else {
        game::play_game(&args, sample_board, mult_locs);
    }

    if !args.quiet {
        println!("Finished in {}", HumanDuration(game_run_time.elapsed()));
    }
}
