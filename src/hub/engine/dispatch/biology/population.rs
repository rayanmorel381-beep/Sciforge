//! Dispatch handler for population biology functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "leslie_matrix_multiply" => Ok(RunOutput::Vector(
            bio::population::age_structure::leslie_matrix_multiply(
                get_m(p, "matrix")?,
                get_v(p, "population")?,
            ),
        )),
        "stable_age_distribution" => Ok(RunOutput::Vector(
            bio::population::age_structure::stable_age_distribution(
                get_v(p, "fecundities")?,
                get_v(p, "survivals")?,
            ),
        )),
        "reproductive_value" => Ok(RunOutput::Vector(
            bio::population::age_structure::reproductive_value(
                get_v(p, "lx")?,
                get_v(p, "mx")?,
                get_f(p, "lambda")?,
            ),
        )),
        "euler_lotka_lambda" => Ok(RunOutput::Scalar(
            bio::population::age_structure::euler_lotka_lambda(get_v(p, "lx")?, get_v(p, "mx")?),
        )),
        "sensitivity_element" => Ok(RunOutput::Scalar(
            bio::population::age_structure::sensitivity_element(
                get_f(p, "v_i")?,
                get_f(p, "w_j")?,
                get_f(p, "vw_dot")?,
            ),
        )),
        "elasticity_element" => Ok(RunOutput::Scalar(
            bio::population::age_structure::elasticity_element(
                get_f(p, "sensitivity")?,
                get_f(p, "a_ij")?,
                get_f(p, "lambda")?,
            ),
        )),
        "sir_model" => {
            let (a, b, c) = bio::population::epidemiology::sir_model(
                get_f(p, "s")?,
                get_f(p, "i")?,
                get_f(p, "r")?,
                get_f(p, "beta")?,
                get_f(p, "gamma")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "sir_solve" => Ok(RunOutput::Matrix(
            bio::population::epidemiology::sir_solve(
                get_f(p, "s0")?,
                get_f(p, "i0")?,
                get_f(p, "r0")?,
                get_f(p, "beta")?,
                get_f(p, "gamma")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            )
            .into_iter()
            .map(|(a, b, c)| vec![a, b, c])
            .collect(),
        )),
        "seir_model" => {
            let r = bio::population::epidemiology::seir_model(
                get_f(p, "s")?,
                get_f(p, "e")?,
                get_f(p, "i")?,
                get_f(p, "r")?,
                get_f(p, "beta")?,
                get_f(p, "sigma")?,
                get_f(p, "gamma")?,
            );
            Ok(RunOutput::Vector(vec![r.0, r.1, r.2, r.3]))
        }
        "seir_solve" => Ok(RunOutput::Matrix(
            bio::population::epidemiology::seir_solve(
                get_f(p, "s0")?,
                get_f(p, "e0")?,
                get_f(p, "i0")?,
                get_f(p, "r0")?,
                get_f(p, "beta")?,
                get_f(p, "sigma")?,
                get_f(p, "gamma")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            )
            .into_iter()
            .map(|(a, b, c, d)| vec![a, b, c, d])
            .collect(),
        )),
        "sis_model" => {
            let (a, b) = bio::population::epidemiology::sis_model(
                get_f(p, "s")?,
                get_f(p, "i")?,
                get_f(p, "beta")?,
                get_f(p, "gamma")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "sirs_model" => {
            let (a, b, c) = bio::population::epidemiology::sirs_model(
                get_f(p, "s")?,
                get_f(p, "i")?,
                get_f(p, "r")?,
                get_f(p, "beta")?,
                get_f(p, "gamma")?,
                get_f(p, "xi")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "pop_basic_reproduction_number" => Ok(RunOutput::Scalar(
            bio::population::epidemiology::basic_reproduction_number(
                get_f(p, "beta")?,
                get_f(p, "gamma")?,
            ),
        )),
        "pop_herd_immunity_threshold" => Ok(RunOutput::Scalar(
            bio::population::epidemiology::herd_immunity_threshold(get_f(p, "r0")?),
        )),
        "final_size_equation" => Ok(RunOutput::Scalar(
            bio::population::epidemiology::final_size_equation(
                get_f(p, "r0")?,
                get_f(p, "tolerance")?,
                get_u(p, "max_iter")?,
            ),
        )),
        "pop_generation_time" => Ok(RunOutput::Scalar(
            bio::population::epidemiology::generation_time(
                get_f(p, "incubation")?,
                get_f(p, "infectious_period")?,
            ),
        )),
        "logistic_growth" => Ok(RunOutput::Scalar(bio::population::growth::logistic_growth(
            get_f(p, "n")?,
            get_f(p, "r")?,
            get_f(p, "k")?,
        ))),
        "logistic_solve" => Ok(RunOutput::Vector(bio::population::growth::logistic_solve(
            get_f(p, "n0")?,
            get_f(p, "r")?,
            get_f(p, "k")?,
            get_f(p, "dt")?,
            get_u(p, "steps")?,
        ))),
        "exponential_growth" => Ok(RunOutput::Scalar(
            bio::population::growth::exponential_growth(
                get_f(p, "n0")?,
                get_f(p, "r")?,
                get_f(p, "t")?,
            ),
        )),
        "gompertz" => Ok(RunOutput::Scalar(bio::population::growth::gompertz(
            get_f(p, "n")?,
            get_f(p, "a")?,
            get_f(p, "b")?,
            get_f(p, "k")?,
        ))),
        "allee_effect" => Ok(RunOutput::Scalar(bio::population::growth::allee_effect(
            get_f(p, "n")?,
            get_f(p, "r")?,
            get_f(p, "k")?,
            get_f(p, "a")?,
        ))),
        "beverton_holt" => Ok(RunOutput::Scalar(bio::population::growth::beverton_holt(
            get_f(p, "n")?,
            get_f(p, "r")?,
            get_f(p, "k")?,
        ))),
        "ricker" => Ok(RunOutput::Scalar(bio::population::growth::ricker(
            get_f(p, "n")?,
            get_f(p, "r")?,
            get_f(p, "k")?,
        ))),
        "pop_doubling_time" => Ok(RunOutput::Scalar(bio::population::growth::doubling_time(
            get_f(p, "r")?,
        ))),
        "von_bertalanffy" => Ok(RunOutput::Scalar(bio::population::growth::von_bertalanffy(
            get_f(p, "l_inf")?,
            get_f(p, "k")?,
            get_f(p, "t")?,
            get_f(p, "t0")?,
        ))),
        "theta_logistic" => Ok(RunOutput::Scalar(bio::population::growth::theta_logistic(
            get_f(p, "n")?,
            get_f(p, "r")?,
            get_f(p, "k")?,
            get_f(p, "theta")?,
        ))),
        "moran_process_fixation" => Ok(RunOutput::Scalar(
            bio::population::growth::moran_process_fixation(get_u(p, "n")?, get_f(p, "r")?),
        )),
        "lotka_volterra" => {
            let (a, b) = bio::population::predation::lotka_volterra(
                get_f(p, "prey")?,
                get_f(p, "pred")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "delta")?,
                get_f(p, "gamma")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "lotka_volterra_solve" => Ok(RunOutput::PairVec(
            bio::population::predation::lotka_volterra_solve(
                get_f(p, "prey0")?,
                get_f(p, "pred0")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "delta")?,
                get_f(p, "gamma")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            ),
        )),
        "competitive_lotka_volterra" => {
            let (a, b) = bio::population::predation::competitive_lotka_volterra(
                get_f(p, "n1")?,
                get_f(p, "n2")?,
                get_f(p, "r1")?,
                get_f(p, "r2")?,
                get_f(p, "k1")?,
                get_f(p, "k2")?,
                get_f(p, "a12")?,
                get_f(p, "a21")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "holling_type_ii" => Ok(RunOutput::Scalar(
            bio::population::predation::holling_type_ii(
                get_f(p, "prey")?,
                get_f(p, "attack_rate")?,
                get_f(p, "handling_time")?,
            ),
        )),
        "holling_type_iii" => Ok(RunOutput::Scalar(
            bio::population::predation::holling_type_iii(
                get_f(p, "prey")?,
                get_f(p, "attack_rate")?,
                get_f(p, "handling_time")?,
            ),
        )),
        "pop_rosenzweig_macarthur" => {
            let (a, b) = bio::population::predation::rosenzweig_macarthur(
                get_f(p, "prey")?,
                get_f(p, "pred")?,
                get_f(p, "r")?,
                get_f(p, "k")?,
                get_f(p, "a")?,
                get_f(p, "h")?,
                get_f(p, "e")?,
                get_f(p, "d")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "beddington_deangelis" => Ok(RunOutput::Scalar(
            bio::population::predation::beddington_deangelis(
                get_f(p, "prey")?,
                get_f(p, "pred")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "c")?,
            ),
        )),
        "ratio_dependent" => Ok(RunOutput::Scalar(
            bio::population::predation::ratio_dependent(
                get_f(p, "prey")?,
                get_f(p, "pred")?,
                get_f(p, "a")?,
                get_f(p, "h")?,
            ),
        )),
        "intraguild_predation" => {
            let (a, b, c) = bio::population::predation::intraguild_predation(
                get_f(p, "prey")?,
                get_f(p, "meso")?,
                get_f(p, "top")?,
                get_f(p, "r")?,
                get_f(p, "k")?,
                get_f(p, "a_mp")?,
                get_f(p, "a_tp")?,
                get_f(p, "a_tm")?,
                get_f(p, "e_mp")?,
                get_f(p, "e_tp")?,
                get_f(p, "e_tm")?,
                get_f(p, "d_m")?,
                get_f(p, "d_t")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "apparent_competition" => {
            let (a, b, c) = bio::population::predation::apparent_competition(
                get_f(p, "prey1")?,
                get_f(p, "prey2")?,
                get_f(p, "pred")?,
                get_f(p, "a1")?,
                get_f(p, "a2")?,
                get_f(p, "r1")?,
                get_f(p, "r2")?,
                get_f(p, "k1")?,
                get_f(p, "k2")?,
                get_f(p, "e")?,
                get_f(p, "d")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "diffusion_dispersal" => Ok(RunOutput::Scalar(
            bio::population::spatial::diffusion_dispersal(
                get_f(p, "density")?,
                get_f(p, "diffusion_coeff")?,
                get_f(p, "gradient")?,
            ),
        )),
        "reaction_diffusion_fisher" => Ok(RunOutput::Scalar(
            bio::population::spatial::reaction_diffusion_fisher(
                get_f(p, "n")?,
                get_f(p, "r")?,
                get_f(p, "k")?,
                get_f(p, "d")?,
                get_f(p, "laplacian")?,
            ),
        )),
        "fisher_wave_speed" => Ok(RunOutput::Scalar(
            bio::population::spatial::fisher_wave_speed(get_f(p, "r")?, get_f(p, "d")?),
        )),
        "range_expansion_rate" => Ok(RunOutput::Scalar(
            bio::population::spatial::range_expansion_rate(
                get_f(p, "diffusion")?,
                get_f(p, "growth_rate")?,
            ),
        )),
        "pop_stepping_stone_migration" => Ok(RunOutput::Scalar(
            bio::population::spatial::stepping_stone_migration(
                get_f(p, "source_density")?,
                get_f(p, "target_density")?,
                get_f(p, "migration_rate")?,
            ),
        )),
        "pop_isolation_by_distance" => Ok(RunOutput::Scalar(
            bio::population::spatial::isolation_by_distance(get_f(p, "fst")?),
        )),
        "landscape_resistance" => Ok(RunOutput::Scalar(
            bio::population::spatial::landscape_resistance(
                get_f(p, "distance")?,
                get_f(p, "resistance_cost")?,
            ),
        )),
        "gravity_model_migration" => Ok(RunOutput::Scalar(
            bio::population::spatial::gravity_model_migration(
                get_f(p, "pop_source")?,
                get_f(p, "pop_dest")?,
                get_f(p, "distance")?,
                get_f(p, "alpha")?,
            ),
        )),
        "pop_corridor_effectiveness" => Ok(RunOutput::Scalar(
            bio::population::spatial::corridor_effectiveness(
                get_f(p, "width")?,
                get_f(p, "length")?,
                get_f(p, "habitat_quality")?,
                get_f(p, "species_mobility")?,
            ),
        )),
        "allee_effect_spatial" => Ok(RunOutput::Scalar(
            bio::population::spatial::allee_effect_spatial(
                get_f(p, "density")?,
                get_f(p, "allee_threshold")?,
                get_f(p, "r")?,
                get_f(p, "k")?,
            ),
        )),
        "kernel_based_dispersal" => Ok(RunOutput::Scalar(
            bio::population::spatial::kernel_based_dispersal(
                get_f(p, "distance")?,
                get_f(p, "alpha")?,
                get_f(p, "shape")?,
            ),
        )),
        "basic_reproduction_number" => Ok(RunOutput::Scalar(
            bio::population::epidemiology::basic_reproduction_number(
                get_f(p, "beta")?,
                get_f(p, "gamma")?,
            ),
        )),
        "corridor_effectiveness" => Ok(RunOutput::Scalar(
            bio::population::spatial::corridor_effectiveness(
                get_f(p, "width")?,
                get_f(p, "length")?,
                get_f(p, "habitat_quality")?,
                get_f(p, "species_mobility")?,
            ),
        )),
        "doubling_time" => Ok(RunOutput::Scalar(bio::population::growth::doubling_time(
            get_f(p, "r")?,
        ))),
        "generation_time" => Ok(RunOutput::Scalar(
            bio::population::epidemiology::generation_time(
                get_f(p, "incubation")?,
                get_f(p, "infectious_period")?,
            ),
        )),
        "herd_immunity_threshold" => Ok(RunOutput::Scalar(
            bio::population::epidemiology::herd_immunity_threshold(get_f(p, "r0")?),
        )),
        "isolation_by_distance" => Ok(RunOutput::Scalar(
            bio::population::spatial::isolation_by_distance(get_f(p, "fst")?),
        )),
        "stepping_stone_migration" => Ok(RunOutput::Scalar(
            bio::population::spatial::stepping_stone_migration(
                get_f(p, "source_density")?,
                get_f(p, "target_density")?,
                get_f(p, "migration_rate")?,
            ),
        )),
        "rosenzweig_macarthur" => {
            let (x, y) = bio::population::predation::rosenzweig_macarthur(
                get_f(p, "prey")?,
                get_f(p, "pred")?,
                get_f(p, "r")?,
                get_f(p, "k")?,
                get_f(p, "a")?,
                get_f(p, "h")?,
                get_f(p, "e")?,
                get_f(p, "d")?,
            );
            Ok(RunOutput::Pair(x, y))
        }
        "cohort_generation_time" => {
            let ages = get_v(p, "ages")?;
            let fec = get_v(p, "fecundity")?;
            let af: Vec<(f64, f64)> = ages.iter().zip(fec.iter()).map(|(&a, &f)| (a, f)).collect();
            Ok(RunOutput::Scalar(
                bio::population::age_structure::cohort_generation_time(&af, get_f(p, "lambda")?),
            ))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
