use crate::exact_matches::match_exact;
use crate::smith_waterman::smith_waterman;
use crate::util;
use util::PalindromeData;
use std::fs::File;
use std::io::{prelude::*, BufReader};

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

    for line in reader.lines(){
        match line {
            // rename sequence to "line" make it less confusing.
            Ok(sequence) => {
                if sequence.starts_with(">"){
                    if !seq.is_empty(){
                        run_search(&seq, &mut palins, output)
                    }
                    seq.clear();
                } else {
                    seq += &sequence;
                }
            }
            Err(error) => panic!("Problem opening the file: {error:?}") // this is the wrong error message. Read the docs for reader.lines()
        }
    }
    run_search(&seq, &mut palins, output)
}

fn run_search(seq: &str, palins: &mut Vec<PalindromeData>, output: &mut Vec<PalindromeData>){
    smith_waterman(&seq, palins);
    output.append(palins);
    palins.clear();
}
    

