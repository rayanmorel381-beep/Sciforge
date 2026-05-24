pub fn gompertz_mortality_rate(a: f64, b: f64, age: f64) -> f64 {
    a * (b * age).exp()
}

pub fn gompertz_makeham(a: f64, b: f64, c: f64, age: f64) -> f64 {
    c + a * (b * age).exp()
}

pub fn weibull_mortality_hazard(lambda: f64, k: f64, age: f64) -> f64 {
    lambda * k * (lambda * age).powf(k - 1.0)
}

pub fn mortality_doubling_time(b: f64) -> f64 {
    if b <= 0.0 {
        return f64::INFINITY;
    }
    (2.0_f64).ln() / b
}

pub fn survival_probability(hazard_rates: &[f64], dt: f64) -> f64 {
    let mut s = 1.0;
    for &h in hazard_rates {
        s *= (-h * dt).exp();
    }
    s
}

pub fn life_expectancy(survival_curve: &[(f64, f64)]) -> f64 {
    let mut area = 0.0;
    for i in 1..survival_curve.len() {
        let dt = survival_curve[i].0 - survival_curve[i - 1].0;
        area += (survival_curve[i - 1].1 + survival_curve[i].1) * 0.5 * dt;
    }
    area
}

pub fn deceleration_plateau(age: f64, plateau_age: f64, plateau_rate: f64, a: f64, b: f64) -> f64 {
    if age >= plateau_age {
        plateau_rate
    } else {
        a * (b * age).exp()
    }
}

pub fn frailty_deficit_index(deficits: usize, total_items: usize) -> f64 {
    if total_items == 0 {
        return 0.0;
    }
    deficits as f64 / total_items as f64
}

pub fn phenotypic_age_levine(
    albumin: f64,
    creatinine: f64,
    glucose: f64,
    crp: f64,
    lymphocyte_pct: f64,
    mcv: f64,
    rdw: f64,
    alp: f64,
    wbc: f64,
    chronological_age: f64,
) -> f64 {
    let xb = -19.907 - 0.0336 * albumin
        + 0.0095 * creatinine
        + 0.1953 * glucose.ln()
        + 0.0954 * crp.ln().max(0.0)
        - 0.0120 * lymphocyte_pct
        + 0.0268 * mcv
        + 0.3306 * rdw
        + 0.00188 * alp
        + 0.0554 * wbc;
    let mortality_score =
        1.0 - (-(xb).exp() * ((chronological_age / 85.0).powf(0.5)).ln().exp()).exp();
    chronological_age + (mortality_score - 0.5) * 20.0
}

pub fn horvath_clock(cpg_betas: &[f64], coefficients: &[f64], intercept: f64) -> f64 {
    let n = cpg_betas.len().min(coefficients.len());
    let mut age = intercept;
    for i in 0..n {
        age += cpg_betas[i] * coefficients[i];
    }
    if age < 0.0 {
        age.exp() - 1.0
    } else {
        age + 1.0
    }
}

pub fn cr_lifespan_extension(
    baseline_lifespan: f64,
    restriction_fraction: f64,
    max_extension: f64,
) -> f64 {
    baseline_lifespan * (1.0 + max_extension * restriction_fraction.min(0.5))
}

pub fn reliability_theory_failure(
    initial_elements: usize,
    redundancy: usize,
    failure_rate: f64,
    t: f64,
) -> f64 {
    let n = initial_elements as f64;
    let r = redundancy as f64;
    let q = 1.0 - (-failure_rate * t).exp();
    1.0 - (1.0 - q.powf(r)).powf(n)
}
