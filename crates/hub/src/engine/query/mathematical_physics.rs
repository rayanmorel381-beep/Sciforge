//! Catalog entries for mathematical physics functions.

use super::FunctionInfo;
use super::reg;
use crate::engine::experience::experiment::DomainType::MathematicalPhysics;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        MathematicalPhysics,
        "de_broglie_wavelength",
        &["momentum"],
        "De Broglie matter wavelength",
    );
    reg(
        e,
        MathematicalPhysics,
        "heisenberg_uncertainty_position",
        &["delta_p"],
        "Heisenberg minimum position uncertainty",
    );
    reg(
        e,
        MathematicalPhysics,
        "heisenberg_uncertainty_momentum",
        &["delta_x"],
        "Heisenberg minimum momentum uncertainty",
    );
    reg(
        e,
        MathematicalPhysics,
        "wkb_tunneling_probability",
        &["energy", "potential", "barrier_width", "particle_mass"],
        "WKB quantum tunneling probability",
    );
    reg(
        e,
        MathematicalPhysics,
        "partition_function_harmonic",
        &["omega", "temperature"],
        "Quantum harmonic oscillator partition function",
    );
    reg(
        e,
        MathematicalPhysics,
        "fermi_dirac_distribution",
        &["energy", "chemical_potential", "temperature"],
        "Fermi-Dirac distribution",
    );
    reg(
        e,
        MathematicalPhysics,
        "bose_einstein_distribution",
        &["energy", "chemical_potential", "temperature"],
        "Bose-Einstein distribution",
    );
    reg(
        e,
        MathematicalPhysics,
        "density_of_states_3d_free",
        &["energy", "volume", "mass"],
        "3D free particle density of states",
    );
    reg(
        e,
        MathematicalPhysics,
        "fourier_mode_frequency",
        &["mode_number", "length", "wave_speed"],
        "Standing wave mode frequency",
    );
    reg(
        e,
        MathematicalPhysics,
        "relativistic_energy",
        &["rest_mass", "momentum"],
        "Relativistic energy-momentum relation",
    );
    reg(
        e,
        MathematicalPhysics,
        "thermal_wavelength",
        &["mass", "temperature"],
        "Thermal de Broglie wavelength",
    );
}
