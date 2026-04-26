use crate::constants::{
    BOHR_MAGNETON, BOHR_RADIUS, COMPTON_WAVELENGTH, E_CHARGE, ELECTRON_MASS_KG, EPSILON_0, HBAR,
    MUON_MASS, NEUTRON_MASS_KG, NUCLEAR_MAGNETON, PROTON_MASS_KG, R_INF, RYDBERG_ENERGY, TAU_MASS,
};

pub fn hydrogen_energy(n: u32) -> f64 {
    let n = n as f64;
    -ELECTRON_MASS_KG * E_CHARGE.powi(4)
        / (2.0 * HBAR * HBAR * (4.0 * std::f64::consts::PI * EPSILON_0).powi(2) * n * n)
}

pub fn hydrogen_radial_r10(r: f64) -> f64 {
    let a0 = HBAR * HBAR * 4.0 * std::f64::consts::PI * EPSILON_0
        / (ELECTRON_MASS_KG * E_CHARGE * E_CHARGE);
    2.0 * (1.0 / a0).powf(1.5) * (-r / a0).exp()
}

pub fn hydrogen_radial_r20(r: f64) -> f64 {
    let a0 = HBAR * HBAR * 4.0 * std::f64::consts::PI * EPSILON_0
        / (ELECTRON_MASS_KG * E_CHARGE * E_CHARGE);
    (1.0 / (2.0 * 2.0_f64.sqrt())) * (1.0 / a0).powf(1.5) * (2.0 - r / a0) * (-r / (2.0 * a0)).exp()
}

pub fn hydrogen_radial_r21(r: f64) -> f64 {
    let a0 = HBAR * HBAR * 4.0 * std::f64::consts::PI * EPSILON_0
        / (ELECTRON_MASS_KG * E_CHARGE * E_CHARGE);
    (1.0 / (2.0 * 6.0_f64.sqrt())) * (1.0 / a0).powf(1.5) * (r / a0) * (-r / (2.0 * a0)).exp()
}

pub fn harmonic_oscillator_energy(n: u32, omega: f64) -> f64 {
    HBAR * omega * (n as f64 + 0.5)
}

pub fn harmonic_oscillator_wf(n: u32, x: f64, mass: f64, omega: f64) -> f64 {
    let alpha = mass * omega / HBAR;
    let xi = alpha.sqrt() * x;
    let norm = ((alpha / std::f64::consts::PI).sqrt()
        / (2.0_f64.powi(n as i32) * factorial(n) as f64))
        .sqrt();
    norm * hermite_val(n, xi) * (-xi * xi / 2.0).exp()
}

fn hermite_val(n: u32, x: f64) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if n == 1 {
        return 2.0 * x;
    }
    let mut h_prev2 = 1.0;
    let mut h_prev1 = 2.0 * x;
    for k in 2..=n {
        let h = 2.0 * x * h_prev1 - 2.0 * (k as f64 - 1.0) * h_prev2;
        h_prev2 = h_prev1;
        h_prev1 = h;
    }
    h_prev1
}

fn factorial(n: u32) -> u64 {
    (1..=n as u64).product()
}

pub fn infinite_well_energy(n: u32, length: f64, mass: f64) -> f64 {
    let n = n as f64;
    n * n * std::f64::consts::PI * std::f64::consts::PI * HBAR * HBAR
        / (2.0 * mass * length * length)
}

pub fn infinite_well_wf(n: u32, x: f64, length: f64) -> f64 {
    (2.0 / length).sqrt() * (n as f64 * std::f64::consts::PI * x / length).sin()
}

pub fn tunneling_coefficient(energy: f64, v0: f64, width: f64, mass: f64) -> f64 {
    if energy >= v0 {
        return 1.0;
    }
    let kappa = (2.0 * mass * (v0 - energy)).sqrt() / HBAR;
    let sinh_val = (kappa * width).sinh();
    1.0 / (1.0 + v0 * v0 * sinh_val * sinh_val / (4.0 * energy * (v0 - energy)))
}

pub fn bohr_radius() -> f64 {
    HBAR * HBAR * 4.0 * std::f64::consts::PI * EPSILON_0 / (ELECTRON_MASS_KG * E_CHARGE * E_CHARGE)
}

pub fn landau_levels(n: u32, b_field: f64, mass: f64, charge: f64) -> f64 {
    let omega_c = charge.abs() * b_field / mass;
    HBAR * omega_c * (n as f64 + 0.5)
}

