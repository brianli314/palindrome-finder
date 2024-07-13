use std::cmp::max;
use crate::{matrix::Matrix, util::*};

static GAP_PENALTY: i32 = 2;
static MISMATCH_PENALTY:i32 = 2;
static MATCH_BONUS:u32 = 1;

pub fn smith_waterman(seq: &str, output: &mut Vec<PalindromeData>) {
    let mut matrix = fill_matrix(seq);
    //println!("{}", matrix); 
    traceback(seq, &mut matrix, output)
}

fn add_palin(start: u32,  mismatches: u32, palin: &mut String, output: &mut Vec<PalindromeData>){
    let len = palin.len() as u32;
    if len > PALINDROME_LENGTH {
        output.push(PalindromeData::new(
            start,
            start + len,
            len,
            0,
            mismatches,
            palin.clone(),
        ));
    }
}
fn traceback(seq: &str, matrix: &mut Matrix<u32>, output: &mut Vec<PalindromeData>){
    let mut palin = String::new();
    let mut mismatches = 0;
    loop{
        let mut square = match matrix.get_data().iter().max() {
            Some(data) => *data,
            None => panic!("Bad Matrix"),
        };
        if square <= 13 {
            break;
        }
        let (mut x, mut y) = matrix.get_index(square);
        while square != 0 && (mismatches as f32)/(palin.len() as f32) < MISMATCH_LENGTH_RATIO{
            let sub = matrix[[x - 1, y - 1]];
            let del = matrix[[x - 1, y]];
            let ins = matrix[[x, y - 1]];

            matrix[[x,y]] = 0;
            if sub >= del && sub >= ins {
                x -= 1;
                y -= 1;
            } else if del >= sub && del >= ins {  
                x -= 1;
            } else {
                y -= 1;
            }
            palin += &seq[y - 1..y]; 
            if square < matrix[[x,y]] {
                mismatches += 1;
            }
            square = matrix[[x, y]];
        }
        add_palin(y as u32, mismatches, &mut palin, output);
        palin.clear()
    }
}

fn fill_matrix(seq: &str) -> Matrix<u32>{
    let complement: String = seq_compliment(seq).chars().rev().collect();
    let length = seq.len();
    let mut matrix:Matrix<u32> = Matrix::new(length+1, length+1);
    for row in 1..length+1{
        for col in 1..length+1{
            let mut sub = matrix[[row-1, col-1]];
            if !seq[col-1..=col-1].eq(&complement[row-1..=row-1]) {
                sub = max(0, sub as i32 - MISMATCH_PENALTY)  as u32;
            } else {
                sub += MATCH_BONUS;
            }
            let del = max(matrix[[row-1, col]] as i32 - GAP_PENALTY, 0) as u32;
            let insert = max(matrix[[row, col-1]] as i32 - GAP_PENALTY, 0) as u32;
            matrix[[row, col]] = max(del, max(sub, insert));
        }
    }
    return matrix;
}