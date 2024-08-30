//use indicatif::ProgressBar;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
//use rusqlite::Connection;
use std::collections::HashMap;
//use std::fs::File;
//use std::io::{BufRead, BufReader};

pub struct Dictionary {
    conn: PooledConnection<SqliteConnectionManager>,
    word_cache: HashMap<String, bool>,
    path_cache: HashMap<String, bool>,
    path_queries: u64,
    path_hits: u64,
    word_queries: u64,
    word_hits: u64,
}

impl Dictionary {
    /*
    pub fn new(db_path: &str) -> Self {
        let conn = Connection::open(db_path)
            .unwrap_or_else(|e| panic!("Couldn't open {}: {}", db_path, e));

        Dictionary {
            conn,
            word_cache: HashMap::new(),
            path_cache: HashMap::new(),
        }
    }

    pub fn init_from(&self, source_file: &str, min_word_len: usize) {
        println!("Checking database integrity");
        let rows_res = self.conn.query_row("SELECT count(*) FROM words", [], |r| {
            r.get::<usize, usize>(0)
        });

        match rows_res {
            Ok(count) => {
                println!("Look at the db and found {} rows", count);
                if count == 191_745 {
                    println!("Sounds about right");
                    return;
                }
            }
            _ => {}
        };

        println!("That doesn't look right. Trying to load data.");
        self.init_db(source_file, min_word_len);
    }
    */

    pub fn with_conn(conn: PooledConnection<SqliteConnectionManager>) -> Self {
        Dictionary {
            conn,
            word_cache: HashMap::new(),
            path_cache: HashMap::new(),
            path_queries: 0,
            path_hits: 0,
            word_queries: 0,
            word_hits: 0,
        }
    }

    /*
    fn init_db(&self, source_file: &str, min_word_len: usize) {
        println!("Creating database of valid words from {}", source_file);
        let in_lines = BufReader::new(File::open(source_file).expect("Couldn't read?!"))
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
                )
                ",
                (),
            )
            .unwrap_or_else(|e| panic!("Couldn't create base table: {}", e));

        self.conn
            .execute("BEGIN TRANSACTION", ())
            .unwrap_or_else(|e| panic!("Couldn't even start a transaction: {}", e));
        in_lines.filter(|l| l.len() >= min_word_len).for_each(|w| {
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

        println!("Optimizing");
        self.conn
            .execute_batch(
                "
                CREATE INDEX words_word ON words(word);
                CREATE VIRTUAL TABLE optimized_words USING FTS5(word);
                INSERT INTO optimized_words(word) SELECT word FROM words;
                ",
            )
            .unwrap_or_else(|e| panic!("Failed to optimize: {}", e));

        println!("Done init-ing!");
    }
    */

    pub fn print_stats(&self) {
        println!(
            "has_path: queries = {}, hits {}, hit ratio = {:.4}, db queries = {}",
            self.path_queries,
            self.path_hits,
            100.0 * (self.path_hits as f64 / self.path_queries as f64),
            self.path_queries - self.path_hits
        );
        println!(
            " is_word: queries = {}, hits {}, hit ratio = {:.4}, db queries = {}",
            self.word_queries,
            self.word_hits,
            100.0 * (self.word_hits as f64 / self.word_queries as f64),
            self.word_queries - self.word_hits
        );
    }

    pub fn has_path(&mut self, prefix: &str) -> bool {
        self.path_queries += 1;
        if let Some(ans) = self.path_cache.get(prefix) {
            self.path_hits += 1;
            return *ans;
        }

        // NOTE: Using the FTS5 table for prefix matching is SEVERAL ORDERS OF MAGNITUDE faster!
        let query = format!(
            "SELECT 1 FROM optimized_words WHERE word MATCH '{}*'",
            prefix
        );

        let word_found = self
            .conn
            .query_row(&query, [], |row| row.get(0) as Result<u32, rusqlite::Error>);

        self.path_cache
            .insert(prefix.to_string(), word_found.is_ok());
        word_found.is_ok()
    }

    pub fn is_word(&mut self, prefix: &str) -> bool {
        self.word_queries += 1;
        if let Some(ans) = self.word_cache.get(prefix) {
            self.word_hits += 1;
            return *ans;
        }

        // NOTE: Going the normaly route is SIGNIFICANTLY faster for straight equality checking
        let query = format!("SELECT word FROM words WHERE word = '{}'", prefix);

        let word = self.conn.query_row(&query, [], |row| {
            row.get(0) as Result<String, rusqlite::Error>
        });

        self.word_cache.insert(prefix.to_string(), word.is_ok());
        word.is_ok()
    }
}
