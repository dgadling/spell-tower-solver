mod dictionary;
use dictionary::Dictionary;

fn main() {
    let mut dict = Dictionary::new("words_alpha.txt", "dictionary.db", 3);

    let mut foo = dict.get_candidates_for("pin", &vec!["c", "f", "u", "s", "n", "e"]);
    println!("Candidates for pin = {:?}", &foo);
    foo = dict.get_candidates_for("pinu", &vec!["g", "f", "m", "s"]);
    println!("Candidates for pinu = {:?}", &foo);
    foo = dict.get_candidates_for("pinus", &vec!["f", "e", "n"]);
    println!("Candidates for pinus = {:?}", &foo);
}
