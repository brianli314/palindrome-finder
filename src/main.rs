pub mod exact_matches;
mod fasta_parsing;
pub mod matrix;
pub mod wfa;
pub mod output;
pub mod smith_waterman;
mod command_line;
use std::time::Instant;
use clap::Parser;
use command_line::PalinArgs;
use fasta_parsing::parse_fasta;
use output::write_file;
use output::PalindromeData;

fn main() {
    let now = Instant::now();
    let args = PalinArgs::parse();
    let palins: Vec<PalindromeData> = parse_fasta(&args);
    write_file(palins, &args.output_file);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
