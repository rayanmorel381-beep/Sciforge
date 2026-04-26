pub fn zeitgeber_strength(light_intensity: f64, threshold: f64, saturation: f64) -> f64 {
    (light_intensity - threshold).max(0.0) / (saturation + (light_intensity - threshold).max(0.0))
}

pub fn phase_response_curve(phase: f64, light_pulse_phase: f64, sensitivity: f64) -> f64 {
    -sensitivity * (phase - light_pulse_phase).sin()
}

pub fn jet_lag_recovery(timezone_shift: f64, adaptation_rate: f64, days: f64) -> f64 {
    timezone_shift * (-adaptation_rate * days).exp()
}

pub fn shift_work_desynchrony(internal_phase: f64, external_phase: f64) -> f64 {
    let diff = (internal_phase - external_phase) % (2.0 * std::f64::consts::PI);
    diff.abs().min(2.0 * std::f64::consts::PI - diff.abs())
}

pub fn seasonal_photoperiod(day_of_year: usize, latitude: f64) -> f64 {
    let declination =
        23.44 * (2.0 * std::f64::consts::PI * (284.0 + day_of_year as f64) / 365.0).sin();
    let lat_rad = latitude * std::f64::consts::PI / 180.0;
    let dec_rad = declination * std::f64::consts::PI / 180.0;
    let ha = (-lat_rad.tan() * dec_rad.tan()).clamp(-1.0, 1.0).acos();
    2.0 * ha * 12.0 / std::f64::consts::PI
}

pub fn melatonin_suppression(light_intensity: f64, ic50: f64, hill_n: f64) -> f64 {
    1.0 - light_intensity.powf(hill_n) / (ic50.powf(hill_n) + light_intensity.powf(hill_n))
}

pub fn social_zeitgeber_strength(regularity: f64, social_contacts: f64) -> f64 {
    regularity * social_contacts / (1.0 + social_contacts)
}

pub fn food_entrainment(feeding_time: f64, clock_phase: f64, coupling: f64) -> f64 {
    coupling * (feeding_time - clock_phase).sin()
}

pub fn chronotype_score(midpoint_sleep: f64) -> f64 {
    midpoint_sleep
}

pub fn circadian_amplitude_damping(initial_amplitude: f64, damping_rate: f64, t: f64) -> f64 {
    initial_amplitude * (-damping_rate * t).exp()
}
