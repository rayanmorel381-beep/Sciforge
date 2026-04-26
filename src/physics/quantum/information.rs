use crate::maths::complex::Complex;

pub fn von_neumann_entropy(rho: &[Vec<Complex>]) -> f64 {
    let eigenvalues = hermitian_eigenvalues(rho);
    let mut s = 0.0;
    for &lam in &eigenvalues {
        if lam > 1e-15 {
            s -= lam * lam.ln();
        }
    }
    s
}

pub fn purity(rho: &[Vec<Complex>]) -> f64 {
    let mut tr = 0.0;
    for (i, rho_i) in rho.iter().enumerate() {
        for (rho_k, &rik) in rho.iter().zip(rho_i) {
            tr += (rik * rho_k[i]).re;
        }
    }
    tr
}

pub fn fidelity_pure(psi: &[Complex], phi: &[Complex]) -> f64 {
    let mut ip = Complex::zero();
    for (&pi, &ph) in psi.iter().zip(phi.iter()) {
        ip = ip + pi.conj() * ph;
    }
    ip.norm_sq()
}

pub fn fidelity_mixed(rho: &[Vec<Complex>], sigma: &[Vec<Complex>]) -> f64 {
    let sqrt_rho = matrix_sqrt(rho);
    let product = complex_matmul(&complex_matmul(&sqrt_rho, sigma), &sqrt_rho);
    let sqrt_product = matrix_sqrt(&product);
    let mut tr = Complex::zero();
    for (i, row) in sqrt_product.iter().enumerate() {
        tr = tr + row[i];
    }
    tr.re * tr.re
}

pub fn concurrence_2qubit(rho: &[Vec<Complex>]) -> f64 {
    let sy = [
        [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
        [Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)],
    ];
    let sysy = tensor_product_2x2(&sy, &sy);

    let rho_conj: Vec<Vec<Complex>> = rho
        .iter()
        .map(|row| row.iter().map(|c| c.conj()).collect())
        .collect();
    let rho_tilde = complex_matmul(&complex_matmul(&sysy, &rho_conj), &sysy);
    let r_matrix = complex_matmul(rho, &rho_tilde);

    let eigenvalues = hermitian_eigenvalues(&r_matrix);
    let mut sq: Vec<f64> = eigenvalues
        .iter()
        .map(|&e| if e > 0.0 { e.sqrt() } else { 0.0 })
        .collect();
    sq.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let c = sq[0]
        - sq.get(1).copied().unwrap_or(0.0)
        - sq.get(2).copied().unwrap_or(0.0)
        - sq.get(3).copied().unwrap_or(0.0);
    c.max(0.0)
}

fn tensor_product_2x2(a: &[[Complex; 2]; 2], b: &[[Complex; 2]; 2]) -> Vec<Vec<Complex>> {
    let mut result = vec![vec![Complex::zero(); 4]; 4];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                for l in 0..2 {
                    result[i * 2 + k][j * 2 + l] = a[i][j] * b[k][l];
                }
            }
        }
    }
    result
}

pub fn bell_phi_plus() -> Vec<Complex> {
    let s = 1.0 / 2.0_f64.sqrt();
    vec![
        Complex::new(s, 0.0),
        Complex::zero(),
        Complex::zero(),
        Complex::new(s, 0.0),
    ]
}

pub fn bell_phi_minus() -> Vec<Complex> {
    let s = 1.0 / 2.0_f64.sqrt();
    vec![
        Complex::new(s, 0.0),
        Complex::zero(),
        Complex::zero(),
        Complex::new(-s, 0.0),
    ]
}

pub fn bell_psi_plus() -> Vec<Complex> {
    let s = 1.0 / 2.0_f64.sqrt();
    vec![
        Complex::zero(),
        Complex::new(s, 0.0),
        Complex::new(s, 0.0),
        Complex::zero(),
    ]
}

pub fn bell_psi_minus() -> Vec<Complex> {
    let s = 1.0 / 2.0_f64.sqrt();
    vec![
        Complex::zero(),
        Complex::new(s, 0.0),
        Complex::new(-s, 0.0),
        Complex::zero(),
    ]
}

pub fn hadamard_gate() -> Vec<Vec<Complex>> {
    let s = 1.0 / 2.0_f64.sqrt();
    vec![
        vec![Complex::new(s, 0.0), Complex::new(s, 0.0)],
        vec![Complex::new(s, 0.0), Complex::new(-s, 0.0)],
    ]
}

