use crate::maths::complex::Complex;

pub fn pauli_x() -> [[Complex; 2]; 2] {
    [
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    ]
}

pub fn pauli_y() -> [[Complex; 2]; 2] {
    [
        [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
        [Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)],
    ]
}

pub fn pauli_z() -> [[Complex; 2]; 2] {
    [
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0)],
    ]
}

pub fn identity_2() -> [[Complex; 2]; 2] {
    [
        [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ]
}

pub fn spin_up() -> [Complex; 2] {
    [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]
}

pub fn spin_down() -> [Complex; 2] {
    [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]
}

pub fn spin_plus_x() -> [Complex; 2] {
    let s = 1.0 / 2.0_f64.sqrt();
    [Complex::new(s, 0.0), Complex::new(s, 0.0)]
}

pub fn spin_minus_x() -> [Complex; 2] {
    let s = 1.0 / 2.0_f64.sqrt();
    [Complex::new(s, 0.0), Complex::new(-s, 0.0)]
}

pub fn spin_plus_y() -> [Complex; 2] {
    let s = 1.0 / 2.0_f64.sqrt();
    [Complex::new(s, 0.0), Complex::new(0.0, s)]
}

pub fn spin_minus_y() -> [Complex; 2] {
    let s = 1.0 / 2.0_f64.sqrt();
    [Complex::new(s, 0.0), Complex::new(0.0, -s)]
}

pub fn dirac_gamma0() -> [[Complex; 4]; 4] {
    let z = Complex::new(0.0, 0.0);
    let p = Complex::new(1.0, 0.0);
    let m = Complex::new(-1.0, 0.0);
    [[p, z, z, z], [z, p, z, z], [z, z, m, z], [z, z, z, m]]
}

pub fn dirac_gamma1() -> [[Complex; 4]; 4] {
    let z = Complex::new(0.0, 0.0);
    let p = Complex::new(1.0, 0.0);
    let m = Complex::new(-1.0, 0.0);
    [[z, z, z, p], [z, z, p, z], [z, m, z, z], [m, z, z, z]]
}

pub fn dirac_gamma2() -> [[Complex; 4]; 4] {
    let z = Complex::new(0.0, 0.0);
    let pi = Complex::new(0.0, 1.0);
    let mi = Complex::new(0.0, -1.0);
    [[z, z, z, mi], [z, z, pi, z], [z, mi, z, z], [pi, z, z, z]]
}

pub fn dirac_gamma3() -> [[Complex; 4]; 4] {
    let z = Complex::new(0.0, 0.0);
    let p = Complex::new(1.0, 0.0);
    let m = Complex::new(-1.0, 0.0);
    [[z, z, p, z], [z, z, z, m], [m, z, z, z], [z, p, z, z]]
}

pub fn gamma5() -> [[Complex; 4]; 4] {
    let z = Complex::new(0.0, 0.0);
    let p = Complex::new(1.0, 0.0);
    [[z, z, p, z], [z, z, z, p], [p, z, z, z], [z, p, z, z]]
}

pub fn rotation_operator(angle: f64, pauli: &[[Complex; 2]; 2]) -> [[Complex; 2]; 2] {
    let cos_half = (angle / 2.0).cos();
    let sin_half = (angle / 2.0).sin();
    let i = Complex::new(0.0, 1.0);
    let mut result = [[Complex::new(0.0, 0.0); 2]; 2];
    for r in 0..2 {
        for c in 0..2 {
            let id = if r == c {
                Complex::new(1.0, 0.0)
            } else {
                Complex::new(0.0, 0.0)
            };
            result[r][c] =
                id * Complex::new(cos_half, 0.0) - i * pauli[r][c] * Complex::new(sin_half, 0.0);
        }
    }
    result
}

pub fn ladder_operator_plus(j: f64) -> Vec<Vec<Complex>> {
    let dim = (2.0 * j + 1.0) as usize;
    let mut op = vec![vec![Complex::new(0.0, 0.0); dim]; dim];
    for idx in 0..dim - 1 {
        let m = -j + idx as f64;
        let val = (j * (j + 1.0) - m * (m + 1.0)).sqrt();
        op[idx + 1][idx] = Complex::new(val, 0.0);
    }
    op
}

pub fn ladder_operator_minus(j: f64) -> Vec<Vec<Complex>> {
    let dim = (2.0 * j + 1.0) as usize;
    let mut op = vec![vec![Complex::new(0.0, 0.0); dim]; dim];
    for idx in 1..dim {
        let m = -j + idx as f64;
        let val = (j * (j + 1.0) - m * (m - 1.0)).sqrt();
        op[idx - 1][idx] = Complex::new(val, 0.0);
    }
    op
}

pub fn jz_operator(j: f64) -> Vec<Vec<Complex>> {
    let dim = (2.0 * j + 1.0) as usize;
    let mut op = vec![vec![Complex::new(0.0, 0.0); dim]; dim];
    for (idx, op_row) in op.iter_mut().enumerate() {
        let m = -j + idx as f64;
        op_row[idx] = Complex::new(m, 0.0);
    }
    op
}
