//! Dispatch handler for interpolation functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::maths;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "lagrange_interpolate" => {
            let pts: Vec<(f64, f64)> = get_m(p, "points")?.iter().map(|r| (r[0], r[1])).collect();
            Ok(RunOutput::Scalar(
                maths::interpolation::lagrange_interpolate(&pts, get_f(p, "x")?),
            ))
        }
        "lagrange_barycentric" => {
            let pts: Vec<(f64, f64)> = get_m(p, "points")?.iter().map(|r| (r[0], r[1])).collect();
            Ok(RunOutput::Scalar(
                maths::interpolation::lagrange_barycentric(&pts, get_f(p, "x")?),
            ))
        }
        "neville" => {
            let pts: Vec<(f64, f64)> = get_m(p, "points")?.iter().map(|r| (r[0], r[1])).collect();
            Ok(RunOutput::Scalar(maths::interpolation::neville(
                &pts,
                get_f(p, "x")?,
            )))
        }
        "divided_differences" => {
            let pts: Vec<(f64, f64)> = get_m(p, "points")?.iter().map(|r| (r[0], r[1])).collect();
            Ok(RunOutput::Vector(
                maths::interpolation::divided_differences(&pts),
            ))
        }
        "newton_interpolate" => {
            let pts: Vec<(f64, f64)> = get_m(p, "points")?.iter().map(|r| (r[0], r[1])).collect();
            Ok(RunOutput::Scalar(maths::interpolation::newton_interpolate(
                &pts,
                get_f(p, "x")?,
            )))
        }
        "lagrange_derivative" => {
            let pts: Vec<(f64, f64)> = get_m(p, "points")?.iter().map(|r| (r[0], r[1])).collect();
            Ok(RunOutput::Scalar(
                maths::interpolation::lagrange_derivative(&pts, get_f(p, "x")?),
            ))
        }
        "chebyshev_nodes" => Ok(RunOutput::Vector(maths::interpolation::chebyshev_nodes(
            get_u(p, "n")?,
            get_f(p, "a")?,
            get_f(p, "b")?,
        ))),
        "rational_interpolate" => {
            let pts: Vec<(f64, f64)> = get_m(p, "points")?.iter().map(|r| (r[0], r[1])).collect();
            Ok(RunOutput::Scalar(
                maths::interpolation::rational_interpolate(&pts, get_f(p, "x")?),
            ))
        }
        "newton_forward_difference" => {
            let pts: Vec<(f64, f64)> = get_m(p, "points")?.iter().map(|r| (r[0], r[1])).collect();
            Ok(RunOutput::Scalar(
                maths::interpolation::newton_forward_difference(&pts, get_f(p, "x")?),
            ))
        }
        "interpolation_error_bound" => {
            let pts: Vec<(f64, f64)> = get_m(p, "points")?.iter().map(|r| (r[0], r[1])).collect();
            Ok(RunOutput::Scalar(
                maths::interpolation::interpolation_error_bound(
                    &pts,
                    get_f(p, "x")?,
                    get_f(p, "max_deriv")?,
                ),
            ))
        }
        "lebesgue_constant" => Ok(RunOutput::Scalar(maths::interpolation::lebesgue_constant(
            get_v(p, "nodes")?,
            get_v(p, "eval_points")?,
        ))),
        "least_squares_fit" => {
            let pts: Vec<(f64, f64)> = get_m(p, "points")?.iter().map(|r| (r[0], r[1])).collect();
            Ok(RunOutput::Vector(maths::interpolation::least_squares_fit(
                &pts,
                get_u(p, "degree")?,
            )))
        }
        "interp_moving_average" => Ok(RunOutput::Vector(maths::interpolation::moving_average(
            get_v(p, "data")?,
            get_u(p, "window")?,
        ))),
        "savitzky_golay_5_interp" => Ok(RunOutput::Vector(maths::interpolation::savitzky_golay_5(
            get_v(p, "data")?,
        ))),
        "interp_exponential_moving_average" => Ok(RunOutput::Vector(
            maths::interpolation::exponential_moving_average(get_v(p, "data")?, get_f(p, "alpha")?),
        )),
        "interp_weighted_moving_average" => Ok(RunOutput::Vector(
            maths::interpolation::weighted_moving_average(get_v(p, "data")?, get_v(p, "weights")?),
        )),
        "gaussian_kernel_smooth" => Ok(RunOutput::Vector(
            maths::interpolation::gaussian_kernel_smooth(
                get_v(p, "data")?,
                get_v(p, "xs")?,
                get_f(p, "bandwidth")?,
            ),
        )),
        "loess_smooth" => Ok(RunOutput::Scalar(maths::interpolation::loess_smooth(
            get_v(p, "xs")?,
            get_v(p, "ys")?,
            get_f(p, "x")?,
            get_f(p, "span")?,
        ))),
        "savitzky_golay_7" => Ok(RunOutput::Vector(maths::interpolation::savitzky_golay_7(
            get_v(p, "data")?,
        ))),
        "interp_median_filter" => Ok(RunOutput::Vector(maths::interpolation::median_filter(
            get_v(p, "data")?,
            get_u(p, "window")?,
        ))),
        "whittaker_smooth" => Ok(RunOutput::Vector(maths::interpolation::whittaker_smooth(
            get_v(p, "data")?,
            get_f(p, "lambda")?,
            get_u(p, "order")?,
        ))),
        "total_variation_denoise" => Ok(RunOutput::Vector(
            maths::interpolation::total_variation_denoise(
                get_v(p, "data")?,
                get_f(p, "lambda")?,
                get_u(p, "iterations")?,
            ),
        )),
        "linear_interpolate" => Ok(RunOutput::Scalar(maths::interpolation::linear_interpolate(
            get_v(p, "xs")?,
            get_v(p, "ys")?,
            get_f(p, "x")?,
        ))),
        "bilinear_interpolate" => Ok(RunOutput::Scalar(
            maths::interpolation::bilinear_interpolate(
                get_f(p, "x")?,
                get_f(p, "y")?,
                get_f(p, "x1")?,
                get_f(p, "x2")?,
                get_f(p, "y1")?,
                get_f(p, "y2")?,
                get_f(p, "q11")?,
                get_f(p, "q12")?,
                get_f(p, "q21")?,
                get_f(p, "q22")?,
            ),
        )),
        "hermite_interpolate" => Ok(RunOutput::Scalar(
            maths::interpolation::hermite_interpolate(
                get_f(p, "x0")?,
                get_f(p, "y0")?,
                get_f(p, "m0")?,
                get_f(p, "x1")?,
                get_f(p, "y1")?,
                get_f(p, "m1")?,
                get_f(p, "x")?,
            ),
        )),
        "akima_interpolate" => Ok(RunOutput::Scalar(maths::interpolation::akima_interpolate(
            get_v(p, "xs")?,
            get_v(p, "ys")?,
            get_f(p, "x")?,
        ))),
        "catmull_rom" => Ok(RunOutput::Scalar(maths::interpolation::catmull_rom(
            get_f(p, "p0")?,
            get_f(p, "p1")?,
            get_f(p, "p2")?,
            get_f(p, "p3")?,
            get_f(p, "t")?,
        ))),
        "monotone_cubic" => Ok(RunOutput::Scalar(maths::interpolation::monotone_cubic(
            get_v(p, "xs")?,
            get_v(p, "ys")?,
            get_f(p, "x")?,
        ))),
        "pchip_interpolate" => Ok(RunOutput::Scalar(maths::interpolation::pchip_interpolate(
            get_v(p, "xs")?,
            get_v(p, "ys")?,
            get_f(p, "x")?,
        ))),
        "nearest_neighbor" => Ok(RunOutput::Scalar(maths::interpolation::nearest_neighbor(
            get_v(p, "xs")?,
            get_v(p, "ys")?,
            get_f(p, "x")?,
        ))),
        "trilinear_interpolate" => {
            let c_arr = get_v(p, "c")?;
            let c: [f64; 8] = [
                c_arr[0], c_arr[1], c_arr[2], c_arr[3], c_arr[4], c_arr[5], c_arr[6], c_arr[7],
            ];
            Ok(RunOutput::Scalar(
                maths::interpolation::trilinear_interpolate(
                    get_f(p, "x")?,
                    get_f(p, "y")?,
                    get_f(p, "z")?,
                    get_f(p, "x0")?,
                    get_f(p, "x1")?,
                    get_f(p, "y0")?,
                    get_f(p, "y1")?,
                    get_f(p, "z0")?,
                    get_f(p, "z1")?,
                    &c,
                ),
            ))
        }
        "bezier_cubic" => Ok(RunOutput::Scalar(maths::interpolation::bezier_cubic(
            get_f(p, "p0")?,
            get_f(p, "p1")?,
            get_f(p, "p2")?,
            get_f(p, "p3")?,
            get_f(p, "t")?,
        ))),
        "bspline_basis" => Ok(RunOutput::Scalar(maths::interpolation::bspline_basis(
            get_v(p, "knots")?,
            get_u(p, "i")?,
            get_u(p, "degree")?,
            get_f(p, "t")?,
        ))),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
