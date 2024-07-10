

use crate::util::*;





pub fn match_exact(seq: &str, output: &mut Vec<PalindromeData>){
    for i in 0..seq.len() as u32 {
        let mut j = 1;
        while i >= j && j <= GAP_LENGTH {
            let length = count_palindrome(i, i + j, &seq);
            if length >= PALINDROME_LENGTH {
                let palin = PalindromeData::new(
                    i + 1 - length,
                    i + length + j - 1,
                    length,
                    seq[(i + 1 - length) as usize..(i + length + j) as usize].to_owned()); // clarify what i + 1 - length and i + length + j mean
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