use indicatif::ProgressBar;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rusqlite::{Connection, Statement};

pub struct Dictionary {
    source_file: String,
    words_db_path: String,
    min_word_len: usize,
    conn: rusqlite::Connection,
}

impl Dictionary {
    pub fn new(src: &str, db_path: &str, min_word_len: usize) -> Self {
        let conn = Connection::open(db_path)
            .unwrap_or_else(|e| panic!("Couldn't open {}: {}", db_path, e));

        let us = Dictionary {
            source_file: src.to_string(),
            words_db_path: db_path.to_string(),
            min_word_len,
            conn,
        };

        us.init();

        us
    }

    fn init(&self) {
        if Path::new(&self.words_db_path).exists() {
            println!("Checking database integrity");
            let rows_res = self.conn.query_row("SELECT count(*) FROM words", [], |r| {
                r.get::<usize, usize>(0)
            });

            match rows_res {
                Ok(count) => {
                    println!("Look at the db and found {} rows", count);
                    if count > 369_000 {
                        println!("Sounds about right");
                        return;
                    }
                }
                _ => {}
            };
        }

        println!("That doesn't look right. Trying to load data.");
        self.init_db();
    }

    fn init_db(&self) {
        println!(
            "Creating database of valid words from {}",
            &self.source_file
        );
        let in_lines = io::BufReader::new(File::open(&self.source_file).expect("Couldn't read?!"))
            .lines()
            .map(|l| l.unwrap());

        let bar = ProgressBar::new(370105);

        self.conn
            .execute(
                "
            CREATE TABLE words (
                id INTEGER PRIMARY KEY,
                word TEXT NOT NULL,
                base_points INTEGER,
                UNIQUE(word)
            )",
                (),
            )
            .unwrap_or_else(|e| panic!("Couldn't create base table: {}", e));

        self.conn
            .execute("BEGIN TRANSACTION", ())
            .unwrap_or_else(|e| panic!("Couldn't even start a transaction: {}", e));
        in_lines
            .filter(|l| l.len() >= self.min_word_len)
            .for_each(|w| {
                self.conn
                    .execute(
                        "INSERT INTO words (word, base_points) VALUES (?1, ?2)",
                        (&w, 1),
                    )
                    .unwrap_or_else(|e| panic!("Couldn't insert {} into the database: {}", w, e));
                bar.inc(1)
            });
        bar.finish();
        self.conn
            .execute("COMMIT", ())
            .unwrap_or_else(|e| panic!("Couldn't commit the transaction: {}", e));

        println!("Creating an index");
        self.conn
            .execute("CREATE INDEX words_word ON words(word)", ())
            .unwrap_or_else(|e| panic!("Failed to create index: {}", e));
        println!("Done init-ing!");
    }

    fn get_query_for(&mut self, options_len: usize) -> Statement {
        let placeholders = std::iter::repeat("?")
            .take(options_len)
            .collect::<Vec<_>>()
            .join(", ");

        self.conn
            .prepare(&format!(
                "SELECT DISTINCT substr(word, ?, 1) FROM words WHERE substr(word, 1, ?) IN ({})",
                placeholders
            ))
            .expect("Couldn't prepare a statement?!")
    }

    pub fn get_candidates_for(&mut self, prefix: &str, options: &Vec<&str>) -> Vec<String> {
        let mut stmt = self.get_query_for(options.len());

        stmt.raw_bind_parameter(1, prefix.len() + 1).unwrap();
        stmt.raw_bind_parameter(2, prefix.len() + 1).unwrap();

        for option_idx in 0..options.len() {
            let to_bind = prefix.to_owned() + options.get(option_idx).unwrap();
            stmt.raw_bind_parameter(option_idx + 3, to_bind).unwrap();
        }

        let mut rows = stmt.raw_query();
        let mut candidates = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            let foo: String = row.get_unwrap(0);
            candidates.push(foo);
        }

        return candidates;
    }
}
