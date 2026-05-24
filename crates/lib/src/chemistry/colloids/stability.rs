pub fn dlvo_total_energy(van_der_waals: f64, electrostatic: f64) -> f64 {
    van_der_waals + electrostatic
}

pub fn hamaker_sphere_sphere(a_h: f64, r1: f64, r2: f64, d: f64) -> f64 {
    -a_h / 6.0 * r1 * r2 / ((r1 + r2) * d).max(1e-30)
}

pub fn hamaker_sphere_surface(a_h: f64, r: f64, d: f64) -> f64 {
    -a_h * r / (6.0 * d).max(1e-30)
}

pub fn debye_length(epsilon_r: f64, t: f64, ionic_strength: f64) -> f64 {
    use crate::constants::{E_CHARGE, EPSILON_0, K_B, N_A};
    (epsilon_r * EPSILON_0 * K_B * t
        / (2.0 * N_A * E_CHARGE * E_CHARGE * ionic_strength).max(1e-30))
    .sqrt()
}

pub fn electrostatic_repulsion(epsilon_r: f64, r: f64, psi0: f64, kappa: f64, d: f64) -> f64 {
    2.0 * std::f64::consts::PI
        * epsilon_r
        * crate::constants::EPSILON_0
        * r
        * psi0
        * psi0
        * (-kappa * d).exp()
}

pub fn zeta_potential_smoluchowski(mobility: f64, viscosity: f64, epsilon: f64) -> f64 {
    mobility * viscosity / epsilon.max(1e-30)
}

pub fn schulze_hardy_ccc(z: i32) -> f64 {
    1.0 / (z.abs() as f64).powi(6).max(1e-30)
}

pub fn critical_coagulation_concentration(
    epsilon: f64,
    t: f64,
    psi0: f64,
    a_h: f64,
    z: f64,
) -> f64 {
    let factor = epsilon.powi(3) * (crate::constants::K_B * t).powi(5);
    let denom = a_h.powi(2) * (crate::constants::E_CHARGE * z).powi(6);
    factor / denom.max(1e-30) * psi0.powi(4)
}
