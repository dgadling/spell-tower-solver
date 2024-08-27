use crate::board::{Board, FoundWord, Position};
use std::{collections::HashMap, time::SystemTime};

/*
*/

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

pub fn board_tests(dict_path: &str) {
    /*
    id_test();
    evolution_test();
    */

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

    let mut all_boards = HashMap::new();
    let mut b = Board::new_from(sample_board, mult_locs);

    let mut total_points = 0;
    for _ in 0..1000 {
        println!("Looking at\n-----\n{}", b);
        let now = SystemTime::now();
        b.find_words(dict_path);
        if b.is_terminal() {
            println!("We're done with {} points!", total_points);
            break;
        }
        println!(
            "Found {} words in {}ms!",
            b.words().len(),
            now.elapsed().unwrap().as_millis()
        );
        let best = b.words().get(0).unwrap().clone();
        println!("The best is {}, taking it", best);
        total_points += best.score;
        let new = b.evolve_via(b.id.clone(), best);
        all_boards.insert(b.id, b);
        b = new;
    }
}
