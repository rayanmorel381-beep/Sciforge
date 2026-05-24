pub fn linear_quadratic_survival(dose: f64, alpha: f64, beta: f64) -> f64 {
    (-alpha * dose - beta * dose * dose).exp()
}

pub fn biologically_effective_dose(n: f64, d: f64, alpha_beta: f64) -> f64 {
    n * d * (1.0 + d / alpha_beta)
}

pub fn equivalent_dose_2gy(bed: f64, alpha_beta: f64) -> f64 {
    bed / (1.0 + 2.0 / alpha_beta)
}

pub fn tcp(n_cells: f64, survival_fraction: f64) -> f64 {
    (-n_cells * survival_fraction).exp()
}

pub fn ntcp_lyman(dose: f64, td50: f64, m: f64) -> f64 {
    let t = (dose - td50) / (m * td50);
    0.5 * (1.0 + erf_approx(t / std::f64::consts::SQRT_2))
}

fn erf_approx(x: f64) -> f64 {
    let sign = if x >= 0.0 { 1.0 } else { -1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let poly = t
        * (0.254829592
            + t * (-0.284496736 + t * (1.421413741 + t * (-1.453152027 + t * 1.061405429))));
    sign * (1.0 - poly * (-x * x).exp())
}

pub fn oxygen_enhancement_ratio(dose_hypoxic: f64, dose_oxic: f64) -> f64 {
    dose_hypoxic / dose_oxic
}

pub fn dna_dsb_yield(dose: f64, yield_per_gray: f64) -> f64 {
    dose * yield_per_gray
}

pub fn repair_kinetics(dsb0: f64, repair_rate: f64, t: f64) -> f64 {
    dsb0 * (-repair_rate * t).exp()
}

pub fn fractionation_survival(
    n_fractions: usize,
    dose_per_fraction: f64,
    alpha: f64,
    beta: f64,
    repair_factor: f64,
) -> f64 {
    let sf_per_fraction = linear_quadratic_survival(dose_per_fraction, alpha, beta * repair_factor);
    sf_per_fraction.powi(n_fractions as i32)
}

pub fn relative_biological_effectiveness(dose_ref: f64, dose_test: f64) -> f64 {
    dose_ref / dose_test
}

pub fn let_to_rbe(let_val: f64, rbe_max: f64, let_half: f64) -> f64 {
    rbe_max * let_val / (let_half + let_val)
}

pub fn protraction_factor(dose_rate: f64, repair_half_time: f64, total_time: f64) -> f64 {
    let mu = std::f64::consts::LN_2 / repair_half_time;
    let total_dose = dose_rate * total_time;
    let x = mu * total_time;
    if x < 1e-6 {
        return 1.0;
    }
    let g = 2.0 * (x - 1.0 + (-x).exp()) / (x * x);
    if total_dose > 0.0 { g } else { 1.0 }
}

pub fn bystander_effect(dose: f64, max_effect: f64, dose_half: f64) -> f64 {
    max_effect * (1.0 - (-dose / dose_half).exp())
}

pub fn adaptive_response(
    priming_dose: f64,
    challenge_dose: f64,
    alpha: f64,
    beta: f64,
    reduction_factor: f64,
) -> f64 {
    let sf_unprimed = linear_quadratic_survival(challenge_dose, alpha, beta);
    let sf_primed = linear_quadratic_survival(
        challenge_dose,
        alpha * reduction_factor,
        beta * reduction_factor,
    );
    if priming_dose > 0.0 {
        sf_primed
    } else {
        sf_unprimed
    }
}

pub fn low_dose_hypersensitivity(dose: f64, alpha_r: f64, alpha_s: f64, dc: f64, beta: f64) -> f64 {
    let alpha_eff = alpha_r + (alpha_s - alpha_r) * (-dose / dc).exp();
    (-alpha_eff * dose - beta * dose * dose).exp()
}

pub fn tumor_growth_delay(dose: f64, alpha: f64, beta: f64, doubling_time: f64) -> f64 {
    let sf = linear_quadratic_survival(dose, alpha, beta);
    -sf.ln() * doubling_time / std::f64::consts::LN_2
}

pub fn complication_free_cure(tcp_val: f64, ntcp_val: f64) -> f64 {
    tcp_val * (1.0 - ntcp_val)
}

pub fn isoeffect_dose(n1: f64, d1: f64, alpha_beta: f64, n2: f64) -> f64 {
    let bed = n1 * d1 * (1.0 + d1 / alpha_beta);
    let disc = alpha_beta * alpha_beta + 4.0 * alpha_beta * bed / n2;
    (-alpha_beta + disc.sqrt()) / 2.0
}
