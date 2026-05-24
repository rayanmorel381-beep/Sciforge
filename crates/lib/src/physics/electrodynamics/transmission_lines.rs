use std::f64::consts::PI;

pub fn characteristic_impedance_lc(inductance_h_per_m: f64, capacitance_f_per_m: f64) -> f64 {
    (inductance_h_per_m / capacitance_f_per_m).sqrt()
}

pub fn characteristic_impedance_rlgc(
    r_ohm_per_m: f64,
    l_h_per_m: f64,
    g_s_per_m: f64,
    c_f_per_m: f64,
    omega: f64,
) -> (f64, f64) {
    let z_re = r_ohm_per_m;
    let z_im = omega * l_h_per_m;
    let y_re = g_s_per_m;
    let y_im = omega * c_f_per_m;
    let mag = ((z_re * z_re + z_im * z_im) / (y_re * y_re + y_im * y_im)).sqrt().sqrt();
    let phase = 0.5 * (z_im.atan2(z_re) - y_im.atan2(y_re));
    (mag * phase.cos(), mag * phase.sin())
}

pub fn coaxial_impedance(
    epsilon_r: f64,
    r_outer_m: f64,
    r_inner_m: f64,
) -> f64 {
    60.0 / epsilon_r.sqrt() * (r_outer_m / r_inner_m).ln()
}

pub fn microstrip_impedance(width_m: f64, height_m: f64, epsilon_r: f64) -> f64 {
    let w_h = width_m / height_m;
    let eps_eff = (epsilon_r + 1.0) / 2.0
        + (epsilon_r - 1.0) / 2.0 * (1.0 + 12.0 / w_h).powf(-0.5);
    if w_h < 1.0 {
        60.0 / eps_eff.sqrt() * (8.0 / w_h + w_h / 4.0).ln()
    } else {
        120.0 * PI / (eps_eff.sqrt() * (w_h + 1.393 + 0.667 * (w_h + 1.444).ln()))
    }
}

pub fn microstrip_effective_permittivity(width_m: f64, height_m: f64, epsilon_r: f64) -> f64 {
    let w_h = width_m / height_m;
    (epsilon_r + 1.0) / 2.0 + (epsilon_r - 1.0) / 2.0 * (1.0 + 12.0 / w_h).powf(-0.5)
}

pub fn stripline_impedance(width_m: f64, separation_m: f64, epsilon_r: f64) -> f64 {
    let w_b = width_m / separation_m;
    60.0 / epsilon_r.sqrt() * (4.0 * separation_m / (PI * width_m * (1.0 + w_b))).ln()
}

pub fn reflection_coefficient(z_load: f64, z_0: f64) -> f64 {
    (z_load - z_0) / (z_load + z_0)
}

pub fn reflection_coefficient_complex(
    z_load_re: f64,
    z_load_im: f64,
    z_0: f64,
) -> (f64, f64) {
    let num_re = z_load_re - z_0;
    let num_im = z_load_im;
    let den_re = z_load_re + z_0;
    let den_im = z_load_im;
    let den_mag2 = den_re * den_re + den_im * den_im;
    (
        (num_re * den_re + num_im * den_im) / den_mag2,
        (num_im * den_re - num_re * den_im) / den_mag2,
    )
}

pub fn vswr_from_reflection(reflection_magnitude: f64) -> f64 {
    (1.0 + reflection_magnitude.abs()) / (1.0 - reflection_magnitude.abs())
}

pub fn return_loss_db(reflection_magnitude: f64) -> f64 {
    -20.0 * reflection_magnitude.abs().log10()
}

pub fn insertion_loss_db(s21_magnitude: f64) -> f64 {
    -20.0 * s21_magnitude.log10()
}

pub fn input_impedance(
    z_load_re: f64,
    z_load_im: f64,
    z_0: f64,
    beta_per_m: f64,
    length_m: f64,
) -> (f64, f64) {
    let tan_bl = (beta_per_m * length_m).tan();
    let nr = z_load_re;
    let ni = z_load_im + z_0 * tan_bl;
    let dr = z_0 - z_load_im * tan_bl;
    let di = z_load_re * tan_bl;
    let dm2 = dr * dr + di * di;
    (
        z_0 * (nr * dr + ni * di) / dm2,
        z_0 * (ni * dr - nr * di) / dm2,
    )
}

pub fn quarter_wave_transformer(z_source: f64, z_load: f64) -> f64 {
    (z_source * z_load).sqrt()
}

pub fn propagation_velocity(epsilon_r: f64) -> f64 {
    crate::constants::C / epsilon_r.sqrt()
}

pub fn attenuation_constant_low_loss(
    r_ohm_per_m: f64,
    g_s_per_m: f64,
    z_0: f64,
) -> f64 {
    0.5 * (r_ohm_per_m / z_0 + g_s_per_m * z_0)
}

pub fn phase_constant(omega_rad_per_s: f64, l_h_per_m: f64, c_f_per_m: f64) -> f64 {
    omega_rad_per_s * (l_h_per_m * c_f_per_m).sqrt()
}

pub fn smith_chart_normalized(z_re: f64, z_im: f64, z_0: f64) -> (f64, f64) {
    (z_re / z_0, z_im / z_0)
}
