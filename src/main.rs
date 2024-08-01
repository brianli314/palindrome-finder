pub mod exact_matches;
mod fasta_parsing;
pub mod matrix;
pub mod wfa;
pub mod output;
mod command_line;
use std::time::Instant;

use anyhow::Ok;
use anyhow::Result;
use clap::Parser;
use command_line::PalinArgs;
use fasta_parsing::parse_fasta;
use output::write_file;


fn main() -> Result<()>{
    let now = Instant::now();

    let args = PalinArgs::parse();

    let palins  = parse_fasta(&args)?;

    write_file(palins, &args.output_file)?;

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
