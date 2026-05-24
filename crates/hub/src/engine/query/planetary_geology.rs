//! Catalog entries for planetary geology functions.

use super::FunctionInfo;
use super::reg;
use crate::engine::experience::experiment::DomainType::PlanetaryGeology;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        PlanetaryGeology,
        "impact_energy",
        &["projectile_mass", "impact_velocity"],
        "Impact kinetic energy",
    );
    reg(
        e,
        PlanetaryGeology,
        "crater_scaling_diameter",
        &[
            "projectile_diameter",
            "projectile_density",
            "target_density",
            "velocity",
            "gravity",
        ],
        "Pi-scaling crater diameter",
    );
    reg(
        e,
        PlanetaryGeology,
        "tidal_heating_rate",
        &[
            "eccentricity",
            "mean_motion",
            "planet_radius",
            "quality_param",
            "primary_mass",
            "semi_major",
        ],
        "Tidal dissipation heating rate",
    );
    reg(
        e,
        PlanetaryGeology,
        "surface_temperature_equilibrium",
        &["solar_flux", "albedo", "emissivity"],
        "Equilibrium surface temperature",
    );
    reg(
        e,
        PlanetaryGeology,
        "lava_flow_cooling_time",
        &["thickness", "thermal_diffusivity"],
        "Lava flow conductive cooling time",
    );
    reg(
        e,
        PlanetaryGeology,
        "regolith_depth",
        &["flux_rate", "surface_density", "time"],
        "Regolith gardening depth",
    );
    reg(
        e,
        PlanetaryGeology,
        "lithospheric_thickness",
        &[
            "thermal_conductivity",
            "heat_flow",
            "base_temp",
            "surface_temp",
        ],
        "Thermal lithospheric thickness",
    );
    reg(
        e,
        PlanetaryGeology,
        "gravitational_differentiation_time",
        &["radius", "density_diff", "gravity", "viscosity"],
        "Gravitational differentiation timescale",
    );
    reg(
        e,
        PlanetaryGeology,
        "crater_counting_surface_age",
        &["crater_density", "production_rate"],
        "Surface age from crater counting",
    );
    reg(
        e,
        PlanetaryGeology,
        "volcanic_effusion_rate",
        &["thermal_flux", "specific_heat", "delta_t", "latent_heat"],
        "Volcanic lava effusion rate",
    );
    reg(
        e,
        PlanetaryGeology,
        "ejecta_blanket_thickness",
        &["crater_radius", "distance_from_center", "rim_thickness"],
        "Ejecta blanket thickness profile",
    );
}
