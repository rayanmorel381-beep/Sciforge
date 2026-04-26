//! Dispatch handler for biomathematics functions.

use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::cross_domain::biomathematics;
use crate::hub::engine::dispatch::params::*;
use crate::hub::engine::experience::runner::RunOutput;

/// Dispatches a biomathematics function call by name and returns the computed result.
pub fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "logistic_growth_rate" => Ok(RunOutput::Scalar(biomathematics::logistic_growth_rate(
            get_f(p, "r")?,
            get_f(p, "carrying_capacity")?,
            get_f(p, "population")?,
        ))),
        "lotka_volterra_prey_rate" => {
            Ok(RunOutput::Scalar(biomathematics::lotka_volterra_prey_rate(
                get_f(p, "growth_rate")?,
                get_f(p, "predation_rate")?,
                get_f(p, "prey")?,
                get_f(p, "predator")?,
            )))
        }
        "lotka_volterra_predator_rate" => Ok(RunOutput::Scalar(
            biomathematics::lotka_volterra_predator_rate(
                get_f(p, "conversion_rate")?,
                get_f(p, "death_rate")?,
                get_f(p, "prey")?,
                get_f(p, "predator")?,
            ),
        )),
        "sir_infection_rate" => Ok(RunOutput::Scalar(biomathematics::sir_infection_rate(
            get_f(p, "beta")?,
            get_f(p, "susceptible")?,
            get_f(p, "infected")?,
            get_f(p, "total")?,
        ))),
        "sir_recovery_rate" => Ok(RunOutput::Scalar(biomathematics::sir_recovery_rate(
            get_f(p, "gamma")?,
            get_f(p, "infected")?,
        ))),
        "basic_reproduction_number" => Ok(RunOutput::Scalar(
            biomathematics::basic_reproduction_number(get_f(p, "beta")?, get_f(p, "gamma")?),
        )),
        "shannon_diversity_index" => Ok(RunOutput::Scalar(
            biomathematics::shannon_diversity_index(get_v(p, "proportions")?),
        )),
        "simpson_diversity_index" => Ok(RunOutput::Scalar(
            biomathematics::simpson_diversity_index(get_v(p, "proportions")?),
        )),
        "molecular_clock_distance" => {
            Ok(RunOutput::Scalar(biomathematics::molecular_clock_distance(
                get_f(p, "substitution_rate")?,
                get_f(p, "time")?,
            )))
        }
        "coalescent_expected_time" => {
            Ok(RunOutput::Scalar(biomathematics::coalescent_expected_time(
                get_f(p, "effective_population")?,
                get_f(p, "n_lineages")?,
            )))
        }
        "wright_fisher_heterozygosity" => Ok(RunOutput::Scalar(
            biomathematics::wright_fisher_heterozygosity(
                get_f(p, "h0")?,
                get_f(p, "effective_population")?,
                get_f(p, "generations")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!(
            "biomathematics: unknown function: {func}"
        ))),
    }
}
