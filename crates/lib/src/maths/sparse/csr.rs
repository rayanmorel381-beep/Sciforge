use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Debug)]
pub struct SparseMatrix {
    pub rows: usize,
    pub cols: usize,
    pub row_ptr: Vec<usize>,
    pub col_idx: Vec<usize>,
    pub values: Vec<f64>,
}

impl SparseMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            row_ptr: vec![0; rows + 1],
            col_idx: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn from_triplets(rows: usize, cols: usize, triplets: &mut [(usize, usize, f64)]) -> Self {
        triplets.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        let mut row_ptr = vec![0; rows + 1];
        let mut col_idx = Vec::with_capacity(triplets.len());
        let mut values = Vec::with_capacity(triplets.len());

        for &(r, c, v) in triplets.iter() {
            if v.abs() < 1e-30 {
                continue;
            }
            row_ptr[r + 1] += 1;
            col_idx.push(c);
            values.push(v);
        }
        for i in 0..rows {
            row_ptr[i + 1] += row_ptr[i];
        }
        Self {
            rows,
            cols,
            row_ptr,
            col_idx,
            values,
        }
    }

    pub fn identity(n: usize) -> Self {
        let mut triplets: Vec<(usize, usize, f64)> = (0..n).map(|i| (i, i, 1.0)).collect();
        Self::from_triplets(n, n, &mut triplets)
    }

    pub fn nnz(&self) -> usize {
        self.values.len()
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        let start = self.row_ptr[row];
        let end = self.row_ptr[row + 1];
        for k in start..end {
            if self.col_idx[k] == col {
                return self.values[k];
            }
        }
        0.0
    }

    pub fn mul_vec(&self, x: &[f64]) -> Vec<f64> {
        assert_eq!(x.len(), self.cols);
        let mut result = vec![0.0; self.rows];
        for (i, ri) in result.iter_mut().enumerate() {
            for k in self.row_ptr[i]..self.row_ptr[i + 1] {
                *ri += self.values[k] * x[self.col_idx[k]];
            }
        }
        result
    }

    pub fn transpose(&self) -> Self {
        let mut triplets: Vec<(usize, usize, f64)> = Vec::with_capacity(self.nnz());
        for i in 0..self.rows {
            for k in self.row_ptr[i]..self.row_ptr[i + 1] {
                triplets.push((self.col_idx[k], i, self.values[k]));
            }
        }
        Self::from_triplets(self.cols, self.rows, &mut triplets)
    }

    pub fn add(&self, other: &Self) -> Self {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let mut triplets = Vec::new();
        for i in 0..self.rows {
            for k in self.row_ptr[i]..self.row_ptr[i + 1] {
                triplets.push((i, self.col_idx[k], self.values[k]));
            }
            for k in other.row_ptr[i]..other.row_ptr[i + 1] {
                triplets.push((i, other.col_idx[k], other.values[k]));
            }
        }
        Self::from_triplets(self.rows, self.cols, &mut triplets)
    }

    pub fn scale(&self, s: f64) -> Self {
        Self {
            rows: self.rows,
            cols: self.cols,
            row_ptr: self.row_ptr.clone(),
            col_idx: self.col_idx.clone(),
            values: self.values.iter().map(|v| v * s).collect(),
        }
    }

    pub fn matmul(&self, other: &Self) -> Self {
        assert_eq!(self.cols, other.rows);
        let mut triplets = Vec::new();
        let ot = other.transpose();
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                let (mut p, mut q) = (self.row_ptr[i], ot.row_ptr[j]);
                let (pe, qe) = (self.row_ptr[i + 1], ot.row_ptr[j + 1]);
                while p < pe && q < qe {
                    if self.col_idx[p] == ot.col_idx[q] {
                        sum += self.values[p] * ot.values[q];
                        p += 1;
                        q += 1;
                    } else if self.col_idx[p] < ot.col_idx[q] {
                        p += 1;
                    } else {
                        q += 1;
                    }
                }
                if sum.abs() > 1e-30 {
                    triplets.push((i, j, sum));
                }
            }
        }
        Self::from_triplets(self.rows, other.cols, &mut triplets)
    }

    pub fn diagonal(&self) -> Vec<f64> {
        let n = self.rows.min(self.cols);
        (0..n).map(|i| self.get(i, i)).collect()
    }

    pub fn frobenius_norm(&self) -> f64 {
        self.values.iter().map(|v| v * v).sum::<f64>().sqrt()
    }

    pub fn to_dense(&self) -> Vec<Vec<f64>> {
        let mut dense = vec![vec![0.0; self.cols]; self.rows];
        for (i, dense_row) in dense.iter_mut().enumerate() {
            for k in self.row_ptr[i]..self.row_ptr[i + 1] {
                dense_row[self.col_idx[k]] = self.values[k];
            }
        }
        dense
    }
}

impl fmt::Display for SparseMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SparseMatrix({}x{}, nnz={})",
            self.rows,
            self.cols,
            self.nnz()
        )
    }
}

impl Add for &SparseMatrix {
    type Output = SparseMatrix;
    fn add(self, rhs: Self) -> SparseMatrix {
        self.add(rhs)
    }
}

impl Sub for &SparseMatrix {
    type Output = SparseMatrix;
    fn sub(self, rhs: Self) -> SparseMatrix {
        self.add(&rhs.scale(-1.0))
    }
}

impl Mul for &SparseMatrix {
    type Output = SparseMatrix;
    fn mul(self, rhs: Self) -> SparseMatrix {
        self.matmul(rhs)
    }
}

impl Mul<f64> for &SparseMatrix {
    type Output = SparseMatrix;
    fn mul(self, s: f64) -> SparseMatrix {
        self.scale(s)
    }
}

impl Neg for &SparseMatrix {
    type Output = SparseMatrix;
    fn neg(self) -> SparseMatrix {
        self.scale(-1.0)
    }
}
