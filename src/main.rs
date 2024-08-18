mod dictionary;
use dictionary::Dictionary;

fn main() {
    let dict = Dictionary::new("words_alpha.txt", "dictionary.db", 3);
}
