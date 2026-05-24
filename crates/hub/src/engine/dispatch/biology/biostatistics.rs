//! Dispatch handler for biostatistics functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "odds_ratio" => Ok(RunOutput::Scalar(bio::biostatistics::clinical::odds_ratio(
            get_u(p, "a")?,
            get_u(p, "b")?,
            get_u(p, "c")?,
            get_u(p, "d")?,
        ))),
        "relative_risk" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::relative_risk(
                get_u(p, "a")?,
                get_u(p, "b")?,
                get_u(p, "c")?,
                get_u(p, "d")?,
            ),
        )),
        "absolute_risk_reduction" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::absolute_risk_reduction(
                get_f(p, "risk_control")?,
                get_f(p, "risk_treatment")?,
            ),
        )),
        "number_needed_to_treat" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::number_needed_to_treat(get_f(p, "arr")?),
        )),
        "sensitivity" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::sensitivity(get_u(p, "tp")?, get_u(p, "fn_count")?),
        )),
        "specificity" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::specificity(get_u(p, "tn")?, get_u(p, "fp")?),
        )),
        "positive_predictive_value" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::positive_predictive_value(
                get_u(p, "tp")?,
                get_u(p, "fp")?,
            ),
        )),
        "negative_predictive_value" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::negative_predictive_value(
                get_u(p, "tn")?,
                get_u(p, "fn_count")?,
            ),
        )),
        "f1_score" => Ok(RunOutput::Scalar(bio::biostatistics::clinical::f1_score(
            get_u(p, "tp")?,
            get_u(p, "fp")?,
            get_u(p, "fn_count")?,
        ))),
        "cohens_kappa" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::cohens_kappa(
                get_f(p, "observed_agreement")?,
                get_f(p, "expected_agreement")?,
            ),
        )),
        "likelihood_ratio_positive" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::likelihood_ratio_positive(
                get_f(p, "sensitivity")?,
                get_f(p, "specificity")?,
            ),
        )),
        "likelihood_ratio_negative" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::likelihood_ratio_negative(
                get_f(p, "sensitivity")?,
                get_f(p, "specificity")?,
            ),
        )),
        "diagnostic_odds_ratio" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::diagnostic_odds_ratio(
                get_u(p, "tp")?,
                get_u(p, "fp")?,
                get_u(p, "fn_count")?,
                get_u(p, "tn")?,
            ),
        )),
        "youden_index" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::youden_index(
                get_f(p, "sensitivity")?,
                get_f(p, "specificity")?,
            ),
        )),
        "matthews_correlation_coefficient" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::matthews_correlation_coefficient(
                get_u(p, "tp")?,
                get_u(p, "tn")?,
                get_u(p, "fp")?,
                get_u(p, "fn_count")?,
            ),
        )),
        "prevalence_adjusted_ppv" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::prevalence_adjusted_ppv(
                get_f(p, "sensitivity")?,
                get_f(p, "specificity")?,
                get_f(p, "prevalence")?,
            ),
        )),
        "sample_size_two_proportions" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::sample_size_two_proportions(
                get_f(p, "p1")?,
                get_f(p, "p2")?,
                get_f(p, "alpha_z")?,
                get_f(p, "power_z")?,
            ),
        )),
        "confidence_interval_proportion" => {
            let (a, b) = bio::biostatistics::clinical::confidence_interval_proportion(
                get_f(p, "p")?,
                get_u(p, "n")?,
                get_f(p, "z")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "attributable_risk" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::attributable_risk(
                get_f(p, "risk_exposed")?,
                get_f(p, "risk_unexposed")?,
            ),
        )),
        "population_attributable_fraction" => Ok(RunOutput::Scalar(
            bio::biostatistics::clinical::population_attributable_fraction(
                get_f(p, "risk_exposed")?,
                get_f(p, "risk_unexposed")?,
                get_f(p, "prevalence_exposure")?,
            ),
        )),
        "meta_analysis_fixed_effect" => {
            let (a, b) = bio::biostatistics::meta_analysis::meta_analysis_fixed_effect(
                get_v(p, "effects")?,
                get_v(p, "variances")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "cochran_q" => Ok(RunOutput::Scalar(
            bio::biostatistics::meta_analysis::cochran_q(
                get_v(p, "effects")?,
                get_v(p, "variances")?,
            ),
        )),
        "i_squared" => Ok(RunOutput::Scalar(
            bio::biostatistics::meta_analysis::i_squared(get_f(p, "q")?, get_u(p, "k")?),
        )),
        "tau_squared_dsl" => Ok(RunOutput::Scalar(
            bio::biostatistics::meta_analysis::tau_squared_dsl(
                get_f(p, "q")?,
                get_u(p, "k")?,
                get_v(p, "variances")?,
            ),
        )),
        "meta_analysis_random_effects" => {
            let (a, b) = bio::biostatistics::meta_analysis::meta_analysis_random_effects(
                get_v(p, "effects")?,
                get_v(p, "variances")?,
                get_f(p, "tau2")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "funnel_plot_asymmetry" => Ok(RunOutput::Scalar(
            bio::biostatistics::meta_analysis::funnel_plot_asymmetry(
                get_v(p, "effects")?,
                get_v(p, "se")?,
            ),
        )),
        "trim_and_fill" => {
            let (a, b) = bio::biostatistics::meta_analysis::trim_and_fill(get_v(p, "effects")?);
            Ok(RunOutput::Pair(a, b as f64))
        }
        "fail_safe_n" => Ok(RunOutput::Scalar(
            bio::biostatistics::meta_analysis::fail_safe_n(
                get_v(p, "effects")?,
                get_v(p, "variances")?,
                get_f(p, "alpha_z")?,
            ),
        )),
        "prediction_interval" => {
            let (a, b) = bio::biostatistics::meta_analysis::prediction_interval(
                get_f(p, "pooled")?,
                get_f(p, "tau2")?,
                get_f(p, "se_pooled")?,
                get_u(p, "k")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "egger_regression" => {
            let (a, b) = bio::biostatistics::meta_analysis::egger_regression(
                get_v(p, "effects")?,
                get_v(p, "se")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "cumulative_meta_analysis" => Ok(RunOutput::PairVec(
            bio::biostatistics::meta_analysis::cumulative_meta_analysis(
                get_v(p, "effects")?,
                get_v(p, "variances")?,
            ),
        )),
        "influence_analysis" => Ok(RunOutput::Vector(
            bio::biostatistics::meta_analysis::influence_analysis(
                get_v(p, "effects")?,
                get_v(p, "variances")?,
            ),
        )),
        "h_squared" => Ok(RunOutput::Scalar(
            bio::biostatistics::meta_analysis::h_squared(get_f(p, "q")?, get_u(p, "k")?),
        )),
        "meta_regression_slope" => Ok(RunOutput::Scalar(
            bio::biostatistics::meta_analysis::meta_regression_slope(
                get_v(p, "effects")?,
                get_v(p, "variances")?,
                get_v(p, "covariate")?,
            ),
        )),
        "simple_linear_regression" => {
            let (a, b) = bio::biostatistics::regression::simple_linear_regression(
                get_v(p, "x")?,
                get_v(p, "y")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "r_squared" => Ok(RunOutput::Scalar(
            bio::biostatistics::regression::r_squared(get_v(p, "y")?, get_v(p, "y_pred")?),
        )),
        "logistic_regression_probability" => Ok(RunOutput::Scalar(
            bio::biostatistics::regression::logistic_regression_probability(
                get_v(p, "beta")?,
                get_v(p, "x")?,
            ),
        )),
        "aic" => Ok(RunOutput::Scalar(bio::biostatistics::regression::aic(
            get_f(p, "log_likelihood")?,
            get_u(p, "k")?,
        ))),
        "bic" => Ok(RunOutput::Scalar(bio::biostatistics::regression::bic(
            get_f(p, "log_likelihood")?,
            get_u(p, "k")?,
            get_u(p, "n")?,
        ))),
        "residual_standard_error" => Ok(RunOutput::Scalar(
            bio::biostatistics::regression::residual_standard_error(
                get_v(p, "residuals")?,
                get_u(p, "p")?,
            ),
        )),
        "chi_squared_statistic" => Ok(RunOutput::Scalar(
            bio::biostatistics::regression::chi_squared_statistic(
                get_v(p, "observed")?,
                get_v(p, "expected")?,
            ),
        )),
        "welch_t_statistic" => Ok(RunOutput::Scalar(
            bio::biostatistics::regression::welch_t_statistic(
                get_f(p, "m1")?,
                get_f(p, "m2")?,
                get_f(p, "s1")?,
                get_f(p, "s2")?,
                get_u(p, "n1")?,
                get_u(p, "n2")?,
            ),
        )),
        "mann_whitney_u" => Ok(RunOutput::Scalar(
            bio::biostatistics::regression::mann_whitney_u(
                get_v(p, "ranks_group1")?,
                get_u(p, "n1")?,
                get_u(p, "n2")?,
            ),
        )),
        "bonferroni_correction" => Ok(RunOutput::Scalar(
            bio::biostatistics::regression::bonferroni_correction(
                get_f(p, "p_value")?,
                get_u(p, "n_tests")?,
            ),
        )),
        "fishers_exact_test_odds" => Ok(RunOutput::Scalar(
            bio::biostatistics::regression::fishers_exact_test_odds(
                get_u(p, "a")?,
                get_u(p, "b")?,
                get_u(p, "c")?,
                get_u(p, "d")?,
            ),
        )),
        "spearman_rank_correlation" => Ok(RunOutput::Scalar(
            bio::biostatistics::regression::spearman_rank_correlation(
                get_v(p, "rank_x")?,
                get_v(p, "rank_y")?,
            ),
        )),
        "power_analysis_two_sample" => Ok(RunOutput::Scalar(
            bio::biostatistics::regression::power_analysis_two_sample(
                get_f(p, "effect_size")?,
                get_f(p, "alpha_z")?,
                get_f(p, "power_z")?,
            ),
        )),
        "kaplan_meier" => {
            let events_v = get_v(p, "events")?;
            let events: Vec<bool> = events_v.iter().map(|&x| x != 0.0).collect();
            Ok(RunOutput::PairVec(
                bio::biostatistics::survival::kaplan_meier(get_v(p, "times")?, &events),
            ))
        }
        "log_rank_statistic" => {
            let events1_v = get_v(p, "events1")?;
            let events1: Vec<bool> = events1_v.iter().map(|&x| x != 0.0).collect();
            let events2_v = get_v(p, "events2")?;
            let events2: Vec<bool> = events2_v.iter().map(|&x| x != 0.0).collect();
            Ok(RunOutput::Scalar(
                bio::biostatistics::survival::log_rank_statistic(
                    get_v(p, "times1")?,
                    &events1,
                    get_v(p, "times2")?,
                    &events2,
                ),
            ))
        }
        "hazard_ratio" => Ok(RunOutput::Scalar(
            bio::biostatistics::survival::hazard_ratio(
                get_u(p, "events_treatment")?,
                get_f(p, "time_treatment")?,
                get_u(p, "events_control")?,
                get_f(p, "time_control")?,
            ),
        )),
        "nelson_aalen" => {
            let events_v = get_v(p, "events")?;
            let events: Vec<bool> = events_v.iter().map(|&x| x != 0.0).collect();
            Ok(RunOutput::PairVec(
                bio::biostatistics::survival::nelson_aalen(get_v(p, "times")?, &events),
            ))
        }
        "exponential_survival" => Ok(RunOutput::Scalar(
            bio::biostatistics::survival::exponential_survival(get_f(p, "lambda")?, get_f(p, "t")?),
        )),
        "biostat_weibull_survival" => Ok(RunOutput::Scalar(
            bio::biostatistics::survival::weibull_survival(
                get_f(p, "lambda")?,
                get_f(p, "k")?,
                get_f(p, "t")?,
            ),
        )),
        "cumulative_incidence" => {
            let events_v = get_v(p, "events")?;
            let events: Vec<bool> = events_v.iter().map(|&x| x != 0.0).collect();
            let competing_v = get_v(p, "competing")?;
            let competing: Vec<bool> = competing_v.iter().map(|&x| x != 0.0).collect();
            Ok(RunOutput::PairVec(
                bio::biostatistics::survival::cumulative_incidence(
                    get_v(p, "times")?,
                    &events,
                    &competing,
                ),
            ))
        }
        "log_logistic_survival" => Ok(RunOutput::Scalar(
            bio::biostatistics::survival::log_logistic_survival(
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "t")?,
            ),
        )),
        "biostat_gompertz_survival" => Ok(RunOutput::Scalar(
            bio::biostatistics::survival::gompertz_survival(
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "t")?,
            ),
        )),
        "cox_partial_likelihood_contribution" => Ok(RunOutput::Scalar(
            bio::biostatistics::survival::cox_partial_likelihood_contribution(
                get_f(p, "beta_x")?,
                get_f(p, "risk_set_sum")?,
            ),
        )),
        "breslow_cumulative_hazard" => Ok(RunOutput::PairVec(
            bio::biostatistics::survival::breslow_cumulative_hazard(
                get_v(p, "event_times")?,
                get_v(p, "risk_set_sums")?,
            ),
        )),
        "survival_from_hazard" => Ok(RunOutput::Scalar(
            bio::biostatistics::survival::survival_from_hazard(get_f(p, "cumulative_hazard")?),
        )),
        "conditional_survival" => Ok(RunOutput::Scalar(
            bio::biostatistics::survival::conditional_survival(
                get_f(p, "s_t")?,
                get_f(p, "s_t_plus_x")?,
            ),
        )),
        "cure_fraction_model" => Ok(RunOutput::Scalar(
            bio::biostatistics::survival::cure_fraction_model(
                get_f(p, "cure_rate")?,
                get_f(p, "lambda")?,
                get_f(p, "t")?,
            ),
        )),
        "aalen_johansen" => {
            let events = get_str(p, "events")?.as_bytes();
            Ok(RunOutput::Matrix(
                bio::biostatistics::survival::aalen_johansen(
                    get_v(p, "times")?,
                    events,
                    get_u(p, "n_causes")?,
                )
                .into_iter()
                .map(|v| v.into_iter().flat_map(|(a, b)| vec![a, b]).collect())
                .collect(),
            ))
        }
        "median_survival" => {
            let v = get_v(p, "times")?;
            let s = get_v(p, "survivals")?;
            let curve: Vec<(f64, f64)> = v.iter().zip(s.iter()).map(|(&t, &sv)| (t, sv)).collect();
            Ok(RunOutput::Scalar(
                bio::biostatistics::survival::median_survival(&curve),
            ))
        }
        "restricted_mean_survival_time" => {
            let v = get_v(p, "times")?;
            let s = get_v(p, "survivals")?;
            let curve: Vec<(f64, f64)> = v.iter().zip(s.iter()).map(|(&t, &sv)| (t, sv)).collect();
            Ok(RunOutput::Scalar(
                bio::biostatistics::survival::restricted_mean_survival_time(
                    &curve,
                    get_f(p, "t_max")?,
                ),
            ))
        }
        "greenwood_variance" => {
            let v = get_v(p, "times")?;
            let s = get_v(p, "survivals")?;
            let curve: Vec<(f64, f64)> = v.iter().zip(s.iter()).map(|(&t, &sv)| (t, sv)).collect();
            let ar = get_v(p, "at_risk")?;
            let ev = get_v(p, "events")?;
            let at_risk: Vec<usize> = ar.iter().map(|&x| x as usize).collect();
            let events: Vec<usize> = ev.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Vector(
                bio::biostatistics::survival::greenwood_variance(&curve, &at_risk, &events),
            ))
        }
        "life_table" => {
            let ages_lo = get_v(p, "age_lo")?;
            let ages_hi = get_v(p, "age_hi")?;
            let age_groups: Vec<(f64, f64)> = ages_lo
                .iter()
                .zip(ages_hi.iter())
                .map(|(&a, &b)| (a, b))
                .collect();
            let deaths = get_v(p, "deaths")?;
            let pop = get_v(p, "population")?;
            let res = bio::biostatistics::survival::life_table(&age_groups, deaths, pop);
            let flat: Vec<f64> = res.iter().flat_map(|(a, b, c)| vec![*a, *b, *c]).collect();
            Ok(RunOutput::Vector(flat))
        }
        "roc_auc" => {
            let sv = get_v(p, "scores")?;
            let lv = get_v(p, "labels")?;
            let pairs: Vec<(f64, bool)> = sv
                .iter()
                .zip(lv.iter())
                .map(|(&s, &l)| (s, l > 0.5))
                .collect();
            Ok(RunOutput::Scalar(bio::biostatistics::clinical::roc_auc(
                &pairs,
            )))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
