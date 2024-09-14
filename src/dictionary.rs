use crate::Args;
use indicatif::{ProgressBar, ProgressStyle};
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

        if !args.quiet {
            println!("Reading words from {}", &args.dict_path);
        }

        // NOTE: The only reason we're doing this in two steps is so that we can
        // have the ProgressBar. If we ever decide we don't want that this can
        // all happen with one long chain.
        let words =
            BufReader::new(File::open(&args.dict_path).unwrap_or_else(|e| {
                panic!("Couldn't read word list from {}: {}", &args.dict_path, e)
            }))
            .lines()
            .map(|l| l.unwrap())
            .filter(|w| w.len() >= args.min_word_length)
            .collect::<Vec<String>>();

        let bar: ProgressBar;

        if args.quiet {
            bar = ProgressBar::hidden();
        } else {
            bar = ProgressBar::new(words.len() as u64);
        }

        bar.set_style(
            ProgressStyle::with_template(
                "{msg} {elapsed} {wide_bar:.blue} {human_pos:>}/{human_len}",
            )
            .unwrap()
            .progress_chars("-> "),
        );

        bar.set_message("Populating caches");

        words.iter().for_each(|word|{
            for l in 2..=word.len() {
                let mut prefix = String::with_capacity(l);
                prefix.push_str(&word[0..l]);
                d.path_cache.insert(prefix);
            }
            d.word_cache.insert(word.clone());
            bar.inc(1);
        });
        bar.finish();

        d
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
