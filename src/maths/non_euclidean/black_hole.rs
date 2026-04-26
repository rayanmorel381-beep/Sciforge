use crate::constants::{C, EPSILON_0, G};

pub fn schwarzschild_radius(mass_kg: f64) -> f64 {
    2.0 * G * mass_kg / (C * C)
}

pub fn proper_time_schwarzschild(coord_time: f64, r: f64, rs: f64) -> f64 {
    if r <= rs {
        return 0.0;
    }
    coord_time * (1.0 - rs / r).sqrt()
}

pub fn gravitational_redshift(freq_emit: f64, r_emit: f64, r_obs: f64, rs: f64) -> f64 {
    if r_emit <= rs || r_obs <= rs {
        return 0.0;
    }
    freq_emit * ((1.0 - rs / r_emit) / (1.0 - rs / r_obs)).sqrt()
}

pub fn kerr_ergosphere_radius(m: f64, a: f64, theta: f64) -> f64 {
    let rs = 2.0 * G * m / (C * C);
    0.5 * rs + (0.25 * rs * rs - a * a * theta.cos() * theta.cos()).sqrt()
}

pub fn isco_schwarzschild(rs: f64) -> f64 {
    3.0 * rs
}

pub fn photon_sphere_radius(rs: f64) -> f64 {
    1.5 * rs
}

pub fn hawking_temperature(mass_kg: f64) -> f64 {
    use crate::constants::{HBAR, K_B};
    HBAR * C * C * C / (8.0 * std::f64::consts::PI * G * mass_kg * K_B)
}

pub fn black_hole_entropy(mass_kg: f64) -> f64 {
    use crate::constants::{HBAR, K_B};
    let rs = schwarzschild_radius(mass_kg);
    let area = 4.0 * std::f64::consts::PI * rs * rs;
    let lp2 = HBAR * G / (C * C * C);
    K_B * area / (4.0 * lp2)
}

pub fn kerr_event_horizon(m: f64, a: f64) -> f64 {
    let rs = 2.0 * G * m / (C * C);
    rs / 2.0 + (rs * rs / 4.0 - a * a).max(0.0).sqrt()
}

pub fn kerr_cauchy_horizon(m: f64, a: f64) -> f64 {
    let rs = 2.0 * G * m / (C * C);
    rs / 2.0 - (rs * rs / 4.0 - a * a).max(0.0).sqrt()
}

pub fn schwarzschild_time_dilation(r: f64, rs: f64) -> f64 {
    if r <= rs {
        return 0.0;
    }
    (1.0 - rs / r).sqrt()
}

pub fn gravitational_lensing_angle(mass_kg: f64, impact_param: f64) -> f64 {
    4.0 * G * mass_kg / (C * C * impact_param)
}

pub fn orbital_velocity_schwarzschild(rs: f64, r: f64) -> f64 {
    if r <= 1.5 * rs {
        return C;
    }
    (0.5 * C * C * rs / r).sqrt()
}

pub fn tidal_force_schwarzschild(mass_kg: f64, r: f64, delta_r: f64) -> f64 {
    2.0 * G * mass_kg * delta_r / (r * r * r)
}

pub fn hawking_luminosity(mass_kg: f64) -> f64 {
    use crate::constants::HBAR;
    HBAR * C * C * C * C * C * C / (15360.0 * std::f64::consts::PI * G * G * mass_kg * mass_kg)
}

pub fn black_hole_evaporation_time(mass_kg: f64) -> f64 {
    use crate::constants::HBAR;
    5120.0 * std::f64::consts::PI * G * G * mass_kg * mass_kg * mass_kg / (HBAR * C * C * C * C)
}

pub fn bekenstein_bound(energy_j: f64, radius_m: f64) -> f64 {
    use crate::constants::{HBAR, K_B};
    2.0 * std::f64::consts::PI * energy_j * radius_m / (HBAR * C * K_B.ln())
}

pub fn penrose_energy_extraction(m: f64, a: f64) -> f64 {
    let rs = 2.0 * G * m / (C * C);
    let r_plus = rs / 2.0 + (rs * rs / 4.0 - a * a).max(0.0).sqrt();
    let m_irr = (r_plus * r_plus + a * a).sqrt() / (2.0 * G / (C * C));
    let _ = m_irr;
    let efficiency =
        1.0 - (0.5 + (0.25 - (a * C * C / (2.0 * G * m)).powi(2)).max(0.0).sqrt()).sqrt();
    m * C * C * efficiency
}

pub fn frame_dragging_rate(mass_kg: f64, angular_momentum: f64, r: f64) -> f64 {
    2.0 * G * angular_momentum / (C * C * r * r * r) + 0.0 * mass_kg
}

pub fn reissner_nordstrom_outer_horizon(mass_kg: f64, charge_c: f64) -> f64 {
    let rs = schwarzschild_radius(mass_kg);
    let rq2 = G * charge_c * charge_c / (4.0 * std::f64::consts::PI * EPSILON_0 * C * C * C * C);
    rs / 2.0 + (rs * rs / 4.0 - rq2).max(0.0).sqrt()
}
