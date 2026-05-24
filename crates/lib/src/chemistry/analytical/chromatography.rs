pub fn retention_factor_rf(distance_solute: f64, distance_solvent: f64) -> f64 {
    distance_solute / distance_solvent.max(1e-30)
}

pub fn hplc_resolution(tr1: f64, tr2: f64, w1: f64, w2: f64) -> f64 {
    2.0 * (tr2 - tr1) / (w1 + w2).max(1e-30)
}

pub fn theoretical_plates(tr: f64, w: f64) -> f64 {
    16.0 * (tr / w.max(1e-30)).powi(2)
}

pub fn height_equivalent_theoretical_plate(column_length: f64, n_plates: f64) -> f64 {
    column_length / n_plates.max(1e-30)
}

pub fn van_deemter(a: f64, b: f64, c: f64, u: f64) -> f64 {
    a + b / u.max(1e-30) + c * u
}

pub fn selectivity_factor(k2: f64, k1: f64) -> f64 {
    k2 / k1.max(1e-30)
}

pub fn capacity_factor(tr: f64, t0: f64) -> f64 {
    (tr - t0) / t0.max(1e-30)
}
