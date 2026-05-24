pub fn chemical_shift_ppm(
    frequency_sample: f64,
    frequency_reference: f64,
    spectrometer: f64,
) -> f64 {
    (frequency_sample - frequency_reference) / spectrometer.max(1e-30) * 1e6
}

pub fn coupling_constant_first_order(line_separation_hz: f64) -> f64 {
    line_separation_hz
}

pub fn multiplicity(n_neighbors: u32) -> u32 {
    n_neighbors + 1
}

pub fn larmor_frequency(gamma: f64, b0: f64) -> f64 {
    gamma * b0 / (2.0 * std::f64::consts::PI)
}

pub fn t1_inversion_recovery(m0: f64, t1: f64, tau: f64) -> f64 {
    m0 * (1.0 - 2.0 * (-tau / t1.max(1e-30)).exp())
}

pub fn t2_spin_echo(m0: f64, t2: f64, tau: f64) -> f64 {
    m0 * (-tau / t2.max(1e-30)).exp()
}

pub fn linewidth_from_t2(t2: f64) -> f64 {
    1.0 / (std::f64::consts::PI * t2.max(1e-30))
}

pub fn nuclear_overhauser_enhancement(gamma_irradiated: f64, gamma_observed: f64) -> f64 {
    1.0 + gamma_irradiated / (2.0 * gamma_observed.abs().max(1e-30))
}
