//! Catalog entries for astrobiology functions.

use super::FunctionInfo;
use super::reg;
use crate::engine::experience::experiment::DomainType::Astrobiology;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        Astrobiology,
        "planet_equilibrium_temperature",
        &["stellar_luminosity", "distance", "albedo"],
        "Planetary equilibrium temperature",
    );
    reg(
        e,
        Astrobiology,
        "habitable_zone_inner",
        &["luminosity"],
        "Inner habitable zone distance",
    );
    reg(
        e,
        Astrobiology,
        "habitable_zone_outer",
        &["luminosity"],
        "Outer habitable zone distance",
    );
    reg(
        e,
        Astrobiology,
        "atmospheric_escape_parameter",
        &[
            "temperature",
            "planet_mass",
            "planet_radius",
            "molecular_mass",
        ],
        "Jeans atmospheric escape parameter",
    );
    reg(
        e,
        Astrobiology,
        "tidal_locking_timescale",
        &[
            "planet_mass",
            "semi_major",
            "star_mass",
            "planet_radius",
            "tidal_quality",
        ],
        "Tidal locking timescale",
    );
    reg(
        e,
        Astrobiology,
        "energy_limited_mass_loss",
        &["xuv_flux", "efficiency", "planet_mass", "planet_radius"],
        "Energy-limited atmospheric mass loss",
    );
    reg(
        e,
        Astrobiology,
        "biosignature_column_density",
        &[
            "mixing_ratio",
            "surface_pressure",
            "gravity",
            "mean_molecular_mass",
        ],
        "Biosignature gas column density",
    );
    reg(
        e,
        Astrobiology,
        "uv_surface_flux",
        &["incident_flux", "optical_depth"],
        "UV surface flux with attenuation",
    );
    reg(
        e,
        Astrobiology,
        "photosynthetic_flux_limit",
        &["photon_flux", "quantum_efficiency", "photon_energy"],
        "Photosynthetic energy flux limit",
    );
    reg(
        e,
        Astrobiology,
        "drake_equation",
        &[
            "rate_star_formation",
            "fraction_planets",
            "habitable_per_system",
            "fraction_life",
            "fraction_intelligence",
            "fraction_communication",
            "civilization_lifetime",
        ],
        "Drake equation for civilizations",
    );
    reg(
        e,
        Astrobiology,
        "surface_gravity",
        &["mass", "radius"],
        "Planetary surface gravity",
    );
}
