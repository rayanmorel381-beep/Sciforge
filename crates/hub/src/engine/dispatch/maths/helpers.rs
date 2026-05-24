//! Shared helpers for maths dispatch handlers.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;
pub use sciforge_lib::maths::complex::Complex;
pub use sciforge_lib::maths::complex::Quaternion;
pub use sciforge_lib::maths::polynomial::Polynomial;
pub use sciforge_lib::maths::sparse::SparseMatrix;
pub use sciforge_lib::maths::tensor::Tensor;
pub use sciforge_lib::maths::vector::Vec3;

pub fn mk_vec3(p: &Params, name: &str) -> HubResult<Vec3> {
    let v = get_v(p, name)?;
    if v.len() < 3 {
        return Err(HubError::InvalidInput(format!("{name}: need 3 components")));
    }
    Ok(Vec3::new(v[0], v[1], v[2]))
}

pub fn mk_poly(p: &Params, name: &str) -> HubResult<Polynomial> {
    Ok(Polynomial::new(get_poly(p, name)?.to_vec()))
}

pub fn mk_tensor(p: &Params, name: &str) -> HubResult<Tensor> {
    let (data, shape) = get_tensor(p, name)?;
    Ok(Tensor::from_vec(shape, data.to_vec()))
}

pub fn mk_sparse(p: &Params, name: &str) -> HubResult<SparseMatrix> {
    let (rows, cols, row_ptr, col_idx, values) = get_sparse(p, name)?;
    Ok(SparseMatrix {
        rows,
        cols,
        row_ptr: row_ptr.to_vec(),
        col_idx: col_idx.to_vec(),
        values: values.to_vec(),
    })
}

pub fn out_vec3(v: Vec3) -> RunOutput {
    RunOutput::Triple(v.x, v.y, v.z)
}

pub fn out_poly(p: Polynomial) -> RunOutput {
    RunOutput::PolynomialOut(p.coeffs)
}

pub fn out_tensor(t: Tensor) -> RunOutput {
    RunOutput::TensorOut {
        data: t.data().to_vec(),
        shape: t.shape().to_vec(),
    }
}

pub fn out_sparse(s: SparseMatrix) -> RunOutput {
    RunOutput::SparseOut {
        rows: s.rows,
        cols: s.cols,
        row_ptr: s.row_ptr,
        col_idx: s.col_idx,
        values: s.values,
    }
}

pub fn out_quat(q: Quaternion) -> RunOutput {
    RunOutput::Vector(vec![q.w, q.x, q.y, q.z])
}

pub fn out_c(c: Complex) -> RunOutput {
    RunOutput::Complex(c.re, c.im)
}

pub fn out_cv(v: Vec<Complex>) -> RunOutput {
    RunOutput::ComplexVector(v.iter().map(|c| (c.re, c.im)).collect())
}

pub fn out_cm(m: Vec<Vec<Complex>>) -> RunOutput {
    RunOutput::ComplexMatrix(
        m.iter()
            .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
            .collect(),
    )
}
