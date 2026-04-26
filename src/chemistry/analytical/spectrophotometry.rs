pub fn beer_lambert(epsilon: f64, path_length: f64, concentration: f64) -> f64 {
    epsilon * path_length * concentration
}

pub fn absorbance_to_transmittance(absorbance: f64) -> f64 {
    10.0_f64.powf(-absorbance)
}

pub fn transmittance_to_absorbance(transmittance: f64) -> f64 {
    -transmittance.max(1e-30).log10()
}

pub fn concentration_from_absorbance(absorbance: f64, epsilon: f64, path_length: f64) -> f64 {
    absorbance / (epsilon * path_length).max(1e-30)
}

pub fn signal_to_noise(signal: f64, noise: f64) -> f64 {
    signal / noise.max(1e-30)
}
