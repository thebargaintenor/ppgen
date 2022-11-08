use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
#[clap(name = "ppgen")]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to EFF wordlist
    #[arg(short, long)]
    wordlist: String,

    /// Number of words in passphrase
    #[arg(short, long, default_value_t = 4)]
    size: i8,
}

fn main() {
    let args = Args::parse();

    let mut rng = rand::thread_rng();

    // canonical order of words from EFF list
    let keys: Vec<String> = (0..args.size)
        .map(|_| {
            let rolls: Vec<String> = (0..5)
                .map(|_| rng.gen_range(1..=6))
                .map(|i| i.to_string())
                .collect();
            rolls.join("")
        })
        .collect();

    let mut keymap: HashMap<String, String> = keys.iter().map(|k| (k.clone(), "".into())).collect();

    if let Ok(lines) = read_lines(args.wordlist) {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                let mut record = line.split_ascii_whitespace();
                let key = record.next().unwrap_or("").to_string();
                let word = record.next().unwrap_or("").to_string();

                if keymap.contains_key(&key) {
                    keymap.entry(key).and_modify(|e| *e = word);
                }
            }
        }
    }

    keys.iter()
        .for_each(|k| println!("{}", keymap.get(k).unwrap_or(&String::from(""))));
}

// from rust by example
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
