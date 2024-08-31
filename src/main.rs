pub mod board;
pub mod dictionary;
mod game;

//use dictionary::Dictionary;

fn main() {
    let dictionary_db_name = "dictionary.db";
    //let mut dict = Dictionary::new(&dictionary_db_name);
    //dict.init_from("nwl/nwl2020.txt", 3);

    let sample_board = vec![
        "i.ssbtpod".chars().map(|c| c.to_string()).collect(),
        "mcisneice".chars().map(|c| c.to_string()).collect(),
        "hcrqsovaa".chars().map(|c| c.to_string()).collect(),
        "ln.sgsnnr".chars().map(|c| c.to_string()).collect(),
        // "eiusyijme".chars().map(|c| c.to_string()).collect(),
        // "olmgapelf".chars().map(|c| c.to_string()).collect(),
        // "tsaeeudhn".chars().map(|c| c.to_string()).collect(),
        // "bsoenditr".chars().map(|c| c.to_string()).collect(),
        // "cwoopteaf".chars().map(|c| c.to_string()).collect(),
        // "itzoutner".chars().map(|c| c.to_string()).collect(),
        // ".upriigal".chars().map(|c| c.to_string()).collect(),
        // "tkayee.ld".chars().map(|c| c.to_string()).collect(),
        // "xlihcrras".chars().map(|c| c.to_string()).collect(),
    ];

    let mult_locs: Vec<(usize, usize)> = vec![(0, 8), (1, 2), (9, 6)];

    game::play_game(dictionary_db_name, sample_board, mult_locs);
}
