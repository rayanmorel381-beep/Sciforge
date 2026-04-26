pub fn photolysis_rate(quantum_yield: f64, absorption_cross_section: f64, flux: f64) -> f64 {
    quantum_yield * absorption_cross_section * flux
}

pub fn stern_volmer(i0: f64, ksv: f64, quencher: f64) -> f64 {
    i0 / (1.0 + ksv * quencher)
}

pub fn stern_volmer_ratio(ksv: f64, quencher: f64) -> f64 {
    1.0 + ksv * quencher
}

pub fn rate_intersystem_crossing(
    total_rate: f64,
    rate_fluorescence: f64,
    rate_internal_conversion: f64,
) -> f64 {
    total_rate - rate_fluorescence - rate_internal_conversion
}

pub fn phosphorescence_lifetime(rate_phosphorescence: f64, rate_non_radiative: f64) -> f64 {
    1.0 / (rate_phosphorescence + rate_non_radiative).max(1e-30)
}

pub fn forster_radius(
    quantum_yield_donor: f64,
    kappa_sq: f64,
    overlap_integral: f64,
    n_refraction: f64,
) -> f64 {
    (8.79e-25 * quantum_yield_donor * kappa_sq * overlap_integral / n_refraction.powi(4).max(1e-30))
        .powf(1.0 / 6.0)
}

pub fn fret_efficiency(r: f64, r0: f64) -> f64 {
    let r0_6 = r0.powi(6);
    r0_6 / (r0_6 + r.powi(6)).max(1e-30)
}
