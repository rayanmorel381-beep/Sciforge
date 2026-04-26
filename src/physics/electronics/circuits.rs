pub fn ohm_voltage(i: f64, r: f64) -> f64 {
    i * r
}
pub fn ohm_current(v: f64, r: f64) -> f64 {
    v / r
}
pub fn ohm_resistance(v: f64, i: f64) -> f64 {
    v / i
}

pub fn series_resistance(resistors: &[f64]) -> f64 {
    resistors.iter().sum()
}

pub fn parallel_resistance(resistors: &[f64]) -> f64 {
    let sum_inv: f64 = resistors.iter().map(|r| 1.0 / r).sum();
    1.0 / sum_inv
}

pub fn voltage_divider(v_in: f64, r1: f64, r2: f64) -> f64 {
    v_in * r2 / (r1 + r2)
}

pub fn current_divider(i_total: f64, r_branch: f64, r_total_parallel: f64) -> f64 {
    i_total * r_total_parallel / r_branch
}

pub fn power_dc(v: f64, i: f64) -> f64 {
    v * i
}

pub fn rc_charging(v_source: f64, t: f64, r: f64, c: f64) -> f64 {
    v_source * (1.0 - (-t / (r * c)).exp())
}

pub fn rc_discharging(v0: f64, t: f64, r: f64, c: f64) -> f64 {
    v0 * (-t / (r * c)).exp()
}

pub fn rl_current_rise(v: f64, r: f64, l: f64, t: f64) -> f64 {
    (v / r) * (1.0 - (-r * t / l).exp())
}

pub fn rl_current_decay(i0: f64, r: f64, l: f64, t: f64) -> f64 {
    i0 * (-r * t / l).exp()
}

pub fn rlc_resonant_frequency(l: f64, c: f64) -> f64 {
    1.0 / (2.0 * std::f64::consts::PI * (l * c).sqrt())
}

pub fn rlc_quality_factor(r: f64, l: f64, c: f64) -> f64 {
    (1.0 / r) * (l / c).sqrt()
}

pub fn rlc_bandwidth(f0: f64, q: f64) -> f64 {
    f0 / q
}

pub fn impedance_capacitor(c: f64, freq: f64) -> (f64, f64) {
    let xc = 1.0 / (2.0 * std::f64::consts::PI * freq * c);
    (0.0, -xc)
}

pub fn impedance_inductor(l: f64, freq: f64) -> (f64, f64) {
    let xl = 2.0 * std::f64::consts::PI * freq * l;
    (0.0, xl)
}

pub fn impedance_magnitude(re: f64, im: f64) -> f64 {
    (re * re + im * im).sqrt()
}

pub fn impedance_phase(re: f64, im: f64) -> f64 {
    im.atan2(re)
}

pub fn capacitor_energy(c: f64, v: f64) -> f64 {
    0.5 * c * v * v
}
pub fn inductor_energy(l: f64, i: f64) -> f64 {
    0.5 * l * i * i
}

pub fn wheatstone_bridge_voltage(v_in: f64, r1: f64, r2: f64, r3: f64, r4: f64) -> f64 {
    v_in * (r3 / (r3 + r4) - r1 / (r1 + r2))
}

pub fn thevenin_voltage(v_oc: f64) -> f64 {
    v_oc
}
pub fn thevenin_resistance(v_oc: f64, i_sc: f64) -> f64 {
    v_oc / i_sc
}

pub fn max_power_transfer(v_th: f64, r_th: f64) -> f64 {
    v_th * v_th / (4.0 * r_th)
}
