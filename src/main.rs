pub mod board;
pub mod dictionary;
mod game;

use dictionary::Dictionary;

fn main() {
    let dictionary_db_name = "dictionary.db";
    let dict = Dictionary::new(&dictionary_db_name);
    dict.init_from("nwl/nwl2020.txt", 3);

    game::board_tests(dictionary_db_name);
}
