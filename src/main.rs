pub mod board;
pub mod dictionary;
mod game;
pub mod position;

use std::io::Read;

use board::Board;
use clap::Parser;
use clio::*;
use deepsize::DeepSizeOf;
use dictionary::Dictionary;
use indicatif::HumanDuration;
use serde::{Deserialize, Serialize};

/// Figure out the optimial set of moves in a game of SpellTower
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path of the dictionary to use
    #[arg(long, default_value = "nwl/nwl2023.txt")]
    dict_path: String,

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

    /// When evolving, how many boards to do in parallel
    #[arg(long, default_value_t = 100)]
    evolution_batch_size: usize,

    /// Quiet - don't show any output: overrides --memory-debug
    #[arg(short, long, default_value_t = false)]
    quiet: bool,

    /// Input board
    #[clap(value_parser, default_value = "-")]
    input_f: Input,

    /// Max number of boards to process in any given generation
    #[arg(short = 'g', long, default_value_t = 1_000_000)]
    max_gen_size: usize,

    /// Max number of generations. When we hit this generation we'll just stop.
    #[arg(long, default_value_t = u32::MAX)]
    max_generations: u32,

    /// Don't actually run anything, just do a size test
    #[arg(long, default_value_t = false)]
    size_test: bool,
}

fn size_test(args: Args) {
    let boards = vec![
        [
            "i.ssbtpod".chars().collect::<Vec<char>>().try_into().unwrap(),
            "mcisneice".chars().collect::<Vec<char>>().try_into().unwrap(),
            "hcrqsovaa".chars().collect::<Vec<char>>().try_into().unwrap(),
            "ln.sgsnnr".chars().collect::<Vec<char>>().try_into().unwrap(),
            "eiusyijme".chars().collect::<Vec<char>>().try_into().unwrap(),
            "olmgapelf".chars().collect::<Vec<char>>().try_into().unwrap(),
            "tsaeeudhn".chars().collect::<Vec<char>>().try_into().unwrap(),
            "bsoenditr".chars().collect::<Vec<char>>().try_into().unwrap(),
            "cwoopteaf".chars().collect::<Vec<char>>().try_into().unwrap(),
            "itzoutner".chars().collect::<Vec<char>>().try_into().unwrap(),
            ".upriigal".chars().collect::<Vec<char>>().try_into().unwrap(),
            "tkayee.ld".chars().collect::<Vec<char>>().try_into().unwrap(),
            "xlihcrras".chars().collect::<Vec<char>>().try_into().unwrap(),
        ],
        [
            "         ".chars().collect::<Vec<char>>().try_into().unwrap(),
            "         ".chars().collect::<Vec<char>>().try_into().unwrap(),
            "         ".chars().collect::<Vec<char>>().try_into().unwrap(),
            "         ".chars().collect::<Vec<char>>().try_into().unwrap(),
            "eiusy    ".chars().collect::<Vec<char>>().try_into().unwrap(),
            "o.mga    ".chars().collect::<Vec<char>>().try_into().unwrap(),
            "ts.ee    ".chars().collect::<Vec<char>>().try_into().unwrap(),
            "bsoen    ".chars().collect::<Vec<char>>().try_into().unwrap(),
            "cwoop    ".chars().collect::<Vec<char>>().try_into().unwrap(),
            "itzoutn  ".chars().collect::<Vec<char>>().try_into().unwrap(),
            ".upriig  ".chars().collect::<Vec<char>>().try_into().unwrap(),
            "tkayee.l ".chars().collect::<Vec<char>>().try_into().unwrap(),
            "xlihcrra ".chars().collect::<Vec<char>>().try_into().unwrap(),
        ],
    ];

    let mult_locs: Vec<(usize, usize)> = vec![(0, 8), (1, 2), (9, 6)];

    let dict = Dictionary::new(&args);

    for board in boards {
        println!("\nBoard\n----------------------------------------");
        println!("    input board = {} bytes", &board.deep_size_of());
        let mut b = Board::new_from(board, mult_locs.clone(), 3);
        println!("             id = {}", b.id);
        println!("  board pre-pop = {} bytes", b.deep_size_of());
        let words = b.find_words(&dict, 100_000);
        println!(
            "          words = {} bytes / {} words",
            words.deep_size_of(),
            words.len()
        );
        let words_word_sizes = words.iter().map(|w| w.word.deep_size_of()).sum::<usize>();
        let words_path_sizes = words.iter().map(|w| w.path.deep_size_of()).sum::<usize>();
        let total_positions = words.iter().map(|w| w.path.len()).sum::<usize>();
        println!("     words word = {} bytes", words_word_sizes);
        println!("     words path = {} bytes", words_path_sizes);
        println!("words paths num = {}", total_positions);

        let w1 = words[0].clone();
        println!("     w1 overall = {}", w1.deep_size_of());
        println!(
            "        w1.path = {} / {} path items",
            w1.path.deep_size_of(),
            w1.path.len()
        );
        println!("        w1.word = {}", w1.word.deep_size_of());
        println!("       w1.score = {}", w1.score.deep_size_of());

        b.set_words(words);
        println!("  board ful-pop = {} bytes", b.deep_size_of());
        b.clean();
        println!("  board cleaned = {} bytes", b.deep_size_of());
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct InputBoard {
    board: [[char; Board::WIDTH]; Board::HEIGHT],
    mults: Vec<(usize, usize)>,
}

fn main() {
    let mut args = Args::parse();

    if args.size_test {
        size_test(args);
        return;
    }

    let mut input_str = String::new();
    args.input_f
        .read_to_string(&mut input_str)
        .unwrap_or_else(|e| panic!("Error reading {}: {}", args.input_f.path(), e));
    let input_board: InputBoard = ron::from_str(&input_str).unwrap_or_else(|e| {
        panic!(
            "{} doesn't look like the right kind of file: {}",
            args.input_f.path(),
            e
        )
    });

    /*
    let tiles = input_board
        .board
        .iter()
        .map(|r| r.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    */

    let game_run_time = std::time::Instant::now();
    if let Some(start) = args.start_max_children {
        for child_count in start..=args.max_children {
            args.max_children = child_count;
            game::play_game(
                &args,
                input_board.board,
                input_board.mults.clone(),
                game_run_time,
            )
        }
    } else {
        game::play_game(&args, input_board.board, input_board.mults.clone(), game_run_time)
    }

    if !args.quiet {
        println!("Finished in {}", HumanDuration(game_run_time.elapsed()));
    }
}
