use std::f64::consts::PI;

pub fn beam_natural_frequency(
    mode: u32,
    e_pa: f64,
    i_m4: f64,
    rho_a_kg_per_m: f64,
    length_m: f64,
    end_conditions: &str,
) -> f64 {
    let beta_l = match end_conditions {
        "simply_supported" => (mode as f64) * PI,
        "cantilever" => match mode {
            1 => 1.875104,
            2 => 4.694091,
            3 => 7.854757,
            n => (2.0 * n as f64 - 1.0) * PI / 2.0,
        },
        "fixed-fixed" | "free-free" => match mode {
            1 => 4.730041,
            2 => 7.853205,
            3 => 10.995608,
            n => (2.0 * n as f64 + 1.0) * PI / 2.0,
        },
        _ => (mode as f64) * PI,
    };
    (beta_l / length_m).powi(2) / (2.0 * PI) * (e_pa * i_m4 / rho_a_kg_per_m).sqrt()
}

pub fn plate_natural_frequency(
    mode_m: u32,
    mode_n: u32,
    e_pa: f64,
    nu: f64,
    thickness_m: f64,
    rho_kg_m3: f64,
    a_m: f64,
    b_m: f64,
) -> f64 {
    let d = e_pa * thickness_m.powi(3) / (12.0 * (1.0 - nu * nu));
    let mass_per_area = rho_kg_m3 * thickness_m;
    let factor = (mode_m as f64 / a_m).powi(2) + (mode_n as f64 / b_m).powi(2);
    PI / 2.0 * (d / mass_per_area).sqrt() * factor
}

pub fn damped_frequency(omega_n: f64, zeta: f64) -> f64 {
    omega_n * (1.0 - zeta * zeta).max(0.0).sqrt()
}

pub fn logarithmic_decrement(zeta: f64) -> f64 {
    2.0 * PI * zeta / (1.0 - zeta * zeta).max(0.0).sqrt()
}

pub fn damping_ratio_from_log_decrement(delta: f64) -> f64 {
    delta / (4.0 * PI * PI + delta * delta).sqrt()
}

pub fn q_factor_to_zeta(q: f64) -> f64 {
    1.0 / (2.0 * q)
}
