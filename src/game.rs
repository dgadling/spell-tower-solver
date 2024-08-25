use crate::board::Board;
use std::time::SystemTime;

/*
use phf::phf_set;
static CLEARS_ROW: phf::Set<char> = phf_set!('j', 'q', 'x', 'z');
*/

pub fn board_tests(dict_path: &str) {
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

    let b = Board::new_from(sample_board, mult_locs);
    let now = SystemTime::now();
    let found_words = b.find_words(dict_path);
    println!(
        "Found {} words in {}ms! Here's the highest scoring 15!",
        found_words.len(),
        now.elapsed().unwrap().as_millis()
    );
    for (word, paths) in found_words.into_iter().take(15) {
        println!("  {} via {:?}", word, paths);
    }
}
