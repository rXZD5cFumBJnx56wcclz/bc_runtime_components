use std::{
    ops::{Deref, Index},
    slice::SliceIndex,
};

use bc_utils::other::{roll_slice1, transpose, transpose_set};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Buffer {
    // row data
    pub d: Vec<Vec<f64>>,
    pub window: usize,
    pub is_transposed: bool,
}

impl Buffer {
    pub fn from_row(src: Vec<Vec<f64>>, window: usize) -> Self {
        Self {
            d: src[src.len() - window..].to_vec(),
            window,
            is_transposed: false,
        }
    }

    pub fn from_column(src: Vec<Vec<f64>>, window: usize) -> Self {
        Self::from_row(transpose(src), window)
    }
}

impl Buffer {
    pub fn update(&mut self, src: Vec<f64>) {
        roll_slice1(&mut self.d, -1);
        let l = self.d.len() - 1;
        self.d[l] = src;
    }
    pub fn update_extend(&mut self, src: &[Vec<f64>]) {
        roll_slice1(&mut self.d, -(src.len() as i32));
        for _ in 0..src.len() {
            self.d.pop();
        }
        self.d.extend_from_slice(src);
    }
}

impl Buffer {
    pub fn iter(&self) -> impl Iterator<Item = &Vec<f64>> {
        self.d.iter()
    }
    pub fn first(&self) -> Option<&Vec<f64>> {
        self.d.first()
    }
    pub fn last(&self) -> Option<&Vec<f64>> {
        self.d.last()
    }
    pub fn last1(&self) -> Option<&Vec<f64>> {
        self.d.get(self.len() - 2)
    }
    pub fn len(&self) -> usize {
        self.d.len()
    }
    pub fn as_slice(&self) -> &[Vec<f64>] {
        self.d.as_slice()
    }
    pub fn transpose(mut self) -> Self {
        self.d = transpose(self.d);
        self.window = self.d.len();
        self.is_transposed = !self.is_transposed;
        self
    }
    pub fn transpose_set(&mut self) {
        transpose_set(&mut self.d);
        self.window = self.d.len();
        self.is_transposed = !self.is_transposed;
    }
}

impl<T: SliceIndex<[Vec<f64>]>> Index<T> for Buffer {
    type Output = T::Output;
    fn index(&self, index: T) -> &Self::Output {
        &self.d[index]
    }
}

impl Extend<Vec<f64>> for Buffer {
    fn extend<T: IntoIterator<Item = Vec<f64>>>(&mut self, iter: T) {
        self.d.extend(iter);
    }
}

impl Deref for Buffer {
    type Target = [Vec<f64>];
    fn deref(&self) -> &Self::Target {
        self.d.as_slice()
    }
}

pub trait ToBuff {
    fn to_buff(&self) -> Buffer;
}

impl ToBuff for [Vec<f64>] {
    fn to_buff(&self) -> Buffer {
        Buffer {
            d: self.to_vec(),
            window: self.len(),
            is_transposed: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bc_test_kit::prelude::*;

    static BF: LazyLock<fn() -> Buffer> = LazyLock::new(|| || SRC.to_buff());

    #[test]
    fn from_row_res_1() {
        let bf = Buffer::from_row(SRC.clone(), 50);
        assert_eq!(bf.len(), 50);
    }

    #[test]
    fn from_column_res_1() {
        let bf = Buffer::from_column(SRC_TRANSPOSE.clone(), 50);
        assert_eq!(bf.len(), 50);
    }

    #[test]
    fn transpose_res_1() {
        let mut bf = Buffer::from_column(SRC_TRANSPOSE.clone(), 50);
        bf.transpose_set();
        assert!(bf.is_transposed);
        bf.transpose_set();
        assert!(!bf.is_transposed);
    }

    #[test]
    fn update_res_1() {
        let mut bf = BF();
        let mut res = BF();
        bf.update(SRC_EL.to_vec());
        roll_slice1(&mut res.d, -1);
        let l = res.d.len() - 1;
        res.d[l] = SRC_EL.to_vec();
        assert_eq_pr!(bf, res);
    }

    #[test]
    fn update_extend_res_1() {
        let mut bf = BF();
        let mut res = BF();
        bf.update_extend(&SRC);
        roll_slice1(&mut res.d, -(SRC.len() as i32));
        for _ in 0..SRC.len() {
            res.d.pop();
        }
        res.d.extend_from_slice(&SRC);
        assert_eq_pr!(bf, res);
    }
}
