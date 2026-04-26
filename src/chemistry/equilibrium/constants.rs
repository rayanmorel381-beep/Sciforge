use crate::constants::R_GAS;

pub fn equilibrium_constant_from_gibbs(delta_g: f64, t: f64) -> f64 {
    (-delta_g / (R_GAS * t)).exp()
}

pub fn reaction_quotient(products: &[(f64, f64)], reactants: &[(f64, f64)]) -> f64 {
    let num: f64 = products.iter().map(|&(c, n)| c.powf(n)).product();
    let den: f64 = reactants.iter().map(|&(c, n)| c.powf(n)).product();
    num / den.max(1e-30)
}

pub fn le_chatelier_shift(q: f64, keq: f64) -> i32 {
    if q < keq {
        1
    } else if q > keq {
        -1
    } else {
        0
    }
}

pub fn kp_from_kc(kc: f64, t: f64, delta_n: f64) -> f64 {
    kc * (0.08206 * t).powf(delta_n)
}

pub fn vant_hoff(k1: f64, delta_h: f64, t1: f64, t2: f64) -> f64 {
    k1 * (delta_h / R_GAS * (1.0 / t1 - 1.0 / t2)).exp()
}

pub fn degree_of_dissociation(keq: f64, c0: f64) -> f64 {
    let alpha = (keq / (keq + 4.0 * c0)).sqrt();
    alpha.min(1.0)
}

pub fn temperature_dependence_keq(k_ref: f64, delta_h: f64, t_ref: f64, t: f64) -> f64 {
    k_ref * (delta_h / R_GAS * (1.0 / t_ref - 1.0 / t)).exp()
}

pub fn gibbs_from_keq(keq: f64, t: f64) -> f64 {
    -R_GAS * t * keq.max(1e-30).ln()
}

pub fn pressure_effect_on_keq(keq: f64, delta_v: f64, p1: f64, p2: f64, t: f64) -> f64 {
    keq * (-delta_v * (p2 - p1) / (R_GAS * t)).exp()
}
