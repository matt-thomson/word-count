use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = match env::args().nth(1) {
        Some(f) => f,
        None => {
            println!("Usage: word-count <input_filename>");
            return;
        }
    };

    let file = BufReader::new(File::open(filename).unwrap());
    let mut counts = HashMap::new();

    for line in file.lines() {
        for word in line.unwrap().split_whitespace() {
            let count = counts.entry(word.to_string()).or_insert(0);
            *count += 1;
        }
    }

    for (word, count) in counts.iter() {
        println!("{}\t{}", word, count);
    }
}
