use core::fmt;
use std::{fs::File, io::{BufWriter, Write}};

use anyhow::Result;

pub const BUFF_SIZE: usize = 1<<20;

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
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.start, self.end, self.length, self.gap, self.mismatches, self.fasta, self.sequence,
        )
    }
}

pub fn write_file(palins: Vec<PalindromeData>, file_name: &str) -> Result<()> {
    let output = File::create(file_name)?;
    let mut writer = BufWriter::with_capacity(BUFF_SIZE, output);

    let _ = writeln!(
        writer,
        "Start\tEnd\tLength\tGap Length\tMismatches\tSeq name\tSequence\n"
    );
    for palin in palins {
        let _ = writeln!(writer, "{}", palin);
    }
    writer.flush()?;

    Ok(())
}
