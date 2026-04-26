pub fn cytokine_production_rate(stimulus: f64, cell_count: f64, max_rate: f64, ec50: f64) -> f64 {
    max_rate * cell_count * stimulus / (ec50 + stimulus)
}

pub fn cytokine_decay(concentration: f64, half_life: f64, dt: f64) -> f64 {
    concentration * (-0.693 * dt / half_life.max(1e-30)).exp()
}

pub fn th1_th2_balance(ifn_gamma: f64, il4: f64) -> f64 {
    ifn_gamma / (ifn_gamma + il4).max(1e-30)
}

pub fn th17_regulatory_balance(il17: f64, il10: f64, tgf_beta: f64) -> f64 {
    il17 / (il17 + il10 + tgf_beta).max(1e-30)
}

pub fn cytokine_storm_severity(
    il6: f64,
    tnf: f64,
    il1b: f64,
    threshold_il6: f64,
    threshold_tnf: f64,
    threshold_il1b: f64,
) -> f64 {
    il6 / threshold_il6 + tnf / threshold_tnf + il1b / threshold_il1b
}

pub fn jak_stat_signaling(
    cytokine: f64,
    receptor_density: f64,
    jak_activity: f64,
    socs_inhibition: f64,
) -> f64 {
    cytokine * receptor_density * jak_activity / (1.0 + socs_inhibition)
}

pub fn nfkb_activation(stimulus: f64, ikk_activity: f64, ikb_level: f64) -> f64 {
    stimulus * ikk_activity / (1.0 + ikb_level)
}

pub fn chemokine_gradient(
    source_conc: f64,
    distance: f64,
    diffusion_coeff: f64,
    decay_rate: f64,
) -> f64 {
    let lambda = (diffusion_coeff / decay_rate.max(1e-30)).sqrt();
    source_conc * (-distance / lambda).exp()
}

pub fn autocrine_loop(production_rate: f64, receptor_sensitivity: f64, degradation: f64) -> f64 {
    let steady_state = production_rate / degradation.max(1e-30);
    steady_state * receptor_sensitivity / (1.0 + receptor_sensitivity * steady_state)
}

pub fn paracrine_signaling(
    source_cells: f64,
    target_distance: f64,
    diffusion: f64,
    production: f64,
    decay: f64,
) -> f64 {
    let lambda = (diffusion / decay.max(1e-30)).sqrt();
    source_cells * production / (2.0 * std::f64::consts::PI * diffusion).max(1e-30)
        * (-target_distance / lambda).exp()
}

pub fn nlrp3_inflammasome_activation(
    damp_signal: f64,
    nlrp3: f64,
    asc: f64,
    threshold: f64,
) -> f64 {
    let combined = damp_signal * nlrp3 * asc;
    if combined > threshold { combined } else { 0.0 }
}
