use core::fmt;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

use anyhow::Result;

pub const BUFF_SIZE: usize = 1 << 20;

#[derive(Debug)]
pub struct PalindromeData {
    start: u32,
    end: u32,
    arm_length: u32,
    gap: u32,
    overall_length: u32,
    mismatches: u32,
    fasta: String,
    sequence: String,
}
impl PalindromeData {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        start: u32,
        end: u32,
        arm_length: u32,
        gap: u32,
        overall_length: u32,
        mismatches: u32,
        fasta: String,
        sequence: String,
    ) -> Self {
        Self {
            start,
            end,
            arm_length,
            gap,
            overall_length,
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
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.start,
            self.end,
            self.arm_length,
            self.gap,
            self.overall_length,
            self.mismatches,
            self.fasta.split_once(' ').unwrap().0,
            self.sequence,
        )
    }
}

pub fn write_file(palins: Vec<PalindromeData>, file_name: &str) -> Result<()> {
    let output = File::create(file_name)?;
    let mut writer = BufWriter::with_capacity(BUFF_SIZE, output);

    let _ = writeln!(
        writer,
        "Start\tEnd\tApprox Arm-Length\tApprox Gap\tLength\tMismatches\tSeq-name\tSequence\n"
    );
    for palin in palins {
        let _ = writeln!(writer, "{}", palin);
    }
    writer.flush()?;

    Ok(())
}
