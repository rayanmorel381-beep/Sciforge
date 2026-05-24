//! Dispatch handler for statistics functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::maths;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "mean" => Ok(RunOutput::Scalar(maths::statistics::mean(get_v(
            p, "data",
        )?))),
        "variance" => Ok(RunOutput::Scalar(maths::statistics::variance(get_v(
            p, "data",
        )?))),
        "std_dev" => Ok(RunOutput::Scalar(maths::statistics::std_dev(get_v(
            p, "data",
        )?))),
        "sample_variance" => Ok(RunOutput::Scalar(maths::statistics::sample_variance(
            get_v(p, "data")?,
        ))),
        "sample_std_dev" => Ok(RunOutput::Scalar(maths::statistics::sample_std_dev(get_v(
            p, "data",
        )?))),
        "median" => Ok(RunOutput::Scalar(maths::statistics::median(
            &mut get_v(p, "data")?.to_vec(),
        ))),
        "percentile" => Ok(RunOutput::Scalar(maths::statistics::percentile(
            &mut get_v(p, "data")?.to_vec(),
            get_f(p, "p")?,
        ))),
        "mode" => Ok(RunOutput::Scalar(maths::statistics::mode(get_v(
            p, "data",
        )?))),
        "skewness" => Ok(RunOutput::Scalar(maths::statistics::skewness(get_v(
            p, "data",
        )?))),
        "kurtosis" => Ok(RunOutput::Scalar(maths::statistics::kurtosis(get_v(
            p, "data",
        )?))),
        "covariance" => Ok(RunOutput::Scalar(maths::statistics::covariance(
            get_v(p, "x")?,
            get_v(p, "y")?,
        ))),
        "correlation" => Ok(RunOutput::Scalar(maths::statistics::correlation(
            get_v(p, "x")?,
            get_v(p, "y")?,
        ))),
        "weighted_mean" => Ok(RunOutput::Scalar(maths::statistics::weighted_mean(
            get_v(p, "data")?,
            get_v(p, "weights")?,
        ))),
        "geometric_mean" => Ok(RunOutput::Scalar(maths::statistics::geometric_mean(get_v(
            p, "data",
        )?))),
        "harmonic_mean" => Ok(RunOutput::Scalar(maths::statistics::harmonic_mean(get_v(
            p, "data",
        )?))),
        "entropy" => Ok(RunOutput::Scalar(maths::statistics::entropy(get_v(
            p, "probs",
        )?))),
        "stat_kl_divergence" => Ok(RunOutput::Scalar(maths::statistics::kl_divergence(
            get_v(p, "p")?,
            get_v(p, "q")?,
        ))),
        "stat_normal_pdf" => Ok(RunOutput::Scalar(maths::statistics::normal_pdf(
            get_f(p, "x")?,
            get_f(p, "mu")?,
            get_f(p, "sigma")?,
        ))),
        "stat_normal_cdf" => Ok(RunOutput::Scalar(maths::statistics::normal_cdf(
            get_f(p, "x")?,
            get_f(p, "mu")?,
            get_f(p, "sigma")?,
        ))),
        "erf" => Ok(RunOutput::Scalar(maths::statistics::erf(get_f(p, "x")?))),
        "erfc" => Ok(RunOutput::Scalar(maths::statistics::erfc(get_f(p, "x")?))),
        "stat_poisson_pmf" => Ok(RunOutput::Scalar(maths::statistics::poisson_pmf(
            get_i(p, "k")? as u64,
            get_f(p, "lambda")?,
        ))),
        "stat_binomial_pmf" => Ok(RunOutput::Scalar(maths::statistics::binomial_pmf(
            get_i(p, "n")? as u64,
            get_i(p, "k")? as u64,
            get_f(p, "p")?,
        ))),
        "stat_exponential_pdf" => Ok(RunOutput::Scalar(maths::statistics::exponential_pdf(
            get_f(p, "x")?,
            get_f(p, "lambda")?,
        ))),
        "stat_exponential_cdf" => Ok(RunOutput::Scalar(maths::statistics::exponential_cdf(
            get_f(p, "x")?,
            get_f(p, "lambda")?,
        ))),
        "stat_chi_squared_pdf" => Ok(RunOutput::Scalar(maths::statistics::chi_squared_pdf(
            get_f(p, "x")?,
            get_i(p, "k")? as u32,
        ))),
        "stat_student_t_pdf" => Ok(RunOutput::Scalar(maths::statistics::student_t_pdf(
            get_f(p, "x")?,
            get_f(p, "nu")?,
        ))),
        "stat_cauchy_pdf" => Ok(RunOutput::Scalar(maths::statistics::cauchy_pdf(
            get_f(p, "x")?,
            get_f(p, "x0")?,
            get_f(p, "gamma_val")?,
        ))),
        "stat_uniform_pdf" => Ok(RunOutput::Scalar(maths::statistics::uniform_pdf(
            get_f(p, "x")?,
            get_f(p, "a")?,
            get_f(p, "b")?,
        ))),
        "stat_beta_pdf" => Ok(RunOutput::Scalar(maths::statistics::beta_pdf(
            get_f(p, "x")?,
            get_f(p, "alpha")?,
            get_f(p, "beta")?,
        ))),
        "stat_gamma_pdf" => Ok(RunOutput::Scalar(maths::statistics::gamma_pdf(
            get_f(p, "x")?,
            get_f(p, "shape")?,
            get_f(p, "rate")?,
        ))),
        "gamma" => Ok(RunOutput::Scalar(maths::statistics::gamma(get_f(p, "x")?))),
        "ln_gamma" => Ok(RunOutput::Scalar(maths::statistics::ln_gamma(get_f(
            p, "x",
        )?))),
        "beta_function" => Ok(RunOutput::Scalar(maths::statistics::beta_function(
            get_f(p, "a")?,
            get_f(p, "b")?,
        ))),
        "incomplete_gamma_lower" => Ok(RunOutput::Scalar(
            maths::statistics::incomplete_gamma_lower(
                get_f(p, "a")?,
                get_f(p, "x")?,
                get_u(p, "terms")?,
            ),
        )),
        "z_test" => {
            let (z, pv) = maths::statistics::z_test(
                get_f(p, "sample_mean")?,
                get_f(p, "pop_mean")?,
                get_f(p, "pop_std")?,
                get_u(p, "n")?,
            );
            Ok(RunOutput::Pair(z, pv))
        }
        "t_test_one_sample" => {
            let (t, pv) = maths::statistics::t_test_one_sample(get_v(p, "data")?, get_f(p, "mu0")?);
            Ok(RunOutput::Pair(t, pv))
        }
        "t_test_two_sample" => {
            let (t, pv) = maths::statistics::t_test_two_sample(get_v(p, "a")?, get_v(p, "b")?);
            Ok(RunOutput::Pair(t, pv))
        }
        "chi_squared_test" => {
            let (chi, pv) =
                maths::statistics::chi_squared_test(get_v(p, "observed")?, get_v(p, "expected")?);
            Ok(RunOutput::Pair(chi, pv))
        }
        "anova_one_way" => {
            let groups_raw = get_m(p, "groups")?;
            let refs: Vec<&[f64]> = groups_raw.iter().map(|g| g.as_slice()).collect();
            let (f_val, pv) = maths::statistics::anova_one_way(&refs);
            Ok(RunOutput::Pair(f_val, pv))
        }
        "mann_whitney_u" => {
            let (u_val, pv) = maths::statistics::mann_whitney_u(get_v(p, "a")?, get_v(p, "b")?);
            Ok(RunOutput::Pair(u_val, pv))
        }
        "regularized_incomplete_beta" => Ok(RunOutput::Scalar(
            maths::statistics::regularized_incomplete_beta(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "x")?,
            ),
        )),
        "cf_beta" => Ok(RunOutput::Scalar(maths::statistics::cf_beta(
            get_f(p, "a")?,
            get_f(p, "b")?,
            get_f(p, "x")?,
        ))),
        "regularized_gamma_lower" => Ok(RunOutput::Scalar(
            maths::statistics::regularized_gamma_lower(get_f(p, "a")?, get_f(p, "x")?),
        )),
        "paired_t_test" => {
            let (t, pv) = maths::statistics::paired_t_test(get_v(p, "a")?, get_v(p, "b")?);
            Ok(RunOutput::Pair(t, pv))
        }
        "welch_t_test" => {
            let (t, pv) = maths::statistics::welch_t_test(get_v(p, "a")?, get_v(p, "b")?);
            Ok(RunOutput::Pair(t, pv))
        }
        "kruskal_wallis" => {
            let groups_raw = get_m(p, "groups")?;
            let refs: Vec<&[f64]> = groups_raw.iter().map(|g| g.as_slice()).collect();
            Ok(RunOutput::Scalar(maths::statistics::kruskal_wallis(&refs)))
        }
        "levene_test" => {
            let groups_raw = get_m(p, "groups")?;
            let refs: Vec<&[f64]> = groups_raw.iter().map(|g| g.as_slice()).collect();
            Ok(RunOutput::Scalar(maths::statistics::levene_test(&refs)))
        }
        "fisher_exact_2x2" => Ok(RunOutput::Scalar(maths::statistics::fisher_exact_2x2(
            get_i(p, "a")? as u64,
            get_i(p, "b")? as u64,
            get_i(p, "c")? as u64,
            get_i(p, "d")? as u64,
        ))),
        "spearman_rank_correlation" => Ok(RunOutput::Scalar(
            maths::statistics::spearman_rank_correlation(get_v(p, "x")?, get_v(p, "y")?),
        )),
        "kendall_tau" => Ok(RunOutput::Scalar(maths::statistics::kendall_tau(
            get_v(p, "x")?,
            get_v(p, "y")?,
        ))),
        "linear_regression" => {
            let (a, b) = maths::statistics::linear_regression(get_v(p, "x")?, get_v(p, "y")?);
            Ok(RunOutput::Pair(a, b))
        }
        "standard_error_slope" => Ok(RunOutput::Scalar(maths::statistics::standard_error_slope(
            get_v(p, "x")?,
            get_v(p, "y")?,
        ))),
        "stat_r_squared" => Ok(RunOutput::Scalar(maths::statistics::r_squared(
            get_v(p, "x")?,
            get_v(p, "y")?,
        ))),
        "polynomial_regression" => Ok(RunOutput::Vector(maths::statistics::polynomial_regression(
            get_v(p, "x")?,
            get_v(p, "y")?,
            get_u(p, "degree")?,
        ))),
        "exponential_regression" => {
            let (a, b) = maths::statistics::exponential_regression(get_v(p, "x")?, get_v(p, "y")?);
            Ok(RunOutput::Pair(a, b))
        }
        "multiple_linear_regression" => Ok(RunOutput::Vector(
            maths::statistics::multiple_linear_regression(get_m(p, "x_matrix")?, get_v(p, "y")?),
        )),
        "logarithmic_regression" => {
            let (a, b) = maths::statistics::logarithmic_regression(get_v(p, "x")?, get_v(p, "y")?);
            Ok(RunOutput::Pair(a, b))
        }
        "power_regression" => {
            let (a, b) = maths::statistics::power_regression(get_v(p, "x")?, get_v(p, "y")?);
            Ok(RunOutput::Pair(a, b))
        }
        "adjusted_r_squared" => Ok(RunOutput::Scalar(maths::statistics::adjusted_r_squared(
            get_v(p, "x")?,
            get_v(p, "y")?,
            get_u(p, "p")?,
        ))),
        "stat_residuals" => Ok(RunOutput::Vector(maths::statistics::residuals(
            get_v(p, "x")?,
            get_v(p, "y")?,
        ))),
        "sum_of_squared_residuals" => Ok(RunOutput::Scalar(
            maths::statistics::sum_of_squared_residuals(get_v(p, "x")?, get_v(p, "y")?),
        )),
        "mean_squared_error" => Ok(RunOutput::Scalar(maths::statistics::mean_squared_error(
            get_v(p, "x")?,
            get_v(p, "y")?,
        ))),
        "root_mean_squared_error" => Ok(RunOutput::Scalar(
            maths::statistics::root_mean_squared_error(get_v(p, "x")?, get_v(p, "y")?),
        )),
        "durbin_watson" => Ok(RunOutput::Scalar(maths::statistics::durbin_watson(
            get_v(p, "x")?,
            get_v(p, "y")?,
        ))),
        "leverage_hat" => Ok(RunOutput::Vector(maths::statistics::leverage_hat(get_v(
            p, "x",
        )?))),
        "cook_distance" => Ok(RunOutput::Vector(maths::statistics::cook_distance(
            get_v(p, "x")?,
            get_v(p, "y")?,
        ))),
        "aic" => Ok(RunOutput::Scalar(maths::statistics::aic(
            get_u(p, "n")?,
            get_u(p, "k")?,
            get_f(p, "ssr")?,
        ))),
        "bic" => Ok(RunOutput::Scalar(maths::statistics::bic(
            get_u(p, "n")?,
            get_u(p, "k")?,
            get_f(p, "ssr")?,
        ))),
        "ridge_regression" => Ok(RunOutput::Vector(maths::statistics::ridge_regression(
            get_m(p, "x_matrix")?,
            get_v(p, "y")?,
            get_f(p, "lambda")?,
        ))),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
