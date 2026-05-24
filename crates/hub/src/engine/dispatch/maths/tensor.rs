//! Dispatch handler for tensor functions.

use super::super::params::*;
use super::helpers::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::maths;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "tensor_reshape" => Ok(out_tensor(maths::tensor::reshape(
            &mk_tensor(p, "t")?,
            get_uv(p, "new_shape")?,
        ))),
        "tensor_transpose" => Ok(out_tensor(maths::tensor::transpose(
            &mk_tensor(p, "t")?,
            get_uv(p, "axes")?,
        ))),
        "tensor_contract" => Ok(out_tensor(maths::tensor::contract(
            &mk_tensor(p, "a")?,
            &mk_tensor(p, "b")?,
            get_u(p, "axis_a")?,
            get_u(p, "axis_b")?,
        ))),
        "tensor_outer" => Ok(out_tensor(maths::tensor::outer(
            &mk_tensor(p, "a")?,
            &mk_tensor(p, "b")?,
        ))),
        "tensor_kronecker" => Ok(out_tensor(maths::tensor::kronecker(
            &mk_tensor(p, "a")?,
            &mk_tensor(p, "b")?,
        ))),
        "levi_civita" => Ok(out_tensor(maths::tensor::levi_civita(get_u(p, "n")?))),
        "metric_raise" => Ok(out_tensor(maths::tensor::metric_raise(
            &mk_tensor(p, "t")?,
            &mk_tensor(p, "metric_inv")?,
            get_u(p, "index")?,
        ))),
        "metric_lower" => Ok(out_tensor(maths::tensor::metric_lower(
            &mk_tensor(p, "t")?,
            &mk_tensor(p, "metric")?,
            get_u(p, "index")?,
        ))),
        "tensor_trace" => Ok(RunOutput::Scalar(maths::tensor::trace(&mk_tensor(p, "t")?))),
        "tensor_matmul" => Ok(out_tensor(maths::tensor::matmul(
            &mk_tensor(p, "a")?,
            &mk_tensor(p, "b")?,
        ))),
        "tensor_minor" => Ok(out_tensor(maths::tensor::minor(
            &mk_tensor(p, "t")?,
            get_u(p, "row")?,
            get_u(p, "col")?,
        ))),
        "tensor_determinant" => Ok(RunOutput::Scalar(maths::tensor::determinant(&mk_tensor(
            p, "t",
        )?))),
        "tensor_inverse" => {
            let r = maths::tensor::inverse(&mk_tensor(p, "t")?);
            match r {
                Some(t) => Ok(out_tensor(t)),
                None => Err(HubError::ComputationFailed("singular matrix".into())),
            }
        }
        "tensor_eigenvalues_2x2" => {
            let (a, b) = maths::tensor::eigenvalues_2x2(&mk_tensor(p, "t")?);
            Ok(RunOutput::Pair(a, b))
        }
        "tensor_lu_decompose" => {
            let (l, u) = maths::tensor::lu_decompose(&mk_tensor(p, "t")?);
            Ok(RunOutput::Matrix(vec![
                l.data().to_vec(),
                u.data().to_vec(),
            ]))
        }
        "tensor_solve" => Ok(RunOutput::Vector(maths::tensor::solve(
            &mk_tensor(p, "a")?,
            get_v(p, "b")?,
        ))),
        "tensor_qr_decompose" => {
            let (q, r) = maths::tensor::qr_decompose(&mk_tensor(p, "a")?);
            Ok(RunOutput::Matrix(vec![
                q.data().to_vec(),
                r.data().to_vec(),
            ]))
        }
        "tensor_cholesky" => {
            let r = maths::tensor::cholesky(&mk_tensor(p, "a")?);
            match r {
                Some(t) => Ok(out_tensor(t)),
                None => Err(HubError::ComputationFailed("not positive definite".into())),
            }
        }
        "tensor_eigenvalues_qr" => Ok(RunOutput::Vector(maths::tensor::eigenvalues_qr(
            &mk_tensor(p, "a")?,
            get_u(p, "max_iter")?,
            get_f(p, "tol")?,
        ))),
        "tensor_eigenvectors_qr" => {
            let (vals, vecs) = maths::tensor::eigenvectors_qr(
                &mk_tensor(p, "a")?,
                get_u(p, "max_iter")?,
                get_f(p, "tol")?,
            );
            Ok(RunOutput::Matrix(vec![vals, vecs.data().to_vec()]))
        }
        "tensor_svd" => {
            let (u, s, vt) = maths::tensor::svd(&mk_tensor(p, "a")?);
            Ok(RunOutput::Matrix(vec![
                u.data().to_vec(),
                s,
                vt.data().to_vec(),
            ]))
        }
        "tensor_pseudoinverse" => Ok(out_tensor(maths::tensor::pseudoinverse(&mk_tensor(
            p, "a",
        )?))),
        "tensor_condition_number" => Ok(RunOutput::Scalar(maths::tensor::condition_number(
            &mk_tensor(p, "a")?,
        ))),
        "tensor_matrix_exp" => Ok(out_tensor(maths::tensor::matrix_exp(
            &mk_tensor(p, "a")?,
            get_u(p, "terms")?,
        ))),
        "tensor_matrix_norm_1" => Ok(RunOutput::Scalar(maths::tensor::matrix_norm_1(&mk_tensor(
            p, "a",
        )?))),
        "tensor_matrix_norm_inf" => Ok(RunOutput::Scalar(maths::tensor::matrix_norm_inf(
            &mk_tensor(p, "a")?,
        ))),
        "tensor_power_iteration" => {
            let (val, vec) = maths::tensor::power_iteration(
                &mk_tensor(p, "a")?,
                get_u(p, "max_iter")?,
                get_f(p, "tol")?,
            );
            Ok(RunOutput::Matrix(vec![vec![val], vec]))
        }

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
