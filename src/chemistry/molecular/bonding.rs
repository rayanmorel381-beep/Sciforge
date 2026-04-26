pub fn bond_order(bonding_electrons: u32, antibonding_electrons: u32) -> f64 {
    (bonding_electrons as f64 - antibonding_electrons as f64) / 2.0
}

pub fn dipole_moment(charges: &[(f64, [f64; 3])]) -> [f64; 3] {
    let mut mu = [0.0; 3];
    for &(q, r) in charges {
        mu[0] += q * r[0];
        mu[1] += q * r[1];
        mu[2] += q * r[2];
    }
    mu
}

pub fn dipole_magnitude(mu: &[f64; 3]) -> f64 {
    (mu[0] * mu[0] + mu[1] * mu[1] + mu[2] * mu[2]).sqrt()
}

pub fn coulomb_integral(z_eff: f64, n: f64) -> f64 {
    -13.6 * z_eff * z_eff / (n * n)
}

pub fn slater_shielding(electrons_below: &[u32], shielding_constants: &[f64]) -> f64 {
    electrons_below
        .iter()
        .zip(shielding_constants.iter())
        .map(|(&n, &s)| n as f64 * s)
        .sum()
}

pub fn electronegativity_mulliken(ie: f64, ea: f64) -> f64 {
    (ie + ea) / 2.0
}

pub fn formal_charge(valence: i32, lone_pair: i32, bonding: i32) -> i32 {
    valence - lone_pair - bonding / 2
}

pub fn molar_mass(atomic_masses: &[f64], counts: &[u32]) -> f64 {
    atomic_masses
        .iter()
        .zip(counts.iter())
        .map(|(&m, &c)| m * c as f64)
        .sum()
}

pub fn percent_composition(element_mass: f64, element_count: u32, molar_mass: f64) -> f64 {
    element_mass * element_count as f64 / molar_mass * 100.0
}

pub fn electronegativity_allred_rochow(z_eff: f64, r_cov: f64) -> f64 {
    0.359 * z_eff / (r_cov * r_cov).max(1e-30) + 0.744
}

pub fn polarizability_estimate(volume_angstrom3: f64) -> f64 {
    volume_angstrom3 * 4.0 * std::f64::consts::PI * crate::constants::EPSILON_0
}

pub fn bond_dissociation_energy_pauling(d_aa: f64, d_bb: f64, en_diff: f64) -> f64 {
    (d_aa * d_bb).sqrt() + 96.485 * en_diff * en_diff
}
