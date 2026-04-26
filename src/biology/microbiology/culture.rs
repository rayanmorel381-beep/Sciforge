pub fn chemostat_steady_state_biomass(y: f64, s_in: f64, ks: f64, mu_max: f64, d: f64) -> f64 {
    if d >= mu_max {
        return 0.0;
    }
    let s_star = d * ks / (mu_max - d);
    y * (s_in - s_star)
}

pub fn chemostat_washout_dilution(mu_max: f64, s_in: f64, ks: f64) -> f64 {
    mu_max * s_in / (ks + s_in)
}

pub fn minimum_inhibitory_concentration(
    e0: f64,
    emax: f64,
    ec50: f64,
    n: f64,
    target_kill: f64,
) -> f64 {
    if target_kill >= emax {
        return f64::INFINITY;
    }
    let ratio = target_kill / (emax - target_kill + e0);
    ec50 * ratio.powf(1.0 / n)
}

pub fn competitive_exclusion(
    x1: f64,
    x2: f64,
    s: f64,
    mu1: f64,
    mu2: f64,
    ks1: f64,
    ks2: f64,
    y1: f64,
    y2: f64,
    d: f64,
    s_in: f64,
) -> (f64, f64, f64) {
    let g1 = mu1 * s / (ks1 + s);
    let g2 = mu2 * s / (ks2 + s);
    let dx1 = (g1 - d) * x1;
    let dx2 = (g2 - d) * x2;
    let ds = d * (s_in - s) - g1 * x1 / y1 - g2 * x2 / y2;
    (dx1, dx2, ds)
}

pub fn serial_dilution(
    n0: f64,
    dilution_factor: f64,
    transfers: usize,
    growth_per_cycle: f64,
) -> Vec<f64> {
    let mut result = Vec::with_capacity(transfers + 1);
    let mut n = n0;
    result.push(n);
    for _ in 0..transfers {
        n *= growth_per_cycle;
        n /= dilution_factor;
        result.push(n);
    }
    result
}

pub fn biofilm_formation(
    planktonic: f64,
    attachment_rate: f64,
    detachment_rate: f64,
    biofilm: f64,
    carrying_capacity: f64,
) -> (f64, f64) {
    let attach = attachment_rate * planktonic * (1.0 - biofilm / carrying_capacity);
    let detach = detachment_rate * biofilm;
    (-attach + detach, attach - detach)
}

pub fn quorum_sensing(
    cell_density: f64,
    autoinducer_production: f64,
    threshold: f64,
    n_hill: f64,
) -> f64 {
    let signal = autoinducer_production * cell_density;
    let sn = signal.powf(n_hill);
    sn / (threshold.powf(n_hill) + sn)
}

pub fn colony_forming_units(od600: f64, calibration_factor: f64) -> f64 {
    od600 * calibration_factor
}

pub fn turbidostat(biomass: f64, target_od: f64, mu: f64, dt: f64) -> f64 {
    let growth = mu * biomass * dt;
    let dilution = if biomass > target_od {
        biomass - target_od
    } else {
        0.0
    };
    biomass + growth - dilution
}

pub fn ph_growth_response(ph: f64, ph_opt: f64, ph_min: f64, ph_max: f64) -> f64 {
    if ph < ph_min || ph > ph_max {
        return 0.0;
    }
    let range = (ph_max - ph_min) / 2.0;
    let deviation = (ph - ph_opt).abs();
    (1.0 - (deviation / range).powi(2)).max(0.0)
}
