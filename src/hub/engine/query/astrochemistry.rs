//! Catalog entries for astrochemistry functions.

use super::FunctionInfo;
use super::reg;
use crate::hub::engine::experience::experiment::DomainType::Astrochemistry;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        Astrochemistry,
        "jeans_mass",
        &["temperature", "number_density", "mean_molecular_weight"],
        "Jeans mass for gravitational collapse",
    );
    reg(
        e,
        Astrochemistry,
        "jeans_length",
        &["temperature", "number_density", "mean_molecular_weight"],
        "Jeans length for gravitational instability",
    );
    reg(
        e,
        Astrochemistry,
        "freefall_time",
        &["number_density", "mean_molecular_weight"],
        "Free-fall collapse timescale",
    );
    reg(
        e,
        Astrochemistry,
        "cloud_thermal_velocity",
        &["temperature", "mean_molecular_weight"],
        "Thermal velocity in molecular cloud",
    );
    reg(
        e,
        Astrochemistry,
        "bonnor_ebert_mass",
        &["temperature", "external_pressure", "mean_molecular_weight"],
        "Bonnor-Ebert critical mass",
    );
    reg(
        e,
        Astrochemistry,
        "photodissociation_rate",
        &[
            "unshielded_rate",
            "uv_field_habing",
            "shielding_factor",
            "visual_extinction",
        ],
        "Photodissociation rate with UV shielding",
    );
    reg(
        e,
        Astrochemistry,
        "thermal_desorption_rate",
        &["attempt_frequency", "binding_energy", "dust_temperature"],
        "Thermal desorption rate from dust grains",
    );
    reg(
        e,
        Astrochemistry,
        "h2_formation_rate_on_dust",
        &[
            "sticking_coefficient",
            "grain_cross_section",
            "grain_density",
            "temperature",
        ],
        "H₂ formation rate on dust grain surfaces",
    );
    reg(
        e,
        Astrochemistry,
        "saha_ionization_ratio",
        &[
            "temperature",
            "electron_density",
            "ionization_energy",
            "partition_ratio",
        ],
        "Saha ionization equilibrium ratio",
    );
    reg(
        e,
        Astrochemistry,
        "stroemgren_radius",
        &[
            "ionizing_photon_rate",
            "hydrogen_density",
            "recombination_coeff",
        ],
        "Strömgren sphere radius",
    );
    reg(
        e,
        Astrochemistry,
        "dust_equilibrium_temperature",
        &["luminosity", "distance", "absorption_efficiency"],
        "Dust grain equilibrium temperature",
    );
}
