use std::{cmp::max, mem};

use crate::{
    exact_matches::{get_complement, PALINDROME_LENGTH}, fasta_parsing::Fasta, output::PalindromeData, smith_waterman::MISMATCH_LENGTH_RATIO
};

pub fn wfa_palins(fasta: Fasta, output: &mut Vec<PalindromeData>){
    let seq = fasta.get_sequence();
    let mut prev = String::new();
    let mut index = 0;
    while index <= seq.len(){
        let len = wfa(&seq[index..], &prev);
        if len >= PALINDROME_LENGTH{
            output.push(PalindromeData::new(
                index as u32,
                index as u32+len,
                len,
                0, 
                0,
                fasta.get_name(), 
                String::from(&seq[index..index+len as usize])))
        }
    }
}

pub fn wfa(seq: &str, seq2: &str) -> u32{
    let len = 2*max(seq.len(), seq2.len()) + 3;
    let mut edit_dist = 0;
    let mut wavefront: Vec<u32> = vec![0; len];
    let mut new_wavefront: Vec<u32> = vec![0; len];
    let mut wf_len = 3;
    loop {
        for i in 0..wf_len{
            let (mut x, mut y) = get_xy(wf_len, i, wavefront[i] as usize);
            while x < seq.len() && y < seq2.len() && &seq[x..=x] == &seq2[y..=y]{
                dbg!(x, y, &seq[x..=x], &seq2[y..=y]);
                wavefront[i] += 1;
                x += 1;
                y += 1;
            }
            if x == seq.len() && y == seq2.len(){
                return edit_dist;
            }
        }
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
        edit_dist += 1;
        mem::swap(&mut wavefront, &mut new_wavefront);
        wf_len += 2;
    }
}

fn get_xy(wf_len: usize, index: usize, length: usize) -> (usize, usize){
    let offset = (((wf_len - 3)/2) + 1) as i32 - (index as i32);
    if offset >= 0 {
        return (length, length + offset as usize)
    } else {
        return (length + (offset.abs()) as usize, length);
    }
}