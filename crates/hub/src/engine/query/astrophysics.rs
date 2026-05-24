//! Catalog entries for astrophysics functions.

use super::FunctionInfo;
use super::reg;
use crate::engine::experience::experiment::DomainType::Astrophysics;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        Astrophysics,
        "schwarzschild_radius",
        &["mass"],
        "Schwarzschild radius of a mass",
    );
    reg(
        e,
        Astrophysics,
        "eddington_luminosity",
        &["mass"],
        "Eddington luminosity limit",
    );
    reg(
        e,
        Astrophysics,
        "chandrasekhar_mass",
        &["mu_e"],
        "Chandrasekhar mass limit",
    );
    reg(
        e,
        Astrophysics,
        "virial_temperature",
        &["mass", "radius", "mean_molecular_weight"],
        "Virial temperature of a system",
    );
    reg(
        e,
        Astrophysics,
        "bondi_accretion_rate",
        &["mass", "density", "sound_speed"],
        "Bondi spherical accretion rate",
    );
    reg(
        e,
        Astrophysics,
        "compton_wavelength",
        &["mass"],
        "Compton wavelength of a particle",
    );
    reg(
        e,
        Astrophysics,
        "gravitational_redshift",
        &["mass", "radius"],
        "Gravitational redshift factor",
    );
    reg(
        e,
        Astrophysics,
        "synchrotron_critical_frequency",
        &["gamma_factor", "magnetic_field"],
        "Synchrotron critical frequency",
    );
    reg(
        e,
        Astrophysics,
        "photon_sphere_radius",
        &["mass"],
        "Photon sphere radius",
    );
    reg(
        e,
        Astrophysics,
        "hawking_temperature",
        &["mass"],
        "Hawking temperature of a black hole",
    );
    reg(
        e,
        Astrophysics,
        "relativistic_doppler",
        &["frequency", "velocity"],
        "Relativistic Doppler shift",
    );
}
