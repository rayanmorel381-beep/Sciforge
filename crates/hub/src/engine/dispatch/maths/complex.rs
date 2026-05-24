//! Dispatch handler for complex number functions.

use super::super::params::*;
use super::helpers::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::maths;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "roots_of_unity" => Ok(out_cv(maths::complex::roots_of_unity(get_u(p, "n")?))),
        "complex_polynomial_eval" => Ok(out_c(maths::complex::complex_polynomial_eval(
            &get_cv(p, "coeffs")?,
            get_c(p, "z")?,
        ))),
        "mandelbrot_iterate" => Ok(RunOutput::Integer(maths::complex::mandelbrot_iterate(
            get_c(p, "c")?,
            get_i(p, "max_iter")? as u32,
        ) as i64)),
        "julia_iterate" => Ok(RunOutput::Integer(maths::complex::julia_iterate(
            get_c(p, "z0")?,
            get_c(p, "c")?,
            get_i(p, "max_iter")? as u32,
        ) as i64)),
        "newton_fractal_step" => Ok(out_c(maths::complex::newton_fractal_step(
            get_c(p, "z")?,
            &get_cv(p, "coeffs")?,
            &get_cv(p, "deriv_coeffs")?,
        ))),
        "complex_matrix_mul" => {
            let a: Vec<Vec<Complex>> = get_m(p, "a")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let b: Vec<Vec<Complex>> = get_m(p, "b")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            Ok(out_cm(maths::complex::complex_matrix_mul(&a, &b)))
        }
        "complex_matrix_det" => {
            let m: Vec<Vec<Complex>> = get_m(p, "m")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            Ok(out_c(maths::complex::complex_matrix_det(&m)))
        }
        "complex_exp" => Ok(out_c(maths::complex::complex_exp(get_c(p, "z")?))),
        "complex_log" => Ok(out_c(maths::complex::complex_log(get_c(p, "z")?))),
        "complex_sin" => Ok(out_c(maths::complex::complex_sin(get_c(p, "z")?))),
        "complex_cos" => Ok(out_c(maths::complex::complex_cos(get_c(p, "z")?))),
        "complex_tan" => Ok(out_c(maths::complex::complex_tan(get_c(p, "z")?))),
        "complex_sinh" => Ok(out_c(maths::complex::complex_sinh(get_c(p, "z")?))),
        "complex_cosh" => Ok(out_c(maths::complex::complex_cosh(get_c(p, "z")?))),
        "complex_sqrt" => Ok(out_c(maths::complex::complex_sqrt(get_c(p, "z")?))),
        "complex_power" => Ok(out_c(maths::complex::complex_power(
            get_c(p, "z")?,
            get_c(p, "w")?,
        ))),
        "complex_power_real" => Ok(out_c(maths::complex::complex_power_real(
            get_c(p, "z")?,
            get_f(p, "n")?,
        ))),
        "mobius_transform" => Ok(out_c(maths::complex::mobius_transform(
            get_c(p, "z")?,
            get_c(p, "a")?,
            get_c(p, "b")?,
            get_c(p, "c")?,
            get_c(p, "d")?,
        ))),
        "bilinear_transform" => Ok(out_c(maths::complex::bilinear_transform(
            get_c(p, "s")?,
            get_f(p, "t_sample")?,
        ))),
        "complex_conjugate_transpose" => {
            let m: Vec<Vec<Complex>> = get_m(p, "m")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            Ok(out_cm(maths::complex::complex_conjugate_transpose(&m)))
        }
        "complex_matrix_trace" => {
            let m: Vec<Vec<Complex>> = get_m(p, "m")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            Ok(out_c(maths::complex::complex_matrix_trace(&m)))
        }
        "complex_matrix_add" => {
            let a: Vec<Vec<Complex>> = get_m(p, "a")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let b: Vec<Vec<Complex>> = get_m(p, "b")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            Ok(out_cm(maths::complex::complex_matrix_add(&a, &b)))
        }
        "complex_matrix_scale" => {
            let m: Vec<Vec<Complex>> = get_m(p, "m")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            Ok(out_cm(maths::complex::complex_matrix_scale(
                &m,
                get_c(p, "s")?,
            )))
        }
        "complex_dft" => Ok(out_cv(maths::complex::complex_dft(&get_cv(p, "input")?))),
        "complex_idft" => Ok(out_cv(maths::complex::complex_idft(&get_cv(p, "input")?))),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
