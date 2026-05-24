pub fn atp_hydrolysis_free_energy(delta_g0: f64, atp: f64, adp: f64, pi: f64, t: f64) -> f64 {
    use crate::constants::{K_B, N_A};
    let rt = K_B * N_A * t;
    delta_g0 + rt * ((adp * pi) / atp.max(1e-30)).ln()
}

pub fn p_o_ratio(atp_produced: f64, oxygen_consumed: f64) -> f64 {
    atp_produced / oxygen_consumed.max(1e-30)
}

pub fn respiratory_control_index(state3_rate: f64, state4_rate: f64) -> f64 {
    state3_rate / state4_rate.max(1e-30)
}

pub fn uncoupling_heat(pmf: f64, proton_leak: f64) -> f64 {
    pmf * proton_leak
}

pub fn chemiosmotic_atp_rate(pmf: f64, atp_synthase_activity: f64, h_per_atp: f64) -> f64 {
    atp_synthase_activity * pmf / h_per_atp
}

pub fn shuttle_efficiency_malate_aspartate(nadh_cytoplasmic: f64, transfer_rate: f64) -> f64 {
    transfer_rate * nadh_cytoplasmic * 2.5
}

pub fn shuttle_efficiency_glycerol_3p(nadh_cytoplasmic: f64, transfer_rate: f64) -> f64 {
    transfer_rate * nadh_cytoplasmic * 1.5
}

pub fn metabolic_water(glucose_oxidized: f64) -> f64 {
    6.0 * glucose_oxidized
}

pub fn adenylate_energy_charge(atp: f64, adp: f64, amp: f64) -> f64 {
    (atp + 0.5 * adp) / (atp + adp + amp).max(1e-30)
}

pub fn phosphocreatine_equilibrium(creatine: f64, atp: f64, k_eq: f64) -> f64 {
    k_eq * creatine * atp
}

pub fn myosin_atpase_rate(load_fraction: f64, vmax: f64) -> f64 {
    vmax * (1.0 - load_fraction)
}

pub fn ionic_gradient_energy(z: f64, vm: f64, c_out: f64, c_in: f64, t: f64) -> f64 {
    use crate::constants::{E_CHARGE, K_B, N_A};
    let rt = K_B * N_A * t;
    rt * (c_in / c_out.max(1e-30)).ln() + z * E_CHARGE * N_A * vm
}
