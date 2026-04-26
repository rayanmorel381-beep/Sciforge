//! Catalog entries for physics functions.

use super::FunctionInfo;
use super::reg;
use crate::hub::engine::experience::experiment::DomainType::Physics;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        Physics,
        "lorentz_factor",
        &["v"],
        "Relativistic Lorentz factor γ",
    );
    reg(
        e,
        Physics,
        "time_dilation",
        &["t0", "v"],
        "Relativistic time dilation",
    );
    reg(
        e,
        Physics,
        "length_contraction",
        &["l0", "v"],
        "Relativistic length contraction",
    );
    reg(
        e,
        Physics,
        "relativistic_momentum",
        &["m", "v"],
        "Relativistic momentum",
    );
    reg(
        e,
        Physics,
        "relativistic_kinetic_energy",
        &["m", "v"],
        "Relativistic kinetic energy",
    );
    reg(
        e,
        Physics,
        "planck_distribution",
        &["frequency", "temperature"],
        "Planck blackbody spectral radiance",
    );
    reg(
        e,
        Physics,
        "photoelectric_energy",
        &["frequency", "work_function"],
        "Photoelectric kinetic energy",
    );
    reg(
        e,
        Physics,
        "de_broglie_wavelength",
        &["momentum"],
        "de Broglie wavelength",
    );
    reg(
        e,
        Physics,
        "uncertainty_momentum",
        &["delta_x"],
        "Heisenberg uncertainty for momentum",
    );
    reg(
        e,
        Physics,
        "coulomb_force",
        &["q1", "q2", "r"],
        "Coulomb force between charges",
    );
    reg(
        e,
        Physics,
        "electric_field",
        &["q", "r"],
        "Electric field at distance r",
    );
    reg(e, Physics, "ohm_voltage", &["i", "r"], "Ohm's law V=IR");
    reg(
        e,
        Physics,
        "carnot_efficiency",
        &["t_hot", "t_cold"],
        "Carnot efficiency",
    );
    reg(
        e,
        Physics,
        "ideal_gas_pressure",
        &["n", "t", "v"],
        "Ideal gas pressure PV=nRT",
    );
    reg(
        e,
        Physics,
        "snell_refraction",
        &["n1", "theta1", "n2"],
        "Snell's law refraction angle",
    );
    reg(
        e,
        Physics,
        "doppler_shift",
        &["f", "v_source", "v_observer", "v_sound"],
        "Doppler frequency shift",
    );
    reg(
        e,
        Physics,
        "stefan_boltzmann_luminosity",
        &["radius", "temperature"],
        "Stefan-Boltzmann luminosity",
    );
    reg(
        e,
        Physics,
        "magnetic_force",
        &["q", "v", "b"],
        "Lorentz magnetic force on charge",
    );
}
