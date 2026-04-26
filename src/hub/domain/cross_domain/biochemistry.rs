//! Biochemistry: Michaelis-Menten kinetics, Nernst equation,
//! Henderson-Hasselbalch equilibria, and enzyme thermodynamics.

use crate::constants::physics::{FARADAY, R_GAS};

pub fn michaelis_menten_rate(vmax: f64, substrate: f64, km: f64) -> f64 {
    vmax * substrate / (km + substrate)
}

pub fn henderson_hasselbalch(pka: f64, base_conc: f64, acid_conc: f64) -> f64 {
    pka + (base_conc / acid_conc).log10()
}

pub fn gibbs_free_energy(delta_h: f64, delta_s: f64, temperature: f64) -> f64 {
    delta_h - temperature * delta_s
}

pub fn nernst_potential(z: f64, c_out: f64, c_in: f64, temperature: f64) -> f64 {
    (R_GAS * temperature / (z * FARADAY)) * (c_out / c_in).ln()
}

pub fn enzyme_turnover_number(vmax: f64, enzyme_conc: f64) -> f64 {
    vmax / enzyme_conc
}

pub fn competitive_inhibition_rate(
    vmax: f64,
    substrate: f64,
    km: f64,
    inhibitor: f64,
    ki: f64,
) -> f64 {
    vmax * substrate / (km * (1.0 + inhibitor / ki) + substrate)
}

pub fn osmotic_pressure(molarity: f64, temperature: f64, i_factor: f64) -> f64 {
    i_factor * molarity * R_GAS * temperature
}

pub fn arrhenius_rate(prefactor: f64, activation_energy: f64, temperature: f64) -> f64 {
    prefactor * (-activation_energy / (R_GAS * temperature)).exp()
}

pub fn binding_free_energy(kd: f64, temperature: f64) -> f64 {
    R_GAS * temperature * kd.ln()
}

pub fn cooperativity_hill(substrate: f64, k_half: f64, n_hill: f64) -> f64 {
    substrate.powf(n_hill) / (k_half.powf(n_hill) + substrate.powf(n_hill))
}

pub fn ph_from_concentration(h_concentration: f64) -> f64 {
    -h_concentration.log10()
}
