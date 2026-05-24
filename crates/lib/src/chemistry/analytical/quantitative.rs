pub fn dilution(c1: f64, v1: f64, v2: f64) -> f64 {
    c1 * v1 / v2.max(1e-30)
}

pub fn titration_equivalence_volume(
    c_analyte: f64,
    v_analyte: f64,
    c_titrant: f64,
    stoich_ratio: f64,
) -> f64 {
    c_analyte * v_analyte * stoich_ratio / c_titrant.max(1e-30)
}

pub fn limit_of_detection(blank_std: f64, slope: f64) -> f64 {
    3.3 * blank_std / slope.max(1e-30)
}

pub fn limit_of_quantitation(blank_std: f64, slope: f64) -> f64 {
    10.0 * blank_std / slope.max(1e-30)
}

pub fn percent_recovery(measured: f64, expected: f64) -> f64 {
    measured / expected.max(1e-30) * 100.0
}

pub fn relative_standard_deviation(std_dev: f64, mean: f64) -> f64 {
    std_dev / mean.abs().max(1e-30) * 100.0
}

pub fn gravimetric_factor(mw_analyte: f64, mw_precipitate: f64, stoich: f64) -> f64 {
    stoich * mw_analyte / mw_precipitate.max(1e-30)
}

pub fn karl_fischer_water_content(
    volume_reagent: f64,
    reagent_factor: f64,
    sample_mass: f64,
) -> f64 {
    volume_reagent * reagent_factor / sample_mass.max(1e-30) * 100.0
}
