use std::io::Read;

use anyhow::{ensure, Result};

use crate::{
    command_line::{PalinArgs, WfaArgs},
    exact_matches::match_exact,
    fasta_parsing::FastaIterator,
    output::PalindromeData,
    wfa::wfa_palins,
};

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
        0.0 < wfa_args.mismatch_len_ratio && wfa_args.mismatch_len_ratio < 1.0,
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

pub fn run_exact_match<T: Read>(
    args: &PalinArgs,
    iterator: FastaIterator<T>,
    output: &mut Vec<PalindromeData>,
) -> Result<()> {
    let mut palins = Vec::new();
    for line in iterator {
        match_exact(line?, output, args)?;
        output.append(&mut palins);
        palins.clear();
    }

    Ok(())
}
