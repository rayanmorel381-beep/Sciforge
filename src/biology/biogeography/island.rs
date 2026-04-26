pub fn island_species_area(c: f64, z: f64, area: f64) -> f64 {
    c * area.powf(z)
}

pub fn island_immigration_rate(s: f64, p: f64, i_max: f64) -> f64 {
    i_max * (1.0 - s / p)
}

pub fn island_extinction_rate(s: f64, e_max: f64) -> f64 {
    e_max * s
}

pub fn macarthur_wilson_equilibrium(i_max: f64, e_max: f64, p: f64) -> f64 {
    i_max * p / (i_max + e_max)
}

pub fn macarthur_wilson_turnover(i_max: f64, e_max: f64, p: f64) -> f64 {
    let s_eq = i_max * p / (i_max + e_max);
    i_max * (1.0 - s_eq / p)
}

pub fn distance_decay(similarity_0: f64, decay_rate: f64, distance: f64) -> f64 {
    similarity_0 * (-decay_rate * distance).exp()
}

pub fn rescue_effect(extinction_base: f64, immigration: f64) -> f64 {
    extinction_base / (1.0 + immigration).max(1e-30)
}

pub fn target_effect(immigration_base: f64, area: f64, area_ref: f64) -> f64 {
    immigration_base * (area / area_ref).sqrt()
}

pub fn species_isolation_index(distances: &[f64]) -> f64 {
    if distances.is_empty() {
        return f64::INFINITY;
    }
    let sum: f64 = distances.iter().map(|&d| (-d).exp()).sum();
    -(sum / distances.len() as f64).ln()
}

pub fn area_effect_on_extinction(e_base: f64, area: f64, z: f64) -> f64 {
    e_base * area.powf(-z)
}

pub fn habitat_diversity(area: f64, k: f64) -> f64 {
    k * area.ln().max(0.0)
}
