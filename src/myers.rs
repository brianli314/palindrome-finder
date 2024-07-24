use std::{
    cmp::{max, min},
    mem,
};

use crate::{exact_matches::seq_complement, fasta_parsing::Fasta, output::PalindromeData};

static PALINDROME_LENGTH: usize = 5;
pub static MISMATCH_LENGTH_RATIO: f32 = 0.2;
static GAP_SIZE: usize = 2;

pub fn wfa_palins(fasta: Fasta, output: &mut Vec<PalindromeData>) {
    let seq = fasta.get_sequence();
    let len = seq.len();
    let mut wf = vec![0; len];
    let mut wf_next = vec![0; len];
    let mut index = 0;

    while index <= len {
        let mut edit_dist = 0;
        let mut wf_len = GAP_SIZE + 1;
        for i in 0..=wf_len {
            wf[i] = 0;
        }
        let mut gap = 0;
        let mut max_index = 0;
        'outer: while (edit_dist as f32) / (wf[max_index] as f32 + 0.001) <= MISMATCH_LENGTH_RATIO {
            for i in 0..wf_len {
                let (mut x, mut y) = get_xy(wf_len, i, wf[i]);
                x += index;

                let mut count = 8;
                let mut counter = 0;

                while len > x && y < index && count >= 8 {
                    let len1 = min(len - x, 8);
                    let len2 = min(index - y, 8);
                    count = count_matching(
                        seq[x..x + len1].as_bytes(),
                        seq_complement(
                            &seq[index - y - len2..index - y]
                                .chars()
                                .rev()
                                .collect::<String>(),
                        )
                        .as_bytes(),
                    );
                    x += count as usize;
                    y += count as usize;
                    counter += count;
                }
                wf[i] += counter as usize;

                /*
                while x < len
                    && y < index
                    && &seq[x..=x] == get_complement(&seq[index - y - 1..=index - y - 1])
                {
                    wf[i] += 1;
                    x += 1;
                    y += 1;
                }
                */
                if wf[i] > wf[max_index] {
                    max_index = i;
                }
                if x >= len || y >= index {
                    break 'outer;
                }
            }
            
            if wf_len == GAP_SIZE + 1{
                gap = max_index;
            }
            next_wave(&mut wf, &mut wf_next, wf_len);
            max_index += 1;
            edit_dist += 1;
            wf_len += 2;
        }
        if wf[max_index] == 0 {
            index += 1;
            continue;
        }
        let (x, y) = get_xy(wf_len, max_index, wf[max_index]);
        if x + y >= PALINDROME_LENGTH {
            let palin = PalindromeData::new(
                (index - y) as u32,
                (index + x - 1) as u32,
                (x + y) as u32,
                gap as u32,
                edit_dist,
                fasta.get_name(),
                seq[index - y..index + x].to_string(),
            );
            output.push(palin);
            dbg!(index);
        }
        index += wf[max_index];
    }
}


fn count_matching(seq1: &[u8], seq2: &[u8]) -> u32 {
    assert!(seq1.len() <= 8);
    assert!(seq2.len() <= 8);
    let mut buf1 = [255; 8];
    let mut buf2 = [0; 8];
    buf1[..seq1.len()].copy_from_slice(seq1);
    buf2[..seq2.len()].copy_from_slice(seq2);
    let num1 = u64::from_le_bytes(buf1);
    let num2 = u64::from_le_bytes(buf2);
    let diff = num1 ^ num2;
    diff.trailing_zeros() / 8
}

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

pub fn wfa(seq: &str, seq2: &str) -> u32 {
    let len = 2 * max(seq.len(), seq2.len()) + 3;
    let mut edit_dist = 0;
    let mut wavefront = vec![0; len];
    let mut new_wavefront = vec![0; len];
    let mut wf_len = 1;
    loop {
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

fn get_xy(wf_len: usize, index: usize, length: usize) -> (usize, usize) {
    let offset = ((wf_len - (GAP_SIZE + 1)) / 2) as i32 - (index as i32);
    if offset >= 0 {
        (length, length + offset as usize)
    } else {
        (length + offset.unsigned_abs() as usize, length)
    }
}
