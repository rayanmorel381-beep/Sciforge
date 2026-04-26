use crate::constants::R_GAS;

pub fn carnot_efficiency(t_hot: f64, t_cold: f64) -> f64 {
    1.0 - t_cold / t_hot
}

pub fn carnot_cop_heating(t_hot: f64, t_cold: f64) -> f64 {
    t_hot / (t_hot - t_cold)
}

pub fn carnot_cop_cooling(t_hot: f64, t_cold: f64) -> f64 {
    t_cold / (t_hot - t_cold)
}

pub fn isothermal_work(n_moles: f64, temperature: f64, v1: f64, v2: f64) -> f64 {
    -n_moles * R_GAS * temperature * (v2 / v1).ln()
}

pub fn adiabatic_work(n_moles: f64, cv: f64, t1: f64, t2: f64) -> f64 {
    n_moles * cv * (t1 - t2)
}

pub fn isobaric_work(pressure: f64, v1: f64, v2: f64) -> f64 {
    pressure * (v2 - v1)
}

pub fn adiabatic_relation_tv(t1: f64, v1: f64, v2: f64, gamma: f64) -> f64 {
    t1 * (v1 / v2).powf(gamma - 1.0)
}

pub fn adiabatic_relation_pv(p1: f64, v1: f64, v2: f64, gamma: f64) -> f64 {
    p1 * (v1 / v2).powf(gamma)
}

pub fn otto_efficiency(compression_ratio: f64, gamma: f64) -> f64 {
    1.0 - 1.0 / compression_ratio.powf(gamma - 1.0)
}

pub fn diesel_efficiency(compression_ratio: f64, cutoff_ratio: f64, gamma: f64) -> f64 {
    1.0 - (cutoff_ratio.powf(gamma) - 1.0)
        / (gamma * (compression_ratio.powf(gamma - 1.0)) * (cutoff_ratio - 1.0))
}

pub fn joule_thomson_coefficient(cp: f64, v_molar: f64, temperature: f64, dv_dt_p: f64) -> f64 {
    (temperature * dv_dt_p - v_molar) / cp
}

pub fn throttling_temperature_change(mu_jt: f64, dp: f64) -> f64 {
    mu_jt * dp
}

pub fn heat_conduction_rate(k: f64, area: f64, dt: f64, dx: f64) -> f64 {
    -k * area * dt / dx
}

pub fn thermal_diffusion_1d(t: &mut [f64], alpha: f64, dx: f64, dt: f64, steps: usize) {
    let n = t.len();
    let r = alpha * dt / (dx * dx);
    let mut t_new = t.to_vec();
    for _ in 0..steps {
        for i in 1..n - 1 {
            t_new[i] = t[i] + r * (t[i + 1] - 2.0 * t[i] + t[i - 1]);
        }
        t.copy_from_slice(&t_new);
    }
}

pub fn mixing_entropy(mole_fractions: &[f64]) -> f64 {
    -R_GAS
        * mole_fractions
            .iter()
            .filter(|&&x| x > 0.0)
            .map(|&x| x * x.ln())
            .sum::<f64>()
}

pub fn reaction_gibbs(delta_g0: f64, temperature: f64, q: f64) -> f64 {
    delta_g0 + R_GAS * temperature * q.ln()
}

pub fn equilibrium_constant(delta_g0: f64, temperature: f64) -> f64 {
    (-delta_g0 / (R_GAS * temperature)).exp()
}

pub fn vant_hoff(k1: f64, delta_h: f64, t1: f64, t2: f64) -> f64 {
    k1 * (delta_h / R_GAS * (1.0 / t1 - 1.0 / t2)).exp()
}
