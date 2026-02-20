use core::fmt;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

use anyhow::Result;

use crate::adapters::Adapter;

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
        // Take the first whitespace-separated token from the header,
        // or fallback to the entire header if there is no whitespace.
        let name = self
            .fasta
            .split_whitespace()
            .next()
            .unwrap_or(&self.fasta);

        write!(
            f,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.start,
            self.end,
            self.arm_length,
            self.gap,
            self.overall_length,
            self.mismatches,
            name,
            self.sequence,
        )
    }
}


pub fn write_palins(palins: &mut Vec<PalindromeData>, file_name: &str) -> Result<()> {
    let output = File::create(file_name)?;
    let mut writer = BufWriter::with_capacity(BUFF_SIZE, output);

    let _ = writeln!(
        writer,
        "Start\tEnd\tArm-Length\tGap\tLength\tMismatches\tSeq-name\tSequence\n"
    );
    for palin in palins {
        let _ = writeln!(writer, "{}", palin);
    }
    writer.flush()?;

    Ok(())
}

pub fn write_adapters(adapters: &mut Vec<Adapter>, file_name: &str) -> Result<()> {
    let output = File::create(file_name)?;
    let mut writer = BufWriter::with_capacity(BUFF_SIZE, output);
    let _ = writeln!(
        writer,
        "Ref name\tQuery name\tCigar\tScore\tQuery index\tRef index\n"
    );
    for adapter in adapters{
        let result = adapter.get_result();
        let _ = writeln!(writer, "{}\t{}\t{}\t{}\t{}\t{}", 
        adapter.get_name(), 
        adapter.get_ref(),  
        adapter.get_seq(),
        result.score,
        result.query_idx,
        result.reference_idx,
    );
    }
    writer.flush()?;
    Ok(())
}