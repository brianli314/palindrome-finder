use std::{io::Read, sync::atomic::{AtomicU64, Ordering}, time::Instant};

use anyhow::{ensure, Result};

use crate::{
    command_line::{FixedArgs, PalinArgs, WfaArgs},
    exact_matches::fixed_match,
    fasta_parsing::FastaIterator,
    output::PalindromeData,
    wfa::wfa_palins,
};

pub static ALGO_TIMER: AtomicU64 = AtomicU64::new(0);

pub fn run_wfa<T: Read>(
    args: &PalinArgs,
    wfa_args: &WfaArgs,
    iterator: FastaIterator<T>,
    output: &mut Vec<PalindromeData>,
) -> Result<()> {
    ensure!(wfa_args.match_bonus > 0.0, "Match bonus not positive");
    ensure!(
        wfa_args.mismatch_penalty > 0.0,
        "Mismatch penalty not positive"
    );
    ensure!(
        0.0 < wfa_args.mismatch_ratio_threshold && wfa_args.mismatch_ratio_threshold < 1.0,
        "Mismatch-length ratio not between 0 and 1"
    );
    ensure!(wfa_args.x_drop > 0.0, "X-drop not positive");

    
    let mut palins = Vec::new();
    for line in iterator {
        let start = Instant::now();
        wfa_palins(line?, output, args, wfa_args)?;
        let duration = start.elapsed();
        ALGO_TIMER.fetch_add(duration.as_millis() as u64, Ordering::Relaxed);
        output.append(&mut palins);
        palins.clear();
    }

    Ok(())
}

pub fn run_fixed_match<T: Read>(
    args: &PalinArgs,
    cmds: &FixedArgs,
    iterator: FastaIterator<T>,
    output: &mut Vec<PalindromeData>,
) -> Result<()> {
    let mut palins = Vec::new();
    for line in iterator {
        let start = Instant::now();
        fixed_match(line?, output, args, cmds)?;
        let duration = start.elapsed();
        ALGO_TIMER.fetch_add(duration.as_millis() as u64, Ordering::Relaxed);
        output.append(&mut palins);
        palins.clear();
    }

    Ok(())
}
