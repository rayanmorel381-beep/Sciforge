//! Dispatch handler for linear algebra functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::maths;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "lu_decompose" => {
            let (l, u) = maths::linalg::lu_decompose(get_m(p, "a")?);
            Ok(RunOutput::Matrix(vec![
                l.into_iter().flatten().collect(),
                u.into_iter().flatten().collect(),
            ]))
        }
        "forward_substitution" => Ok(RunOutput::Vector(maths::linalg::forward_substitution(
            get_m(p, "l")?,
            get_v(p, "b")?,
        ))),
        "back_substitution" => Ok(RunOutput::Vector(maths::linalg::back_substitution(
            get_m(p, "u")?,
            get_v(p, "y")?,
        ))),
        "lu_solve" => Ok(RunOutput::Vector(maths::linalg::lu_solve(
            get_m(p, "a")?,
            get_v(p, "b")?,
        ))),
        "cholesky" => Ok(RunOutput::Matrix(maths::linalg::cholesky(get_m(p, "a")?))),
        "determinant_lu" => Ok(RunOutput::Scalar(maths::linalg::determinant_lu(get_m(
            p, "a",
        )?))),
        "matrix_inverse_lu" => Ok(RunOutput::Matrix(maths::linalg::matrix_inverse_lu(get_m(
            p, "a",
        )?))),
        "lu_decompose_partial_pivot" => {
            let (l, u, piv) = maths::linalg::lu_decompose_partial_pivot(get_m(p, "a")?);
            Ok(RunOutput::Matrix(vec![
                l.into_iter().flatten().collect(),
                u.into_iter().flatten().collect(),
                piv.into_iter().map(|x| x as f64).collect(),
            ]))
        }
        "ldl_decompose" => {
            let (l, d) = maths::linalg::ldl_decompose(get_m(p, "a")?);
            Ok(RunOutput::Matrix(vec![
                l.into_iter().flatten().collect(),
                d,
            ]))
        }
        "cholesky_solve" => Ok(RunOutput::Vector(maths::linalg::cholesky_solve(
            get_m(p, "a")?,
            get_v(p, "b")?,
        ))),
        "crout_decompose" => {
            let (l, u) = maths::linalg::crout_decompose(get_m(p, "a")?);
            Ok(RunOutput::Matrix(vec![
                l.into_iter().flatten().collect(),
                u.into_iter().flatten().collect(),
            ]))
        }
        "determinant_cholesky" => Ok(RunOutput::Scalar(maths::linalg::determinant_cholesky(
            get_m(p, "a")?,
        ))),
        "schur_complement" => Ok(RunOutput::Matrix(maths::linalg::schur_complement(
            get_m(p, "m")?,
            get_u(p, "k")?,
        ))),
        "solve_triangular_lower" => Ok(RunOutput::Vector(maths::linalg::solve_triangular_lower(
            get_m(p, "l")?,
            get_v(p, "b")?,
        ))),
        "solve_triangular_upper" => Ok(RunOutput::Vector(maths::linalg::solve_triangular_upper(
            get_m(p, "u")?,
            get_v(p, "b")?,
        ))),
        "linalg_power_iteration" => {
            let (val, vec) = maths::linalg::power_iteration(
                get_m(p, "a")?,
                get_u(p, "max_iter")?,
                get_f(p, "tol")?,
            );
            Ok(RunOutput::Matrix(vec![vec![val], vec]))
        }
        "rayleigh_quotient" => Ok(RunOutput::Scalar(maths::linalg::rayleigh_quotient(
            get_m(p, "a")?,
            get_v(p, "v")?,
        ))),
        "gershgorin_disks" => Ok(RunOutput::PairVec(maths::linalg::gershgorin_disks(get_m(
            p, "a",
        )?))),
        "linalg_trace" => Ok(RunOutput::Scalar(maths::linalg::trace(get_m(p, "a")?))),
        "spectral_radius" => Ok(RunOutput::Scalar(maths::linalg::spectral_radius(get_v(
            p,
            "eigenvalues",
        )?))),
        "linalg_condition_number" => Ok(RunOutput::Scalar(maths::linalg::condition_number(get_v(
            p,
            "eigenvalues",
        )?))),
        "inverse_iteration" => {
            let (val, vec) = maths::linalg::inverse_iteration(
                get_m(p, "a")?,
                get_f(p, "sigma")?,
                get_u(p, "max_iter")?,
                get_f(p, "tol")?,
            );
            Ok(RunOutput::Matrix(vec![vec![val], vec]))
        }
        "qr_algorithm" => Ok(RunOutput::Vector(maths::linalg::qr_algorithm(
            get_m(p, "a")?,
            get_u(p, "max_iter")?,
            get_f(p, "tol")?,
        ))),
        "eigenvalues_symmetric_tridiag" => Ok(RunOutput::Vector(
            maths::linalg::eigenvalues_symmetric_tridiag(
                get_v(p, "diag")?,
                get_v(p, "off_diag")?,
                get_u(p, "max_iter")?,
                get_f(p, "tol")?,
            ),
        )),
        "deflated_power_iteration" => {
            let pairs = maths::linalg::deflated_power_iteration(
                get_m(p, "a")?,
                get_u(p, "num_eigenvalues")?,
                get_u(p, "max_iter")?,
                get_f(p, "tol")?,
            );
            Ok(RunOutput::Matrix(
                pairs
                    .into_iter()
                    .map(|(val, vec)| {
                        let mut row = vec![val];
                        row.extend(vec);
                        row
                    })
                    .collect(),
            ))
        }
        "is_eigenvalue" => Ok(RunOutput::Boolean(maths::linalg::is_eigenvalue(
            get_m(p, "a")?,
            get_f(p, "lambda")?,
            get_f(p, "tol")?,
        ))),
        "characteristic_polynomial_coeffs" => Ok(RunOutput::Vector(
            maths::linalg::characteristic_polynomial_coeffs(get_m(p, "a")?),
        )),
        "matrix_exponential_diag" => Ok(RunOutput::Matrix(maths::linalg::matrix_exponential_diag(
            get_v(p, "eigenvalues")?,
            get_m(p, "eigenvectors")?,
        ))),
        "qr_gram_schmidt" => {
            let (q, r) = maths::linalg::qr_gram_schmidt(get_m(p, "a")?);
            Ok(RunOutput::Matrix(vec![
                q.into_iter().flatten().collect(),
                r.into_iter().flatten().collect(),
            ]))
        }
        "qr_solve" => Ok(RunOutput::Vector(maths::linalg::qr_solve(
            get_m(p, "a")?,
            get_v(p, "b")?,
        ))),
        "householder_reflection" => Ok(RunOutput::Matrix(maths::linalg::householder_reflection(
            get_v(p, "v")?,
        ))),
        "rank" => Ok(RunOutput::Integer(
            maths::linalg::rank(get_m(p, "a")?, get_f(p, "tol")?) as i64,
        )),
        "null_space_dimension" => Ok(RunOutput::Integer(maths::linalg::null_space_dimension(
            get_m(p, "a")?,
            get_f(p, "tol")?,
        ) as i64)),
        "matrix_norm_frobenius" => Ok(RunOutput::Scalar(maths::linalg::matrix_norm_frobenius(
            get_m(p, "a")?,
        ))),
        "linalg_matrix_norm_inf" => Ok(RunOutput::Scalar(maths::linalg::matrix_norm_inf(get_m(
            p, "a",
        )?))),
        "is_symmetric" => Ok(RunOutput::Boolean(maths::linalg::is_symmetric(
            get_m(p, "a")?,
            get_f(p, "tol")?,
        ))),
        "is_positive_definite" => Ok(RunOutput::Boolean(maths::linalg::is_positive_definite(
            get_m(p, "a")?,
        ))),
        "is_orthogonal" => Ok(RunOutput::Boolean(maths::linalg::is_orthogonal(
            get_m(p, "a")?,
            get_f(p, "tol")?,
        ))),
        "is_diagonal" => Ok(RunOutput::Boolean(maths::linalg::is_diagonal(
            get_m(p, "a")?,
            get_f(p, "tol")?,
        ))),
        "is_upper_triangular" => Ok(RunOutput::Boolean(maths::linalg::is_upper_triangular(
            get_m(p, "a")?,
            get_f(p, "tol")?,
        ))),
        "is_lower_triangular" => Ok(RunOutput::Boolean(maths::linalg::is_lower_triangular(
            get_m(p, "a")?,
            get_f(p, "tol")?,
        ))),
        "linalg_matrix_norm_1" => Ok(RunOutput::Scalar(maths::linalg::matrix_norm_1(get_m(
            p, "a",
        )?))),
        "matrix_condition_frobenius" => Ok(RunOutput::Scalar(
            maths::linalg::matrix_condition_frobenius(get_m(p, "a")?),
        )),
        "gram_matrix" => Ok(RunOutput::Matrix(maths::linalg::gram_matrix(get_m(
            p, "a",
        )?))),
        "column_space_basis" => Ok(RunOutput::Matrix(maths::linalg::column_space_basis(
            get_m(p, "a")?,
            get_f(p, "tol")?,
        ))),
        "normal_equations" => Ok(RunOutput::Vector(maths::linalg::normal_equations(
            get_m(p, "a")?,
            get_v(p, "b")?,
        ))),
        "residual" => Ok(RunOutput::Vector(maths::linalg::residual(
            get_m(p, "a")?,
            get_v(p, "x")?,
            get_v(p, "b")?,
        ))),
        "residual_norm" => Ok(RunOutput::Scalar(maths::linalg::residual_norm(
            get_m(p, "a")?,
            get_v(p, "x")?,
            get_v(p, "b")?,
        ))),
        "linalg_r_squared" => Ok(RunOutput::Scalar(maths::linalg::r_squared(
            get_v(p, "y_actual")?,
            get_v(p, "y_predicted")?,
        ))),
        "polynomial_fit" => Ok(RunOutput::Vector(maths::linalg::polynomial_fit(
            get_v(p, "x")?,
            get_v(p, "y")?,
            get_u(p, "degree")?,
        ))),
        "weighted_least_squares" => Ok(RunOutput::Vector(maths::linalg::weighted_least_squares(
            get_m(p, "a")?,
            get_v(p, "b")?,
            get_v(p, "w")?,
        ))),
        "tikhonov_regularization" => Ok(RunOutput::Vector(maths::linalg::tikhonov_regularization(
            get_m(p, "a")?,
            get_v(p, "b")?,
            get_f(p, "lambda")?,
        ))),
        "total_least_squares" => Ok(RunOutput::Vector(maths::linalg::total_least_squares(
            get_m(p, "a")?,
            get_v(p, "b")?,
        ))),
        "iteratively_reweighted_least_squares" => Ok(RunOutput::Vector(
            maths::linalg::iteratively_reweighted_least_squares(
                get_m(p, "a")?,
                get_v(p, "b")?,
                get_u(p, "max_iter")?,
                get_f(p, "tol")?,
            ),
        )),
        "lasso_coordinate_descent" => {
            Ok(RunOutput::Vector(maths::linalg::lasso_coordinate_descent(
                get_m(p, "a")?,
                get_v(p, "b")?,
                get_f(p, "lambda")?,
                get_u(p, "max_iter")?,
                get_f(p, "tol")?,
            )))
        }
        "elastic_net" => Ok(RunOutput::Vector(maths::linalg::elastic_net(
            get_m(p, "a")?,
            get_v(p, "b")?,
            get_f(p, "lambda1")?,
            get_f(p, "lambda2")?,
            get_u(p, "max_iter")?,
            get_f(p, "tol")?,
        ))),
        "cross_validation_score" => Ok(RunOutput::Scalar(maths::linalg::cross_validation_score(
            get_v(p, "x")?,
            get_v(p, "y")?,
            get_u(p, "degree")?,
            get_u(p, "folds")?,
        ))),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
