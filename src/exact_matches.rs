use core::panic;
use std::fmt;

static PALINDROME_LENGTH: u32 = 5;
static GAP_LENGTH: u32 = 6;
static NUM_MISMATCH: u32 = 0;

#[derive(Debug)]
pub struct PalindromeData {
    start: u32,
    end: u32,
    length: u32,
    gap_length: u32,
    sequence: String,
}

impl fmt::Display for PalindromeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Start: {}, End: {}, Length: {}, Gap Length: {}, Sequence: {}", 
        self.start, self.end, self.length, self.gap_length, self.sequence)
    }
}


pub fn match_exact(seq: &str) -> Vec<PalindromeData> {
    let mut output = Vec::new();
    for i in 0..seq.len() as u32 {
        let mut j = 1;
        while i >= j && j <= GAP_LENGTH {
            let length = count_palindrome(i, i + j, &seq);
            if length >= PALINDROME_LENGTH {
                let palin = PalindromeData {
                    start: i + 1 - length,
                    end: i + length + j - 1,
                    length,
                    gap_length: j - 1,
                    sequence: seq[(i + 1 - length) as usize..(i + length + j) as usize].to_owned(), // clarify what i + 1 - length and i + length + j mean
                };
                output.push(palin);
            }
            j += 1;
        }
    }
    return output;
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

fn smith_waterman(seq: String) {
    let length = seq.len() as u32;
    let complement = seq_compliment(seq);
    // let mut matrix:[[i32; length]; length];
}

fn seq_compliment(seq: String) -> String {
    let mut output = String::new();
    for i in 0..seq.len() {
        output += get_complement(&seq[i..=i]);
    }
    return output;
}
fn get_complement(bp: &str) -> &str {
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
fn is_complement(base1: &str, base2: &str) -> bool {
    return get_complement(base1) == base2;
}
