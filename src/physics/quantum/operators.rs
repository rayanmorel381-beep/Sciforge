use crate::maths::complex::Complex;

pub fn commutator(a: &[Vec<Complex>], b: &[Vec<Complex>]) -> Vec<Vec<Complex>> {
    let ab = complex_matmul(a, b);
    let ba = complex_matmul(b, a);
    let n = a.len();
    let mut result = vec![vec![Complex::new(0.0, 0.0); n]; n];
    for i in 0..n {
        for j in 0..n {
            result[i][j] = ab[i][j] - ba[i][j];
        }
    }
    result
}

pub fn anticommutator(a: &[Vec<Complex>], b: &[Vec<Complex>]) -> Vec<Vec<Complex>> {
    let ab = complex_matmul(a, b);
    let ba = complex_matmul(b, a);
    let n = a.len();
    let mut result = vec![vec![Complex::new(0.0, 0.0); n]; n];
    for i in 0..n {
        for j in 0..n {
            result[i][j] = ab[i][j] + ba[i][j];
        }
    }
    result
}

fn complex_matmul(a: &[Vec<Complex>], b: &[Vec<Complex>]) -> Vec<Vec<Complex>> {
    let n = a.len();
    let m = b[0].len();
    let mut result = vec![vec![Complex::new(0.0, 0.0); m]; n];
    for i in 0..n {
        for j in 0..m {
            for (k, bk) in b.iter().enumerate() {
                result[i][j] = result[i][j] + a[i][k] * bk[j];
            }
        }
    }
    result
}

pub fn expectation_value(operator: &[Vec<Complex>], state: &[Complex]) -> Complex {
    let mut result = Complex::new(0.0, 0.0);
    for (op_row, &si) in operator.iter().zip(state.iter()) {
        let mut op_psi = Complex::new(0.0, 0.0);
        for (&oij, &sj) in op_row.iter().zip(state.iter()) {
            op_psi = op_psi + oij * sj;
        }
        result = result + si.conj() * op_psi;
    }
    result
}

pub fn variance(operator: &[Vec<Complex>], state: &[Complex]) -> f64 {
    let ev = expectation_value(operator, state);
    let n = state.len();
    let mut op2 = vec![vec![Complex::new(0.0, 0.0); n]; n];
    for (i, op2_row) in op2.iter_mut().enumerate() {
        for (j, op2_ij) in op2_row.iter_mut().enumerate() {
            for (&oik, ok) in operator[i].iter().zip(operator) {
                *op2_ij = *op2_ij + oik * ok[j];
            }
        }
    }
    let ev2 = expectation_value(&op2, state);
    ev2.re - ev.re * ev.re - ev.im * ev.im
}

pub fn uncertainty_product(op_a: &[Vec<Complex>], op_b: &[Vec<Complex>], state: &[Complex]) -> f64 {
    let var_a = variance(op_a, state);
    let var_b = variance(op_b, state);
    var_a.abs().sqrt() * var_b.abs().sqrt()
}

pub fn is_hermitian(m: &[Vec<Complex>]) -> bool {
    let n = m.len();
    for (i, mi) in m.iter().enumerate() {
        for j in 0..n {
            let diff = mi[j] - m[j][i].conj();
            if diff.norm() > 1e-12 {
                return false;
            }
        }
    }
    true
}

pub fn is_unitary(m: &[Vec<Complex>]) -> bool {
    let n = m.len();
    let mut mdag = vec![vec![Complex::new(0.0, 0.0); n]; n];
    for (i, mdag_row) in mdag.iter_mut().enumerate() {
        for (j, mdag_ij) in mdag_row.iter_mut().enumerate() {
            *mdag_ij = m[j][i].conj();
        }
    }
    let prod = complex_matmul(m, &mdag);
    for (i, prod_i) in prod.iter().enumerate() {
        for (j, &prod_ij) in prod_i.iter().enumerate() {
            let expected = if i == j {
                Complex::new(1.0, 0.0)
            } else {
                Complex::new(0.0, 0.0)
            };
            if (prod_ij - expected).norm() > 1e-10 {
                return false;
            }
        }
    }
    true
}

pub fn tensor_product(a: &[Vec<Complex>], b: &[Vec<Complex>]) -> Vec<Vec<Complex>> {
    let na = a.len();
    let nb = b.len();
    let n = na * nb;
    let mut result = vec![vec![Complex::new(0.0, 0.0); n]; n];
    for i in 0..na {
        for j in 0..na {
            for k in 0..nb {
                for l in 0..nb {
                    result[i * nb + k][j * nb + l] = a[i][j] * b[k][l];
                }
            }
        }
    }
    result
}

pub fn density_matrix(state: &[Complex]) -> Vec<Vec<Complex>> {
    let n = state.len();
    let mut rho = vec![vec![Complex::new(0.0, 0.0); n]; n];
    for (i, rho_row) in rho.iter_mut().enumerate() {
        for (rho_ij, &sj) in rho_row.iter_mut().zip(state.iter()) {
            *rho_ij = state[i] * sj.conj();
        }
    }
    rho
}

pub fn trace_complex(m: &[Vec<Complex>]) -> Complex {
    let mut t = Complex::new(0.0, 0.0);
    for (i, row) in m.iter().enumerate() {
        t = t + row[i];
    }
    t
}

pub fn partial_trace_b(rho: &[Vec<Complex>], dim_a: usize, dim_b: usize) -> Vec<Vec<Complex>> {
    let mut rho_a = vec![vec![Complex::new(0.0, 0.0); dim_a]; dim_a];
    for i in 0..dim_a {
        for j in 0..dim_a {
            for k in 0..dim_b {
                rho_a[i][j] = rho_a[i][j] + rho[i * dim_b + k][j * dim_b + k];
            }
        }
    }
    rho_a
}
