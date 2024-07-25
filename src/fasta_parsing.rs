use crate::myers::wfa_palins;
use crate::output::PalindromeData;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::mem;

#[derive(Debug, Clone)]
pub struct Fasta {
    name: String,
    sequence: String,
}
impl Fasta {
    pub fn new(name: String, sequence: String) -> Self {
        Self { name, sequence }
    }
    pub fn get_sequence(&self) -> String {
        self.sequence.clone()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
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
        for line in self.lines_reader.by_ref() {
            let line = line.expect("Failed to read from fasta!");
            if line.starts_with('>') {
                let mut name = line.strip_prefix('>').unwrap().to_owned();
                if seq.is_empty() {
                    name.clone_into(&mut self.curr_name);
                    continue;
                }
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
pub fn parse_fasta(name: &str) -> Vec<PalindromeData> {
    let file = match File::open(name) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
    let reader = BufReader::new(file);
    let mut output: Vec<PalindromeData> = Vec::new();
    let mut palins = Vec::new();
    let iterator = FastaIterator::new(reader);
    for line in iterator {
        run_search(line, &mut palins, &mut output)
    }
    output
}

fn run_search(fasta: Fasta, palins: &mut Vec<PalindromeData>, output: &mut Vec<PalindromeData>) {
    wfa_palins(fasta, palins);
    output.append(palins);
    palins.clear();
}
