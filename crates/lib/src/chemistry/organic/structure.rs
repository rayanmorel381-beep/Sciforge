pub fn degree_of_unsaturation(c: u32, h: u32, n: u32, halogens: u32) -> f64 {
    (2.0 * c as f64 + 2.0 + n as f64 - h as f64 - halogens as f64) / 2.0
}

pub fn molecular_formula_mass(c: u32, h: u32, o: u32, n: u32, s: u32) -> f64 {
    c as f64 * 12.011 + h as f64 * 1.008 + o as f64 * 15.999 + n as f64 * 14.007 + s as f64 * 32.06
}

pub fn alkane_boiling_point_estimate(carbon_count: u32) -> f64 {
    if carbon_count == 0 {
        return -161.5;
    }
    -161.5 + 30.0 * (carbon_count as f64 - 1.0) - 0.3 * (carbon_count as f64 - 1.0).powi(2)
}

pub fn huckel_energy_linear(n_atoms: usize, alpha: f64, beta: f64) -> Vec<f64> {
    (0..n_atoms)
        .map(|k| {
            alpha
                + 2.0 * beta * (std::f64::consts::PI * (k + 1) as f64 / (n_atoms + 1) as f64).cos()
        })
        .collect()
}

pub fn huckel_energy_cyclic(n_atoms: usize, alpha: f64, beta: f64) -> Vec<f64> {
    (0..n_atoms)
        .map(|k| {
            alpha + 2.0 * beta * (2.0 * std::f64::consts::PI * k as f64 / n_atoms as f64).cos()
        })
        .collect()
}

pub fn delocalization_energy(total_pi_energy: f64, isolated_energy: f64) -> f64 {
    total_pi_energy - isolated_energy
}

pub fn resonance_stabilization(n_structures: usize) -> f64 {
    if n_structures <= 1 {
        return 0.0;
    }
    (n_structures as f64).ln() * 10.0
}
