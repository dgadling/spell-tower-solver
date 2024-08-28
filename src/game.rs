use crate::board::Board;
use crate::dictionary::Dictionary;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use r2d2_sqlite::SqliteConnectionManager;
/*

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
        let new_board = b.evolve_via(b.id.clone(), findings);
        b = new_board;
        println!("After");
        println!("------");
        println!("{}", b);
    }
}

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
*/

pub fn play_game(dict_path: &str, board: Vec<Vec<String>>, mult_locs: Vec<(usize, usize)>) {
    /*
    id_test();
    evolution_test();
    */

    let stop_now = Arc::new(AtomicBool::new(false));
    let r = stop_now.clone();

    ctrlc::set_handler(move || {
        r.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut stats = HashMap::new();
    let mut all_boards = HashMap::new();
    let mut terminal_boards = Vec::new();
    let mut to_process = Vec::new();

    let starting_board = Board::new_from(board, mult_locs);

    to_process.push(starting_board.id);
    all_boards.insert(starting_board.id, starting_board);

    let bar = ProgressBar::new(1);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed} {wide_bar:.blue} {human_pos:>}/{human_len} @ {per_sec}",
        )
        .unwrap()
        .progress_chars("-> "),
    );

    let mut bump = |name: &str| {
        stats
            .entry(name.to_owned())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    };

    let manager = SqliteConnectionManager::file(dict_path);
    let pool = r2d2::Pool::new(manager).unwrap();
    println!("{:?}", pool.state());

    let mut dict = Dictionary::with_conn(pool.get().unwrap());
    while !to_process.is_empty() {
        let board_id = to_process.pop().unwrap();
        let b = all_boards.get_mut(&board_id).unwrap();

        if b.searched() {
            bump("already_searched");
            bar.inc(1);
            continue;
        }

        if terminal_boards.contains(&b.id) {
            bump("already_found_terminal");
            bar.inc(1);
            continue;
        }

        b.find_words(&mut dict);

        // Now that we're done mutating, let's replace `b` with an immutable reference
        let b = all_boards.get(&board_id).unwrap();

        if b.is_terminal() {
            bump("found_terminal");
            terminal_boards.push(board_id);
            // Technically we don't need to update since  we'll find it in terminal_boards.
            // BUT this makes me feel better and technically saves a hash lookup
            //all_boards.insert(b.id, b);
            bar.inc(1);
            continue;
        }

        // To keep all_boards references immutable, let's keep a separate list of all the
        // Boards we're going to add to all_boards.
        let mut to_insert = HashMap::new();
        for found_word in b.words().clone() {
            let new_board = b.evolve_via(found_word);
            if all_boards.contains_key(&new_board.id) || to_insert.contains_key(&new_board.id) {
                /*
                   No need to push it into to_process only to immediately take it back out.
                   The only way it ended up in all_boards is that it was already discovered
                   and put in to_process.
                   If we already searched it, no need to repeat that work.
                   If we haven't searched it yet, it'll still be in to_process
                */
                bump("rediscovered");
            } else {
                to_process.push(new_board.id);
                to_insert.insert(new_board.id, new_board);
                bar.inc_length(1);
            }
        }

        assert!(b.searched(), "We literally just searched!");

        // Now that we found all the new boards, give them to all_boards
        all_boards.extend(to_insert);
        bar.inc(1);

        if stop_now.load(Ordering::SeqCst) {
            break;
        }
    }

    println!("{:?}", pool.state());
    println!("Stats = {:?}", stats);
    println!("Found {} unique terminal boards", terminal_boards.len());

    terminal_boards.sort_by(|a, b| {
        all_boards
            .get(b)
            .unwrap()
            .get_score()
            .cmp(&all_boards.get(a).unwrap().get_score())
    });

    let winner = all_boards.get(terminal_boards.get(0).unwrap()).unwrap();

    println!("Highest scoring had a score of {}", winner.get_score());

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
    winning_path.reverse();
    println!("Using a path of: {:?}", winning_path);
}
