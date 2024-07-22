use std::{cmp::max, mem};

use crate::{
    exact_matches::{get_complement, PALINDROME_LENGTH},
    fasta_parsing::Fasta,
    output::PalindromeData,
    smith_waterman::MISMATCH_LENGTH_RATIO,
};

pub fn wfa_palins(fasta: Fasta, output: &mut Vec<PalindromeData>) {
    let seq = fasta.get_sequence();
    let len = seq.len();
    let mut wf = vec![0; len];
    let mut wf_next = vec![0; len];
    let mut index = 0;

    while index <= len {
        let mut edit_dist = 0;
        let mut wf_len = 3;
        wf[0] = 0;
        wf[1] = 0;
        wf[2] = 0;
        let mut max_index = 0;
        'outer: while (edit_dist as f32)/(wf[max_index] as f32 + 0.001) <= MISMATCH_LENGTH_RATIO {
            for i in 0..wf_len {
                let (mut x, mut y) = get_xy(wf_len, i, wf[i]);
                x = x + index;
                while x < len
                    && y < index
                    && &seq[x..=x] == get_complement(&seq[index - y - 1..=index - y - 1])
                {
                    wf[i] += 1;
                    x += 1;
                    y += 1;
                }
                if wf[i] > wf[max_index] {
                    max_index = i;
                }
                if x >= len || y >= index {
                    break 'outer;
                }
            }
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
            max_index += 1;
            edit_dist += 1;
            mem::swap(&mut wf, &mut wf_next);
            wf_len += 2;
        }
        if wf[max_index] == 0{
            index += 1;
            continue;
        }
        let (x, y) = get_xy(wf_len, max_index, wf[max_index] - 1);
        if (x + y) as u32 >= 5{
            let palin = PalindromeData::new(
                (index - y) as u32, 
                (index + x - 1) as u32, 
                (x + y) as u32, 
                0, 
                edit_dist, 
                fasta.get_name(), 
                seq[index - y..index + x].to_string());
            output.push(palin);
            
        }
        index += wf[max_index];
    }
}

pub fn wfa(seq: &str, seq2: &str) -> u32 {
    let len = 2 * max(seq.len(), seq2.len()) + 3;
    let mut edit_dist = 0;
    let mut wavefront: Vec<u32> = vec![0; len];
    let mut new_wavefront: Vec<u32> = vec![0; len];
    let mut wf_len = 3;
    loop {
        for i in 0..wf_len {
            let (mut x, mut y) = get_xy(wf_len, i, wavefront[i] as usize);
            while x < seq.len() && y < seq2.len() && &seq[x..=x] == &seq2[y..=y] {
                dbg!(x, y, &seq[x..=x], &seq2[y..=y]);
                wavefront[i] += 1;
                x += 1;
                y += 1;
            }
            if x == seq.len() && y == seq2.len() {
                return edit_dist;
            }
        }
        for i in 0..wf_len {
            if i == 0 {
                new_wavefront[i] = wavefront[i];
                new_wavefront[i + 1] = max(wavefront[i] + 1, wavefront[i + 1]);
            } else if i != wf_len - 1 {
                new_wavefront[i + 1] =
                    max(wavefront[i] + 1, max(wavefront[i - 1], wavefront[i + 1]));
            } else {
                new_wavefront[i + 2] = wavefront[i];
                new_wavefront[i + 1] = max(wavefront[i - 1], wavefront[i] + 1);
            }
        }
        edit_dist += 1;
        mem::swap(&mut wavefront, &mut new_wavefront);
        wf_len += 2;
    }
}

fn get_xy(wf_len: usize, index: usize, length: usize) -> (usize, usize) {
    let offset = (((wf_len - 3) / 2) + 1) as i32 - (index as i32);
    if offset >= 0 {
        return (length, length + offset as usize);
    } else {
        return (length + (offset.abs()) as usize, length);
    }
}
