pub fn sir_immune(
    s: f64,
    i: f64,
    r: f64,
    immune: f64,
    beta: f64,
    gamma: f64,
    k_immune: f64,
    k_decay: f64,
) -> (f64, f64, f64, f64) {
    let infection = beta * s * i / (1.0 + k_immune * immune);
    let ds = -infection;
    let di = infection - gamma * i;
    let dr = gamma * i - 0.0 * r;
    let dimmune = k_immune * i - k_decay * immune;
    (ds, di, dr, dimmune)
}

pub fn sir_immune_simulate(
    s0: f64,
    i0: f64,
    r0: f64,
    immune0: f64,
    beta: f64,
    gamma: f64,
    k_immune: f64,
    k_decay: f64,
    dt: f64,
    steps: usize,
) -> Vec<(f64, f64, f64, f64)> {
    let mut result = Vec::with_capacity(steps + 1);
    let (mut s, mut i, mut r, mut im) = (s0, i0, r0, immune0);
    result.push((s, i, r, im));
    for _ in 0..steps {
        let (ds, di, dr, dim) = sir_immune(s, i, r, im, beta, gamma, k_immune, k_decay);
        s = (s + ds * dt).max(0.0);
        i = (i + di * dt).max(0.0);
        r = (r + dr * dt).max(0.0);
        im = (im + dim * dt).max(0.0);
        result.push((s, i, r, im));
    }
    result
}

pub fn vaccine_efficacy(arr_vacc: f64, arr_placebo: f64) -> f64 {
    1.0 - arr_vacc / arr_placebo
}

pub fn herd_immunity_fraction(r0: f64) -> f64 {
    1.0 - 1.0 / r0
}

pub fn antibody_decay(ab0: f64, half_life: f64, t: f64) -> f64 {
    ab0 * (-((2.0_f64).ln() / half_life) * t).exp()
}

pub fn booster_response(ab_pre: f64, fold_boost: f64, decay_rate: f64, t: f64) -> f64 {
    ab_pre * fold_boost * (-decay_rate * t).exp()
}

pub fn seroconversion_probability(dose: f64, ed50: f64, n: f64) -> f64 {
    dose.powf(n) / (ed50.powf(n) + dose.powf(n))
}

pub fn waning_immunity(immune_fraction: f64, waning_rate: f64, t: f64) -> f64 {
    immune_fraction * (-waning_rate * t).exp()
}

pub fn maternal_antibody_decay(ab0: f64, half_life: f64, t_months: f64) -> f64 {
    ab0 * (-t_months * (2.0_f64).ln() / half_life).exp()
}

pub fn seir_step(
    s: f64,
    e: f64,
    i: f64,
    r: f64,
    beta: f64,
    sigma: f64,
    gamma: f64,
    dt: f64,
) -> (f64, f64, f64, f64) {
    let ds = -beta * s * i;
    let de = beta * s * i - sigma * e;
    let di = sigma * e - gamma * i;
    let dr = gamma * i;
    (
        (s + ds * dt).max(0.0),
        (e + de * dt).max(0.0),
        (i + di * dt).max(0.0),
        (r + dr * dt).max(0.0),
    )
}
