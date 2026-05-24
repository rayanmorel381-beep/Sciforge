use super::ops::transpose;
use super::storage::Tensor;

pub fn trace(t: &Tensor) -> f64 {
    assert!(t.rank() == 2 && t.shape()[0] == t.shape()[1]);
    (0..t.shape()[0]).map(|i| t.get(&[i, i])).sum()
}

pub fn matmul(a: &Tensor, b: &Tensor) -> Tensor {
    assert!(a.rank() == 2 && b.rank() == 2);
    assert_eq!(a.shape()[1], b.shape()[0]);
    let (m, n, p) = (a.shape()[0], a.shape()[1], b.shape()[1]);
    Tensor::from_fn(&[m, p], |idx| {
        (0..n)
            .map(|k| a.get(&[idx[0], k]) * b.get(&[k, idx[1]]))
            .sum()
    })
}

pub fn minor(t: &Tensor, row: usize, col: usize) -> Tensor {
    let n = t.shape()[0];
    let mut data = Vec::with_capacity((n - 1) * (n - 1));
    for i in 0..n {
        if i == row {
            continue;
        }
        for j in 0..n {
            if j == col {
                continue;
            }
            data.push(t.get(&[i, j]));
        }
    }
    Tensor::from_vec(&[n - 1, n - 1], data)
}

pub fn determinant(t: &Tensor) -> f64 {
    assert!(t.rank() == 2 && t.shape()[0] == t.shape()[1]);
    let n = t.shape()[0];
    if n == 1 {
        return t.get(&[0, 0]);
    }
    if n == 2 {
        return t.get(&[0, 0]) * t.get(&[1, 1]) - t.get(&[0, 1]) * t.get(&[1, 0]);
    }
    let mut det = 0.0;
    for j in 0..n {
        let sign = if j % 2 == 0 { 1.0 } else { -1.0 };
        det += sign * t.get(&[0, j]) * determinant(&minor(t, 0, j));
    }
    det
}

pub fn inverse(t: &Tensor) -> Option<Tensor> {
    assert!(t.rank() == 2 && t.shape()[0] == t.shape()[1]);
    let n = t.shape()[0];
    let det = determinant(t);
    if det.abs() < 1e-15 {
        return None;
    }
    let cofactors = Tensor::from_fn(&[n, n], |idx| {
        let sign = if (idx[0] + idx[1]) % 2 == 0 {
            1.0
        } else {
            -1.0
        };
        sign * determinant(&minor(t, idx[0], idx[1]))
    });
    let adjugate = transpose(&cofactors, &[1, 0]);
    Some(adjugate.scale(1.0 / det))
}

pub fn eigenvalues_2x2(t: &Tensor) -> (f64, f64) {
    assert!(t.rank() == 2 && t.shape() == [2, 2]);
    let a = t.get(&[0, 0]);
    let b = t.get(&[0, 1]);
    let c = t.get(&[1, 0]);
    let d = t.get(&[1, 1]);
    let tr = a + d;
    let det = a * d - b * c;
    let disc = (tr * tr - 4.0 * det).max(0.0).sqrt();
    ((tr + disc) / 2.0, (tr - disc) / 2.0)
}

pub fn lu_decompose(t: &Tensor) -> (Tensor, Tensor) {
    assert!(t.rank() == 2 && t.shape()[0] == t.shape()[1]);
    let n = t.shape()[0];
    let mut l = Tensor::identity(n);
    let mut u = t.clone();
    for j in 0..n {
        for i in (j + 1)..n {
            let pivot = u.get(&[j, j]);
            if pivot.abs() < 1e-30 {
                continue;
            }
            let factor = u.get(&[i, j]) / pivot;
            l.set(&[i, j], factor);
            for k in j..n {
                let val = u.get(&[i, k]) - factor * u.get(&[j, k]);
                u.set(&[i, k], val);
            }
        }
    }
    (l, u)
}

pub fn solve(a: &Tensor, b: &[f64]) -> Vec<f64> {
    let (l, u) = lu_decompose(a);
    let n = a.shape()[0];
    let mut y = vec![0.0; n];
    for i in 0..n {
        y[i] = b[i];
        for j in 0..i {
            y[i] -= l.get(&[i, j]) * y[j];
        }
    }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = y[i];
        for j in (i + 1)..n {
            x[i] -= u.get(&[i, j]) * x[j];
        }
        let pivot = u.get(&[i, i]);
        if pivot.abs() > 1e-30 {
            x[i] /= pivot;
        }
    }
    x
}
