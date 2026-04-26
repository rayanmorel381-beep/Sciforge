pub fn bcf_ratio(c_organism: f64, c_water: f64) -> f64 {
    c_organism / c_water.max(1e-30)
}

pub fn bioaccumulation_factor(c_organism: f64, c_environment: f64) -> f64 {
    c_organism / c_environment.max(1e-30)
}

pub fn biomagnification_factor(c_predator: f64, c_prey: f64) -> f64 {
    c_predator / c_prey.max(1e-30)
}

pub fn one_compartment_toxicokinetics(
    c0: f64,
    k_uptake: f64,
    k_elim: f64,
    c_exposure: f64,
    dt: f64,
    steps: usize,
) -> Vec<f64> {
    let mut result = Vec::with_capacity(steps + 1);
    let mut c = c0;
    result.push(c);
    for _ in 0..steps {
        let dc = k_uptake * c_exposure - k_elim * c;
        c = (c + dc * dt).max(0.0);
        result.push(c);
    }
    result
}

pub fn depuration_half_life(k_elim: f64) -> f64 {
    (2.0_f64).ln() / k_elim
}

pub fn toxic_units(concentration: f64, ec50: f64) -> f64 {
    concentration / ec50
}

pub fn mixture_toxicity_concentration_addition(concentrations: &[f64], ec50s: &[f64]) -> f64 {
    let n = concentrations.len().min(ec50s.len());
    let mut sum_tu = 0.0;
    for i in 0..n {
        sum_tu += concentrations[i] / ec50s[i].max(1e-30);
    }
    sum_tu
}

pub fn haber_ct_product(concentration: f64, time: f64) -> f64 {
    concentration * time
}

pub fn risk_quotient(
    predicted_environmental_concentration: f64,
    predicted_no_effect_concentration: f64,
) -> f64 {
    predicted_environmental_concentration / predicted_no_effect_concentration.max(1e-30)
}

pub fn trophic_magnification_factor(concentrations: &[f64], trophic_levels: &[f64]) -> f64 {
    let n = concentrations.len().min(trophic_levels.len());
    if n < 2 {
        return 1.0;
    }
    let log_c: Vec<f64> = concentrations.iter().map(|&c| c.max(1e-30).ln()).collect();
    let x_mean = trophic_levels.iter().take(n).sum::<f64>() / n as f64;
    let y_mean = log_c.iter().take(n).sum::<f64>() / n as f64;
    let mut num = 0.0;
    let mut den = 0.0;
    for i in 0..n {
        let dx = trophic_levels[i] - x_mean;
        num += dx * (log_c[i] - y_mean);
        den += dx * dx;
    }
    if den < 1e-30 {
        return 1.0;
    }
    (num / den).exp()
}

pub fn two_compartment_toxicokinetics(
    c_fast: f64,
    c_slow: f64,
    k12: f64,
    k21: f64,
    k_elim: f64,
    k_uptake: f64,
    c_exposure: f64,
    dt: f64,
) -> (f64, f64) {
    let dc_fast = k_uptake * c_exposure - k_elim * c_fast - k12 * c_fast + k21 * c_slow;
    let dc_slow = k12 * c_fast - k21 * c_slow;
    (
        (c_fast + dc_fast * dt).max(0.0),
        (c_slow + dc_slow * dt).max(0.0),
    )
}

pub fn steady_state_body_burden(k_uptake: f64, c_exposure: f64, k_elim: f64) -> f64 {
    k_uptake * c_exposure / k_elim
}

pub fn biota_sediment_accumulation_factor(c_organism: f64, c_sediment: f64) -> f64 {
    c_organism / c_sediment.max(1e-30)
}

pub fn lipid_normalized_concentration(c_tissue: f64, lipid_fraction: f64) -> f64 {
    c_tissue / lipid_fraction.max(1e-30)
}

pub fn fugacity_ratio(c_organism: f64, c_environment: f64, k_ow: f64) -> f64 {
    c_organism / (k_ow * c_environment).max(1e-30)
}

pub fn elimination_rate_from_depuration(c_start: f64, c_end: f64, time: f64) -> f64 {
    if time <= 0.0 {
        return 0.0;
    }
    (c_start / c_end.max(1e-30)).ln() / time
}

pub fn critical_body_residue(lc50: f64, bcf: f64) -> f64 {
    lc50 * bcf
}

pub fn dietary_uptake_efficiency(assimilation: f64, feeding_rate: f64, body_weight: f64) -> f64 {
    assimilation * feeding_rate / body_weight.max(1e-30)
}
