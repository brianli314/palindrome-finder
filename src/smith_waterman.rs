use std::cmp::max;
use crate::{matrix::Matrix, util::*};

pub fn smith_waterman(seq: &str, output: &mut Vec<PalindromeData>) {
    let length = seq.len();
    let mut matrix:Matrix<u32> = Matrix::new(length+1, length+1);
    fill_matrix(seq, &mut matrix, length);

    let mut palin = String::new();
    loop{
        let mut square = match matrix.get_data().iter().max(){
            Some(data) => data*1,
            None => panic!("Bad Matrix")
        };
        if square == 0 {
            break;
        }
        let (x, y) = matrix.get_index(square);
        while square != 0{
            palin += &seq[y-1..=y-1];
            square = matrix[[x-1, y-1]];
            matrix[[x,y]] = 0;
        }
        if palin.len() as u32 > PALINDROME_LENGTH {
            output.push(PalindromeData::new(
                y as u32,
                (y + palin.len()) as u32,
                palin.len() as u32,
                palin.clone(),
            ));
            palin.clear()
        }
    }
    
}

fn fill_matrix(seq: &str, matrix: &mut Matrix<u32>, length: usize){
    let complement: String = seq_compliment(seq).chars().rev().collect();
    for row in 1..length+1{
        for col in 1..length+1{
            let mut sub = matrix[[row-1, col-1]] + 1;
            if !seq[col-1..=col-1].eq(&complement[row-1..=row-1]) {
                sub -= 1;
            }
            let del = max(matrix[[row-1, col]] as i32 - 2, 0) as u32;
            let insert = max(matrix[[row, col-1]] as i32 - 2, 0) as u32;
            matrix[[row, col]] = max(del, max(sub, insert));
        }
    }
}