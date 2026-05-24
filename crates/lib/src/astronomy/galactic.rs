use std::f64::consts::PI;

use crate::constants::{
    C, G, HUBBLE_FLOW_VELOCITY_PER_MPC, M31_DISTANCE, M31_MASS, MW_BULGE_RADIUS, MW_DISK_RADIUS,
    MW_DISK_SCALE_HEIGHT, MW_M31_RADIAL_VELOCITY, MW_MASS, MW_STAR_COUNT, MW_STELLAR_MASS,
    SGR_A_STAR_DISTANCE, SGR_A_STAR_MASS, SOLAR_MASS, SUN_GALACTIC_DISTANCE, SUN_GALACTIC_PERIOD,
    SUN_GALACTIC_VELOCITY,
};

pub fn galactic_rotation_velocity(r: f64) -> f64 {
    let m_enc = MW_MASS * SOLAR_MASS * (r / (r + MW_DISK_RADIUS));
    (G * m_enc / r).sqrt()
}

pub fn sun_orbital_period() -> f64 {
    SUN_GALACTIC_PERIOD
}

pub fn sun_orbital_velocity() -> f64 {
    SUN_GALACTIC_VELOCITY
}

pub fn sun_galactocentric_distance() -> f64 {
    SUN_GALACTIC_DISTANCE
}

pub fn galactic_mass_within_radius(r: f64) -> f64 {
    MW_MASS * SOLAR_MASS * (r / (r + MW_DISK_RADIUS))
}

pub fn sgr_a_schwarzschild_radius() -> f64 {
    2.0 * G * SGR_A_STAR_MASS * SOLAR_MASS / (C * C)
}

pub fn sgr_a_sphere_of_influence() -> f64 {
    G * SGR_A_STAR_MASS * SOLAR_MASS / (SUN_GALACTIC_VELOCITY * SUN_GALACTIC_VELOCITY)
}

pub fn sgr_a_distance() -> f64 {
    SGR_A_STAR_DISTANCE
}

pub fn m31_approach_time() -> f64 {
    M31_DISTANCE / MW_M31_RADIAL_VELOCITY
}

pub fn m31_reduced_mass() -> f64 {
    let m1 = MW_MASS * SOLAR_MASS;
    let m2 = M31_MASS * SOLAR_MASS;
    m1 * m2 / (m1 + m2)
}

pub fn hubble_recession_velocity(distance_mpc: f64) -> f64 {
    HUBBLE_FLOW_VELOCITY_PER_MPC * distance_mpc
}

pub fn galactic_disk_density(r: f64, z: f64) -> f64 {
    let sigma_0 = MW_STELLAR_MASS * SOLAR_MASS / (2.0 * PI * MW_DISK_RADIUS * MW_DISK_RADIUS);
    let radial = (-r / MW_DISK_RADIUS).exp();
    let vertical = (-z.abs() / MW_DISK_SCALE_HEIGHT).exp() / (2.0 * MW_DISK_SCALE_HEIGHT);
    sigma_0 * radial * vertical
}

pub fn galactic_escape_velocity(r: f64) -> f64 {
    let m_enc = MW_MASS * SOLAR_MASS * (r / (r + MW_DISK_RADIUS));
    (2.0 * G * m_enc / r).sqrt()
}

pub fn galactic_circular_velocity(r: f64) -> f64 {
    galactic_escape_velocity(r) / 2.0_f64.sqrt()
}

pub fn galactic_dynamical_time(r: f64) -> f64 {
    let v = galactic_rotation_velocity(r);
    2.0 * PI * r / v
}

pub fn stellar_number_density(r: f64, z: f64) -> f64 {
    let n_0 = MW_STAR_COUNT / (4.0 * PI * MW_DISK_RADIUS * MW_DISK_RADIUS * MW_DISK_SCALE_HEIGHT);
    n_0 * (-r / MW_DISK_RADIUS).exp() * (-z.abs() / MW_DISK_SCALE_HEIGHT).exp()
}

pub fn oort_a_constant(r: f64, dr: f64) -> f64 {
    let v1 = galactic_rotation_velocity(r);
    let v2 = galactic_rotation_velocity(r + dr);
    0.5 * (v1 / r - (v2 - v1) / dr)
}

pub fn oort_b_constant(r: f64, dr: f64) -> f64 {
    let v1 = galactic_rotation_velocity(r);
    let v2 = galactic_rotation_velocity(r + dr);
    -0.5 * (v1 / r + (v2 - v1) / dr)
}

pub fn epicyclic_frequency(r: f64, dr: f64) -> f64 {
    let a = oort_a_constant(r, dr);
    let b = oort_b_constant(r, dr);
    (-4.0 * b * (a - b)).sqrt()
}

pub fn bulge_mass_within(r: f64) -> f64 {
    let fraction = r.powi(3) / (r.powi(3) + MW_BULGE_RADIUS.powi(3));
    MW_STELLAR_MASS * SOLAR_MASS * 0.15 * fraction
}

pub fn tidal_radius(m_cluster: f64, r_galactic: f64) -> f64 {
    let m_galaxy = galactic_mass_within_radius(r_galactic);
    r_galactic * (m_cluster / (3.0 * m_galaxy)).powf(1.0 / 3.0)
}
