use crate::board::{Board, FoundWord};
use crate::dictionary::Dictionary;
use crate::Args;

use deepsize::DeepSizeOf;
use indicatif::{HumanBytes, HumanCount, HumanDuration, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[cfg(target_os = "windows")]
use mimalloc::MiMalloc;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub fn play_game(args: &Args, board: Vec<Vec<String>>, mult_locs: Vec<(usize, usize)>, game_start_time: Instant) {
    let mut all_boards = HashMap::new();
    let mut terminal_boards = HashSet::new();
    let mut to_process = Vec::new();

    let starting_board = Board::new_from(board, mult_locs, args.min_word_length);

    to_process.push(starting_board.id);
    all_boards.insert(starting_board.id, starting_board);

    let bar_style = ProgressStyle::with_template(
        "{msg} {elapsed} {wide_bar:.blue} {human_pos:>}/{human_len} @ {per_sec}",
    )
    .unwrap()
    .progress_chars("-> ");

    let dict = Dictionary::new(&args);
    let mut generation = 1_u32;
    while !to_process.is_empty() {
        let to_process_len = to_process.len() as u64;
        let bar: ProgressBar;
        if args.quiet {
            bar = ProgressBar::hidden();
        } else {
            print!("Generation {: >2}", generation);
            if args.memory_debug {
                print!(
                    ": {} boards to process ({}) ; {} terminal boards ; {} boards total ({})",
                    HumanCount(to_process.len() as u64),
                    HumanBytes(to_process.deep_size_of() as u64),
                    HumanCount(terminal_boards.len() as u64),
                    HumanCount(all_boards.len() as u64),
                    HumanBytes(all_boards.deep_size_of() as u64)
                );
            }
            println!();
            bar = ProgressBar::new(to_process_len);
            bar.set_style(bar_style.clone());
            bar.set_message("🔎");
        }

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
        bar.finish();

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
            .collect::<HashSet<u64>>();

        let bar: ProgressBar;
        if args.quiet {
            bar = ProgressBar::hidden();
        } else {
            bar = ProgressBar::new(boards_to_work.len() as u64);
            bar.set_style(bar_style.clone());
            bar.set_message("📈");
        }

        let boards_to_iter = Vec::from_iter(boards_to_work.iter());
        let new_to_process = boards_to_iter
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

                // And update all_boards with all the new boards we found
                all_boards.extend(boards_to_add);
                bar.inc(boards.len() as u64);
                batch_new_to_process
            })
            .flatten()
            .collect::<HashSet<u64>>();
        bar.finish();

        // Now clean up everything that's "dirty"
        let boards_to_clean = all_boards
            .keys()
            .collect::<Vec<&u64>>()
            .into_par_iter()
            .filter_map(|k| {
                if new_to_process.contains(k) || !all_boards.get(k).unwrap().dirty() {
                    None
                } else {
                    Some(k.clone())
                }
            })
            .collect::<Vec<u64>>();

        let bar: ProgressBar;
        if args.quiet {
            bar = ProgressBar::hidden();
        } else {
            bar = ProgressBar::new(boards_to_clean.len() as u64);
            bar.set_style(bar_style.clone());
            bar.set_message("🧹");
        }

        for board_id in boards_to_clean {
            all_boards.entry(board_id).and_modify(|b| b.clean());
            bar.inc(1);
        }
        bar.finish();

        if !args.quiet {
            println!();
        }
        generation += 1;
        if generation > args.max_generations {
            break;
        }

        to_process = Vec::from_iter(new_to_process.into_iter());
        to_process.sort_by(|a, b| {
            all_boards
                .get(b)
                .unwrap()
                .get_score()
                .cmp(&all_boards.get(a).unwrap().get_score())
        });
        to_process.truncate(args.max_gen_size);
    }

    let term_count = terminal_boards.len();

    let mut final_term_boards = terminal_boards.into_iter().collect::<Vec<u64>>();
    final_term_boards.par_sort_by(|a, b| {
        all_boards
            .get(b)
            .unwrap()
            .get_score()
            .cmp(&all_boards.get(a).unwrap().get_score())
    });

    let winner = all_boards.get(final_term_boards.get(0).unwrap()).unwrap();

    // From our winning terimal board, work backwards up to the starting board
    let mut winning_path = vec![];
    let mut curr_board = winner;
    loop {
        if curr_board.evolved_from() == 0 {
            break;
        }

        winning_path.push(curr_board.evolved_via());
        curr_board = all_boards
            .get(&all_boards.get(&curr_board.id).unwrap().evolved_from())
            .unwrap();
    }
    // Now reverse that so winning_path is a list of moves to make from the beginning
    winning_path.reverse();

    if !args.quiet {
        println!(
            "Found {} unique terminal boards",
            HumanCount(term_count as u64)
        );
    }

    println!(
        "{: >5} via {: >2} words",
        HumanCount(winner.get_score() as u64),
        winning_path.len()
    );

    for p in winning_path {
        println!(
            "{: >15}: {:?}",
            p.word,
            p.path
                .iter()
                .map(|pos| format!("{}", pos))
                .collect::<Vec<String>>()
        )
    }
    if !args.quiet {
        println!("Finished playing in {}", HumanDuration(game_start_time.elapsed()));
    }
}
