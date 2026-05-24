pub fn michaelis_menten(s: f64, vmax: f64, km: f64) -> f64 {
    vmax * s / (km + s)
}

pub fn michaelis_menten_competitive(s: f64, vmax: f64, km: f64, inhibitor: f64, ki: f64) -> f64 {
    let km_app = km * (1.0 + inhibitor / ki);
    vmax * s / (km_app + s)
}

pub fn michaelis_menten_uncompetitive(s: f64, vmax: f64, km: f64, inhibitor: f64, ki: f64) -> f64 {
    let factor = 1.0 + inhibitor / ki;
    vmax * s / (km + factor * s)
}

pub fn michaelis_menten_noncompetitive(s: f64, vmax: f64, km: f64, inhibitor: f64, ki: f64) -> f64 {
    let factor = 1.0 + inhibitor / ki;
    vmax * s / (factor * (km + s))
}

pub fn hill_equation(s: f64, vmax: f64, k: f64, n: f64) -> f64 {
    let sn = s.powf(n);
    vmax * sn / (k.powf(n) + sn)
}

pub fn lineweaver_burk(s: &[f64], v: &[f64]) -> (f64, f64) {
    let n = s.len();
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_xx = 0.0;
    for i in 0..n {
        let x = 1.0 / s[i];
        let y = 1.0 / v[i];
        sum_x += x;
        sum_y += y;
        sum_xy += x * y;
        sum_xx += x * x;
    }
    let nf = n as f64;
    let slope = (nf * sum_xy - sum_x * sum_y) / (nf * sum_xx - sum_x * sum_x);
    let intercept = (sum_y - slope * sum_x) / nf;
    let vmax = 1.0 / intercept;
    let km = slope * vmax;
    (vmax, km)
}

pub fn eadie_hofstee(v: &[f64], s: &[f64]) -> (f64, f64) {
    let n = v.len();
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_xx = 0.0;
    for i in 0..n {
        let x = v[i] / s[i];
        let y = v[i];
        sum_x += x;
        sum_y += y;
        sum_xy += x * y;
        sum_xx += x * x;
    }
    let nf = n as f64;
    let slope = (nf * sum_xy - sum_x * sum_y) / (nf * sum_xx - sum_x * sum_x);
    let intercept = (sum_y - slope * sum_x) / nf;
    let vmax = intercept;
    let km = -slope;
    (vmax, km)
}

pub fn kcat(vmax: f64, e_total: f64) -> f64 {
    vmax / e_total
}

pub fn catalytic_efficiency(kcat_val: f64, km: f64) -> f64 {
    kcat_val / km
}
