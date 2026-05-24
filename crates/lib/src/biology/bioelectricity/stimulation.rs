pub fn defibrillation_energy(capacitance: f64, voltage: f64) -> f64 {
    0.5 * capacitance * voltage * voltage
}

pub fn electrode_impedance(resistance: f64, capacitance: f64, frequency: f64) -> f64 {
    let xc = 1.0 / (2.0 * std::f64::consts::PI * frequency * capacitance);
    (resistance * resistance + xc * xc).sqrt()
}

pub fn stimulation_strength_duration(rheobase: f64, chronaxie: f64, pulse_width: f64) -> f64 {
    rheobase * (1.0 + chronaxie / pulse_width)
}

pub fn bioimpedance_cole_model(
    r_inf: f64,
    r_0: f64,
    tau: f64,
    alpha: f64,
    frequency: f64,
) -> (f64, f64) {
    let omega = 2.0 * std::f64::consts::PI * frequency;
    let wt = omega * tau;
    let cos_a = (alpha * std::f64::consts::PI / 2.0).cos();
    let sin_a = (alpha * std::f64::consts::PI / 2.0).sin();
    let denom = 1.0 + 2.0 * wt.powf(alpha) * cos_a + wt.powf(2.0 * alpha);
    let delta_r = r_0 - r_inf;
    let real = r_inf + delta_r * (1.0 + wt.powf(alpha) * cos_a) / denom;
    let imag = -delta_r * wt.powf(alpha) * sin_a / denom;
    (real, imag)
}

pub fn transcranial_current_density(current: f64, electrode_area: f64) -> f64 {
    current / electrode_area
}

pub fn neural_recruitment_curve(
    stimulus: f64,
    threshold: f64,
    saturation: f64,
    steepness: f64,
) -> f64 {
    1.0 / (1.0 + (-(stimulus - threshold) / steepness).exp()) * saturation
}

pub fn charge_density(charge: f64, electrode_area: f64) -> f64 {
    charge / electrode_area
}

pub fn cathodic_charge_balanced(
    anodic_amplitude: f64,
    anodic_duration: f64,
    cathodic_duration: f64,
) -> f64 {
    anodic_amplitude * anodic_duration / cathodic_duration
}

pub fn pulse_train_energy(
    amplitude: f64,
    pulse_width: f64,
    frequency: f64,
    duration: f64,
    impedance: f64,
) -> f64 {
    let n_pulses = frequency * duration;
    n_pulses * amplitude * amplitude * pulse_width / impedance
}

pub fn tissue_heating(
    current_density: f64,
    conductivity: f64,
    duration: f64,
    specific_heat: f64,
    density: f64,
) -> f64 {
    current_density * current_density * duration / (conductivity * specific_heat * density)
}

pub fn tms_induced_efield(di_dt: f64, coil_inductance: f64, distance: f64) -> f64 {
    coil_inductance * di_dt / (2.0 * std::f64::consts::PI * distance)
}

pub fn dbs_volume_tissue_activated(current: f64, impedance: f64, threshold_efield: f64) -> f64 {
    let voltage = current * impedance;
    let radius = (voltage / threshold_efield).sqrt();
    (4.0 / 3.0) * std::f64::consts::PI * radius.powi(3)
}

pub fn cochlear_implant_spread(current: f64, distance: f64, sigma: f64) -> f64 {
    current / (4.0 * std::f64::consts::PI * sigma * distance)
}

pub fn fes_fatigue_index(initial_force: f64, final_force: f64) -> f64 {
    (initial_force - final_force) / initial_force
}

pub fn shannon_safety_limit(charge_per_phase_uc: f64, electrode_area_cm2: f64) -> f64 {
    (charge_per_phase_uc / electrode_area_cm2).log10()
}

pub fn biphasic_pulse_charge(amplitude: f64, phase_duration: f64) -> f64 {
    amplitude * phase_duration
}

pub fn interphase_gap_effect(threshold_no_gap: f64, gap_duration: f64, time_constant: f64) -> f64 {
    threshold_no_gap / (1.0 - (-gap_duration / time_constant).exp())
}

pub fn electrochemical_safety_margin(water_window: f64, electrode_potential: f64) -> f64 {
    water_window - electrode_potential.abs()
}

pub fn warburg_impedance(sigma: f64, frequency: f64) -> (f64, f64) {
    let omega = 2.0 * std::f64::consts::PI * frequency;
    let w = sigma / omega.sqrt();
    (w, -w)
}

pub fn constant_phase_element(q: f64, alpha: f64, frequency: f64) -> (f64, f64) {
    let omega = 2.0 * std::f64::consts::PI * frequency;
    let mag = 1.0 / (q * omega.powf(alpha));
    let phase = -alpha * std::f64::consts::PI / 2.0;
    (mag * phase.cos(), mag * phase.sin())
}

pub fn chronaxie_from_strength_duration(
    rheobase: f64,
    threshold_at_pw: f64,
    pulse_width: f64,
) -> f64 {
    pulse_width * (threshold_at_pw / rheobase - 1.0).recip()
}

pub fn galvanic_skin_response(
    baseline_conductance: f64,
    peak_conductance: f64,
    t: f64,
    tau_rise: f64,
    tau_decay: f64,
) -> f64 {
    let delta = peak_conductance - baseline_conductance;
    if t < 0.0 {
        return baseline_conductance;
    }
    baseline_conductance + delta * (1.0 - (-t / tau_rise).exp()) * (-t / tau_decay).exp()
}

pub fn total_charge_delivered(
    amplitude: f64,
    pulse_width: f64,
    frequency: f64,
    duration: f64,
) -> f64 {
    amplitude * pulse_width * frequency * duration
}

pub fn electrode_polarization_voltage(charge: f64, capacitance: f64) -> f64 {
    charge / capacitance
}

pub fn anodal_break_excitation_threshold(
    membrane_tau: f64,
    pulse_duration: f64,
    rheobase: f64,
) -> f64 {
    rheobase * (1.0 + membrane_tau / pulse_duration)
}
