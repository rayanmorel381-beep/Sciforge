use crate::constants::R_GAS;

pub fn rate_constant_arrhenius(a: f64, ea: f64, t: f64) -> f64 {
    a * (-ea / (R_GAS * t)).exp()
}

pub fn half_life_first_order(k: f64) -> f64 {
    (2.0_f64).ln() / k
}

pub fn concentration_first_order(c0: f64, k: f64, t: f64) -> f64 {
    c0 * (-k * t).exp()
}

pub fn concentration_second_order(c0: f64, k: f64, t: f64) -> f64 {
    c0 / (1.0 + c0 * k * t)
}

pub fn concentration_zero_order(c0: f64, k: f64, t: f64) -> f64 {
    (c0 - k * t).max(0.0)
}

pub fn rate_law(k: f64, concentrations: &[f64], orders: &[f64]) -> f64 {
    let mut rate = k;
    for i in 0..concentrations.len().min(orders.len()) {
        rate *= concentrations[i].powf(orders[i]);
    }
    rate
}

pub fn activation_energy_two_temps(k1: f64, k2: f64, t1: f64, t2: f64) -> f64 {
    R_GAS * (k2 / k1).ln() / (1.0 / t1 - 1.0 / t2)
}

pub fn half_life_second_order(k: f64, c0: f64) -> f64 {
    1.0 / (k * c0).max(1e-30)
}

pub fn half_life_zero_order(k: f64, c0: f64) -> f64 {
    c0 / (2.0 * k).max(1e-30)
}

pub fn integrated_rate_law_nth(c0: f64, k: f64, t: f64, n: f64) -> f64 {
    if (n - 1.0).abs() < 1e-10 {
        return c0 * (-k * t).exp();
    }
    let val = c0.powf(1.0 - n) + (n - 1.0) * k * t;
    if val <= 0.0 {
        return 0.0;
    }
    val.powf(1.0 / (1.0 - n))
}

pub fn eyring_equation(kappa: f64, kb: f64, t: f64, h: f64, delta_g_dagger: f64) -> f64 {
    kappa * kb * t / h * (-delta_g_dagger / (R_GAS * t)).exp()
}

pub fn collision_frequency(na: f64, nb: f64, sigma: f64, t: f64, mu: f64) -> f64 {
    na * nb * sigma * (8.0 * R_GAS * t / (std::f64::consts::PI * mu)).sqrt()
}
