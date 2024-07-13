use crate::{matrix::Matrix, util::*};


fn WFA(seq: &str){
    let len = seq.len();
    let mut mismatches = 0;
    let mut wavefront = vec![0; len];
    let mut index = 1;
    while (mismatches as f32)/(len as f32) < MISMATCH_LENGTH_RATIO{
        for i in 0..len{
            
        }
        let sub = wavefront[index] + 1;
        let del = wavefront[index - 1] + 1;
        let ins = wavefront[index + 1] + 1;
    }

}
