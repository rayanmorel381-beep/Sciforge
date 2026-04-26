pub fn coulter_counter_volume(
    baseline_impedance: f64,
    pulse_amplitude: f64,
    orifice_volume: f64,
) -> f64 {
    orifice_volume * pulse_amplitude / baseline_impedance
}

pub fn dielectrophoresis_force(radius: f64, epsilon_m: f64, cm_factor: f64, grad_e2: f64) -> f64 {
    2.0 * std::f64::consts::PI * epsilon_m * radius.powi(3) * cm_factor * grad_e2
}

pub fn clausius_mossotti(epsilon_p: f64, epsilon_m: f64) -> f64 {
    (epsilon_p - epsilon_m) / (epsilon_p + 2.0 * epsilon_m)
}

pub fn electroporation_threshold(membrane_thickness: f64, breakdown_voltage: f64) -> f64 {
    breakdown_voltage / membrane_thickness
}

pub fn electroporation_pore_density(v_m: f64, v_ep: f64, n0: f64, alpha: f64) -> f64 {
    n0 * (alpha * (v_m / v_ep).powi(2)).exp()
}

pub fn joule_heating(current_density: f64, conductivity: f64, duration: f64) -> f64 {
    current_density * current_density / conductivity * duration
}

pub fn electrode_double_layer_capacitance(epsilon: f64, debye_length: f64) -> f64 {
    epsilon / debye_length
}

pub fn iontophoresis_flux(current: f64, z: f64, transport_number: f64) -> f64 {
    let f = 96485.0;
    transport_number * current / (z.abs() * f)
}

pub fn skin_impedance_model(r_stratum: f64, c_stratum: f64, r_deep: f64, frequency: f64) -> f64 {
    let omega = 2.0 * std::f64::consts::PI * frequency;
    let xc = 1.0 / (omega * c_stratum);
    let z_stratum = (r_stratum * xc) / (r_stratum * r_stratum + xc * xc).sqrt();
    z_stratum + r_deep
}

pub fn ecg_lead_vector(dipole: (f64, f64, f64), lead_direction: (f64, f64, f64)) -> f64 {
    dipole.0 * lead_direction.0 + dipole.1 * lead_direction.1 + dipole.2 * lead_direction.2
}

pub fn eeg_dipole_potential(
    dipole_moment: f64,
    cos_angle: f64,
    distance: f64,
    conductivity: f64,
) -> f64 {
    dipole_moment * cos_angle / (4.0 * std::f64::consts::PI * conductivity * distance * distance)
}

pub fn nerve_conduction_velocity_temperature(v_ref: f64, q10: f64, t: f64, t_ref: f64) -> f64 {
    v_ref * q10.powf((t - t_ref) / 10.0)
}
