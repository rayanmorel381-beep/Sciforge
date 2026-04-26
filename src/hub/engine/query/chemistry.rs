//! Catalog entries for chemistry functions.

use super::FunctionInfo;
use super::reg;
use crate::hub::engine::experience::experiment::DomainType::Chemistry;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        Chemistry,
        "henderson_hasselbalch",
        &["pka", "a_conc", "ha_conc"],
        "Henderson-Hasselbalch pH",
    );
    reg(
        e,
        Chemistry,
        "ph_strong_acid",
        &["concentration"],
        "Strong acid pH",
    );
    reg(
        e,
        Chemistry,
        "ph_strong_base",
        &["concentration"],
        "Strong base pH",
    );
    reg(
        e,
        Chemistry,
        "nernst_potential",
        &["e0", "n", "q_r", "t"],
        "Nernst equation cell potential",
    );
    reg(
        e,
        Chemistry,
        "arrhenius_rate",
        &["a", "ea", "t"],
        "Arrhenius rate constant",
    );
    reg(
        e,
        Chemistry,
        "ideal_gas_volume",
        &["n", "t", "p"],
        "Ideal gas volume",
    );
    reg(
        e,
        Chemistry,
        "boyle_p2",
        &["p1", "v1", "v2"],
        "Boyle's law P₂",
    );
    reg(
        e,
        Chemistry,
        "charles_v2",
        &["v1", "t1", "t2"],
        "Charles's law V₂",
    );
    reg(
        e,
        Chemistry,
        "molarity",
        &["moles", "volume_l"],
        "Molarity calculation",
    );
    reg(
        e,
        Chemistry,
        "dilution_volume",
        &["c1", "v1", "c2"],
        "Dilution V₂ = C₁V₁/C₂",
    );
    reg(
        e,
        Chemistry,
        "bond_enthalpy_reaction",
        &["bonds_broken", "bonds_formed"],
        "Bond enthalpy ΔH",
    );
    reg(
        e,
        Chemistry,
        "rate_law_first_order",
        &["k", "a0", "t"],
        "First-order rate law [A](t)",
    );
}
