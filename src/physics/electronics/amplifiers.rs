pub fn inverting_gain(r_f: f64, r_in: f64) -> f64 {
    -r_f / r_in
}

pub fn non_inverting_gain(r_f: f64, r_in: f64) -> f64 {
    1.0 + r_f / r_in
}

pub fn differential_gain(r_f: f64, r_in: f64) -> f64 {
    r_f / r_in
}

pub fn summing_amplifier(v_inputs: &[f64], r_inputs: &[f64], r_f: f64) -> f64 {
    let sum: f64 = v_inputs
        .iter()
        .zip(r_inputs.iter())
        .map(|(v, r)| v / r)
        .sum();
    -r_f * sum
}

pub fn integrator_output(v_in: f64, r: f64, c: f64, t: f64) -> f64 {
    -(1.0 / (r * c)) * v_in * t
}

pub fn differentiator_output(dv_dt: f64, r: f64, c: f64) -> f64 {
    -r * c * dv_dt
}

pub fn gain_bandwidth_product(gain: f64, bandwidth: f64) -> f64 {
    gain * bandwidth
}

pub fn common_emitter_voltage_gain(gm: f64, r_c: f64) -> f64 {
    -gm * r_c
}

pub fn transconductance(i_c: f64, v_t: f64) -> f64 {
    i_c / v_t
}

pub fn thermal_voltage(temperature_k: f64) -> f64 {
    crate::constants::K_B * temperature_k / crate::constants::E_CHARGE
}

pub fn decibel_voltage(v_out: f64, v_in: f64) -> f64 {
    20.0 * (v_out / v_in).abs().log10()
}

pub fn decibel_power(p_out: f64, p_in: f64) -> f64 {
    10.0 * (p_out / p_in).log10()
}

pub fn cascaded_gain(gains_db: &[f64]) -> f64 {
    gains_db.iter().sum()
}

pub fn noise_figure(snr_in: f64, snr_out: f64) -> f64 {
    10.0 * (snr_in / snr_out).log10()
}

pub fn friis_noise_factor(factors: &[f64], gains: &[f64]) -> f64 {
    if factors.is_empty() {
        return 1.0;
    }
    let mut total = factors[0];
    let mut cumulative_gain = 1.0;
    for i in 1..factors.len() {
        if i - 1 < gains.len() {
            cumulative_gain *= gains[i - 1];
        }
        total += (factors[i] - 1.0) / cumulative_gain;
    }
    total
}
