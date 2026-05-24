//! Catalog entries for biology functions.

use super::FunctionInfo;
use super::reg;
use crate::engine::experience::experiment::DomainType::Biology;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        Biology,
        "exponential_growth",
        &["n0", "r", "t"],
        "Exponential population growth",
    );
    reg(
        e,
        Biology,
        "logistic_growth",
        &["n0", "r", "k", "t"],
        "Logistic population growth",
    );
    reg(
        e,
        Biology,
        "michaelis_menten",
        &["vmax", "km", "s"],
        "Michaelis-Menten enzyme kinetics",
    );
    reg(
        e,
        Biology,
        "competitive_inhibition",
        &["vmax", "km", "s", "i", "ki"],
        "Competitive enzyme inhibition",
    );
    reg(
        e,
        Biology,
        "hodgkin_huxley_ina",
        &["gna", "m", "h", "v", "ena"],
        "Hodgkin-Huxley Na+ current",
    );
    reg(
        e,
        Biology,
        "nernst_potential_bio",
        &["z", "c_out", "c_in", "t"],
        "Nernst potential (biology)",
    );
    reg(
        e,
        Biology,
        "hardy_weinberg_p2",
        &["p"],
        "Hardy-Weinberg p² frequency",
    );
    reg(
        e,
        Biology,
        "shannon_diversity",
        &["proportions"],
        "Shannon diversity index",
    );
    reg(
        e,
        Biology,
        "lotka_volterra_prey",
        &["n", "p", "alpha", "beta"],
        "Lotka-Volterra prey dN/dt",
    );
    reg(
        e,
        Biology,
        "lotka_volterra_predator",
        &["n", "p", "delta", "gamma"],
        "Lotka-Volterra predator dP/dt",
    );
    reg(
        e,
        Biology,
        "caloric_restriction_lifespan",
        &["base_lifespan", "restriction_fraction"],
        "CR lifespan extension",
    );
    reg(
        e,
        Biology,
        "telomere_shortening",
        &["initial_length", "rate", "divisions"],
        "Telomere length after divisions",
    );
}
