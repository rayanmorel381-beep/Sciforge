pub fn hormone_clearance(c0: f64, half_life: f64, t: f64) -> f64 {
    c0 * (-std::f64::consts::LN_2 * t / half_life).exp()
}

pub fn hormone_infusion_steady_state(infusion_rate: f64, clearance_rate: f64) -> f64 {
    infusion_rate / clearance_rate
}

pub fn hormone_infusion_transient(infusion_rate: f64, clearance_rate: f64, t: f64) -> f64 {
    let ss = infusion_rate / clearance_rate;
    ss * (1.0 - (-clearance_rate * t).exp())
}

pub fn pulsatile_secretion(
    amplitude: f64,
    frequency: f64,
    phase: f64,
    baseline: f64,
    t: f64,
) -> f64 {
    baseline
        + amplitude
            * (2.0 * std::f64::consts::PI * frequency * t + phase)
                .sin()
                .max(0.0)
}

pub fn negative_feedback(hormone_level: f64, set_point: f64, gain: f64) -> f64 {
    gain * (set_point - hormone_level)
}

pub fn positive_feedback(hormone_level: f64, threshold: f64, gain: f64, max_rate: f64) -> f64 {
    if hormone_level < threshold {
        0.0
    } else {
        let x = hormone_level - threshold;
        max_rate * x / (x + gain)
    }
}

pub fn hpa_axis_step(
    crf: f64,
    acth: f64,
    cortisol: f64,
    k1: f64,
    k2: f64,
    k3: f64,
    d1: f64,
    d2: f64,
    d3: f64,
    neg_gain: f64,
) -> (f64, f64, f64) {
    let dcrf = k1 - d1 * crf - neg_gain * cortisol;
    let dacth = k2 * crf - d2 * acth;
    let dcortisol = k3 * acth - d3 * cortisol;
    (
        (crf + dcrf).max(0.0),
        (acth + dacth).max(0.0),
        (cortisol + dcortisol).max(0.0),
    )
}

pub fn thyroid_axis_tsh_t4(
    tsh: f64,
    t4: f64,
    trh: f64,
    k_stim: f64,
    k_inh: f64,
    k_prod: f64,
    d_tsh: f64,
    d_t4: f64,
) -> (f64, f64) {
    let dtsh = k_stim * trh / (1.0 + k_inh * t4) - d_tsh * tsh;
    let dt4 = k_prod * tsh - d_t4 * t4;
    ((tsh + dtsh).max(0.0), (t4 + dt4).max(0.0))
}

pub fn insulin_secretion_glucose(glucose: f64, beta_cell_mass: f64, km: f64, vmax: f64) -> f64 {
    vmax * beta_cell_mass * glucose.powi(2) / (km.powi(2) + glucose.powi(2))
}

pub fn glucose_insulin_dynamics(
    glucose: f64,
    insulin: f64,
    gin: f64,
    si: f64,
    sg: f64,
    n: f64,
    ib: f64,
    gb: f64,
) -> (f64, f64) {
    let dg = gin - sg * (glucose - gb) - si * insulin * glucose;
    let di = n * (glucose - gb).max(0.0) - n * (insulin - ib);
    (dg, di)
}

pub fn hormone_binding_to_carrier(total_hormone: f64, carrier: f64, kd: f64) -> f64 {
    let sum = total_hormone + carrier + kd;
    0.5 * (sum - (sum * sum - 4.0 * total_hormone * carrier).max(0.0).sqrt())
}

pub fn free_hormone_fraction(total: f64, binding_proteins: f64, kd: f64) -> f64 {
    let bound = total * binding_proteins / (kd + binding_proteins);
    (total - bound) / total.max(1e-30)
}

pub fn cortisol_diurnal_rhythm(t_hours: f64, peak_amplitude: f64, nadir: f64) -> f64 {
    nadir + peak_amplitude * (-(((t_hours - 8.0) % 24.0) / 4.0).powi(2)).exp()
}

pub fn growth_hormone_pulse(t: f64, pulse_times: &[f64], amplitude: f64, half_life: f64) -> f64 {
    let k = std::f64::consts::LN_2 / half_life;
    pulse_times
        .iter()
        .filter(|&&tp| t >= tp)
        .map(|&tp| amplitude * (-k * (t - tp)).exp())
        .sum()
}

pub fn renin_angiotensin_aldosterone(
    renin: f64,
    angiotensinogen: f64,
    ace: f64,
    k_renin: f64,
    k_ace: f64,
    k_aldo: f64,
) -> (f64, f64, f64) {
    let ang_i = k_renin * renin * angiotensinogen;
    let ang_ii = k_ace * ace * ang_i;
    let aldosterone = k_aldo * ang_ii;
    (ang_i, ang_ii, aldosterone)
}

pub fn parathyroid_calcium_response(
    calcium: f64,
    set_point: f64,
    max_pth: f64,
    steepness: f64,
) -> f64 {
    max_pth / (1.0 + (steepness * (calcium - set_point)).exp())
}

pub fn leptin_secretion(fat_mass: f64, sensitivity: f64) -> f64 {
    sensitivity * fat_mass
}

pub fn ghrelin_fasting_profile(
    t_since_meal: f64,
    peak_time: f64,
    amplitude: f64,
    baseline: f64,
) -> f64 {
    baseline + amplitude * (1.0 - (-t_since_meal / peak_time).exp())
}
