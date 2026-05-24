use sciforge_hub::prelude::maths::complex::Complex;
use sciforge_hub::prelude::physics::quantum as q;

pub fn ladder_plus(j: f64) -> Vec<Vec<Complex>> {
    q::ladder_operator_plus(j)
}

pub fn ladder_minus(j: f64) -> Vec<Vec<Complex>> {
    q::ladder_operator_minus(j)
}

pub fn jz_operator(j: f64) -> Vec<Vec<Complex>> {
    q::jz_operator(j)
}

pub fn rotation(angle_rad: f64, pauli: &[[Complex; 2]; 2]) -> [[Complex; 2]; 2] {
    q::rotation_operator(angle_rad, pauli)
}

pub fn clebsch_gordan(j1: f64, m1: f64, j2: f64, m2: f64, j: f64, m: f64) -> f64 {
    q::clebsch_gordan(j1, m1, j2, m2, j, m)
}

pub fn wigner_3j(j1: f64, j2: f64, j3: f64, m1: f64, m2: f64, m3: f64) -> f64 {
    q::wigner_3j(j1, j2, j3, m1, m2, m3)
}

pub fn angular_momentum_coupling(j1: f64, j2: f64) -> Vec<(f64, f64, f64)> {
    q::angular_momentum_coupling(j1, j2)
}
