pub fn ld50_probit(doses: &[f64], responses: &[f64], totals: &[f64]) -> f64 {
    let n = doses.len().min(responses.len()).min(totals.len());
    if n < 2 {
        return 0.0;
    }
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_xx = 0.0;
    let mut count = 0.0;
    for i in 0..n {
        let p = responses[i] / totals[i].max(1.0);
        if p <= 0.0 || p >= 1.0 {
            continue;
        }
        let x = doses[i].ln();
        let y = probit(p);
        sum_x += x;
        sum_y += y;
        sum_xy += x * y;
        sum_xx += x * x;
        count += 1.0;
    }
    if count < 2.0 {
        return 0.0;
    }
    let slope = (count * sum_xy - sum_x * sum_y) / (count * sum_xx - sum_x * sum_x);
    let intercept = (sum_y - slope * sum_x) / count;
    if slope.abs() < 1e-30 {
        return 0.0;
    }
    ((5.0 - intercept) / slope).exp()
}

fn probit(p: f64) -> f64 {
    5.0 + rational_approx_norm_inv(p)
}

fn rational_approx_norm_inv(p: f64) -> f64 {
    let p_clamped = p.clamp(1e-10, 1.0 - 1e-10);
    let t = if p_clamped < 0.5 {
        (-2.0 * p_clamped.ln()).sqrt()
    } else {
        (-2.0 * (1.0 - p_clamped).ln()).sqrt()
    };
    let c0 = 2.515517;
    let c1 = 0.802853;
    let c2 = 0.010328;
    let d1 = 1.432788;
    let d2 = 0.189269;
    let d3 = 0.001308;
    let result = t - (c0 + c1 * t + c2 * t * t) / (1.0 + d1 * t + d2 * t * t + d3 * t * t * t);
    if p_clamped < 0.5 { -result } else { result }
}

pub fn dose_response_loglogistic(dose: f64, ec50: f64, slope: f64, bottom: f64, top: f64) -> f64 {
    bottom + (top - bottom) / (1.0 + (dose / ec50).powf(-slope))
}

pub fn therapeutic_window(ec50: f64, td50: f64) -> f64 {
    td50 / ec50
}

pub fn margin_of_safety(td01: f64, ed99: f64) -> f64 {
    td01 / ed99
}

pub fn benchmark_dose(ec_target: f64, ec50: f64, slope: f64) -> f64 {
    ec50 * (ec_target / (1.0 - ec_target)).powf(1.0 / slope)
}

pub fn haber_rule(concentration: f64, time: f64, n: f64) -> f64 {
    concentration.powf(n) * time
}

pub fn bioconcentration_factor(concentration_organism: f64, concentration_water: f64) -> f64 {
    concentration_organism / concentration_water.max(1e-30)
}

pub fn reference_dose(noael: f64, uncertainty_factors: &[f64]) -> f64 {
    let total_uf: f64 = uncertainty_factors.iter().product();
    noael / total_uf.max(1.0)
}

pub fn hormesis_model(
    dose: f64,
    max_stimulation: f64,
    ec50_stimulation: f64,
    ec50_inhibition: f64,
    slope: f64,
) -> f64 {
    let stimulation = max_stimulation * dose / (ec50_stimulation + dose);
    let inhibition = dose.powf(slope) / (ec50_inhibition.powf(slope) + dose.powf(slope));
    stimulation * (1.0 - inhibition)
}

pub fn weibull_dose_response(dose: f64, alpha: f64, beta: f64) -> f64 {
    1.0 - (-alpha * dose.powf(beta)).exp()
}

pub fn multistage_cancer_model(dose: f64, coefficients: &[f64]) -> f64 {
    let mut exponent = 0.0;
    for (i, &c) in coefficients.iter().enumerate() {
        exponent += c * dose.powi(i as i32 + 1);
    }
    1.0 - (-exponent).exp()
}

pub fn safety_factor_method(noael: f64, interspecies: f64, intraspecies: f64) -> f64 {
    noael / (interspecies * intraspecies)
}

pub fn acute_toxicity_ratio(lc50_48h: f64, environmental_conc: f64) -> f64 {
    lc50_48h / environmental_conc.max(1e-30)
}

pub fn species_sensitivity_distribution(log_hc5: f64, sigma: f64, z_05: f64) -> f64 {
    (log_hc5 - z_05 * sigma).exp()
}

pub fn no_observed_adverse_effect_concentration(
    responses: &[f64],
    controls: &[f64],
) -> Option<usize> {
    if controls.is_empty() {
        return None;
    }
    let control_mean = controls.iter().sum::<f64>() / controls.len() as f64;
    let control_sd = (controls
        .iter()
        .map(|&x| (x - control_mean).powi(2))
        .sum::<f64>()
        / controls.len() as f64)
        .sqrt();
    let threshold = control_mean + 2.0 * control_sd;
    responses.iter().rposition(|&r| r <= threshold)
}

pub fn dose_addition_mixture(concentrations: &[f64], ec50s: &[f64]) -> f64 {
    let n = concentrations.len().min(ec50s.len());
    let sum: f64 = (0..n)
        .map(|i| concentrations[i] / ec50s[i].max(1e-30))
        .sum();
    sum
}

pub fn independent_action_mixture(concentrations: &[f64], ec50s: &[f64], slopes: &[f64]) -> f64 {
    let n = concentrations.len().min(ec50s.len()).min(slopes.len());
    let mut survival = 1.0;
    for i in 0..n {
        let effect = 1.0 / (1.0 + (ec50s[i] / concentrations[i].max(1e-30)).powf(slopes[i]));
        survival *= 1.0 - effect;
    }
    1.0 - survival
}
