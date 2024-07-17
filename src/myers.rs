use std::{cmp::max, mem};

use crate::{
    exact_matches::{get_complement, PALINDROME_LENGTH},
    output::PalindromeData,
    smith_waterman::MISMATCH_LENGTH_RATIO,
};

pub fn wfa(seq: &str) {
    let len = seq.len();
    let mut edit_dist = 0;
    let mut wavefront: Vec<u32> = vec![0; len];
    let mut new_wavefront: Vec<u32> = vec![0; len];
    let mut index = 0;
    let mut wf_len = 3;
    while (edit_dist as f32) / (len as f32 + 0.001) < MISMATCH_LENGTH_RATIO {
        
        for i in 0..wf_len {
            if i == 0 {
                new_wavefront[i] = wavefront[i];
                new_wavefront[i+1] = max(wavefront[i] + 1, wavefront[i + 1]);
            } else if i != wf_len - 1 {
                new_wavefront[i+1] = max(wavefront[i] + 1, max(wavefront[i - 1], wavefront[i + 1]));
            } else {
                new_wavefront[i+2] = wavefront[i];
                new_wavefront[i+1] = max( wavefront[i-1], wavefront[i] + 1);
            }
        }
        mem::swap(&mut wavefront, &mut new_wavefront);
        dbg!(&wavefront);
        wf_len += 2;
    }
}

