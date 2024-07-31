use anyhow::{bail, Ok, Result};

use crate::{command_line::PalinArgs, fasta_parsing::Fasta, output::PalindromeData};

pub static PALINDROME_LENGTH: u32 = 5;
pub static GAP_LENGTH: u32 = 2;

pub fn match_exact(fasta: Fasta, output: &mut Vec<PalindromeData>, args: &PalinArgs) -> Result<()>{
    let seq = fasta.get_sequence();
    for i in 0..seq.len() as u32 {
        let mut j = 1;
        while i >= j && j <= args.gap_len as u32 {
            let length = count_palindrome(i, i + j, seq)?;
            if length >= args.min_len as u32 {
                let palin = PalindromeData::new(
                    i + 1 - length,
                    i + length + j - 1,
                    length,
                    j - 1,
                    0,
                    fasta.get_name().to_owned(),
                    seq[(i + 1 - length) as usize..(i + length + j) as usize].to_owned(),
                );
                output.push(palin);
            }
            j += 1;
        }
    }

    Ok(())
}

fn count_palindrome(start: u32, end: u32, seq: &str) -> Result<u32> {
    let mut prev: i32 = start as i32;
    let mut next = end as usize;
    let mut count = 0;
    let mut mismatches = 0;

    while prev >= 0 && next < seq.len() {
        if !(is_complement(&seq[prev as usize..=prev as usize], &seq[next..=next])?) {
            mismatches += 1;
        }

        if mismatches > 0 {
            break;
        }

        count += 1;
        prev -= 1;
        next += 1;
    }

    Ok(count)
}

pub fn seq_complement(seq: &str) -> String {
    let mut output = String::new();
    for i in 0..seq.len() {
        output += get_complement(&seq[i..=i]).unwrap();
    }
    output
}
pub fn get_complement(bp: &str) -> Result<&str> {
    let bpu = bp.to_uppercase();

    let value = match &bpu[0..=0] {
        "A" => "T",
        "T" => "A",
        "C" => "G",
        "G" => "C",
        _ => bail!("Not fasta format"),
    };
    Ok(value)
}
pub fn is_complement(base1: &str, base2: &str) -> Result<bool> {
    let is_equal = get_complement(base1)? == base2;
    Ok(is_equal)
}
