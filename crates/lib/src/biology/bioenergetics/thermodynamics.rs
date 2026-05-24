use crate::constants::{
    BODY_TEMP_KELVIN, FARADAY, K_B, KLEIBER_CONSTANT, KLEIBER_EXPONENT, N_A, R_GAS,
};

pub fn gibbs_free_energy_reaction(delta_h: f64, t: f64, delta_s: f64) -> f64 {
    delta_h - t * delta_s
}

pub fn gibbs_free_energy_body_temp(delta_h: f64, delta_s: f64) -> f64 {
    delta_h - BODY_TEMP_KELVIN * delta_s
}

pub fn equilibrium_constant_body_temp(delta_g0: f64) -> f64 {
    let rt = K_B * N_A * BODY_TEMP_KELVIN;
    (-delta_g0 / rt).exp()
}

pub fn redox_potential_body_temp(e0: f64, n: f64, oxidized: f64, reduced: f64) -> f64 {
    let rt_f = K_B * N_A * BODY_TEMP_KELVIN / FARADAY;
    e0 + (rt_f / n) * (oxidized / reduced.max(1e-30)).ln()
}

pub fn equilibrium_constant(delta_g0: f64, t: f64) -> f64 {
    let rt = K_B * N_A * t;
    (-delta_g0 / rt).exp()
}

pub fn redox_potential(e0: f64, n: f64, oxidized: f64, reduced: f64, t: f64) -> f64 {
    let rt_f = K_B * N_A * t / FARADAY;
    e0 + (rt_f / n) * (oxidized / reduced.max(1e-30)).ln()
}

pub fn energy_charge(atp: f64, adp: f64, amp: f64) -> f64 {
    let total = atp + adp + amp;
    if total < 1e-30 {
        return 0.0;
    }
    (atp + 0.5 * adp) / total
}

pub fn metabolic_rate_kleiber(mass: f64) -> f64 {
    KLEIBER_CONSTANT * mass.powf(KLEIBER_EXPONENT)
}

pub fn oxygen_consumption_rate(metabolic_rate: f64, oxycaloric_equivalent: f64) -> f64 {
    metabolic_rate / oxycaloric_equivalent
}

pub fn coupling_efficiency(delta_g_atp: f64, delta_g_substrate: f64, n_atp: f64) -> f64 {
    (n_atp * delta_g_atp.abs()) / delta_g_substrate.abs().max(1e-30)
}

pub fn heat_dissipation(delta_g_reaction: f64, useful_work: f64) -> f64 {
    (delta_g_reaction.abs() - useful_work.abs()).max(0.0)
}

pub fn metabolic_rate_q10(rate_ref: f64, t: f64, t_ref: f64, q10: f64) -> f64 {
    rate_ref * q10.powf((t - t_ref) / 10.0)
}

pub fn arrhenius_metabolic(rate_ref: f64, ea: f64, t: f64, t_ref: f64) -> f64 {
    rate_ref * (ea / R_GAS * (1.0 / t_ref - 1.0 / t)).exp()
}

pub fn thermogenic_cost(delta_h: f64, efficiency: f64) -> f64 {
    delta_h * (1.0 - efficiency)
}

pub fn proton_gradient_energy(n_protons: f64, delta_mu: f64) -> f64 {
    n_protons * delta_mu
}

pub fn nad_redox_potential(nad_ox: f64, nad_red: f64, e0: f64, t: f64) -> f64 {
    let rt_nf = K_B * N_A * t / (2.0 * FARADAY);
    e0 + rt_nf * (nad_ox / nad_red.max(1e-30)).ln()
}

pub fn entropy_production_rate(heat_flux: f64, temperature: f64) -> f64 {
    heat_flux / temperature
}

pub fn exergy_content(delta_h: f64, t0: f64, delta_s: f64) -> f64 {
    delta_h - t0 * delta_s
}

pub fn muscle_mechanical_efficiency(work_output: f64, metabolic_input: f64) -> f64 {
    work_output / metabolic_input.max(1e-30)
}

pub fn basal_metabolic_scaling(m0: f64, mass: f64, exponent: f64) -> f64 {
    m0 * mass.powf(exponent)
}
