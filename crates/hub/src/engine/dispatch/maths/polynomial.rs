//! Dispatch handler for polynomial functions.

use super::super::params::*;
use super::helpers::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::maths;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "polynomial_roots_real" => Ok(RunOutput::Vector(maths::polynomial::polynomial_roots_real(
            &mk_poly(p, "poly")?,
            (get_f(p, "range_min")?, get_f(p, "range_max")?),
            get_u(p, "subdivisions")?,
            get_f(p, "tol")?,
        ))),
        "durand_kerner" => Ok(RunOutput::PairVec(maths::polynomial::durand_kerner(
            &mk_poly(p, "poly")?,
            get_u(p, "max_iter")?,
            get_f(p, "tol")?,
        ))),
        "laguerre_root" => Ok(RunOutput::Scalar(maths::polynomial::laguerre_root(
            &mk_poly(p, "poly")?,
            get_f(p, "x0")?,
            get_f(p, "tol")?,
            get_u(p, "max_iter")?,
        ))),
        "polynomial_deflate" => Ok(out_poly(maths::polynomial::polynomial_deflate(
            &mk_poly(p, "poly")?,
            get_f(p, "root")?,
        ))),
        "find_all_roots_real" => Ok(RunOutput::Vector(maths::polynomial::find_all_roots_real(
            &mk_poly(p, "poly")?,
            get_f(p, "tol")?,
            get_u(p, "max_iter")?,
        ))),
        "legendre" => Ok(out_poly(maths::polynomial::legendre(get_u(p, "n")?))),
        "chebyshev_t" => Ok(out_poly(maths::polynomial::chebyshev_t(get_u(p, "n")?))),
        "hermite" => Ok(out_poly(maths::polynomial::hermite(get_u(p, "n")?))),
        "laguerre_poly" => Ok(out_poly(maths::polynomial::laguerre(get_u(p, "n")?))),
        "bernstein_basis" => Ok(RunOutput::Scalar(maths::polynomial::bernstein_basis(
            get_u(p, "n")?,
            get_u(p, "k")?,
            get_f(p, "t")?,
        ))),
        "chebyshev_u" => Ok(out_poly(maths::polynomial::chebyshev_u(get_u(p, "n")?))),
        "gegenbauer" => Ok(out_poly(maths::polynomial::gegenbauer(
            get_u(p, "n")?,
            get_f(p, "alpha")?,
        ))),
        "jacobi" => Ok(out_poly(maths::polynomial::jacobi(
            get_u(p, "n")?,
            get_f(p, "alpha")?,
            get_f(p, "beta")?,
        ))),
        "associated_laguerre" => Ok(out_poly(maths::polynomial::associated_laguerre(
            get_u(p, "n")?,
            get_u(p, "k")?,
        ))),
        "rising_factorial" => Ok(RunOutput::Scalar(maths::polynomial::rising_factorial(
            get_f(p, "x")?,
            get_u(p, "n")?,
        ))),
        "falling_factorial" => Ok(RunOutput::Scalar(maths::polynomial::falling_factorial(
            get_f(p, "x")?,
            get_u(p, "n")?,
        ))),
        "bernoulli_polynomial" => Ok(out_poly(maths::polynomial::bernoulli_polynomial(get_u(
            p, "n",
        )?))),
        "euler_polynomial" => Ok(out_poly(maths::polynomial::euler_polynomial(get_u(
            p, "n",
        )?))),
        "abel_polynomial" => Ok(out_poly(maths::polynomial::abel_polynomial(
            get_u(p, "n")?,
            get_f(p, "a")?,
        ))),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
