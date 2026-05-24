pub fn yield_offset_strain(total_strain: f64, offset: f64) -> f64 {
    total_strain - offset
}

pub fn ramberg_osgood(stress: f64, e: f64, k: f64, n: f64) -> f64 {
    stress / e + (stress / k).powf(1.0 / n)
}

pub fn true_stress(engineering_stress: f64, engineering_strain: f64) -> f64 {
    engineering_stress * (1.0 + engineering_strain)
}

pub fn true_strain(engineering_strain: f64) -> f64 {
    (1.0 + engineering_strain).ln()
}

pub fn hardening_power_law(k: f64, strain_plastic: f64, n: f64) -> f64 {
    k * strain_plastic.powf(n)
}

pub fn von_mises(s1: f64, s2: f64, s3: f64) -> f64 {
    (0.5 * ((s1 - s2).powi(2) + (s2 - s3).powi(2) + (s3 - s1).powi(2))).sqrt()
}

pub fn tresca(s1: f64, s2: f64, s3: f64) -> f64 {
    let max = s1.max(s2).max(s3);
    let min = s1.min(s2).min(s3);
    max - min
}

pub fn isotropic_hardening(yield_0: f64, h: f64, plastic_strain: f64) -> f64 {
    yield_0 + h * plastic_strain
}

pub fn bauschinger_effect(forward_yield: f64, reverse_yield: f64) -> f64 {
    (forward_yield - reverse_yield.abs()) / forward_yield
}

pub fn plastic_work(stress: &[f64], d_plastic_strain: &[f64]) -> f64 {
    stress
        .iter()
        .zip(d_plastic_strain.iter())
        .map(|(&s, &de)| s * de)
        .sum()
}

pub fn necking_criterion(n: f64) -> f64 {
    n
}

pub fn strain_rate_sensitivity(stress1: f64, stress2: f64, rate1: f64, rate2: f64) -> f64 {
    (stress2 / stress1).ln() / (rate2 / rate1).ln()
}
