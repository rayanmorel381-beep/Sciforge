use crate::constants::{E_CHARGE, EPSILON_0, FARADAY, K_B, R_GAS};

pub fn nernst_potential(
    z: f64,
    temperature: f64,
    concentration_out: f64,
    concentration_in: f64,
) -> f64 {
    let t_k = temperature + 273.15;
    (R_GAS * t_k / (z * FARADAY)) * (concentration_out / concentration_in).ln()
}

pub fn goldman_hodgkin_katz(
    pk: f64,
    k_out: f64,
    k_in: f64,
    pna: f64,
    na_out: f64,
    na_in: f64,
    pcl: f64,
    cl_out: f64,
    cl_in: f64,
    temperature: f64,
) -> f64 {
    let t_k = temperature + 273.15;
    let rt_f = K_B * t_k / E_CHARGE;
    let numerator = pk * k_out + pna * na_out + pcl * cl_in;
    let denominator = pk * k_in + pna * na_in + pcl * cl_out;
    rt_f * (numerator / denominator).ln() * 1000.0
}

pub fn cable_equation_steady_state(v0: f64, x: f64, lambda: f64) -> f64 {
    v0 * (-x.abs() / lambda).exp()
}

pub fn membrane_time_constant(rm: f64, cm: f64) -> f64 {
    rm * cm
}

pub fn length_constant(rm: f64, ri: f64) -> f64 {
    (rm / ri).sqrt()
}

pub fn gap_junction_conductance(
    n_channels: f64,
    single_channel_conductance: f64,
    open_probability: f64,
) -> f64 {
    n_channels * single_channel_conductance * open_probability
}

pub fn electrodiffusion_flux(
    permeability: f64,
    z: f64,
    vm: f64,
    c_out: f64,
    c_in: f64,
    temperature: f64,
) -> f64 {
    let t_k = temperature + 273.15;
    let u = z * E_CHARGE * vm / (K_B * t_k);
    if u.abs() < 1e-6 {
        return permeability * z * FARADAY * (c_out - c_in);
    }
    permeability * z * FARADAY * u * (c_in * u.exp() - c_out) / (u.exp() - 1.0)
}

pub fn hodgkin_huxley_sodium_current(g_na: f64, m: f64, h: f64, v: f64, e_na: f64) -> f64 {
    g_na * m.powi(3) * h * (v - e_na)
}

pub fn hodgkin_huxley_potassium_current(g_k: f64, n: f64, v: f64, e_k: f64) -> f64 {
    g_k * n.powi(4) * (v - e_k)
}

pub fn hodgkin_huxley_leak_current(g_l: f64, v: f64, e_l: f64) -> f64 {
    g_l * (v - e_l)
}

pub fn hodgkin_huxley_dv_dt(cm: f64, i_ext: f64, i_na: f64, i_k: f64, i_l: f64) -> f64 {
    (i_ext - i_na - i_k - i_l) / cm
}

pub fn gating_alpha_n(v: f64) -> f64 {
    let dv = v + 55.0;
    if dv.abs() < 1e-6 {
        return 0.1;
    }
    0.01 * dv / (1.0 - (-0.1 * dv).exp())
}

pub fn gating_beta_n(v: f64) -> f64 {
    0.125 * (-(v + 65.0) / 80.0).exp()
}

pub fn gating_alpha_m(v: f64) -> f64 {
    let dv = v + 40.0;
    if dv.abs() < 1e-6 {
        return 1.0;
    }
    0.1 * dv / (1.0 - (-0.1 * dv).exp())
}

pub fn gating_beta_m(v: f64) -> f64 {
    4.0 * (-(v + 65.0) / 18.0).exp()
}

pub fn gating_alpha_h(v: f64) -> f64 {
    0.07 * (-(v + 65.0) / 20.0).exp()
}

pub fn gating_beta_h(v: f64) -> f64 {
    1.0 / (1.0 + (-(v + 35.0) / 10.0).exp())
}

pub fn gating_steady_state(alpha: f64, beta: f64) -> f64 {
    alpha / (alpha + beta)
}

pub fn gating_time_constant(alpha: f64, beta: f64) -> f64 {
    1.0 / (alpha + beta)
}

pub fn reversal_potential_two_ion(g1: f64, e1: f64, g2: f64, e2: f64) -> f64 {
    (g1 * e1 + g2 * e2) / (g1 + g2)
}

pub fn membrane_capacitance_current(cm: f64, dv_dt: f64) -> f64 {
    cm * dv_dt
}

pub fn ion_channel_open_probability(v: f64, v_half: f64, slope: f64) -> f64 {
    1.0 / (1.0 + (-(v - v_half) / slope).exp())
}

pub fn synaptic_conductance_alpha(g_max: f64, t: f64, tau: f64) -> f64 {
    g_max * (t / tau) * (-1.0 + t / tau).exp().recip() * (-(t / tau)).exp()
}

pub fn synaptic_current(g_syn: f64, v_post: f64, e_syn: f64) -> f64 {
    g_syn * (v_post - e_syn)
}

pub fn calcium_nernst(temperature: f64, ca_out: f64, ca_in: f64) -> f64 {
    nernst_potential(2.0, temperature, ca_out, ca_in)
}

pub fn chloride_equilibrium(temperature: f64, cl_out: f64, cl_in: f64) -> f64 {
    nernst_potential(-1.0, temperature, cl_out, cl_in)
}

pub fn resting_potential_contribution(
    conductance: f64,
    reversal: f64,
    total_conductance: f64,
) -> f64 {
    conductance * reversal / total_conductance
}

pub fn space_clamp_error(distance: f64, lambda: f64) -> f64 {
    1.0 - (-distance / lambda).exp()
}

pub fn action_potential_threshold_estimate(v_rest: f64, depolarization: f64) -> f64 {
    v_rest + depolarization
}

pub fn conduction_velocity(diameter: f64, myelinated: bool) -> f64 {
    if myelinated {
        6.0 * diameter
    } else {
        diameter.sqrt()
    }
}

pub fn saltatory_conduction_delay(internode_distance: f64, velocity: f64) -> f64 {
    internode_distance / velocity
}

pub fn membrane_resistance_per_area(resistivity: f64, thickness: f64) -> f64 {
    resistivity * thickness
}

pub fn specific_membrane_capacitance(epsilon_r: f64, thickness: f64) -> f64 {
    epsilon_r * EPSILON_0 / thickness
}
