use indicatif::ProgressBar;
use std::fs::File;
use std::io::{self, BufRead, LineWriter, Write};

fn filter_dict() {
    let in_lines = io::BufReader::new(File::open("words_alpha.txt").expect("Couldn't read?!"))
        .lines()
        .map(|l| l.unwrap());
    let mut out_f = LineWriter::new(File::create("words.txt").expect("Couldn't make new file?!"));

    let bar = ProgressBar::new(370105);

    in_lines.filter(|l| l.len() >= 3).for_each(|w| {
        out_f.write_all(format!("{}\n", w).as_bytes()).unwrap();
        bar.inc(1)
    });
    bar.finish();
}

fn main() {}
