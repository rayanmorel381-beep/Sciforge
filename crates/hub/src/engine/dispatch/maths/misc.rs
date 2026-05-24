//! Dispatch handler for miscellaneous maths functions.

use super::super::params::*;
use super::helpers::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::maths;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "quaternion_pure" => Ok(out_quat(maths::complex::Quaternion::pure(
            get_f(p, "x")?,
            get_f(p, "y")?,
            get_f(p, "z")?,
        ))),
        "quaternion_from_axis_angle" => {
            let a = get_v(p, "axis")?;
            Ok(out_quat(maths::complex::Quaternion::from_axis_angle(
                [a[0], a[1], a[2]],
                get_f(p, "angle")?,
            )))
        }
        "quaternion_from_euler" => Ok(out_quat(maths::complex::Quaternion::from_euler(
            get_f(p, "roll")?,
            get_f(p, "pitch")?,
            get_f(p, "yaw")?,
        ))),
        "poly_zero" => Ok(out_poly(maths::polynomial::Polynomial::zero())),
        "poly_one" => Ok(out_poly(maths::polynomial::Polynomial::one())),
        "poly_monomial" => Ok(out_poly(maths::polynomial::Polynomial::monomial(
            get_u(p, "degree")?,
            get_f(p, "coeff")?,
        ))),
        "poly_from_roots" => Ok(out_poly(maths::polynomial::Polynomial::from_roots(get_v(
            p, "roots",
        )?))),
        "tensor_zeros" => Ok(out_tensor(maths::tensor::Tensor::zeros(get_uv(
            p, "shape",
        )?))),
        "tensor_ones" => Ok(out_tensor(maths::tensor::Tensor::ones(get_uv(p, "shape")?))),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
