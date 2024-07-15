use std::{arch::x86_64::_MM_FROUND_NO_EXC, cmp::max, mem};

use crate::{
    exact_matches::{get_complement, PALINDROME_LENGTH},
    matrix::Matrix,
    output::PalindromeData,
    smith_waterman::MISMATCH_LENGTH_RATIO,
};

pub fn wfa(seq: &str, seq2: &str) {
    let len = max(seq.len(), seq2.len());
    let mut mismatches = 0;
    let mut wavefront: Vec<u32> = vec![0; len];
    let mut new_wavefront: Vec<u32> = vec![0; len];
    let mut index = 1;
    let mut wf_len = 3;
    while (mismatches as f32) / (len as f32 + 0.001) < MISMATCH_LENGTH_RATIO {
        for i in 0..wf_len {
            if i == 0 {
                new_wavefront[i] = wavefront[i];
                if wavefront[i] > wavefront[i + 1] {
                    new_wavefront[i+1] = wavefront[i] + 1;
                } else {
                    new_wavefront[i+1] = wavefront[i + 1];
                }
            } else if i != wf_len - 1 {
                let max = max(wavefront[i], max(wavefront[i - 1], wavefront[i + 1]));
                if max == wavefront[i] {
                    new_wavefront[i+1] = max + 1;
                } else {
                    new_wavefront[i+1] = max;
                }
            } else {
                new_wavefront[i+2] = wavefront[i];
                if wavefront[i-1] > wavefront[i]{
                    new_wavefront[i+1] = wavefront[i-1]
                } else {
                    new_wavefront[i+1] = wavefront[i] + 1;
                }
            }
        }
        mem::swap(&mut wavefront, &mut new_wavefront);
        wf_len += 2;
    }
}

fn compute_next_wf(){
    
}
