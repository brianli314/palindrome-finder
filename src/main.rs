use std::fs::File;
use std::io::{prelude::*, BufReader};

static PALINDROME_LENGTH: u32 = 5;
static GAP_LENGTH: u32 = 6;
static NUM_MISMATCH:u32 = 2;



// add commment
// this is useless

fn main() -> std::io::Result<()> {
    let file = File::open("sequence.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }
    exact_match(String::from("AGGCTACAAAAACCAGCCTGCATCGAACCCCTTCGAT"));

    Ok(())
}

fn exact_match(seq: String) -> Vec<u32> {
    let mut output:Vec<u32> = Vec::new();
    for i in 0..seq.len() as u32{
        let mut j=0;
        while i>=j && j <= GAP_LENGTH{
            let length = get_palindrome(i, i+j, &seq);
            if length >= PALINDROME_LENGTH && !output.contains(&(i+1-length)){
                output.push(i+1 - length);
                println!("{}", &seq[(i+1-length) as usize..(i+length+j) as usize]);
            }
            j += 1;
        }
        
    }
    println!("{:?}", output);
    return output;
}


fn get_palindrome(start: u32, end: u32, seq: &String) -> u32 {
    let mut prev:i32 = start.try_into().unwrap();
    let mut next = end as usize;
    let mut count = 0;
    let mut mismatches = 0;
    while prev >= 0 && next < seq.len() && mismatches < NUM_MISMATCH {
        if!(is_compliment(&seq[prev as usize..=prev as usize], &seq[next..=next])){
            mismatches += 1;
        }
        count += 1;
        prev -= 1;
        next += 1;
    }
    
    return count;
}

fn is_compliment(base1: &str, base2: &str) -> bool {
    let base1u = base1.to_uppercase();
    let base2u = base2.to_uppercase();
    
    (base1u == "A" && base2u == "T") ||
    (base1u == "T" && base2u == "A") || 
    (base1u == "C" && base2u == "G") || 
    (base1u == "G" && base2u == "C")
}