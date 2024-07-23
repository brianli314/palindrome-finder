pub mod exact_matches;
mod fasta_parsing;
pub mod matrix;
pub mod output;
pub mod smith_waterman;
pub mod myers;
use fasta_parsing::parse_fasta;
use myers::count_matching;
use myers::wfa;
use output::write_file;
use output::PalindromeData;

fn main() {
    //let palins:Vec<PalindromeData> = parse_fasta("dna1.fasta");
    //dbg!(&palins);
    //write_file(palins, "palindrome_data.txt");
    let count = count_matching("ATCGAAAA".as_bytes(), "ATCGGGGG".as_bytes());
    dbg!(count,"ATCGAAAA".as_bytes(), "ATCGGGGG".as_bytes());
}