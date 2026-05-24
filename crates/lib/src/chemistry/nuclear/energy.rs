use crate::constants::{AMU_TO_MEV, NEUTRON_MASS_AMU, PROTON_MASS_AMU};

pub fn mass_defect(z: u32, n: u32, atomic_mass: f64) -> f64 {
    z as f64 * PROTON_MASS_AMU + n as f64 * NEUTRON_MASS_AMU - atomic_mass
}

pub fn binding_energy(mass_defect: f64) -> f64 {
    mass_defect * AMU_TO_MEV
}

pub fn binding_energy_per_nucleon(binding_energy: f64, a: u32) -> f64 {
    binding_energy / a.max(1) as f64
}

pub fn q_value(reactant_masses: &[f64], product_masses: &[f64]) -> f64 {
    let sum_r: f64 = reactant_masses.iter().sum();
    let sum_p: f64 = product_masses.iter().sum();
    (sum_r - sum_p) * AMU_TO_MEV
}

pub fn semi_empirical_mass_formula(
    a: u32,
    z: u32,
    av: f64,
    as_: f64,
    ac: f64,
    aa: f64,
    ap: f64,
) -> f64 {
    let af = a as f64;
    let zf = z as f64;
    let nf = af - zf;
    let volume = av * af;
    let surface = -as_ * af.powf(2.0 / 3.0);
    let coulomb = -ac * zf * (zf - 1.0) / af.powf(1.0 / 3.0);
    let asymmetry = -aa * (nf - zf).powi(2) / af;
    let pairing = if !a.is_multiple_of(2) {
        0.0
    } else if z.is_multiple_of(2) {
        ap / af.powf(0.5)
    } else {
        -ap / af.powf(0.5)
    };
    volume + surface + coulomb + asymmetry + pairing
}

pub fn threshold_energy(q: f64, mass_projectile: f64, mass_target: f64) -> f64 {
    if q >= 0.0 {
        return 0.0;
    }
    -q * (1.0 + mass_projectile / mass_target)
}
