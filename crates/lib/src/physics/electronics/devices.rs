use crate::constants::EPSILON_0;

pub fn parallel_plate_capacitance(eps_r: f64, area_m2: f64, gap_m: f64) -> f64 {
    eps_r * EPSILON_0 * area_m2 / gap_m
}

pub fn coaxial_capacitance_per_m(eps_r: f64, r_inner: f64, r_outer: f64) -> f64 {
    2.0 * std::f64::consts::PI * eps_r * EPSILON_0 / (r_outer / r_inner).ln()
}

pub fn solenoid_inductance(mu_r: f64, n_turns: f64, area_m2: f64, length_m: f64) -> f64 {
    let mu0 = crate::constants::MU_0;
    mu_r * mu0 * n_turns.powi(2) * area_m2 / length_m
}

pub fn paschen_breakdown(p_pa: f64, d_m: f64, a_per_pa_m: f64, b_v_per_pa_m: f64, gamma: f64) -> f64 {
    let pd = p_pa * d_m;
    let denom = (a_per_pa_m * pd / (1.0 + 1.0 / gamma).ln()).ln();
    if denom <= 0.0 {
        return f64::INFINITY;
    }
    b_v_per_pa_m * pd / denom
}

pub fn intrinsic_carrier_concentration(ni_ref: f64, eg_ev: f64, t_k: f64, t_ref: f64) -> f64 {
    let kb_ev = crate::constants::K_B / crate::constants::E_CHARGE;
    let ratio = (t_k / t_ref).powf(1.5);
    let exp_term = (-eg_ev / (2.0 * kb_ev) * (1.0 / t_k - 1.0 / t_ref)).exp();
    ni_ref * ratio * exp_term
}

pub fn built_in_voltage(vt_v: f64, na_per_m3: f64, nd_per_m3: f64, ni_per_m3: f64) -> f64 {
    vt_v * (na_per_m3 * nd_per_m3 / (ni_per_m3 * ni_per_m3)).ln()
}

pub fn depletion_width(eps_r: f64, v_bi: f64, v_applied: f64, n_doping_per_m3: f64) -> f64 {
    let q = crate::constants::E_CHARGE;
    (2.0 * eps_r * EPSILON_0 * (v_bi - v_applied) / (q * n_doping_per_m3)).sqrt()
}

pub fn seebeck_voltage(s_a_v_per_k: f64, s_b_v_per_k: f64, t_hot_k: f64, t_cold_k: f64) -> f64 {
    (s_a_v_per_k - s_b_v_per_k) * (t_hot_k - t_cold_k)
}

pub fn peltier_heat(s_v_per_k: f64, t_k: f64, current_a: f64) -> f64 {
    s_v_per_k * t_k * current_a
}

pub fn hall_voltage(b_t: f64, current_a: f64, thickness_m: f64, carrier_density_per_m3: f64, charge_c: f64) -> f64 {
    b_t * current_a / (carrier_density_per_m3 * charge_c * thickness_m)
}
