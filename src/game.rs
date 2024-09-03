use crate::board::Board;
use crate::dictionary::Dictionary;
use crate::Args;

#[allow(unused_imports)]
use deepsize::DeepSizeOf;

#[allow(unused_imports)]
use indicatif::{HumanBytes, HumanCount, ProgressBar, ProgressStyle};

use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
#[allow(unused_imports)]
use std::sync::atomic::{AtomicBool, Ordering};
#[allow(unused_imports)]
use std::sync::Arc;

#[allow(dead_code)]
use crate::board::{FoundWord, Position};

#[allow(dead_code)]
fn evolution_test() {
    let sample_board = vec![
        /*
         012345678
        */
        "i.ssbtpod".chars().map(|c| c.to_string()).collect(), // 0
        "mcisneice".chars().map(|c| c.to_string()).collect(), // 1
        "hcrqsovaa".chars().map(|c| c.to_string()).collect(), // 2
        "ln.sgsnnr".chars().map(|c| c.to_string()).collect(), // 3
        "eiusyijme".chars().map(|c| c.to_string()).collect(), // 4
    ];

    let word_pickings = vec![
        FoundWord {
            score: 1,
            word: "ice".to_string(),
            path: vec![
                Position { row: 1, col: 6 },
                Position { row: 1, col: 7 },
                Position { row: 1, col: 8 },
            ],
        },
        FoundWord {
            score: 1,
            word: "icicle".to_string(),
            path: vec![
                Position { row: 0, col: 0 },
                Position { row: 1, col: 1 },
                Position { row: 1, col: 2 },
                Position { row: 2, col: 1 },
                Position { row: 3, col: 0 },
                Position { row: 4, col: 0 },
            ],
        },
    ];

    let mut b = Board::new_from(sample_board, vec![]);
    println!("Before");
    println!("------");
    println!("{}", b);

    for findings in word_pickings {
        println!("Taking {}", findings.word);
        let new_board = b.evolve_via(findings);
        b = new_board;
        println!("After");
        println!("------");
        println!("{}", b);
    }
}

#[allow(dead_code)]
fn id_test() {
    let sample_b1 = vec![
        /*
         012345678
        */
        "i.ssbtpod".chars().map(|c| c.to_string()).collect(), // 0
        "mcisneice".chars().map(|c| c.to_string()).collect(), // 1
        "hcrqsovaa".chars().map(|c| c.to_string()).collect(), // 2
        "ln.sgsnnr".chars().map(|c| c.to_string()).collect(), // 3
        "eiusyijme".chars().map(|c| c.to_string()).collect(), // 4
    ];
    let sample_b2 = vec![
        /*
         012345678
        */
        "i.ssbtpod".chars().map(|c| c.to_string()).collect(), // 0
        "mcisneice".chars().map(|c| c.to_string()).collect(), // 1
        "hcrqsovaa".chars().map(|c| c.to_string()).collect(), // 2
        "ln.sgsnnr".chars().map(|c| c.to_string()).collect(), // 3
        "eiusyijme".chars().map(|c| c.to_string()).collect(), // 4
    ];

    let b1 = Board::new_from(sample_b1, vec![]);
    let b2 = Board::new_from(sample_b2, vec![]);

    println!("b1.id = {}, b2.id = {}", b1.id, b2.id);
}

