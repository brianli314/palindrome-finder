use std::io::Read;
use anyhow::{ensure, Result};

use crate::{
    command_line::{
        AlgorithmType::FixedMismatch, AlgorithmType::Wfa, FixedArgs, PalinArgs, WfaArgs,
    },
    exact_matches::fixed_match,
    fasta_parsing::FastaIterator,
    output::PalindromeData,
    wfa::wfa_palins,
};

pub fn run<T: Read>(args: &PalinArgs, iterator: FastaIterator<T>) -> Result<Vec<PalindromeData>> {
    let mut palins = Vec::new();
    match &args.command {
        Wfa(cmds) => run_wfa(args, cmds, iterator, &mut palins)?,
        FixedMismatch(cmds) => run_fixed_match(args, cmds, iterator, &mut palins)?,
    }
    Ok(palins)
}

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
        0.0 < wfa_args.mismatch_proportion && wfa_args.mismatch_proportion < 1.0,
        "Mismatch-length ratio not between 0 and 1"
    );
    ensure!(wfa_args.x_drop > 0.0, "X-drop not positive");

    let mut palins = Vec::new();
    for line in iterator {
        wfa_palins(line?, output, args, wfa_args)?;
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
        fixed_match(line?, output, args, cmds)?;
        output.append(&mut palins);
        palins.clear();
    }

    Ok(())
}
