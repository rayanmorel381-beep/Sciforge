//! Catalog entries for biophysics functions.

use super::FunctionInfo;
use super::reg;
use crate::hub::engine::experience::experiment::DomainType::Biophysics;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        Biophysics,
        "diffusion_coefficient_stokes_einstein",
        &["temperature", "viscosity", "radius"],
        "Stokes-Einstein diffusion coefficient",
    );
    reg(
        e,
        Biophysics,
        "membrane_capacitance",
        &["area", "thickness", "dielectric_constant"],
        "Parallel-plate membrane capacitance",
    );
    reg(
        e,
        Biophysics,
        "stokes_drag_force",
        &["viscosity", "radius", "velocity"],
        "Stokes drag on a sphere",
    );
    reg(
        e,
        Biophysics,
        "sedimentation_coefficient",
        &[
            "particle_mass",
            "solvent_density",
            "particle_density",
            "friction_coefficient",
        ],
        "Svedberg sedimentation coefficient",
    );
    reg(
        e,
        Biophysics,
        "thermal_fluctuation_amplitude",
        &["temperature", "spring_constant"],
        "RMS thermal fluctuation amplitude",
    );
    reg(
        e,
        Biophysics,
        "worm_like_chain_extension",
        &[
            "force",
            "contour_length",
            "persistence_length",
            "temperature",
        ],
        "WLC polymer extension under force",
    );
    reg(
        e,
        Biophysics,
        "reynolds_number",
        &["density", "velocity", "length", "viscosity"],
        "Reynolds number",
    );
    reg(
        e,
        Biophysics,
        "fick_diffusion_flux",
        &["diffusion_coeff", "concentration_gradient"],
        "Fick first law diffusion flux",
    );
    reg(
        e,
        Biophysics,
        "debye_screening_length",
        &["temperature", "ionic_strength", "dielectric_constant"],
        "Debye electrostatic screening length",
    );
    reg(
        e,
        Biophysics,
        "electrophoretic_mobility",
        &["charge", "friction_coefficient"],
        "Electrophoretic mobility",
    );
    reg(
        e,
        Biophysics,
        "helfrich_bending_energy",
        &[
            "bending_modulus",
            "mean_curvature",
            "spontaneous_curvature",
            "area",
        ],
        "Helfrich membrane bending energy",
    );
}
