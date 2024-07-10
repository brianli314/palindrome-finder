use crate::exact_matches::{self, match_exact};
use crate::{smith_waterman, util};
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
                        match_exact(&seq, &mut palins);
                        output.append(&mut palins);
                        palins.clear();
                    }
                    seq.clear();
                } else {
                    seq += &sequence;
                }
            }
            Err(error) => panic!("Problem opening the file: {error:?}")
        }
    }
    match_exact(&seq, &mut palins);
    output.append(&mut palins);
    palins.clear();
}
    

