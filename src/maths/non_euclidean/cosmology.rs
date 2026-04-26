use crate::constants::{CMB_RECOMBINATION_REDSHIFT, SPEED_OF_LIGHT_KM_S};

pub fn flrw_scale_factor(h0: f64, omega_m: f64, omega_lambda: f64, t: f64) -> f64 {
    let dt = t / 10000.0;
    let mut a = 1e-6_f64;
    let mut time = 0.0;
    while time < t {
        let h = h0 * (omega_m / (a * a * a) + omega_lambda).sqrt();
        a += a * h * dt;
        time += dt;
    }
    a
}

pub fn hubble_parameter(h0: f64, omega_m: f64, omega_lambda: f64, z: f64) -> f64 {
    let a = 1.0 / (1.0 + z);
    h0 * (omega_m / (a * a * a) + omega_lambda).sqrt()
}

pub fn comoving_distance(h0: f64, omega_m: f64, omega_lambda: f64, z: f64) -> f64 {
    let n = 1000;
    let h = z / n as f64;
    let f = |zi: f64| SPEED_OF_LIGHT_KM_S / hubble_parameter(h0, omega_m, omega_lambda, zi);
    let mut s = f(1e-12) + f(z);
    for i in 1..n {
        let zi = i as f64 * h;
        s += if i % 2 == 0 { 2.0 } else { 4.0 } * f(zi);
    }
    s * h / 3.0
}

pub fn luminosity_distance(h0: f64, omega_m: f64, omega_lambda: f64, z: f64) -> f64 {
    comoving_distance(h0, omega_m, omega_lambda, z) * (1.0 + z)
}

pub fn angular_diameter_distance(h0: f64, omega_m: f64, omega_lambda: f64, z: f64) -> f64 {
    comoving_distance(h0, omega_m, omega_lambda, z) / (1.0 + z)
}

pub fn lookback_time(h0: f64, omega_m: f64, omega_lambda: f64, z: f64) -> f64 {
    let n = 1000;
    let h = z / n as f64;
    let f = |zi: f64| 1.0 / (hubble_parameter(h0, omega_m, omega_lambda, zi) * (1.0 + zi));
    let mut s = f(1e-12) + f(z);
    for i in 1..n {
        let zi = i as f64 * h;
        s += if i % 2 == 0 { 2.0 } else { 4.0 } * f(zi);
    }
    s * h / 3.0
}

pub fn distance_modulus(h0: f64, omega_m: f64, omega_lambda: f64, z: f64) -> f64 {
    let dl = luminosity_distance(h0, omega_m, omega_lambda, z);
    5.0 * dl.log10() + 25.0
}

pub fn age_of_universe(h0: f64, omega_m: f64, omega_lambda: f64) -> f64 {
    lookback_time(h0, omega_m, omega_lambda, CMB_RECOMBINATION_REDSHIFT)
}

pub fn critical_density(h0_km_s_mpc: f64) -> f64 {
    let h_si = h0_km_s_mpc * 1e3 / 3.0857e22;
    3.0 * h_si * h_si / (8.0 * std::f64::consts::PI * crate::constants::G)
}

pub fn deceleration_parameter(omega_m: f64, omega_lambda: f64) -> f64 {
    0.5 * omega_m - omega_lambda
}

pub fn cosmic_time_matter_dominated(h0: f64, z: f64) -> f64 {
    2.0 / (3.0 * h0 * (1.0 + z).powf(1.5))
}

pub fn horizon_distance(h0: f64, omega_m: f64, z: f64) -> f64 {
    2.0 * SPEED_OF_LIGHT_KM_S / (h0 * omega_m.sqrt() * (1.0 + z).sqrt())
}

pub fn dark_energy_equation_of_state(omega_lambda: f64, omega_m: f64, z: f64) -> f64 {
    let a = 1.0 / (1.0 + z);
    let rho_de = omega_lambda;
    let rho_m = omega_m / (a * a * a);
    -rho_de / (rho_m + rho_de)
}

pub fn proper_distance(h0: f64, omega_m: f64, omega_lambda: f64, z: f64) -> f64 {
    comoving_distance(h0, omega_m, omega_lambda, z) / (1.0 + z)
}

pub fn volume_comoving(h0: f64, omega_m: f64, omega_lambda: f64, z: f64) -> f64 {
    let dc = comoving_distance(h0, omega_m, omega_lambda, z);
    4.0 / 3.0 * std::f64::consts::PI * dc.powi(3)
}

pub fn cmb_temperature_at_redshift(t0: f64, z: f64) -> f64 {
    t0 * (1.0 + z)
}

pub fn recombination_redshift() -> f64 {
    CMB_RECOMBINATION_REDSHIFT
}

pub fn matter_radiation_equality_redshift(omega_m: f64, omega_r: f64) -> f64 {
    omega_m / omega_r - 1.0
}
