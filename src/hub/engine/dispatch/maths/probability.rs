//! Dispatch handler for probability functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::maths;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "prob_uniform_pdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::uniform_pdf(
                get_f(p, "x")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
            ),
        )),
        "prob_uniform_cdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::uniform_cdf(
                get_f(p, "x")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
            ),
        )),
        "prob_normal_pdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::normal_pdf(
                get_f(p, "x")?,
                get_f(p, "mu")?,
                get_f(p, "sigma")?,
            ),
        )),
        "prob_normal_cdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::normal_cdf(
                get_f(p, "x")?,
                get_f(p, "mu")?,
                get_f(p, "sigma")?,
            ),
        )),
        "prob_exponential_pdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::exponential_pdf(get_f(p, "x")?, get_f(p, "lambda")?),
        )),
        "prob_exponential_cdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::exponential_cdf(get_f(p, "x")?, get_f(p, "lambda")?),
        )),
        "prob_poisson_pmf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::poisson_pmf(
                get_i(p, "k")? as u64,
                get_f(p, "lambda")?,
            ),
        )),
        "prob_binomial_pmf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::binomial_pmf(
                get_i(p, "k")? as u64,
                get_i(p, "n")? as u64,
                get_f(p, "p")?,
            ),
        )),
        "prob_geometric_pmf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::geometric_pmf(get_i(p, "k")? as u64, get_f(p, "p")?),
        )),
        "prob_gamma_pdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::gamma_pdf(
                get_f(p, "x")?,
                get_f(p, "k")?,
                get_f(p, "theta")?,
            ),
        )),
        "prob_beta_pdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::beta_pdf(
                get_f(p, "x")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
            ),
        )),
        "prob_cauchy_pdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::cauchy_pdf(
                get_f(p, "x")?,
                get_f(p, "x0")?,
                get_f(p, "gamma")?,
            ),
        )),
        "prob_chi_squared_pdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::chi_squared_pdf(
                get_f(p, "x")?,
                get_i(p, "k")? as u32,
            ),
        )),
        "prob_weibull_pdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::weibull_pdf(
                get_f(p, "x")?,
                get_f(p, "lambda")?,
                get_f(p, "k")?,
            ),
        )),
        "prob_weibull_cdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::weibull_cdf(
                get_f(p, "x")?,
                get_f(p, "lambda")?,
                get_f(p, "k")?,
            ),
        )),
        "prob_log_normal_pdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::log_normal_pdf(
                get_f(p, "x")?,
                get_f(p, "mu")?,
                get_f(p, "sigma")?,
            ),
        )),
        "prob_student_t_pdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::student_t_pdf(get_f(p, "x")?, get_f(p, "nu")?),
        )),
        "prob_pareto_pdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::pareto_pdf(
                get_f(p, "x")?,
                get_f(p, "x_m")?,
                get_f(p, "alpha")?,
            ),
        )),
        "prob_pareto_cdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::pareto_cdf(
                get_f(p, "x")?,
                get_f(p, "x_m")?,
                get_f(p, "alpha")?,
            ),
        )),
        "prob_laplace_pdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::laplace_pdf(
                get_f(p, "x")?,
                get_f(p, "mu")?,
                get_f(p, "b")?,
            ),
        )),
        "prob_rayleigh_pdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::rayleigh_pdf(get_f(p, "x")?, get_f(p, "sigma")?),
        )),
        "prob_rayleigh_cdf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::rayleigh_cdf(get_f(p, "x")?, get_f(p, "sigma")?),
        )),
        "prob_negative_binomial_pmf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::negative_binomial_pmf(
                get_i(p, "k")? as u64,
                get_i(p, "r")? as u64,
                get_f(p, "p")?,
            ),
        )),
        "prob_hypergeometric_pmf" => Ok(RunOutput::Scalar(
            maths::probability::distributions::hypergeometric_pmf(
                get_i(p, "k")? as u64,
                get_i(p, "big_n")? as u64,
                get_i(p, "big_k")? as u64,
                get_i(p, "n")? as u64,
            ),
        )),
        "prob_entropy_discrete" => Ok(RunOutput::Scalar(
            maths::probability::distributions::entropy_discrete(get_v(p, "probs")?),
        )),
        "prob_kl_divergence" => Ok(RunOutput::Scalar(
            maths::probability::distributions::kl_divergence(get_v(p, "p")?, get_v(p, "q")?),
        )),

        "transition_probability" => Ok(RunOutput::Scalar(
            maths::probability::markov::transition_probability(
                get_m(p, "matrix")?,
                get_u(p, "state")?,
                get_u(p, "next_state")?,
            ),
        )),
        "state_after_n_steps" => Ok(RunOutput::Vector(
            maths::probability::markov::state_after_n_steps(
                get_m(p, "matrix")?,
                get_v(p, "initial")?,
                get_u(p, "steps")?,
            ),
        )),
        "steady_state" => Ok(RunOutput::Vector(maths::probability::markov::steady_state(
            get_m(p, "matrix")?,
            get_u(p, "max_iter")?,
            get_f(p, "tol")?,
        ))),
        "is_ergodic" => Ok(RunOutput::Boolean(maths::probability::markov::is_ergodic(
            get_m(p, "matrix")?,
        ))),
        "absorbing_states" => Ok(RunOutput::Vector(
            maths::probability::markov::absorbing_states(get_m(p, "matrix")?)
                .into_iter()
                .map(|x| x as f64)
                .collect(),
        )),
        "expected_visits" => Ok(RunOutput::Vector(
            maths::probability::markov::expected_visits(
                get_m(p, "matrix")?,
                get_u(p, "start")?,
                get_u(p, "steps")?,
            ),
        )),
        "mean_first_passage_time" => Ok(RunOutput::Vector(
            maths::probability::markov::mean_first_passage_time(
                get_m(p, "matrix")?,
                get_u(p, "target")?,
                get_u(p, "max_iter")?,
            ),
        )),
        "communicate_classes" => Ok(RunOutput::Matrix(
            maths::probability::markov::communicate_classes(get_m(p, "matrix")?)
                .into_iter()
                .map(|c| c.into_iter().map(|x| x as f64).collect())
                .collect(),
        )),
        "transition_matrix_power" => Ok(RunOutput::Matrix(
            maths::probability::markov::transition_matrix_power(
                get_m(p, "matrix")?,
                get_u(p, "power")?,
            ),
        )),
        "hitting_probability" => Ok(RunOutput::Scalar(
            maths::probability::markov::hitting_probability(
                get_m(p, "matrix")?,
                get_u(p, "start")?,
                get_u(p, "target")?,
                get_u(p, "max_iter")?,
            ),
        )),
        "birth_death_steady_state" => Ok(RunOutput::Vector(
            maths::probability::markov::birth_death_steady_state(
                get_v(p, "birth_rates")?,
                get_v(p, "death_rates")?,
            ),
        )),
        "markov_chain_entropy_rate" => Ok(RunOutput::Scalar(
            maths::probability::markov::markov_chain_entropy_rate(
                get_m(p, "matrix")?,
                get_v(p, "stationary")?,
            ),
        )),
        "is_doubly_stochastic" => Ok(RunOutput::Boolean(
            maths::probability::markov::is_doubly_stochastic(get_m(p, "matrix")?),
        )),
        "mixing_time_estimate" => Ok(RunOutput::Integer(
            maths::probability::markov::mixing_time_estimate(
                get_m(p, "matrix")?,
                get_f(p, "epsilon")?,
                get_u(p, "max_steps")?,
            ) as i64,
        )),

        "monte_carlo_pi" => Ok(RunOutput::Scalar(
            maths::probability::monte_carlo::monte_carlo_pi(
                get_u(p, "n")?,
                get_i(p, "seed")? as u64,
            ),
        )),
        "bootstrap_mean" => {
            let (m, s) = maths::probability::monte_carlo::bootstrap_mean(
                get_v(p, "data")?,
                get_u(p, "n_bootstrap")?,
                get_i(p, "seed")? as u64,
            );
            Ok(RunOutput::Pair(m, s))
        }
        "permutation_test" => Ok(RunOutput::Scalar(
            maths::probability::monte_carlo::permutation_test(
                get_v(p, "group_a")?,
                get_v(p, "group_b")?,
                get_u(p, "n_permutations")?,
                get_i(p, "seed")? as u64,
            ),
        )),

        "latin_hypercube" => Ok(RunOutput::Matrix(
            maths::probability::sampling::latin_hypercube(
                get_u(p, "n_samples")?,
                get_u(p, "n_dims")?,
                get_i(p, "seed")? as u64,
            ),
        )),
        "stratified_sampling" => Ok(RunOutput::Vector(
            maths::probability::sampling::stratified_sampling(
                get_u(p, "n_strata")?,
                get_i(p, "seed")? as u64,
            ),
        )),
        "sobol_sequence_1d" => Ok(RunOutput::Vector(
            maths::probability::sampling::sobol_sequence_1d(get_u(p, "n")?),
        )),
        "sampling_halton_sequence" => Ok(RunOutput::Vector(
            maths::probability::sampling::halton_sequence(get_u(p, "n")?, get_i(p, "base")? as u64),
        )),
        "inverse_transform_exponential" => Ok(RunOutput::Scalar(
            maths::probability::sampling::inverse_transform_exponential(
                get_f(p, "u")?,
                get_f(p, "lambda")?,
            ),
        )),
        "box_muller" => {
            let (a, b) = maths::probability::sampling::box_muller(get_f(p, "u1")?, get_f(p, "u2")?);
            Ok(RunOutput::Pair(a, b))
        }
        "reservoir_sampling" => Ok(RunOutput::Vector(
            maths::probability::sampling::reservoir_sampling(
                get_v(p, "stream")?,
                get_u(p, "k")?,
                get_i(p, "seed")? as u64,
            ),
        )),
        "alias_table_build" => {
            let (prob, alias) = maths::probability::sampling::alias_table_build(get_v(p, "probs")?);
            Ok(RunOutput::Matrix(vec![
                prob,
                alias.into_iter().map(|x| x as f64).collect(),
            ]))
        }
        "alias_sample" => Ok(RunOutput::Integer(
            maths::probability::sampling::alias_sample(
                get_v(p, "prob")?,
                get_uv(p, "alias")?,
                get_i(p, "seed")? as u64,
            ) as i64,
        )),
        "systematic_sampling" => Ok(RunOutput::Vector(
            maths::probability::sampling::systematic_sampling(
                get_u(p, "n_samples")?,
                get_i(p, "seed")? as u64,
            ),
        )),
        "importance_resampling" => Ok(RunOutput::Vector(
            maths::probability::sampling::importance_resampling(
                get_v(p, "weights")?,
                get_u(p, "n_samples")?,
                get_i(p, "seed")? as u64,
            )
            .into_iter()
            .map(|x| x as f64)
            .collect(),
        )),
        "van_der_corput" => Ok(RunOutput::Vector(
            maths::probability::sampling::van_der_corput(get_u(p, "n")?, get_i(p, "base")? as u64),
        )),
        "hammersley_sequence" => Ok(RunOutput::PairVec(
            maths::probability::sampling::hammersley_sequence(
                get_u(p, "n")?,
                get_i(p, "base")? as u64,
            ),
        )),
        "weighted_sampling" => Ok(RunOutput::Vector(
            maths::probability::sampling::weighted_sampling(
                get_v(p, "items")?,
                get_v(p, "weights")?,
                get_u(p, "n_samples")?,
                get_i(p, "seed")? as u64,
            ),
        )),
        "poisson_disk_sampling_1d" => Ok(RunOutput::Vector(
            maths::probability::sampling::poisson_disk_sampling_1d(
                get_f(p, "domain_min")?,
                get_f(p, "domain_max")?,
                get_f(p, "min_dist")?,
                get_i(p, "seed")? as u64,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
