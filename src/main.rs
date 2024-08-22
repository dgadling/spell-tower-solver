mod board;
pub mod dictionary;

use board::Board;
use dictionary::Dictionary;

/*
fn dictionary_tests(dict: &mut Dictionary) {
    let mut foo = dict.get_candidates_for("pin", &vec!["c", "f", "u", "s", "n", "e"]);
    println!("Candidates for pin = {:?}", &foo);
    foo = dict.get_candidates_for("pinu", &vec!["g", "f", "m", "s"]);
    println!("Candidates for pinu = {:?}", &foo);
    foo = dict.get_candidates_for("pinus", &vec!["f", "e", "n"]);
    println!("Candidates for pinus = {:?}", &foo);
}

fn scoring_tests() {
    let lie = vec![Tile::new("l", 1), Tile::new("i", 1), Tile::new("e", 1)];
    let tear = vec![
        Tile::new("t", 2),
        Tile::new("e", 1),
        Tile::new("a", 1),
        Tile::new("r", 1),
    ];
    let rear = vec![
        Tile::new("r", 1),
        Tile::new("e", 1),
        Tile::new("a", 1),
        Tile::new("r", 1),
    ];
    let swan = vec![
        Tile::new("s", 1),
        Tile::new("w", 1),
        Tile::new("a", 1),
        Tile::new("n", 1),
    ];
    let squalid = vec![
        Tile::new("s", 1),
        Tile::new("q", 1),
        Tile::new("u", 1),
        Tile::new("a", 1),
        Tile::new("l", 2),
        Tile::new("i", 1),
        Tile::new("d", 1),
    ];
    println!("Score for lie = {}", Board::score_for(&lie));
    println!("Score for 2x tear = {}", Board::score_for(&tear));
    println!("Score for rear = {}", Board::score_for(&rear));
    println!("Score for squalid = {}", Board::score_for(&squalid));
    println!("Score for swan = {}", Board::score_for(&swan));
}
*/

fn board_tests(dict_path: &str) {
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
    b.find_words(dict_path);
}

fn main() {
    let dictionary_db_name = "dictionary.db";
    let dict = Dictionary::new(&dictionary_db_name);
    dict.init_from("words_alpha.txt", 3);
    /*
    dictionary_tests(&mut dict);
    scoring_tests();
    */
    board_tests(dictionary_db_name);
}
