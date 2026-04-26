//! Catalog entries for geochemistry functions.

use super::FunctionInfo;
use super::reg;
use crate::hub::engine::experience::experiment::DomainType::Geochemistry;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        Geochemistry,
        "partition_coefficient",
        &["c_solid", "c_liquid"],
        "Mineral-melt partition coefficient",
    );
    reg(
        e,
        Geochemistry,
        "batch_melting",
        &["c0", "melt_fraction", "bulk_d"],
        "Batch partial melting concentration",
    );
    reg(
        e,
        Geochemistry,
        "fractional_crystallization",
        &["c0", "fraction_remaining", "bulk_d"],
        "Rayleigh fractional crystallization",
    );
    reg(
        e,
        Geochemistry,
        "delta_notation",
        &["r_sample", "r_standard"],
        "Isotope delta notation in permil",
    );
    reg(
        e,
        Geochemistry,
        "rayleigh_fractionation",
        &["r0", "fraction_remaining", "alpha"],
        "Rayleigh isotope fractionation",
    );
    reg(
        e,
        Geochemistry,
        "weathering_rate",
        &["rate_constant", "surface_area", "saturation_state"],
        "Chemical weathering rate",
    );
    reg(
        e,
        Geochemistry,
        "activity_coefficient_debye_huckel",
        &["z", "ionic_strength"],
        "Debye-Huckel activity coefficient",
    );
    reg(
        e,
        Geochemistry,
        "solubility_product_temperature",
        &["ksp0", "delta_h", "t", "t0"],
        "Temperature-dependent solubility product",
    );
    reg(
        e,
        Geochemistry,
        "eh_ph_boundary",
        &["e0", "n_electrons", "n_protons", "ph", "temperature"],
        "Eh-pH diagram boundary",
    );
    reg(
        e,
        Geochemistry,
        "distribution_coefficient",
        &["c_adsorbed", "c_solution"],
        "Sorption distribution coefficient",
    );
    reg(
        e,
        Geochemistry,
        "mixing_two_component",
        &["c1", "c2", "fraction_1"],
        "Two-component mixing model",
    );
}