pub fn phase_gate(phi: f64) -> Vec<Vec<Complex>> {
    vec![
        vec![Complex::one(), Complex::zero()],
        vec![Complex::zero(), Complex::from_polar(1.0, phi)],
    ]
}

pub fn t_gate() -> Vec<Vec<Complex>> {
    phase_gate(std::f64::consts::FRAC_PI_4)
}

pub fn s_gate() -> Vec<Vec<Complex>> {
    phase_gate(std::f64::consts::FRAC_PI_2)
}

pub fn rx_gate(theta: f64) -> Vec<Vec<Complex>> {
    let c = (theta / 2.0).cos();
    let s = (theta / 2.0).sin();
    vec![
        vec![Complex::new(c, 0.0), Complex::new(0.0, -s)],
        vec![Complex::new(0.0, -s), Complex::new(c, 0.0)],
    ]
}

pub fn ry_gate(theta: f64) -> Vec<Vec<Complex>> {
    let c = (theta / 2.0).cos();
    let s = (theta / 2.0).sin();
    vec![
        vec![Complex::new(c, 0.0), Complex::new(-s, 0.0)],
        vec![Complex::new(s, 0.0), Complex::new(c, 0.0)],
    ]
}

pub fn rz_gate(theta: f64) -> Vec<Vec<Complex>> {
    vec![
        vec![Complex::from_polar(1.0, -theta / 2.0), Complex::zero()],
        vec![Complex::zero(), Complex::from_polar(1.0, theta / 2.0)],
    ]
}

pub fn cnot_gate() -> Vec<Vec<Complex>> {
    let z = Complex::zero();
    let o = Complex::one();
    vec![
        vec![o, z, z, z],
        vec![z, o, z, z],
        vec![z, z, z, o],
        vec![z, z, o, z],
    ]
}

pub fn swap_gate() -> Vec<Vec<Complex>> {
    let z = Complex::zero();
    let o = Complex::one();
    vec![
        vec![o, z, z, z],
        vec![z, z, o, z],
        vec![z, o, z, z],
        vec![z, z, z, o],
    ]
}

pub fn cz_gate() -> Vec<Vec<Complex>> {
    let z = Complex::zero();
    let o = Complex::one();
    let m = Complex::new(-1.0, 0.0);
    vec![
        vec![o, z, z, z],
        vec![z, o, z, z],
        vec![z, z, o, z],
        vec![z, z, z, m],
    ]
}

pub fn toffoli_gate() -> Vec<Vec<Complex>> {
    let z = Complex::zero();
    let o = Complex::one();
    let mut m = vec![vec![z; 8]; 8];
    for (i, mi) in m.iter_mut().enumerate().take(6) {
        mi[i] = o;
    }
    m[6][7] = o;
    m[7][6] = o;
    m
}

pub fn apply_gate(gate: &[Vec<Complex>], state: &[Complex]) -> Vec<Complex> {
    let n = state.len();
    let mut result = vec![Complex::zero(); n];
    for (ri, gate_row) in result.iter_mut().zip(gate.iter()) {
        for (&gij, &sj) in gate_row.iter().zip(state.iter()) {
            *ri = *ri + gij * sj;
        }
    }
    result
}

pub fn apply_single_qubit_gate(
    gate: &[Vec<Complex>],
    state: &mut [Complex],
    target: usize,
    n_qubits: usize,
) {
    let dim = 1 << n_qubits;
    let step = 1 << target;
    let mut i = 0;
    while i < dim {
        for j in 0..step {
            let idx0 = i + j;
            let idx1 = i + j + step;
            let a = state[idx0];
            let b = state[idx1];
            state[idx0] = gate[0][0] * a + gate[0][1] * b;
            state[idx1] = gate[1][0] * a + gate[1][1] * b;
        }
        i += step << 1;
    }
}

pub fn measure_probabilities(state: &[Complex]) -> Vec<f64> {
    state.iter().map(|c| c.norm_sq()).collect()
}

pub fn bloch_vector(state: &[Complex; 2]) -> (f64, f64, f64) {
    let rho00 = state[0].norm_sq();
    let rho11 = state[1].norm_sq();
    let rho01 = state[0] * state[1].conj();
    let x = 2.0 * rho01.re;
    let y = 2.0 * rho01.im;
    let z = rho00 - rho11;
    (x, y, z)
}

pub fn entanglement_entropy(state: &[Complex], dim_a: usize) -> f64 {
    let dim_b = state.len() / dim_a;
    let mut rho_a = vec![vec![Complex::zero(); dim_a]; dim_a];
    for i in 0..dim_a {
        for j in 0..dim_a {
            for k in 0..dim_b {
                rho_a[i][j] = rho_a[i][j] + state[i * dim_b + k] * state[j * dim_b + k].conj();
            }
        }
    }
    von_neumann_entropy(&rho_a)
}

