pub fn hormone_synthesis_rate(enzyme_conc: f64, substrate: f64, km: f64, vmax: f64) -> f64 {
    vmax * enzyme_conc * substrate / (km + substrate)
}

pub fn hormone_half_life_clearance(concentration: f64, half_life: f64, t: f64) -> f64 {
    concentration * (-0.693 * t / half_life.max(1e-30)).exp()
}

pub fn pulsatile_release(amplitude: f64, frequency: f64, t: f64, basal: f64) -> f64 {
    basal + amplitude * (0.5 * (1.0 + (2.0 * std::f64::consts::PI * frequency * t).sin()))
}

pub fn negative_feedback_loop(setpoint: f64, current: f64, gain: f64) -> f64 {
    gain * (setpoint - current)
}

pub fn positive_feedback_loop(stimulus: f64, hormone_level: f64, gain: f64, threshold: f64) -> f64 {
    if hormone_level > threshold {
        gain * (hormone_level - threshold) * stimulus
    } else {
        0.0
    }
}

pub fn receptor_saturation(hormone: f64, kd: f64, receptor_total: f64) -> f64 {
    receptor_total * hormone / (kd + hormone)
}

pub fn hormone_free_fraction(total: f64, binding_protein: f64, kd: f64) -> f64 {
    let a = 1.0;
    let b = binding_protein + kd - total;
    let c = -total * kd;
    let disc = (b * b - 4.0 * a * c).max(0.0).sqrt();
    (-b + disc) / (2.0 * a)
}

pub fn circadian_hormone_profile(amplitude: f64, phase: f64, t_hours: f64, mesor: f64) -> f64 {
    mesor + amplitude * (2.0 * std::f64::consts::PI * (t_hours - phase) / 24.0).cos()
}

pub fn steroidogenesis_rate(cholesterol: f64, star_protein: f64, enzyme_activity: f64) -> f64 {
    cholesterol * star_protein * enzyme_activity
}

pub fn thyroid_hormone_conversion(t4: f64, deiodinase_activity: f64, km: f64) -> f64 {
    deiodinase_activity * t4 / (km + t4)
}

pub fn insulin_sensitivity_index(glucose: f64, insulin: f64) -> f64 {
    1.0 / (glucose * insulin).max(1e-30)
}

pub fn homa_ir(fasting_glucose_mmol: f64, fasting_insulin_mu_per_ml: f64) -> f64 {
    fasting_glucose_mmol * fasting_insulin_mu_per_ml / 22.5
}

pub fn homa_beta(fasting_insulin_mu_per_ml: f64, fasting_glucose_mmol: f64) -> f64 {
    20.0 * fasting_insulin_mu_per_ml / (fasting_glucose_mmol - 3.5).max(1e-30)
}
