pub mod exact_matches;
mod fasta_parsing;
pub mod matrix;
pub mod myers;
pub mod output;
pub mod smith_waterman;
use fasta_parsing::parse_fasta;
use output::write_file;
use output::PalindromeData;

fn main() {
    let palins: Vec<PalindromeData> = parse_fasta("dna1.fasta");
    write_file(palins, "palindrome_data.txt");
}
