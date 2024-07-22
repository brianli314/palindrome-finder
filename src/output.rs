use core::fmt;
use std::{fs::{self, File}, io::Error, io::Write};

use crate::fasta_parsing::Fasta;

#[derive(Debug)]
pub struct PalindromeData {
    start: u32,
    end: u32,
    length: u32,
    gap: u32,
    mismatches: u32,
    fasta: String,
    sequence: String,
}
impl PalindromeData {
    pub fn new(
        start: u32,
        end: u32,
        length: u32,
        gap: u32,
        mismatches: u32,
        fasta: String,
        sequence: String,
    ) -> Self {
        Self {
            start,
            end,
            length,
            gap,
            mismatches,
            fasta,
            sequence,
        }
    }

}
impl fmt::Display for PalindromeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Start: {}, End: {}, Length: {}, Gap Length: {}, Mismatches: {}, Name: {}, Sequence: {}",
            self.start, self.end, self.length, self.gap, self.mismatches, self.fasta, self.sequence, 
        )
    }
}

pub fn write_file(palins: Vec<PalindromeData>, file_name: &str){
    let mut output = match File::create(file_name){
        Ok(file) => file,
        Err(error) => panic!("Problem opening file {error}"),
    };
    for palin in palins{
        let _ = output.write(&(palin.to_string() + "\n").as_bytes());
    }
}