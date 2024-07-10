use crate::exact_matches::{self, match_exact};
use exact_matches::PalindromeData;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn parse_fasta(name: &str, output: &mut Vec<PalindromeData>) {
    let file = match File::open(name) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(sequence) => {
                let sequence = sequence;
                let mut palins = match_exact(&sequence);
                output.append(&mut palins);
            }
            Err(error) => panic!("Problem opening the file: {error:?}"),
        }
    }
}
