use crate::{fasta_parsing::Fasta, output::PalindromeData};

pub static PALINDROME_LENGTH: u32 = 5;
pub static GAP_LENGTH: u32 = 6;
pub static NUM_MISMATCH: u32 = 0;

pub fn match_exact(fasta: Fasta, output: &mut Vec<PalindromeData>){
    let seq = fasta.get_sequence();
    for i in 0..seq.len() as u32 {
        let mut j = 1;
        while i >= j && j <= GAP_LENGTH {
            let length = count_palindrome(i, i + j, &seq);
            if length >= PALINDROME_LENGTH {
                let palin = PalindromeData::new(
                    i + 1 - length,
                    i + length + j - 1,
                    length,
                    j-1,
                    0,
                    fasta.get_name(),
                    seq[(i + 1 - length) as usize..(i + length + j) as usize].to_owned());
                output.push(palin);
            }
            j += 1;
        }
    }
}

fn count_palindrome(start: u32, end: u32, seq: &str) -> u32 {
    let mut prev: i32 = start as i32;
    let mut next = end as usize;
    let mut count = 0;
    let mut mismatches = 0;

    while prev >= 0 && next < seq.len() {
        if !(is_complement(&seq[prev as usize..=prev as usize], &seq[next..=next])) {
            mismatches += 1;
        }

        if mismatches > NUM_MISMATCH {
            break;
        }

        count += 1;
        prev -= 1;
        next += 1;
    }

    return count;
}

pub fn seq_complement(seq: &str) -> String {
    let mut output = String::new();
    for i in 0..seq.len() {
        output += get_complement(&seq[i..=i]);
    }
    return output;
}
pub fn get_complement(bp: &str) -> &str {
    let bpu = bp.to_uppercase();
    let letter = match &bpu[0..=0] {
        "A" => "T",
        "T" => "A",
        "C" => "G",
        "G" => "C",
        _ => panic!("Not a base pair"),
    };
    return letter;
}
pub fn is_complement(base1: &str, base2: &str) -> bool {
    return get_complement(base1) == base2;
}