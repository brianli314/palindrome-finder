use crate::exact_matches::match_exact;
use crate::smith_waterman::smith_waterman;
use crate::output::PalindromeData;
use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};
use std::mem;

pub struct Fasta {
    name: String,
    sequence: String,
}
impl Fasta {
    pub fn new(name: String, sequence: String) -> Self {
        Self { name, sequence }
    }
}

pub struct FastaIterator {
    lines_reader: Lines<BufReader<File>>,
    curr_name: String,
}

impl Iterator for FastaIterator {
    type Item = Fasta;

    fn next(&mut self) -> Option<Self::Item> {
        let mut seq = String::new();
        while let Some(line) = self.lines_reader.next() {
            let line = line.expect("Failed to read from fasta!");
            if line.starts_with(">") {
                if seq.len() == 0 {
                    continue;
                }

                let mut name = line.to_owned();
                mem::swap(&mut name, &mut self.curr_name);
                return Some(Fasta {
                    name,
                    sequence: seq,
                });
            } else {
                seq += &line;
            }
        }
        if seq.is_empty() {
            None
        } else {
            Some(Fasta {
                name: mem::take(&mut self.curr_name),
                sequence: seq,
            })
        }
    }
}

impl FastaIterator {
    pub fn new(bufreader: BufReader<File>) -> Self {
        Self {
            lines_reader: bufreader.lines(),
            curr_name: String::new(),
        }
    }
}

// TODO: Try and make this function more generic. You can do this in two ways:
// 1. Have this output an owned string which incurs some overhead but not much
// 2. Have this function take in a closure as an input. Read https://doc.rust-lang.org/book/ch13-01-closures.html
pub fn parse_fasta(name: &str, output: &mut Vec<PalindromeData>) {
    let file = match File::open(name) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
    let reader = BufReader::new(file);
    
    let mut seq = String::new();
    let mut palins = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.starts_with(">") {
                    if !seq.is_empty() {
                        run_search(&seq, &mut palins, output)
                    }
                    seq.clear();
                } else {
                    seq += &line;
                }
            }
            Err(error) => panic!("{error:?}"),
        }
    }
    run_search(&seq, &mut palins, output)
}

fn run_search(seq: &str, palins: &mut Vec<PalindromeData>, output: &mut Vec<PalindromeData>) {
    smith_waterman(&seq, palins);
    output.append(palins);
    palins.clear();
}
