//! Catalog entries for atmospheric physics functions.

use super::FunctionInfo;
use super::reg;
use crate::engine::experience::experiment::DomainType::AtmosphericPhysics;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        AtmosphericPhysics,
        "planck_radiance",
        &["wavelength", "temperature"],
        "Planck spectral radiance",
    );
    reg(
        e,
        AtmosphericPhysics,
        "stefan_boltzmann_flux",
        &["temperature"],
        "Stefan-Boltzmann radiative flux",
    );
    reg(
        e,
        AtmosphericPhysics,
        "brightness_temperature",
        &["radiance", "wavelength"],
        "Brightness temperature from radiance",
    );
    reg(
        e,
        AtmosphericPhysics,
        "rayleigh_scattering_cross_section",
        &["wavelength", "refractive_index", "number_density"],
        "Rayleigh scattering cross section",
    );
    reg(
        e,
        AtmosphericPhysics,
        "optical_depth",
        &["cross_section", "number_density", "path_length"],
        "Optical depth",
    );
    reg(
        e,
        AtmosphericPhysics,
        "atmospheric_scale_height",
        &["temperature", "mean_molecular_mass", "gravity"],
        "Atmospheric pressure scale height",
    );
    reg(
        e,
        AtmosphericPhysics,
        "pressure_at_altitude",
        &["surface_pressure", "scale_height", "altitude"],
        "Barometric pressure at altitude",
    );
    reg(
        e,
        AtmosphericPhysics,
        "dry_adiabatic_lapse_rate",
        &["gravity", "specific_heat"],
        "Dry adiabatic lapse rate",
    );
    reg(
        e,
        AtmosphericPhysics,
        "wien_peak_wavelength",
        &["temperature"],
        "Wien displacement peak wavelength",
    );
    reg(
        e,
        AtmosphericPhysics,
        "schwarzschild_radiative_transfer",
        &["source_function", "initial_radiance", "optical_depth"],
        "Schwarzschild radiative transfer",
    );
    reg(
        e,
        AtmosphericPhysics,
        "effective_emission_temperature",
        &["outgoing_flux", "emissivity"],
        "Effective emission temperature",
    );
}
