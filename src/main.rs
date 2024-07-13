pub mod exact_matches;
mod fasta_parsing;
pub mod matrix;
pub mod util;
pub mod smith_waterman;
pub mod myers;
use fasta_parsing::parse_fasta;
use util::PalindromeData;

fn main() {
    let mut palins:Vec<PalindromeData> = Vec::new();
    parse_fasta("sequence.txt", &mut palins);
    for p in &palins {
        println!("{}", p)
    }
    palins.clear();
}