use core::fmt;

use crate::fasta_parsing::Fasta;

#[derive(Debug)]
pub struct PalindromeData {
    start: u32,
    end: u32,
    length: u32,
    gap: u32,
    mismatches: u32,
    fasta: Fasta,
    sequence: String,
}
impl PalindromeData {
    pub fn new(
        start: u32,
        end: u32,
        length: u32,
        gap: u32,
        mismatches: u32,
        fasta: Fasta,
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
            self.start, self.end, self.length, self.gap, self.mismatches, self.fasta.get_name(), self.sequence
        )
    }
}