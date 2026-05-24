use sciforge_hub::prelude::maths::complex::Complex;
use sciforge_hub::prelude::physics::quantum as q;

pub fn bell_phi_plus() -> Vec<Complex> {
    q::bell_phi_plus()
}

pub fn bell_phi_minus() -> Vec<Complex> {
    q::bell_phi_minus()
}

pub fn bell_psi_plus() -> Vec<Complex> {
    q::bell_psi_plus()
}

pub fn bell_psi_minus() -> Vec<Complex> {
    q::bell_psi_minus()
}

pub fn concurrence(rho: &[Vec<Complex>]) -> f64 {
    q::concurrence_2qubit(rho)
}

pub fn fidelity_pure(psi: &[Complex], phi: &[Complex]) -> f64 {
    q::fidelity_pure(psi, phi)
}

pub fn fidelity_mixed(rho: &[Vec<Complex>], sigma: &[Vec<Complex>]) -> f64 {
    q::fidelity_mixed(rho, sigma)
}

pub fn von_neumann_entropy(rho: &[Vec<Complex>]) -> f64 {
    q::von_neumann_entropy(rho)
}

pub fn purity(rho: &[Vec<Complex>]) -> f64 {
    q::purity(rho)
}

pub fn entanglement_entropy(state: &[Complex], dim_a: usize) -> f64 {
    q::entanglement_entropy(state, dim_a)
}

pub fn density_matrix(state: &[Complex]) -> Vec<Vec<Complex>> {
    q::density_matrix(state)
}

pub fn partial_trace_b(
    rho: &[Vec<Complex>],
    dim_a: usize,
    dim_b: usize,
) -> Vec<Vec<Complex>> {
    q::partial_trace_b(rho, dim_a, dim_b)
}