pub fn zeeman_splitting(m_l: i32, b_field: f64) -> f64 {
    use crate::constants::BOHR_MAGNETON;
    m_l as f64 * BOHR_MAGNETON * b_field
}

pub fn bohr_radius_nth(n: u32, z_eff: f64) -> f64 {
    BOHR_RADIUS * (n as f64) * (n as f64) / z_eff
}

pub fn hydrogen_energy_level(n: u32) -> f64 {
    -RYDBERG_ENERGY / (n as f64 * n as f64)
}

pub fn hydrogen_energy_level_z(n: u32, z: f64) -> f64 {
    -RYDBERG_ENERGY * z * z / (n as f64 * n as f64)
}

pub fn rydberg_wavelength(n1: u32, n2: u32) -> f64 {
    let inv = R_INF * ((1.0 / (n1 as f64).powi(2)) - (1.0 / (n2 as f64).powi(2))).abs();
    1.0 / inv
}

pub fn compton_wavelength_shift(theta: f64) -> f64 {
    COMPTON_WAVELENGTH * (1.0 - theta.cos())
}

pub fn cyclotron_frequency(b_field: f64) -> f64 {
    E_CHARGE * b_field / ELECTRON_MASS_KG
}

pub fn cyclotron_frequency_particle(charge: f64, mass: f64, b_field: f64) -> f64 {
    charge.abs() * b_field / mass
}

pub fn larmor_radius(v_perp: f64, b_field: f64) -> f64 {
    ELECTRON_MASS_KG * v_perp / (E_CHARGE * b_field)
}

pub fn larmor_radius_particle(mass: f64, v_perp: f64, charge: f64, b_field: f64) -> f64 {
    mass * v_perp / (charge.abs() * b_field)
}

pub fn nuclear_zeeman_splitting(m_i: f64, b_field: f64) -> f64 {
    m_i * NUCLEAR_MAGNETON * b_field
}

pub fn anomalous_zeeman_splitting(m_j: f64, g_j: f64, b_field: f64) -> f64 {
    m_j * g_j * BOHR_MAGNETON * b_field
}

pub fn muonic_hydrogen_energy(n: u32) -> f64 {
    let reduced_mass = MUON_MASS * PROTON_MASS_KG / (MUON_MASS + PROTON_MASS_KG);
    let a0_mu =
        HBAR * HBAR * 4.0 * std::f64::consts::PI * EPSILON_0 / (reduced_mass * E_CHARGE * E_CHARGE);
    let n_f = n as f64;
    -E_CHARGE * E_CHARGE / (8.0 * std::f64::consts::PI * EPSILON_0 * a0_mu * n_f * n_f)
}

pub fn muonic_bohr_radius() -> f64 {
    let reduced_mass = MUON_MASS * PROTON_MASS_KG / (MUON_MASS + PROTON_MASS_KG);
    BOHR_RADIUS * ELECTRON_MASS_KG / reduced_mass
}

pub fn tau_lepton_mass() -> f64 {
    TAU_MASS
}

pub fn muon_mass_kg() -> f64 {
    MUON_MASS
}

pub fn proton_gyromagnetic_ratio(b_field: f64) -> f64 {
    E_CHARGE * b_field / (2.0 * PROTON_MASS_KG)
}

pub fn neutron_mass() -> f64 {
    NEUTRON_MASS_KG
}

pub fn proton_mass() -> f64 {
    PROTON_MASS_KG
}

pub fn reduced_mass(m1: f64, m2: f64) -> f64 {
    m1 * m2 / (m1 + m2)
}

pub fn de_broglie_wavelength(mass: f64, velocity: f64) -> f64 {
    HBAR * 2.0 * std::f64::consts::PI / (mass * velocity)
}

pub fn magnetic_moment_orbital(m_l: i32) -> f64 {
    -(m_l as f64) * BOHR_MAGNETON
}

pub fn fine_structure_splitting(n: u32, j: f64, z: f64) -> f64 {
    let alpha = crate::constants::ALPHA_FINE;
    RYDBERG_ENERGY * z.powi(4) * alpha * alpha / (n as f64).powi(3)
        * (1.0 / (j + 0.5) - 3.0 / (4.0 * n as f64))
}

pub fn hyperfine_splitting_hydrogen() -> f64 {
    let alpha = crate::constants::ALPHA_FINE;
    let me_mp = ELECTRON_MASS_KG / PROTON_MASS_KG;
    (4.0 / 3.0) * alpha.powi(4) * me_mp * RYDBERG_ENERGY * crate::constants::EV_TO_JOULE
}
