use super::LiquidState;

pub fn heat_exchanged_j_kg(state: &LiquidState, delta_t_k: f64) -> f64 {
    state.liquid.specific_heat_j_kgk * delta_t_k
}

pub fn temperature_rise_k(state: &LiquidState, heat_per_kg_j: f64) -> f64 {
    heat_per_kg_j / state.liquid.specific_heat_j_kgk
}

pub fn lmtd_counterflow_k(
    t_hot_in: f64,
    t_hot_out: f64,
    t_cold_in: f64,
    t_cold_out: f64,
) -> f64 {
    let d1 = t_hot_in - t_cold_out;
    let d2 = t_hot_out - t_cold_in;
    if (d1 - d2).abs() < 1e-10 {
        d1
    } else {
        (d1 - d2) / (d1 / d2).ln()
    }
}

pub fn lmtd_parallel_k(
    t_hot_in: f64,
    t_hot_out: f64,
    t_cold_in: f64,
    t_cold_out: f64,
) -> f64 {
    let d1 = t_hot_in - t_cold_in;
    let d2 = t_hot_out - t_cold_out;
    if (d1 - d2).abs() < 1e-10 {
        d1
    } else {
        (d1 - d2) / (d1 / d2).ln()
    }
}

pub fn heat_exchanger_area_m2(q_w: f64, u_w_m2k: f64, lmtd_k: f64) -> f64 {
    q_w / (u_w_m2k * lmtd_k)
}

pub fn ntu(u_w_m2k: f64, area_m2: f64, c_min: f64) -> f64 {
    u_w_m2k * area_m2 / c_min
}

pub fn effectiveness_counterflow(ntu: f64, c_ratio: f64) -> f64 {
    if (c_ratio - 1.0).abs() < 1e-10 {
        ntu / (1.0 + ntu)
    } else {
        let exp = (-(1.0 - c_ratio) * ntu).exp();
        (1.0 - exp) / (1.0 - c_ratio * exp)
    }
}

pub fn effectiveness_parallel(ntu: f64, c_ratio: f64) -> f64 {
    (1.0 - (-(1.0 + c_ratio) * ntu).exp()) / (1.0 + c_ratio)
}

pub fn thermal_capacity_rate_w_k(state: &LiquidState, mass_flow_kg_s: f64) -> f64 {
    mass_flow_kg_s * state.liquid.specific_heat_j_kgk
}