pub fn play_game(args: &Args, board: Vec<Vec<String>>, mult_locs: Vec<(usize, usize)>) {
    let mut all_boards = HashMap::new();
    let mut terminal_boards = HashSet::new();
    let mut to_process = Vec::new();

    let starting_board = Board::new_from(board, mult_locs);

    to_process.push(starting_board.id);
    all_boards.insert(starting_board.id, starting_board);

    let bar_style = ProgressStyle::with_template(
        "{msg} {elapsed} {wide_bar:.blue} {human_pos:>}/{human_len} @ {per_sec}",
    )
    .unwrap()
    .progress_chars("-> ");
    let dict = Dictionary::new(&args.db_path);
    let mut generation = 1;
    while !to_process.is_empty() {
        print!("Generation {: >2}", generation);
        if args.memory_debug {
            print!(
                ": {} boards to process ({}) ; {} boards total ({})",
                HumanCount(to_process.len() as u64),
                HumanBytes(to_process.deep_size_of() as u64),
                HumanCount(all_boards.len() as u64),
                HumanBytes(all_boards.deep_size_of() as u64)
            );
        }
        println!();

        let to_process_len = to_process.len() as u64;
        let bar = ProgressBar::new(to_process_len);
        bar.set_style(bar_style.clone());
        bar.set_message("🔎");

        // Search the boards in this generation, provided they're not somehow dupes
        let newly_searched = to_process
            .par_iter()
            .map(|board_id| {
                let b = all_boards.get(&board_id).unwrap();

                if b.searched() {
                    bar.inc(1);
                    return None;
                }

                if terminal_boards.contains(board_id) {
                    bar.inc(1);
                    return None;
                }

                let words = b.find_words(&dict, args.max_children);
                bar.inc(1);
                Some((board_id.clone(), words))
            })
            .flatten()
            .collect::<Vec<(u64, Vec<FoundWord>)>>();

        /*
        Now do a few things:
        1. Update the Board (living in all_boards) with its word list
        2. If the board is terminal, update terminal_boards with its id
        3. Otherwise, emit the ID as a board to be used next to make the next generation
         */
        let boards_to_work = newly_searched
            .iter()
            .map(|(board_id, new_words)| {
                all_boards
                    .entry(*board_id)
                    .and_modify(|b| b.set_words(new_words.clone()));

                if new_words.len() == 0 {
                    terminal_boards.insert(*board_id);
                    None
                } else {
                    Some(*board_id)
                }
            })
            .flatten()
            .collect::<Vec<u64>>();

        let bar = ProgressBar::new(boards_to_work.len() as u64);
        bar.set_style(bar_style.clone());
        bar.set_message("📈");

        let new_to_process = boards_to_work
            .chunks(args.evolution_batch_size)
            .map(|boards| {
                let boards_to_add = boards
                    .par_iter()
                    .map(|b_id| {
                        let b = all_boards.get(b_id).unwrap();

                        // To keep all_boards references immutable, let's keep a separate list of all the
                        // Boards we're going to add to all_boards.
                        let mut new_boards: HashMap<u64, Board> = HashMap::new();
                        for found_word in b.words().clone() {
                            let new_board = b.evolve_via(found_word);

                            // Now let's check if this new board is *actually* new
                            if new_boards.contains_key(&new_board.id) {
                                // TODO: Figure out if we want to replace all_boards[new_board.id] with this one
                                // (e.g. for higher score) and what would need to happen if we did. Since this board state
                                // hasn't been searched yet, maybe a simple swap is OK.

                                // One of our siblings (with the same/higher score) has the same net-effect, skip this one
                                continue;
                            } else if boards_to_work.contains(&new_board.id) {
                                // TODO: Figure out if we want to replace all_boards[new_board.id] with this one
                                // (e.g. for higher score) and what would need to happen if we did. Since this board state
                                // hasn't been searched yet, maybe a simple swap is OK.

                                // b managed to evolve one of it siblings, skip it
                                continue;
                            } else if all_boards.contains_key(&new_board.id) {
                                // TODO: Figure out if we want to replace all_boards[new_board.id] with this one
                                // (e.g. for higher score) and what would need to happen if we did. Since this board state
                                // **HAS** been searched, we'd need to update any descendants scores with the delta

                                // This board was born in a previous generation
                                continue;
                            }
                            new_boards.insert(new_board.id, new_board);
                        }
                        new_boards
                    })
                    .flatten()
                    .collect::<HashMap<u64, Board>>();

                // Update to_process with all the new boards we found
                let batch_new_to_process = boards_to_add
                    .par_iter()
                    .map(|(b_id, _)| b_id.clone())
                    .collect::<Vec<u64>>();

                for board_id in boards {
                    // Now that we've generated our children boards we don't need to hold on
                    // to our tiles any longer
                    all_boards.entry(*board_id).and_modify(|b| b.clean());
                }

                // And update all_boards with all the new boards we found
                all_boards.extend(boards_to_add);
                bar.inc(boards.len() as u64);
                batch_new_to_process
            })
            .flatten()
            .collect::<Vec<u64>>();

        println!();
        generation += 1;

        to_process = new_to_process;
    }

    println!(
        "Found {} unique terminal boards",
        HumanCount(terminal_boards.len() as u64)
    );

    let mut final_term_boards = terminal_boards.into_iter().collect::<Vec<u64>>();
    final_term_boards.par_sort_by(|a, b| {
        all_boards
            .get(b)
            .unwrap()
            .get_score()
            .cmp(&all_boards.get(a).unwrap().get_score())
    });

    let winner = all_boards.get(final_term_boards.get(0).unwrap()).unwrap();

    println!("Highest scoring had a score of {}", winner.get_score());

    // From our winning terimal board, work backwards up to the starting board
    let mut winning_path = vec![];
    let mut curr_board = winner;
    loop {
        if curr_board.evolved_from() == 0 {
            break;
        }

        winning_path.push(curr_board.evolved_via().word);
        curr_board = all_boards
            .get(&all_boards.get(&curr_board.id).unwrap().evolved_from())
            .unwrap();
    }
    // Now reverse that so winning_path is a list of moves to make from the beginning
    winning_path.reverse();
    println!("Using a path of: {:?}", winning_path);
}
