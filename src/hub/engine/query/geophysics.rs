//! Catalog entries for geophysics functions.

use super::FunctionInfo;
use super::reg;
use crate::hub::engine::experience::experiment::DomainType::Geophysics;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        Geophysics,
        "radiogenic_heat_production",
        &["c_u238", "c_th232", "c_k40", "density"],
        "Radiogenic heat production from U/Th/K",
    );
    reg(
        e,
        Geophysics,
        "bouguer_anomaly",
        &[
            "observed_gravity",
            "reference_gravity",
            "elevation",
            "slab_density",
        ],
        "Bouguer gravity anomaly",
    );
    reg(
        e,
        Geophysics,
        "gravity_anomaly_buried_sphere",
        &["delta_rho", "radius", "center_depth", "horizontal_distance"],
        "Gravity anomaly of a buried sphere",
    );
    reg(
        e,
        Geophysics,
        "magnetic_anomaly_vertical_dipole",
        &["moment", "depth", "horizontal_distance"],
        "Magnetic anomaly of a vertical dipole",
    );
    reg(
        e,
        Geophysics,
        "seismic_wave_attenuation",
        &["amplitude_0", "frequency", "travel_time", "quality_factor"],
        "Seismic wave amplitude attenuation",
    );
    reg(
        e,
        Geophysics,
        "seismic_impedance_reflection",
        &["rho1", "v1", "rho2", "v2"],
        "Seismic impedance reflection coefficient",
    );
    reg(
        e,
        Geophysics,
        "lithospheric_flexure_wavelength",
        &[
            "flexural_rigidity",
            "mantle_density",
            "infill_density",
            "g_surface",
        ],
        "Lithospheric flexure wavelength",
    );
    reg(
        e,
        Geophysics,
        "isostatic_rebound_timescale",
        &["viscosity", "mantle_density", "g_surface", "wavelength"],
        "Post-glacial isostatic rebound timescale",
    );
    reg(
        e,
        Geophysics,
        "curie_depth",
        &[
            "surface_heat_flow",
            "thermal_conductivity",
            "curie_temperature",
            "surface_temperature",
        ],
        "Curie isotherm depth",
    );
    reg(
        e,
        Geophysics,
        "radiative_thermal_conductivity",
        &["temperature", "refractive_index", "absorption_coefficient"],
        "Radiative thermal conductivity in mantle",
    );
    reg(
        e,
        Geophysics,
        "electromagnetic_skin_depth",
        &["resistivity", "frequency"],
        "Electromagnetic skin depth",
    );
}
