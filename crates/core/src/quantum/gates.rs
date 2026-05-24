use sciforge_hub::prelude::maths::complex::Complex;
use sciforge_hub::prelude::physics::quantum as q;

pub fn hadamard() -> Vec<Vec<Complex>> {
    q::hadamard_gate()
}

pub fn pauli_x() -> [[Complex; 2]; 2] {
    q::pauli_x()
}

pub fn pauli_y() -> [[Complex; 2]; 2] {
    q::pauli_y()
}

pub fn pauli_z() -> [[Complex; 2]; 2] {
    q::pauli_z()
}

pub fn identity() -> [[Complex; 2]; 2] {
    q::identity_2()
}

pub fn phase(phi_rad: f64) -> Vec<Vec<Complex>> {
    q::phase_gate(phi_rad)
}

pub fn t_gate() -> Vec<Vec<Complex>> {
    q::t_gate()
}

pub fn s_gate() -> Vec<Vec<Complex>> {
    q::s_gate()
}

pub fn rx(theta_rad: f64) -> Vec<Vec<Complex>> {
    q::rx_gate(theta_rad)
}

pub fn ry(theta_rad: f64) -> Vec<Vec<Complex>> {
    q::ry_gate(theta_rad)
}

pub fn rz(theta_rad: f64) -> Vec<Vec<Complex>> {
    q::rz_gate(theta_rad)
}

pub fn cnot() -> Vec<Vec<Complex>> {
    q::cnot_gate()
}

pub fn cz() -> Vec<Vec<Complex>> {
    q::cz_gate()
}

pub fn swap() -> Vec<Vec<Complex>> {
    q::swap_gate()
}

pub fn toffoli() -> Vec<Vec<Complex>> {
    q::toffoli_gate()
}
