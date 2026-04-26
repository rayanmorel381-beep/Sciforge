use crate::constants::{NMDA_MG_KD, NMDA_VOLTAGE_COEFF, SYNAPTIC_CA_HALF_SAT};

pub fn ltp_magnitude(
    stimulus_frequency: f64,
    calcium_influx: f64,
    threshold: f64,
    max_potentiation: f64,
) -> f64 {
    if calcium_influx < threshold {
        return 0.0;
    }
    max_potentiation * (1.0 - (-stimulus_frequency * (calcium_influx - threshold)).exp())
}

pub fn ltd_magnitude(
    calcium_level: f64,
    low_threshold: f64,
    high_threshold: f64,
    max_depression: f64,
) -> f64 {
    if calcium_level < low_threshold || calcium_level > high_threshold {
        return 0.0;
    }
    let normalized = (calcium_level - low_threshold) / (high_threshold - low_threshold);
    max_depression * (1.0 - (2.0 * normalized - 1.0).powi(2))
}

pub fn stdp_weight_change(
    delta_t_ms: f64,
    a_plus: f64,
    a_minus: f64,
    tau_plus: f64,
    tau_minus: f64,
) -> f64 {
    if delta_t_ms > 0.0 {
        a_plus * (-delta_t_ms / tau_plus).exp()
    } else {
        -a_minus * (delta_t_ms / tau_minus).exp()
    }
}

pub fn synaptic_vesicle_release_probability(calcium: f64, n_vesicles: usize, p_single: f64) -> f64 {
    let p_ca = p_single * calcium / (calcium + SYNAPTIC_CA_HALF_SAT);
    1.0 - (1.0 - p_ca).powi(n_vesicles as i32)
}

pub fn short_term_facilitation(baseline_p: f64, facilitation: f64, tau: f64, isi: f64) -> f64 {
    baseline_p + facilitation * (-isi / tau).exp()
}

pub fn short_term_depression(available: f64, release_p: f64, recovery_tau: f64, isi: f64) -> f64 {
    1.0 - (1.0 - available * (1.0 - release_p)) * (-isi / recovery_tau).exp()
}

pub fn miniature_epsp_amplitude(
    quantal_size: f64,
    n_receptors: f64,
    receptor_conductance: f64,
) -> f64 {
    quantal_size * n_receptors * receptor_conductance
}

pub fn nmda_voltage_dependence(voltage: f64, mg_conc: f64) -> f64 {
    1.0 / (1.0 + mg_conc / NMDA_MG_KD * (-NMDA_VOLTAGE_COEFF * voltage).exp())
}

pub fn dendritic_spine_volume_change(
    calcium: f64,
    actin_polymerization: f64,
    spine_volume: f64,
    max_growth: f64,
) -> f64 {
    let growth_signal = calcium * actin_polymerization;
    spine_volume * (1.0 + max_growth * growth_signal / (1.0 + growth_signal))
}

pub fn homeostatic_synaptic_scaling(
    target_rate: f64,
    current_rate: f64,
    scaling_tau: f64,
    dt: f64,
) -> f64 {
    1.0 + dt / scaling_tau * (target_rate - current_rate) / target_rate.max(1e-30)
}

pub fn glutamate_clearance(released: f64, transporter_density: f64, vmax: f64, km: f64) -> f64 {
    vmax * transporter_density * released / (km + released)
}
