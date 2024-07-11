use crate::exact_matches::match_exact;
use crate::smith_waterman::smith_waterman;
use crate::util;
use util::PalindromeData;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn parse_fasta(name: &str, output: &mut Vec<PalindromeData>) {
    let file = match File::open(name) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
    let reader = BufReader::new(file);

    let mut seq = String::new();
    let mut palins = Vec::new();
    
    for line in reader.lines(){
        match line{
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
            Err(error) => panic!("Problem opening the file: {error:?}")
        }
    }
    run_search(&seq, &mut palins, output)
}

fn run_search(seq: &str, palins: &mut Vec<PalindromeData>, output: &mut Vec<PalindromeData>){
    smith_waterman(&seq, palins);
    output.append(palins);
    palins.clear();
}
    

