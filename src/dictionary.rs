use indicatif::ProgressBar;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rusqlite::Connection;

#[derive(Debug)]
struct Word {
    id: u32,
    word: String,
    base_points: u8,
}

const SOURCE_WORDS: &'static str = "words_alpha.txt";
const WORDS_DB: &'static str = "dictionary.db";
const MIN_WORD_LEN: usize = 3;

pub fn init() {
    if Path::new(WORDS_DB).exists() {
        return;
    }

    println!("Creating database of valid words from {}", SOURCE_WORDS);
    let in_lines = io::BufReader::new(File::open(SOURCE_WORDS).expect("Couldn't read?!"))
        .lines()
        .map(|l| l.unwrap());

    let bar = ProgressBar::new(370105);
    let conn =
        Connection::open(WORDS_DB).unwrap_or_else(|e| panic!("Couldn't open {}: {}", WORDS_DB, e));

    conn.execute(
        "
        CREATE TABLE words (
            id INTEGER PRIMARY KEY,
            word TEXT NOT NULL,
            base_points INTEGER
    )",
        (),
    )
    .unwrap_or_else(|e| panic!("Couldn't create base table: {}", e));

    conn.execute("BEGIN TRANSACTION", ())
        .unwrap_or_else(|e| panic!("Couldn't even start a transaction: {}", e));
    in_lines.filter(|l| l.len() >= MIN_WORD_LEN).for_each(|w| {
        conn.execute(
            "INSERT INTO words (word, base_points) VALUES (?1, ?2)",
            (&w, 1),
        )
        .unwrap_or_else(|e| panic!("Couldn't insert {} into the database: {}", w, e));
        bar.inc(1)
    });
    bar.finish();
    conn.execute("COMMIT", ())
        .unwrap_or_else(|e| panic!("Couldn't commit the transaction: {}", e));

    println!("Creating an index");
    conn.execute("CREATE INDEX words_word ON words(word)", ())
        .unwrap_or_else(|e| panic!("Failed to create index: {}", e));
    println!("Done init-ing!");
}
