pub fn hpa_axis_cortisol(
    crh: f64,
    acth_gain: f64,
    cortisol_gain: f64,
    feedback_strength: f64,
    cortisol_current: f64,
) -> (f64, f64) {
    let acth = acth_gain * crh / (1.0 + feedback_strength * cortisol_current);
    let cortisol = cortisol_gain * acth;
    (acth, cortisol)
}

pub fn hpg_axis_testosterone(
    gnrh: f64,
    lh_gain: f64,
    testosterone_gain: f64,
    feedback: f64,
    testosterone_current: f64,
) -> (f64, f64) {
    let lh = lh_gain * gnrh / (1.0 + feedback * testosterone_current);
    let testosterone = testosterone_gain * lh;
    (lh, testosterone)
}

pub fn hpt_axis_t4(
    trh: f64,
    tsh_gain: f64,
    t4_gain: f64,
    feedback: f64,
    t4_current: f64,
) -> (f64, f64) {
    let tsh = tsh_gain * trh / (1.0 + feedback * t4_current);
    let t4 = t4_gain * tsh;
    (tsh, t4)
}

pub fn glucose_insulin_model_step(
    glucose: f64,
    insulin: f64,
    glucose_input: f64,
    dt: f64,
    si: f64,
    sg: f64,
    n: f64,
    gamma: f64,
    g_threshold: f64,
) -> (f64, f64) {
    let dg = glucose_input - sg * glucose - si * insulin * glucose;
    let di = if glucose > g_threshold {
        gamma * (glucose - g_threshold) - n * insulin
    } else {
        -n * insulin
    };
    (glucose + dg * dt, (insulin + di * dt).max(0.0))
}

pub fn calcium_pth_feedback(calcium: f64, setpoint: f64, pth_max: f64, steepness: f64) -> f64 {
    pth_max / (1.0 + (steepness * (calcium - setpoint)).exp())
}

pub fn raas_angiotensin(renin: f64, angiotensinogen: f64, ace_activity: f64) -> f64 {
    renin * angiotensinogen * ace_activity
}

pub fn aldosterone_response(
    angiotensin_ii: f64,
    potassium: f64,
    gain_ang: f64,
    gain_k: f64,
) -> f64 {
    gain_ang * angiotensin_ii + gain_k * potassium
}

pub fn growth_hormone_igf1(gh: f64, liver_response: f64, feedback: f64, igf1_current: f64) -> f64 {
    liver_response * gh / (1.0 + feedback * igf1_current)
}

pub fn leptin_energy_feedback(
    fat_mass: f64,
    leptin_sensitivity: f64,
    energy_expenditure_base: f64,
) -> f64 {
    energy_expenditure_base * (1.0 + leptin_sensitivity * fat_mass)
}

pub fn cortisol_awakening_response(
    basal_cortisol: f64,
    car_amplitude: f64,
    time_after_wake_min: f64,
) -> f64 {
    if time_after_wake_min < 0.0 {
        return basal_cortisol;
    }
    let peak_time = 30.0;
    let decay = 0.02;
    basal_cortisol
        + car_amplitude
            * (-(time_after_wake_min - peak_time).powi(2) / (2.0 * (peak_time / 2.0).powi(2))).exp()
            * (-decay * time_after_wake_min).exp()
}
