//! Dispatch handler for sparse matrix functions.

use super::super::params::*;
use super::helpers::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::maths;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "sparse_kronecker" => Ok(out_sparse(maths::sparse::sparse_kronecker(
            &mk_sparse(p, "a")?,
            &mk_sparse(p, "b")?,
        ))),
        "sparse_add" => Ok(out_sparse(maths::sparse::sparse_add(
            &mk_sparse(p, "a")?,
            &mk_sparse(p, "b")?,
        ))),
        "sparse_scale" => Ok(out_sparse(maths::sparse::sparse_scale(
            &mk_sparse(p, "a")?,
            get_f(p, "scalar")?,
        ))),
        "sparse_mat_mul" => Ok(out_sparse(maths::sparse::sparse_mat_mul(
            &mk_sparse(p, "a")?,
            &mk_sparse(p, "b")?,
        ))),
        "sparse_trace" => Ok(RunOutput::Scalar(maths::sparse::sparse_trace(&mk_sparse(
            p, "a",
        )?))),
        "sparse_frobenius_norm" => Ok(RunOutput::Scalar(maths::sparse::sparse_frobenius_norm(
            &mk_sparse(p, "a")?,
        ))),
        "sparse_infinity_norm" => Ok(RunOutput::Scalar(maths::sparse::sparse_infinity_norm(
            &mk_sparse(p, "a")?,
        ))),
        "sparse_one_norm" => Ok(RunOutput::Scalar(maths::sparse::sparse_one_norm(
            &mk_sparse(p, "a")?,
        ))),
        "sparse_diagonal" => Ok(RunOutput::Vector(maths::sparse::sparse_diagonal(
            &mk_sparse(p, "a")?,
        ))),
        "sparse_lower_triangular" => Ok(out_sparse(maths::sparse::sparse_lower_triangular(
            &mk_sparse(p, "a")?,
        ))),
        "sparse_upper_triangular" => Ok(out_sparse(maths::sparse::sparse_upper_triangular(
            &mk_sparse(p, "a")?,
        ))),
        "sparse_row_sum" => Ok(RunOutput::Vector(maths::sparse::sparse_row_sum(
            &mk_sparse(p, "a")?,
        ))),
        "sparse_col_sum" => Ok(RunOutput::Vector(maths::sparse::sparse_col_sum(
            &mk_sparse(p, "a")?,
        ))),
        "sparse_is_symmetric" => Ok(RunOutput::Boolean(maths::sparse::sparse_is_symmetric(
            &mk_sparse(p, "a")?,
            get_f(p, "tol")?,
        ))),
        "conjugate_gradient" => Ok(RunOutput::Vector(maths::sparse::conjugate_gradient(
            &mk_sparse(p, "a")?,
            get_v(p, "b")?,
            get_f(p, "tol")?,
            get_u(p, "max_iter")?,
        ))),
        "jacobi_iterate" => Ok(RunOutput::Vector(maths::sparse::jacobi_iterate(
            &mk_sparse(p, "a")?,
            get_v(p, "b")?,
            get_f(p, "tol")?,
            get_u(p, "max_iter")?,
        ))),
        "gauss_seidel" => Ok(RunOutput::Vector(maths::sparse::gauss_seidel(
            &mk_sparse(p, "a")?,
            get_v(p, "b")?,
            get_f(p, "tol")?,
            get_u(p, "max_iter")?,
        ))),
        "sparse_lu_solve" => Ok(RunOutput::Vector(maths::sparse::sparse_lu_solve(
            &mk_sparse(p, "a")?,
            get_v(p, "b")?,
        ))),
        "sor_iterate" => Ok(RunOutput::Vector(maths::sparse::sor_iterate(
            &mk_sparse(p, "a")?,
            get_v(p, "b")?,
            get_f(p, "omega")?,
            get_f(p, "tol")?,
            get_u(p, "max_iter")?,
        ))),
        "steepest_descent" => Ok(RunOutput::Vector(maths::sparse::steepest_descent(
            &mk_sparse(p, "a")?,
            get_v(p, "b")?,
            get_f(p, "tol")?,
            get_u(p, "max_iter")?,
        ))),
        "bicgstab" => Ok(RunOutput::Vector(maths::sparse::bicgstab(
            &mk_sparse(p, "a")?,
            get_v(p, "b")?,
            get_f(p, "tol")?,
            get_u(p, "max_iter")?,
        ))),
        "gmres" => Ok(RunOutput::Vector(maths::sparse::gmres(
            &mk_sparse(p, "a")?,
            get_v(p, "b")?,
            get_f(p, "tol")?,
            get_u(p, "max_iter")?,
            get_u(p, "restart")?,
        ))),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
