pub mod exact_matches;
mod fasta_parsing;
pub mod matrix;

use exact_matches::PalindromeData;
use fasta_parsing::parse_fasta;

fn main() {
    let mut palins:Vec<PalindromeData> = Vec::new();
    parse_fasta("sequence.txt", &mut palins);
    for p in &palins {
        println!("{}", p)
    }
    palins.clear()
}