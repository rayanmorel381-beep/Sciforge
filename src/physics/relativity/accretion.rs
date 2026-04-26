use crate::constants::{C, G, K_B, MU_0, SIGMA_SB};

pub fn schwarzschild_radius(mass: f64) -> f64 {
    2.0 * G * mass / (C * C)
}

pub fn kerr_event_horizon(mass: f64, spin_parameter: f64) -> f64 {
    let rg = G * mass / (C * C);
    rg + (rg * rg - spin_parameter * spin_parameter).sqrt()
}

pub fn kerr_ergosphere(mass: f64, spin_parameter: f64, theta: f64) -> f64 {
    let rg = G * mass / (C * C);
    rg + (rg * rg - spin_parameter * spin_parameter * theta.cos().powi(2)).sqrt()
}

pub fn isco_radius(mass: f64, spin_parameter: f64) -> f64 {
    let rg = G * mass / (C * C);
    let a_star = spin_parameter / rg;
    if a_star.abs() < 1e-10 {
        return 6.0 * rg;
    }
    let z1 = 1.0
        + (1.0 - a_star * a_star).powf(1.0 / 3.0)
            * ((1.0 + a_star).powf(1.0 / 3.0) + (1.0 - a_star).powf(1.0 / 3.0));
    let z2 = (3.0 * a_star * a_star + z1 * z1).sqrt();
    let sign = if a_star >= 0.0 { 1.0 } else { -1.0 };
    rg * (3.0 + z2 - sign * ((3.0 - z1) * (3.0 + z1 + 2.0 * z2)).sqrt())
}

pub fn accretion_disk_temperature(mass: f64, accretion_rate: f64, r: f64, r_inner: f64) -> f64 {
    if r <= r_inner {
        return 0.0;
    }
    let correction = 1.0 - (r_inner / r).sqrt();
    let factor =
        3.0 * G * mass * accretion_rate / (8.0 * std::f64::consts::PI * SIGMA_SB * r.powi(3));
    (factor * correction).powf(0.25)
}

pub fn accretion_disk_luminosity(mass: f64, accretion_rate: f64, r_inner: f64) -> f64 {
    G * mass * accretion_rate / (2.0 * r_inner)
}

pub fn radiative_efficiency(r_isco: f64, mass: f64) -> f64 {
    let rg = G * mass / (C * C);
    1.0 - (1.0 - 2.0 * rg / (3.0 * r_isco)).sqrt()
}

pub fn eddington_accretion_rate(mass: f64) -> f64 {
    let l_edd = 4.0 * std::f64::consts::PI * G * mass * C / 0.2;
    l_edd / (0.1 * C * C)
}

pub fn disk_surface_density(accretion_rate: f64, viscosity: f64, r: f64, r_inner: f64) -> f64 {
    let correction = 1.0 - (r_inner / r).sqrt();
    accretion_rate * correction / (3.0 * std::f64::consts::PI * viscosity)
}

pub fn alpha_viscosity(alpha: f64, sound_speed: f64, scale_height: f64) -> f64 {
    alpha * sound_speed * scale_height
}

pub fn disk_scale_height(sound_speed: f64, orbital_frequency: f64) -> f64 {
    sound_speed / orbital_frequency
}

pub fn orbital_frequency_kerr(mass: f64, r: f64, spin_parameter: f64) -> f64 {
    let rg = G * mass / (C * C);
    let r_32 = r.powf(1.5);
    C * (rg).sqrt() / (r_32 + spin_parameter * (rg).sqrt())
}

pub fn gravitational_redshift(mass: f64, r: f64) -> f64 {
    let rs = schwarzschild_radius(mass);
    if r <= rs {
        return f64::INFINITY;
    }
    1.0 / (1.0 - rs / r).sqrt() - 1.0
}

pub fn doppler_beaming_factor(velocity_los: f64, bulk_lorentz: f64) -> f64 {
    let beta = velocity_los / C;
    1.0 / (bulk_lorentz * (1.0 - beta))
}

pub fn apparent_superluminal_velocity(beta: f64, viewing_angle: f64) -> f64 {
    beta * viewing_angle.sin() / (1.0 - beta * viewing_angle.cos())
}

pub fn blandford_znajek_power(
    magnetic_field: f64,
    event_horizon_radius: f64,
    spin_parameter: f64,
    mass: f64,
) -> f64 {
    let rg = G * mass / (C * C);
    let a_star = spin_parameter / rg;
    let omega_h = a_star * C / (4.0 * event_horizon_radius);
    let phi = magnetic_field.powi(2) * event_horizon_radius.powi(2);
    phi * (omega_h * event_horizon_radius / C).powi(2) / (6.0 * MU_0)
}

pub fn jet_lorentz_factor_from_ratio(jet_power: f64, mass_loading_rate: f64) -> f64 {
    jet_power / (mass_loading_rate * C * C)
}

pub fn synchrotron_cooling_time(electron_gamma: f64, magnetic_field: f64) -> f64 {
    let mu_0 = MU_0;
    let sigma_t = 6.652_458_7e-29;
    let m_e = 9.109_383_7e-31;
    let u_b = magnetic_field.powi(2) / (2.0 * mu_0);
    6.0 * std::f64::consts::PI * m_e * C / (sigma_t * u_b * electron_gamma)
}

pub fn photon_ring_radius(mass: f64) -> f64 {
    3.0 * G * mass / (C * C)
}

pub fn shadow_angular_radius(mass: f64, distance: f64) -> f64 {
    let r_ph = photon_ring_radius(mass);
    let critical_impact = r_ph * 3.0_f64.sqrt();
    critical_impact / distance
}

pub fn advection_dominated_temperature(proton_mass: f64, r: f64, mass: f64) -> f64 {
    let rg = G * mass / (C * C);
    proton_mass * C * C * rg / (3.0 * K_B * r)
}

pub fn comptonization_parameter(electron_temperature: f64, optical_depth: f64) -> f64 {
    let m_e_c2 = 9.109_383_7e-31 * C * C;
    4.0 * K_B * electron_temperature / m_e_c2 * (optical_depth + optical_depth.powi(2))
}

pub fn bondi_accretion_rate(mass: f64, ambient_density: f64, sound_speed: f64) -> f64 {
    4.0 * std::f64::consts::PI * G.powi(2) * mass.powi(2) * ambient_density / sound_speed.powi(3)
}

pub fn tidal_disruption_radius(bh_mass: f64, star_mass: f64, star_radius: f64) -> f64 {
    star_radius * (bh_mass / star_mass).powf(1.0 / 3.0)
}
