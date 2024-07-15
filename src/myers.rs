use crate::{matrix::Matrix, output::PalindromeData, smith_waterman::MISMATCH_LENGTH_RATIO, exact_matches::PALINDROME_LENGTH}; // * imports are not recommended in rust


fn WFA(seq: &str){
    let len = seq.len();
    let mut mismatches = 0;
    let mut wavefront = vec![0; len]; // are we sure that this should be len? Remember that this is diagonal.
    // You should not be modifying the wavefront in place. In general, the way that this algorithm is implemented is to
    // use two seperate buffers. In each iteration, we compute the next wavefront, writing it to the new buffer. Then, we swap the buffers,
    // making the old, outdated buffer a place where we can write the new buffer.
    // This is extremely important because modifying in place could have some really bad side effects. Try to think about what those could be!
    let mut index = 1;
    while (mismatches as f32)/(len as f32) < MISMATCH_LENGTH_RATIO{
        for i in 0..len{
            
        }
        let sub = wavefront[index] + 1;
        let del = wavefront[index - 1] + 1;
        let ins = wavefront[index + 1] + 1;
    }

}
