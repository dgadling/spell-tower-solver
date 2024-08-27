use crate::board::Board;
use crate::dictionary::Dictionary;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

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

pub fn play_game(dict: &mut Dictionary, board: Vec<Vec<String>>, mult_locs: Vec<(usize, usize)>) {
    /*
    id_test();
    evolution_test();
    */

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
    while !to_process.is_empty() {
        let mut b = all_boards.get(&to_process.pop().unwrap()).unwrap().clone();

        if b.searched() {
            bar.inc(1);
            continue;
        }

        b.find_words(dict);
        if b.is_terminal() {
            terminal_boards.push(b.id);
            bar.inc(1);
            continue;
        }

        for found_word in b.words().clone() {
            let new_board = b.evolve_via(b.id, found_word);
            to_process.push(new_board.id);
            all_boards.insert(new_board.id, new_board);
            bar.inc_length(1);
        }
        bar.inc(1);
    }

    println!("Ended with {} terminal boards", terminal_boards.len());
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
