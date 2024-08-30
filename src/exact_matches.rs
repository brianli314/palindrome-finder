use anyhow::{bail, ensure, Ok, Result};

use crate::{
    command_line::{FixedArgs, PalinArgs},
    fasta_parsing::Fasta,
    output::PalindromeData,
};

pub static PALINDROME_LENGTH: u32 = 5;
pub static GAP_LENGTH: u32 = 2;

pub fn fixed_match(
    fasta: Fasta,
    output: &mut Vec<PalindromeData>,
    args: &PalinArgs,
    cmds: &FixedArgs,
) -> Result<()> {
    let seq = fasta.get_sequence();
    let mut i = 0;
    while i < seq.len() as u32 {

        let mut j = 1;
        let mut increment = 1;

        while i >= j && j <= (args.gap_len + 1) as u32 {
            
            let length = count_palindrome(i, i + j, seq, cmds.mismatches)?;
            if length >= args.len as u32 {
                let palin = PalindromeData::new(
                    i + 1 - length,
                    i + length + j - 1,
                    length,
                    j - 1,
                    2 * length + j - 1,
                    cmds.mismatches,
                    fasta.get_name().to_owned(),
                    seq[(i + 1 - length) as usize..(i + length + j) as usize].to_owned(),
                );
                output.push(palin);
                increment = length + j;
                break;
            }
            j += 1;
        }
        i += increment
    }

    Ok(())
}

fn count_palindrome(start: u32, end: u32, seq: &str, mismatch: u32) -> Result<u32> {
    let mut prev: i32 = start as i32;
    let mut next = end as usize;
    let mut count = 0;
    let mut mismatches = 0;

    while prev >= 0 && next < seq.len() {
        if !is_complement(&seq[prev as usize..=prev as usize], &seq[next..=next])? {
            mismatches += 1;
        }
        if mismatches > mismatch {
            break;
        }

        count += 1;
        prev -= 1;
        next += 1;
    }

    Ok(count)
}

pub fn get_complement(bp: &str) -> Result<&str> {
    ensure!(
        bp.len() == 1,
        "Cannot get complement of non-base pair string"
    );

    let bpu = bp.to_uppercase();

    let value = match &bpu[0..=0] {
        "A" => "T",
        "T" => "A",
        "C" => "G",
        "G" => "C",
        _ => bail!("Invalid fasta format"),
    };
    Ok(value)
}
pub fn is_complement(base1: &str, base2: &str) -> Result<bool> {
    let is_equal = get_complement(base1)? == base2.to_uppercase();
    Ok(is_equal)
}
