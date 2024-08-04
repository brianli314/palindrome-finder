use core::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    columns: usize,
}
impl<T> Matrix<T> {
    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn get_row(&self) -> usize {
        self.rows
    }

    pub fn get_col(&self) -> usize {
        self.columns
    }
}

impl Matrix<u32> {
    pub fn get_index(&self, element: u32) -> (usize, usize) {
        for item in self.data.iter().enumerate() {
            if *item.1 == element {
                return ((item.0 / self.columns), item.0 % self.columns);
            }
        }
        panic!("Item not in matrix")
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.columns {
                write!(f, "{} ", self[[i, j]])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            data: vec![T::default(); x * y],
            rows: x,
            columns: y,
        }
    }
}

impl<T> Index<[usize; 2]> for Matrix<T> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        assert!(
            index[0] < self.rows && index[1] < self.columns,
            "Index [{}, {}] out of bounds for matrix of size [{}, {}]",
            index[0],
            index[1],
            self.rows,
            self.columns
        );

        &self.data[self.rows * index[0] + index[1]]
    }
}

impl<T> IndexMut<[usize; 2]> for Matrix<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        assert!(
            index[0] < self.rows && index[1] < self.columns,
            "Index [{}, {}] out of bounds for matrix of size [{}, {}]",
            index[0],
            index[1],
            self.rows,
            self.columns
        );

        &mut self.data[self.rows * index[0] + index[1]]
    }
}
