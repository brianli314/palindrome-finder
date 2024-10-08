pub mod command_line;
pub mod exact_matches;
pub mod fasta_parsing;
pub mod output;
pub mod run_algorithm;
pub mod wfa;

use anyhow::{Ok, Result};
use clap::Parser;
use command_line::PalinArgs;
use fasta_parsing::parse_fasta;
use output::write_file;
use run_algorithm::run;
use std::time::Instant;

fn main() -> Result<()> {
    let global_timer = Instant::now();
    
    let args = PalinArgs::parse();
    let iterator = parse_fasta(&args)?;
    let palins = run(&args, iterator)?;

    write_file(palins, &args.output_file)?;

    let elapsed = global_timer.elapsed();
    println!("Total elapsed time: {:.2?}", elapsed);
    println!();
    println!("---Settings---\n{}", args);
    
    Ok(())
}
