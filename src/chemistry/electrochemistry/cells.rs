use crate::constants::{FARADAY, R_GAS};

pub fn nernst_potential(e0: f64, n: f64, q: f64, t: f64) -> f64 {
    e0 - (R_GAS * t) / (n * FARADAY) * q.ln()
}

pub fn cell_potential(e_cathode: f64, e_anode: f64) -> f64 {
    e_cathode - e_anode
}

pub fn gibbs_from_cell_potential(n: f64, e_cell: f64) -> f64 {
    -n * FARADAY * e_cell
}

pub fn faraday_mass(i: f64, t: f64, m: f64, n: f64) -> f64 {
    i * t * m / (n * FARADAY)
}

pub fn overpotential_tafel(a: f64, b: f64, current_density: f64) -> f64 {
    a + b * current_density.abs().log10()
}

pub fn butler_volmer(i0: f64, alpha_a: f64, alpha_c: f64, eta: f64, t: f64) -> f64 {
    let f = FARADAY / (R_GAS * t);
    i0 * ((alpha_a * f * eta).exp() - (-alpha_c * f * eta).exp())
}

pub fn open_circuit_voltage(e_cathode: f64, e_anode: f64, n_electrons: f64, t: f64, q: f64) -> f64 {
    let e0 = e_cathode - e_anode;
    e0 - R_GAS * t / (n_electrons * FARADAY) * q.max(1e-30).ln()
}

pub fn faradaic_efficiency(actual_yield: f64, theoretical_yield: f64) -> f64 {
    actual_yield / theoretical_yield.max(1e-30) * 100.0
}

pub fn energy_density_battery(voltage: f64, capacity_ah: f64, mass_kg: f64) -> f64 {
    voltage * capacity_ah / mass_kg.max(1e-30)
}

pub fn coulombic_efficiency(discharge_capacity: f64, charge_capacity: f64) -> f64 {
    discharge_capacity / charge_capacity.max(1e-30) * 100.0
}
