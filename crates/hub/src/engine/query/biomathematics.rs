//! Catalog entries for biomathematics functions.

use super::FunctionInfo;
use super::reg;
use crate::engine::experience::experiment::DomainType::Biomathematics;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        Biomathematics,
        "logistic_growth_rate",
        &["r", "carrying_capacity", "population"],
        "Logistic population growth rate",
    );
    reg(
        e,
        Biomathematics,
        "lotka_volterra_prey_rate",
        &["growth_rate", "predation_rate", "prey", "predator"],
        "Lotka-Volterra prey dynamics",
    );
    reg(
        e,
        Biomathematics,
        "lotka_volterra_predator_rate",
        &["conversion_rate", "death_rate", "prey", "predator"],
        "Lotka-Volterra predator dynamics",
    );
    reg(
        e,
        Biomathematics,
        "sir_infection_rate",
        &["beta", "susceptible", "infected", "total"],
        "SIR model infection rate",
    );
    reg(
        e,
        Biomathematics,
        "sir_recovery_rate",
        &["gamma", "infected"],
        "SIR model recovery rate",
    );
    reg(
        e,
        Biomathematics,
        "basic_reproduction_number",
        &["beta", "gamma"],
        "Basic reproduction number R0",
    );
    reg(
        e,
        Biomathematics,
        "shannon_diversity_index",
        &["proportions"],
        "Shannon diversity index",
    );
    reg(
        e,
        Biomathematics,
        "simpson_diversity_index",
        &["proportions"],
        "Simpson diversity index",
    );
    reg(
        e,
        Biomathematics,
        "molecular_clock_distance",
        &["substitution_rate", "time"],
        "Molecular clock genetic distance",
    );
    reg(
        e,
        Biomathematics,
        "coalescent_expected_time",
        &["effective_population", "n_lineages"],
        "Expected coalescent time",
    );
    reg(
        e,
        Biomathematics,
        "wright_fisher_heterozygosity",
        &["h0", "effective_population", "generations"],
        "Wright-Fisher heterozygosity decay",
    );
}
