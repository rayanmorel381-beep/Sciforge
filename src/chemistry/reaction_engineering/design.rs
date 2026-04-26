pub fn damkohler_number(k: f64, tau: f64, c0: f64, order: f64) -> f64 {
    k * tau * c0.powf(order - 1.0)
}

pub fn selectivity(r_desired: f64, r_undesired: f64) -> f64 {
    r_desired / r_undesired.max(1e-30)
}

pub fn yield_reactor(moles_desired: f64, moles_reacted: f64) -> f64 {
    moles_desired / moles_reacted.max(1e-30)
}

pub fn overall_selectivity(moles_desired: f64, moles_all_products: f64) -> f64 {
    moles_desired / moles_all_products.max(1e-30)
}

pub fn thiele_modulus(r: f64, k: f64, d_eff: f64) -> f64 {
    r * (k / d_eff.max(1e-30)).sqrt()
}

pub fn effectiveness_factor_sphere(phi: f64) -> f64 {
    if phi < 1e-10 {
        return 1.0;
    }
    3.0 / phi * (1.0 / phi.tanh() - 1.0 / phi)
}

pub fn weisz_prater_criterion(r_obs: f64, r_particle: f64, d_eff: f64, c_s: f64) -> f64 {
    r_obs * r_particle * r_particle / (d_eff * c_s).max(1e-30)
}

pub fn residence_time_distribution_cstr(t: f64, tau: f64) -> f64 {
    1.0 / tau.max(1e-30) * (-t / tau.max(1e-30)).exp()
}

pub fn residence_time_distribution_pfr(t: f64, tau: f64) -> f64 {
    if (t - tau).abs() < 1e-10 { 1.0 } else { 0.0 }
}

pub fn recycle_ratio_effect(x_single: f64, recycle_ratio: f64) -> f64 {
    x_single * (1.0 + recycle_ratio) / (1.0 + recycle_ratio * x_single)
}