fn complex_matmul(a: &[Vec<Complex>], b: &[Vec<Complex>]) -> Vec<Vec<Complex>> {
    let n = a.len();
    let m = b[0].len();
    let mut result = vec![vec![Complex::zero(); m]; n];
    for i in 0..n {
        for j in 0..m {
            for (k, bk) in b.iter().enumerate() {
                result[i][j] = result[i][j] + a[i][k] * bk[j];
            }
        }
    }
    result
}

fn hermitian_eigenvalues(m: &[Vec<Complex>]) -> Vec<f64> {
    let n = m.len();
    let mut a: Vec<Vec<Complex>> = m.to_vec();
    for _ in 0..200 {
        let (q, r) = qr_decompose(&a);
        a = complex_matmul(&r, &q);
    }
    (0..n).map(|i| a[i][i].re).collect()
}

fn qr_decompose(a: &[Vec<Complex>]) -> (Vec<Vec<Complex>>, Vec<Vec<Complex>>) {
    let n = a.len();
    let m = a[0].len();
    let mut q = vec![vec![Complex::zero(); n]; n];
    let mut r = vec![vec![Complex::zero(); m]; n];

    let columns: Vec<Vec<Complex>> = (0..m).map(|j| (0..n).map(|i| a[i][j]).collect()).collect();

    let mut basis: Vec<Vec<Complex>> = Vec::new();

    for j in 0..m.min(n) {
        let mut v = columns[j].clone();
        for (k, ek) in basis.iter().enumerate() {
            let mut dot = Complex::zero();
            for (&eki, &vi) in ek.iter().zip(v.iter()) {
                dot = dot + eki.conj() * vi;
            }
            r[k][j] = dot;
            for (vi, &eki) in v.iter_mut().zip(ek.iter()) {
                *vi = *vi - eki * dot;
            }
        }
        let norm: f64 = v.iter().map(|vi| vi.norm_sq()).sum::<f64>().sqrt();
        r[j][j] = Complex::new(norm, 0.0);
        if norm > 1e-15 {
            for vi in v.iter_mut() {
                *vi = *vi * Complex::new(1.0 / norm, 0.0);
            }
        }
        for (qi_row, &vi) in q.iter_mut().zip(v.iter()) {
            qi_row[j] = vi;
        }
        basis.push(v);
    }
    (q, r)
}

fn matrix_sqrt(m: &[Vec<Complex>]) -> Vec<Vec<Complex>> {
    let n = m.len();
    let mut y = m.to_vec();
    let mut z = vec![vec![Complex::zero(); n]; n];
    for (i, zi) in z.iter_mut().enumerate() {
        zi[i] = Complex::one();
    }

    for _ in 0..50 {
        let y_inv = invert_complex(&z);
        let z_inv = invert_complex(&y);
        let new_y: Vec<Vec<Complex>> = (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| (y[i][j] + y_inv[i][j]) * Complex::new(0.5, 0.0))
                    .collect()
            })
            .collect();
        let new_z: Vec<Vec<Complex>> = (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| (z[i][j] + z_inv[i][j]) * Complex::new(0.5, 0.0))
                    .collect()
            })
            .collect();
        y = new_y;
        z = new_z;
    }
    y
}

fn invert_complex(m: &[Vec<Complex>]) -> Vec<Vec<Complex>> {
    let n = m.len();
    let mut aug: Vec<Vec<Complex>> = (0..n)
        .map(|i| {
            let mut row = m[i].clone();
            row.resize(2 * n, Complex::zero());
            row[n + i] = Complex::one();
            row
        })
        .collect();

    for col in 0..n {
        let mut pivot = col;
        for row in col + 1..n {
            if aug[row][col].norm() > aug[pivot][col].norm() {
                pivot = row;
            }
        }
        aug.swap(col, pivot);
        let diag = aug[col][col];
        if diag.norm() < 1e-30 {
            continue;
        }
        let inv_diag = diag.inv();
        aug[col].iter_mut().for_each(|v| *v = *v * inv_diag);
        for row in 0..n {
            if row == col {
                continue;
            }
            let factor = aug[row][col];
            let src = aug[col].clone();
            for (d, &s) in aug[row].iter_mut().zip(&src) {
                *d = *d - factor * s;
            }
        }
    }

    (0..n).map(|i| aug[i][n..2 * n].to_vec()).collect()
}
