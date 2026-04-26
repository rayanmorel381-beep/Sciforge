pub fn gompertz_mortality(a: f64, b: f64, age: f64) -> f64 {
    a * (b * age).exp()
}

pub fn gompertz_survival(a: f64, b: f64, age: f64) -> f64 {
    (-(a / b) * ((b * age).exp() - 1.0)).exp()
}

pub fn weibull_mortality(lambda: f64, k: f64, age: f64) -> f64 {
    lambda * k * (lambda * age).powf(k - 1.0)
}

pub fn weibull_survival(lambda: f64, k: f64, age: f64) -> f64 {
    (-(lambda * age).powf(k)).exp()
}

pub fn gompertz_makeham_mortality(a: f64, b: f64, c: f64, age: f64) -> f64 {
    c + a * (b * age).exp()
}

pub fn mortality_rate_doubling_time(b: f64) -> f64 {
    (2.0_f64).ln() / b
}

pub fn life_expectancy_from_survival(survival: impl Fn(f64) -> f64, max_age: f64, dt: f64) -> f64 {
    let steps = (max_age / dt) as usize;
    let mut integral = 0.0;
    for i in 0..steps {
        let t = i as f64 * dt;
        integral += survival(t) * dt;
    }
    integral
}

pub fn siler_mortality(a1: f64, b1: f64, a2: f64, a3: f64, b3: f64, age: f64) -> f64 {
    a1 * (-b1 * age).exp() + a2 + a3 * (b3 * age).exp()
}

pub fn logistic_mortality_plateau(a: f64, b: f64, c: f64, age: f64) -> f64 {
    a * (b * age).exp() / (1.0 + c * a / b * ((b * age).exp() - 1.0))
}

pub fn demographic_entropy(life_table_lx: &[f64]) -> f64 {
    let e0: f64 = life_table_lx.iter().sum::<f64>();
    if e0 <= 0.0 {
        return 0.0;
    }
    life_table_lx
        .iter()
        .filter(|&&lx| lx > 0.0)
        .map(|&lx| lx * (lx).ln())
        .sum::<f64>()
        / -e0
}

pub fn net_reproduction_rate(survivorship: &[f64], fecundity: &[f64]) -> f64 {
    survivorship
        .iter()
        .zip(fecundity.iter())
        .map(|(&l, &m)| l * m)
        .sum()
}

pub fn generation_time(survivorship: &[f64], fecundity: &[f64]) -> f64 {
    let r0: f64 = survivorship
        .iter()
        .zip(fecundity.iter())
        .map(|(&l, &m)| l * m)
        .sum();
    if r0 <= 0.0 {
        return 0.0;
    }
    let numerator: f64 = survivorship
        .iter()
        .zip(fecundity.iter())
        .enumerate()
        .map(|(x, (&l, &m))| x as f64 * l * m)
        .sum();
    numerator / r0
}

pub fn actuarial_senescence_rate(
    mortality_young: f64,
    mortality_old: f64,
    age_interval: f64,
) -> f64 {
    ((mortality_old / mortality_young).ln()) / age_interval
}

pub fn proportional_hazards(baseline_hazard: f64, covariates: &[f64], coefficients: &[f64]) -> f64 {
    let linear_predictor: f64 = covariates
        .iter()
        .zip(coefficients.iter())
        .map(|(&x, &b)| x * b)
        .sum();
    baseline_hazard * linear_predictor.exp()
}

pub fn biological_age_levine(
    chronological_age: f64,
    albumin: f64,
    creatinine: f64,
    glucose: f64,
    crp_ln: f64,
    lymphocyte_pct: f64,
    mcv: f64,
    rdw: f64,
    alkaline_phosphatase: f64,
    wbc: f64,
) -> f64 {
    -19.907 + 0.0804 * chronological_age - 0.0803 * albumin
        + 0.0095 * creatinine
        + 0.00188 * glucose
        + 0.0954 * crp_ln
        - 0.012 * lymphocyte_pct
        + 0.0268 * mcv
        + 0.3306 * rdw
        + 0.00188 * alkaline_phosphatase
        + 0.0554 * wbc
}

pub fn frailty_index(deficits_present: u32, deficits_measured: u32) -> f64 {
    deficits_present as f64 / deficits_measured.max(1) as f64
}

pub fn disability_free_life_expectancy(survival: &[f64], disability_free: &[f64]) -> f64 {
    survival
        .iter()
        .zip(disability_free.iter())
        .map(|(&s, &d)| s * d)
        .sum()
}
