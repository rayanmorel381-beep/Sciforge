use super::csr::SparseMatrix;

pub fn sparse_kronecker(a: &SparseMatrix, b: &SparseMatrix) -> SparseMatrix {
    let mut triplets = Vec::new();
    for ia in 0..a.rows {
        for ka in a.row_ptr[ia]..a.row_ptr[ia + 1] {
            let ja = a.col_idx[ka];
            let va = a.values[ka];
            for ib in 0..b.rows {
                for kb in b.row_ptr[ib]..b.row_ptr[ib + 1] {
                    let jb = b.col_idx[kb];
                    let vb = b.values[kb];
                    triplets.push((ia * b.rows + ib, ja * b.cols + jb, va * vb));
                }
            }
        }
    }
    SparseMatrix::from_triplets(a.rows * b.rows, a.cols * b.cols, &mut triplets)
}

pub fn sparse_from_diagonals(n: usize, diags: &[(i64, Vec<f64>)]) -> SparseMatrix {
    let mut triplets = Vec::new();
    for &(offset, ref vals) in diags {
        for (k, &v) in vals.iter().enumerate() {
            let i = if offset >= 0 {
                k
            } else {
                ((-offset) as usize) + k
            };
            let j = if offset >= 0 { k + offset as usize } else { k };
            if i < n && j < n {
                triplets.push((i, j, v));
            }
        }
    }
    SparseMatrix::from_triplets(n, n, &mut triplets)
}

pub fn sparse_add(a: &SparseMatrix, b: &SparseMatrix) -> SparseMatrix {
    let mut triplets = Vec::new();
    for i in 0..a.rows {
        for k in a.row_ptr[i]..a.row_ptr[i + 1] {
            triplets.push((i, a.col_idx[k], a.values[k]));
        }
    }
    for i in 0..b.rows {
        for k in b.row_ptr[i]..b.row_ptr[i + 1] {
            triplets.push((i, b.col_idx[k], b.values[k]));
        }
    }
    SparseMatrix::from_triplets(a.rows, a.cols, &mut triplets)
}

pub fn sparse_scale(a: &SparseMatrix, scalar: f64) -> SparseMatrix {
    let mut result = a.clone();
    for v in &mut result.values {
        *v *= scalar;
    }
    result
}

pub fn sparse_mat_mul(a: &SparseMatrix, b: &SparseMatrix) -> SparseMatrix {
    let mut triplets = Vec::new();
    for i in 0..a.rows {
        for ka in a.row_ptr[i]..a.row_ptr[i + 1] {
            let ja = a.col_idx[ka];
            let va = a.values[ka];
            for kb in b.row_ptr[ja]..b.row_ptr[ja + 1] {
                triplets.push((i, b.col_idx[kb], va * b.values[kb]));
            }
        }
    }
    SparseMatrix::from_triplets(a.rows, b.cols, &mut triplets)
}

pub fn sparse_trace(a: &SparseMatrix) -> f64 {
    let mut trace = 0.0;
    for i in 0..a.rows.min(a.cols) {
        trace += a.get(i, i);
    }
    trace
}

pub fn sparse_frobenius_norm(a: &SparseMatrix) -> f64 {
    a.values.iter().map(|v| v * v).sum::<f64>().sqrt()
}

pub fn sparse_infinity_norm(a: &SparseMatrix) -> f64 {
    let mut max_row_sum = 0.0_f64;
    for i in 0..a.rows {
        let row_sum: f64 = (a.row_ptr[i]..a.row_ptr[i + 1])
            .map(|k| a.values[k].abs())
            .sum();
        max_row_sum = max_row_sum.max(row_sum);
    }
    max_row_sum
}

pub fn sparse_one_norm(a: &SparseMatrix) -> f64 {
    let mut col_sums = vec![0.0; a.cols];
    for i in 0..a.rows {
        for k in a.row_ptr[i]..a.row_ptr[i + 1] {
            col_sums[a.col_idx[k]] += a.values[k].abs();
        }
    }
    col_sums.iter().cloned().fold(0.0, f64::max)
}

pub fn sparse_diagonal(a: &SparseMatrix) -> Vec<f64> {
    let n = a.rows.min(a.cols);
    (0..n).map(|i| a.get(i, i)).collect()
}

pub fn sparse_lower_triangular(a: &SparseMatrix) -> SparseMatrix {
    let mut triplets = Vec::new();
    for i in 0..a.rows {
        for k in a.row_ptr[i]..a.row_ptr[i + 1] {
            if a.col_idx[k] <= i {
                triplets.push((i, a.col_idx[k], a.values[k]));
            }
        }
    }
    SparseMatrix::from_triplets(a.rows, a.cols, &mut triplets)
}

pub fn sparse_upper_triangular(a: &SparseMatrix) -> SparseMatrix {
    let mut triplets = Vec::new();
    for i in 0..a.rows {
        for k in a.row_ptr[i]..a.row_ptr[i + 1] {
            if a.col_idx[k] >= i {
                triplets.push((i, a.col_idx[k], a.values[k]));
            }
        }
    }
    SparseMatrix::from_triplets(a.rows, a.cols, &mut triplets)
}

pub fn sparse_row_sum(a: &SparseMatrix) -> Vec<f64> {
    (0..a.rows)
        .map(|i| (a.row_ptr[i]..a.row_ptr[i + 1]).map(|k| a.values[k]).sum())
        .collect()
}

pub fn sparse_col_sum(a: &SparseMatrix) -> Vec<f64> {
    let mut sums = vec![0.0; a.cols];
    for i in 0..a.rows {
        for k in a.row_ptr[i]..a.row_ptr[i + 1] {
            sums[a.col_idx[k]] += a.values[k];
        }
    }
    sums
}

pub fn sparse_is_symmetric(a: &SparseMatrix, tol: f64) -> bool {
    if a.rows != a.cols {
        return false;
    }
    for i in 0..a.rows {
        for k in a.row_ptr[i]..a.row_ptr[i + 1] {
            let j = a.col_idx[k];
            if (a.values[k] - a.get(j, i)).abs() > tol {
                return false;
            }
        }
    }
    true
}
