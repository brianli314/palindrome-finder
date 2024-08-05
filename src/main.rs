pub mod command_line;
pub mod exact_matches;
pub mod fasta_parsing;
pub mod output;
pub mod run_algorithm;
pub mod wfa;

use anyhow::{Ok, Result};
use clap::Parser;
use command_line::{AlgorithmType::ExactMatch, AlgorithmType::Wfa, PalinArgs};
use fasta_parsing::parse_fasta;
use output::write_file;
use run_algorithm::{run_exact_match, run_wfa, ALGO_TIMER};
use std::{sync::atomic::Ordering, time::Instant};

fn main() -> Result<()> {
    let global_timer = Instant::now();

    let args = PalinArgs::parse();
    let iterator = parse_fasta(&args)?;
    let mut palins = Vec::new();


    match &args.command {
        Wfa(cmds) => run_wfa(&args, cmds, iterator, &mut palins)?,
        ExactMatch => run_exact_match(&args, iterator, &mut palins)?,
    }

    write_file(palins, &args.output_file)?;

    let elapsed = global_timer.elapsed();
    println!("Total elapsed time: {}", elapsed.as_millis());
    println!("Time spent on algorithm: {}", ALGO_TIMER.load(Ordering::Relaxed));
    Ok(())
}
