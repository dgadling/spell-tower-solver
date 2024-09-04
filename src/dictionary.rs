use crate::Args;
use indicatif::{ProgressBar, ProgressStyle};
use rusqlite::Connection;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Dictionary {
    word_cache: HashSet<String>,
    path_cache: HashSet<String>,
}

impl Dictionary {
    pub fn new(args: &Args) -> Self {
        let mut d = Dictionary {
            word_cache: HashSet::new(),
            path_cache: HashSet::new(),
        };

        d.prime_caches(&args);
        d
    }

    pub fn init_from(args: &Args) {
        if !args.quiet {
            println!("Checking database integrity");
        }
        let conn = Connection::open(&args.db_path)
            .unwrap_or_else(|e| panic!("Couldn't open {}: {}", args.db_path, e));
        let rows_res = conn.query_row("SELECT count(*) FROM words", [], |r| {
            r.get::<usize, usize>(0)
        });

        match rows_res {
            Ok(count) => {
                if !args.quiet {
                    println!("Look at the db and found {} rows", count);
                }
                if count == 191_745 {
                    if !args.quiet {
                        println!("Sounds about right");
                    }
                    return;
                }
            }
            _ => {}
        };

        if !args.quiet {
            println!("That doesn't look right. Trying to load data.");
        }
        Dictionary::init_db(&args);
    }

    fn init_db(args: &Args) {
        if !args.quiet {
            println!("Creating database of valid words from {}", &args.dict_path);
        }
        let in_lines = BufReader::new(File::open(&args.dict_path).expect("Couldn't read?!"))
            .lines()
            .map(|l| l.unwrap());

        let conn = Connection::open(&args.db_path)
            .unwrap_or_else(|e| panic!("Couldn't open {}: {}", args.db_path, e));

        let bar: ProgressBar;
        if args.quiet {
            bar = ProgressBar::hidden();
        } else {
            bar = ProgressBar::new(191745);
        }

        conn.execute(
            "
                CREATE TABLE words (
                    id INTEGER PRIMARY KEY,
                    word TEXT NOT NULL,
                    base_points INTEGER,
                    UNIQUE(word)
                )
                ",
            (),
        )
        .unwrap_or_else(|e| panic!("Couldn't create base table: {}", e));

        conn.execute("BEGIN TRANSACTION", ())
            .unwrap_or_else(|e| panic!("Couldn't even start a transaction: {}", e));
        in_lines
            .filter(|l| l.len() >= args.min_word_length)
            .for_each(|w| {
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
    }

    fn prime_caches(&mut self, args: &Args) {
        if !args.quiet {
            println!("Priming caches");
        }
        let conn = Connection::open(&args.db_path)
            .unwrap_or_else(|e| panic!("Couldn't open {}: {}", &args.db_path, e));

        let bar: ProgressBar;
        if args.quiet {
            bar = ProgressBar::hidden();
        } else {
            bar = ProgressBar::new(191745);
        }
        bar.set_style(
            ProgressStyle::with_template(
                "{msg} {elapsed} {wide_bar:.blue} {human_pos:>}/{human_len}",
            )
            .unwrap()
            .progress_chars("-> "),
        );

        bar.set_message("Loading up whole words");
        let mut stmt = conn.prepare("SELECT word FROM words").unwrap();
        let mut rows = stmt.query([]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let word: String = row.get(0).unwrap();
            self.word_cache.insert(word);
            bar.inc(1);
        }
        bar.finish();

        bar.set_length(1_706_901);
        bar.set_message("Loading up paths");
        for word in self.word_cache.iter() {
            for l in 2..=word.len() {
                let mut prefix = String::with_capacity(l);
                prefix.push_str(&word[0..l]);
                self.path_cache.insert(prefix);
                bar.inc(1);
            }
        }
    }

    pub fn has_path(&self, prefix: &str) -> bool {
        let has = self.path_cache.get(prefix);
        has.is_some()
    }

    pub fn is_word(&self, prefix: &str) -> bool {
        let has = self.word_cache.get(prefix);
        has.is_some()
    }
}
