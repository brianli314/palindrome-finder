pub mod exact_matches;
mod fasta_parsing;
pub mod matrix;
pub mod output;
pub mod smith_waterman;
pub mod myers;
use fasta_parsing::parse_fasta;
use myers::wfa;
use output::PalindromeData;

fn main() {
    let mut palins:Vec<PalindromeData> = parse_fasta("dna1.fasta");
    for p in &palins {
        println!("{}", p)
    }
    palins.clear();

    //wfa("AEFAEFAEFAEFAEFAEFEEFAEFAEFAEF")
}