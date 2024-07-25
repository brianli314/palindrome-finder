use std::{
    cmp::{max, min},
    mem,
};

use crate::{fasta_parsing::Fasta, output::PalindromeData};

static PALINDROME_LENGTH: usize = 5;
pub static MISMATCH_LENGTH_RATIO: f32 = 0.9;
static GAP_SIZE: usize = 2;
static MATCH: f32 = 1.0;
static MIS: f32 = -1.5;

static X: f32 = 5.0;

pub fn wfa_palins(fasta: Fasta, output: &mut Vec<PalindromeData>) {
    //Getting the sequence as bits
    let mut seq_clone = fasta.sequence.clone();
    let seq = fasta.sequence;
    let bytes_seq = unsafe { seq_clone.as_bytes_mut() };
    sequence_to_bytes(bytes_seq);

    //Variables
    let len = seq.len();
    let mut index = 0;

    //Wavefront initialization
    let mut wf = vec![0; len];
    let mut wf_next = vec![0; len];
    let first = vec![0; GAP_SIZE + 2];

    while index <= len {
        //Variables
        let mut edit_dist = 0;
        let mut wf_len = GAP_SIZE + 1;
        let mut gap = 0;

        //Max value trackers
        let mut max_index = 0;
        let mut max_score = 0.0;

        //Filling the starting alues of waverfront
        wf[..=wf_len].copy_from_slice(&first);

        'outer: while (edit_dist as f32) / (wf[max_index] as f32 + 0.001) <= MISMATCH_LENGTH_RATIO {
            for i in 0..wf_len {
                //Extend wave along matching diagonal
                let (mut x, y) = get_xy(wf_len, i, wf[i]);
                x += index;

                wf[i] += extend_wave(x, y, index, bytes_seq) as usize;

                //Updating max value
                if wf[i] > wf[max_index] {
                    max_index = i;
                }

                //Break if reached the endpoint
                if x >= len || y >= index {
                    break 'outer;
                }
            }

            //Updating the max score
            let (x, y) = get_xy(wf_len, max_index, wf[max_index]);
            let curr_score = calculate_score(x + 1, y + 1, edit_dist);
            max_score = f32::max(max_score, curr_score);

            //X-drop
            if curr_score < max_score - X {
                break;
            }

            //Getting the gap
            if wf_len == GAP_SIZE + 1 {
                gap = max_index;
            }

            //Compute next wave
            next_wave(&mut wf, &mut wf_next, wf_len);
            max_index += 1;
            edit_dist += 1;
            wf_len += 2;
        }

        //If no matches, continue w/next index
        if wf[max_index] == 0 {
            index += 1;
            continue;
        }
        //x,y are coordinates of the longest wavepoint, corresponds to the sequence
        let (x, y) = get_xy(wf_len, max_index, wf[max_index]);
        if x + y >= PALINDROME_LENGTH {
            let palin = PalindromeData::new(
                (index - y) as u32,
                (index + x - 1) as u32,
                (x + y) as u32,
                gap as u32,
                edit_dist,
                fasta.name.to_owned(),
                seq[index - y..index + x].to_owned(),
            );
            output.push(palin);
        }
        index += wf[max_index];
    }
}

//Evaluates score using formula from X-drop paper
fn calculate_score(x: usize, y: usize, d: u32) -> f32 {
    (x + y) as f32 * (MATCH / 2.0) - (d as f32) * (MATCH - MIS)
}

//Converts the sequence to bytes, where A = !T, C = !G
fn sequence_to_bytes(seq: &mut [u8]) {
    for i in seq.iter_mut() {
        *i = match i {
            65 | 97 => 2,    // A
            84 | 116 => 253, // T
            67 | 99 => 3,    // C
            71 | 103 => 252, // G
            _ => panic!("Not a base pair"),
        };
    }
}

//Counts the matching sequences with bit manipulation
fn count_matching(seq1: &[u8], seq2: &[u8]) -> u32 {
    assert!(seq1.len() <= 8);
    assert!(seq2.len() <= 8);
    let mut buf1 = [0; 8];
    let mut buf2 = [0; 8];
    buf1[..seq1.len()].copy_from_slice(seq1);
    buf2[8 - seq2.len()..].copy_from_slice(seq2);
    let num1 = u64::from_le_bytes(buf1);
    let num2 = !u64::from_be_bytes(buf2);
    let diff = num1 ^ num2;
    diff.trailing_zeros() / 8
}

//Extends wave along matching nucleotides
fn extend_wave(mut x: usize, mut y: usize, index: usize, seq: &[u8]) -> u32 {
    let len = seq.len();
    let mut count = 8;
    let mut counter = 0;

    while x < len && y < index && count >= 8 {
        let len1 = min(len - x, 8);
        let len2 = min(index - y, 8);
        count = count_matching(&seq[x..x + len1], &seq[index - y - len2..index - y]);
        x += count as usize;
        y += count as usize;
        counter += count;
    }
    counter
}

//Computes and updates the next wave.
fn next_wave(wf: &mut Vec<usize>, wf_next: &mut Vec<usize>, wf_len: usize) {
    for i in 0..wf_len {
        if i == 0 {
            wf_next[i] = wf[i];
            wf_next[i + 1] = max(wf[i] + 1, wf[i + 1]);
        } else if i != wf_len - 1 {
            wf_next[i + 1] = max(wf[i] + 1, max(wf[i - 1], wf[i + 1]));
        } else {
            wf_next[i + 2] = wf[i];
            wf_next[i + 1] = max(wf[i - 1], wf[i] + 1);
        }
    }
    mem::swap(wf, wf_next);
}

//Gets the xy coordinates of a wavefront
fn get_xy(wf_len: usize, index: usize, length: usize) -> (usize, usize) {
    let offset = ((wf_len - (GAP_SIZE + 1)) / 2) as i32 - (index as i32);
    if offset >= 0 {
        (length, length + offset as usize)
    } else {
        (length + offset.unsigned_abs() as usize, length)
    }
}

//Old wfa for computing edit distance as practice.
pub fn wfa(seq: &str, seq2: &str) -> u32 {
    let len = 2 * max(seq.len(), seq2.len()) + 3;
    let mut edit_dist = 0;
    let mut wavefront = vec![0; len];
    let mut new_wavefront = vec![0; len];
    let mut wf_len = 1;
    loop {
        #[allow(clippy::needless_range_loop)]
        for i in 0..wf_len {
            let (mut x, mut y) = get_xy(wf_len, i, wavefront[i]);
            while x < seq.len() && y < seq2.len() && seq[x..=x] == seq2[y..=y] {
                wavefront[i] += 1;
                x += 1;
                y += 1;
            }
            if x == seq.len() && y == seq2.len() {
                return edit_dist;
            }
        }
        next_wave(&mut wavefront, &mut new_wavefront, wf_len);
        edit_dist += 1;
        wf_len += 2;
    }
}
