pub fn glomerular_filtration_rate(kf: f64, p_gc: f64, p_bs: f64, pi_gc: f64) -> f64 {
    kf * (p_gc - p_bs - pi_gc)
}

pub fn creatinine_clearance(urine_cr: f64, urine_volume: f64, plasma_cr: f64) -> f64 {
    (urine_cr * urine_volume) / plasma_cr
}

pub fn fractional_excretion(urine_x: f64, plasma_cr: f64, plasma_x: f64, urine_cr: f64) -> f64 {
    (urine_x * plasma_cr) / (plasma_x * urine_cr) * 100.0
}

pub fn free_water_clearance(urine_volume: f64, urine_osm: f64, plasma_osm: f64) -> f64 {
    urine_volume * (1.0 - urine_osm / plasma_osm)
}

pub fn tubular_reabsorption_rate(filtered_load: f64, excretion_rate: f64) -> f64 {
    filtered_load - excretion_rate
}

pub fn cockcroft_gault(age: f64, weight: f64, serum_cr: f64, is_female: bool) -> f64 {
    let base = (140.0 - age) * weight / (72.0 * serum_cr);
    if is_female { base * 0.85 } else { base }
}

pub fn mdrd_gfr(serum_cr: f64, age: f64, is_female: bool, is_black: bool) -> f64 {
    let mut gfr = 175.0 * serum_cr.powf(-1.154) * age.powf(-0.203);
    if is_female {
        gfr *= 0.742;
    }
    if is_black {
        gfr *= 1.212;
    }
    gfr
}

pub fn tubuloglomerular_feedback(
    nacl_macula: f64,
    nacl_target: f64,
    sensitivity: f64,
    gfr_baseline: f64,
) -> f64 {
    gfr_baseline * (1.0 - sensitivity * (nacl_macula - nacl_target))
}

pub fn urine_concentration_ratio(urine_osm: f64, plasma_osm: f64) -> f64 {
    urine_osm / plasma_osm.max(1e-30)
}

pub fn anion_gap(sodium: f64, chloride: f64, bicarbonate: f64) -> f64 {
    sodium - (chloride + bicarbonate)
}
