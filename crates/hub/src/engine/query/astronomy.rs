//! Catalog entries for astronomy functions.

use super::FunctionInfo;
use super::reg;
use crate::engine::experience::experiment::DomainType::Astronomy;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        Astronomy,
        "kepler_period",
        &["a", "mu"],
        "Orbital period from semi-major axis and gravitational parameter",
    );
    reg(
        e,
        Astronomy,
        "kepler_velocity",
        &["mu", "r", "a"],
        "Vis-viva orbital speed",
    );
    reg(
        e,
        Astronomy,
        "circular_velocity",
        &["mu", "r"],
        "Circular orbit velocity",
    );
    reg(
        e,
        Astronomy,
        "escape_velocity",
        &["mu", "r"],
        "Escape velocity at distance r",
    );
    reg(
        e,
        Astronomy,
        "vis_viva",
        &["mu", "r", "a"],
        "Vis-viva speed",
    );
    reg(
        e,
        Astronomy,
        "orbital_energy",
        &["mu", "a"],
        "Specific orbital energy",
    );
    reg(
        e,
        Astronomy,
        "angular_momentum",
        &["mu", "a", "e"],
        "Specific angular momentum",
    );
    reg(e, Astronomy, "periapsis", &["a", "e"], "Periapsis distance");
    reg(e, Astronomy, "apoapsis", &["a", "e"], "Apoapsis distance");
    reg(
        e,
        Astronomy,
        "true_anomaly_to_radius",
        &["a", "e", "theta"],
        "Radius at true anomaly",
    );
    reg(
        e,
        Astronomy,
        "hohmann_delta_v",
        &["mu", "r1", "r2"],
        "Hohmann transfer delta-v",
    );
    reg(
        e,
        Astronomy,
        "sphere_of_influence",
        &["a", "m_planet", "m_star"],
        "Sphere of influence radius",
    );
    reg(
        e,
        Astronomy,
        "roche_limit",
        &["r_primary", "rho_primary", "rho_secondary"],
        "Roche limit distance",
    );
    reg(
        e,
        Astronomy,
        "luminosity_from_radius_temp",
        &["r", "t"],
        "Stefan-Boltzmann luminosity",
    );
    reg(
        e,
        Astronomy,
        "absolute_magnitude",
        &["apparent_mag", "distance_pc"],
        "Absolute magnitude from apparent + distance",
    );
    reg(
        e,
        Astronomy,
        "distance_modulus",
        &["apparent_mag", "absolute_mag"],
        "Distance modulus",
    );
    reg(
        e,
        Astronomy,
        "stellar_flux",
        &["luminosity", "distance"],
        "Flux at distance",
    );
    reg(
        e,
        Astronomy,
        "wien_peak_wavelength",
        &["temperature"],
        "Wien displacement peak wavelength",
    );
    reg(
        e,
        Astronomy,
        "main_sequence_lifetime",
        &["mass_solar", "luminosity_solar"],
        "Main-sequence lifetime",
    );
    reg(
        e,
        Astronomy,
        "schwarzschild_radius",
        &["mass"],
        "Schwarzschild radius",
    );
    reg(
        e,
        Astronomy,
        "chandrasekhar_limit",
        &[],
        "Chandrasekhar mass limit",
    );
    reg(
        e,
        Astronomy,
        "eddington_luminosity",
        &["mass"],
        "Eddington luminosity",
    );
    reg(
        e,
        Astronomy,
        "hubble_velocity",
        &["h0", "distance"],
        "Hubble recession velocity",
    );
    reg(
        e,
        Astronomy,
        "comoving_distance_from_z",
        &["h0", "omega_m", "z"],
        "Comoving distance from redshift (flat ΛCDM)",
    );
    reg(
        e,
        Astronomy,
        "luminosity_distance_from_z",
        &["h0", "omega_m", "z"],
        "Luminosity distance from redshift",
    );
    reg(
        e,
        Astronomy,
        "angular_diameter_distance_from_z",
        &["h0", "omega_m", "z"],
        "Angular diameter distance from redshift",
    );
    reg(
        e,
        Astronomy,
        "lookback_time_from_z",
        &["h0", "omega_m", "z"],
        "Lookback time from redshift",
    );
    reg(
        e,
        Astronomy,
        "age_lcdm",
        &["h0", "omega_m"],
        "Age of universe (flat ΛCDM)",
    );
    reg(
        e,
        Astronomy,
        "e_z_lcdm",
        &["omega_m", "z"],
        "E(z) for flat ΛCDM",
    );
    reg(
        e,
        Astronomy,
        "hubble_at_z",
        &["h0", "omega_m", "omega_r", "omega_k", "omega_de", "z"],
        "H(z) general",
    );
    reg(
        e,
        Astronomy,
        "growth_factor_approx",
        &["omega_m", "z"],
        "Carroll et al. 1992 growth factor",
    );
    reg(
        e,
        Astronomy,
        "einstein_radius",
        &["h0", "omega_m", "mass", "z_l", "z_s"],
        "Einstein radius for gravitational lensing",
    );
    reg(
        e,
        Astronomy,
        "gravitational_force",
        &["m1", "m2", "r"],
        "Newton gravitational force",
    );
    reg(
        e,
        Astronomy,
        "surface_gravity",
        &["m", "r"],
        "Surface gravity",
    );
    reg(
        e,
        Astronomy,
        "hill_sphere",
        &["a", "m", "m_star", "e"],
        "Hill sphere radius",
    );
}
