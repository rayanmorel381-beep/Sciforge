use crate::constants::{
    ATP_YIELD_AEROBIC, ATP_YIELD_ANAEROBIC, BETA_OX_ACETYL_COA_ATP, BETA_OX_ACTIVATION_COST,
    BETA_OX_FADH2_ATP, BETA_OX_NADH_ATP, CAC_FADH2_PER_ACETYL_COA, CAC_NADH_PER_ACETYL_COA,
    FARADAY, K_B, N_A,
};

pub fn atp_free_energy(delta_g0: f64, atp: f64, adp: f64, pi: f64, t: f64) -> f64 {
    let ratio = (adp * pi) / atp.max(1e-30);
    delta_g0 + K_B * N_A * t * ratio.ln()
}

pub fn atp_synthase_rate(proton_gradient: f64, n_protons: f64, delta_g_atp: f64, t: f64) -> f64 {
    let pmf_energy = n_protons * proton_gradient;
    let ratio = pmf_energy / delta_g_atp.abs().max(1e-30);
    let kbt = K_B * N_A * t;
    ratio * kbt
}

pub fn proton_motive_force(delta_psi: f64, delta_ph: f64, t: f64) -> f64 {
    let r = K_B * N_A;
    let f = FARADAY;
    delta_psi - 2.303 * r * t * delta_ph / f
}

pub fn p_to_o_ratio(atp_produced: f64, oxygen_consumed: f64) -> f64 {
    atp_produced / (2.0 * oxygen_consumed)
}

pub fn respiratory_control_ratio(state3: f64, state4: f64) -> f64 {
    state3 / state4.max(1e-30)
}

pub fn membrane_potential_nernst(z: f64, c_out: f64, c_in: f64, t: f64) -> f64 {
    let r = K_B * N_A;
    let f = FARADAY;
    (r * t / (z * f)) * (c_out / c_in).ln()
}

pub fn uncoupler_effect(pmf: f64, permeability: f64, concentration: f64) -> f64 {
    pmf * (-permeability * concentration).exp()
}

pub fn citric_acid_cycle_nadh_yield(acetyl_coa_flux: f64) -> f64 {
    CAC_NADH_PER_ACETYL_COA * acetyl_coa_flux
}

pub fn citric_acid_cycle_fadh2_yield(acetyl_coa_flux: f64) -> f64 {
    CAC_FADH2_PER_ACETYL_COA * acetyl_coa_flux
}

pub fn electron_transport_efficiency(
    n_electrons: f64,
    delta_e: f64,
    delta_g_atp: f64,
    n_atp: f64,
) -> f64 {
    let total_energy = n_electrons * FARADAY * delta_e;
    (n_atp * delta_g_atp.abs()) / total_energy.abs().max(1e-30)
}

pub fn substrate_level_phosphorylation(n_reactions: f64, delta_g_per_reaction: f64) -> f64 {
    n_reactions * delta_g_per_reaction
}

pub fn anaerobic_atp_yield(glucose_flux: f64) -> f64 {
    ATP_YIELD_ANAEROBIC * glucose_flux
}

pub fn aerobic_atp_yield(glucose_flux: f64) -> f64 {
    ATP_YIELD_AEROBIC * glucose_flux
}

pub fn lactate_production_rate(pyruvate_flux: f64, nad_ratio: f64) -> f64 {
    pyruvate_flux * nad_ratio / (1.0 + nad_ratio)
}

pub fn beta_oxidation_atp_yield(carbon_chain_length: f64) -> f64 {
    let n_cycles = (carbon_chain_length / 2.0) - 1.0;
    let fadh2 = n_cycles;
    let nadh = n_cycles;
    let acetyl_coa = carbon_chain_length / 2.0;
    fadh2 * BETA_OX_FADH2_ATP + nadh * BETA_OX_NADH_ATP + acetyl_coa * BETA_OX_ACETYL_COA_ATP
        - BETA_OX_ACTIVATION_COST
}

pub fn creatine_phosphate_buffer(atp: f64, adp: f64, cr_p: f64, keq: f64) -> f64 {
    keq * cr_p * adp / atp.max(1e-30)
}
