use crate::board::{Board, FoundWord, Position};
use std::time::SystemTime;

/*
use phf::phf_set;
static CLEARS_ROW: phf::Set<char> = phf_set!('j', 'q', 'x', 'z');
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
        let b = b.evolve_via(findings);
        println!("After");
        println!("------");
        println!("{}", b);
    }
}

pub fn board_tests(dict_path: &str) {
    evolution_test();

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

    let mut b = Board::new_from(sample_board, mult_locs);
    let mut total_points = 0;
    for _ in 0..=5 {
        println!("Looking at\n-----\n{}", b);
        let now = SystemTime::now();
        b.find_words(dict_path);
        println!(
            "Found {} words in {}ms!",
            b.words().len(),
            now.elapsed().unwrap().as_millis()
        );
        let best = b.words().get(0).unwrap().clone();
        println!("The best is {}, taking it", best);
        total_points += best.score;
        b = b.evolve_via(best);
    }
    println!("After getting {} points, the board looks like\n-----\n{}", total_points, b);
}
