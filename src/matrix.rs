use std::ops::{Index, IndexMut};

pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    columns: usize,
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


