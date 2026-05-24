//! Dispatch handler for optimization functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::maths;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "project_box" => Ok(RunOutput::Vector(maths::optimization::project_box(
            get_v(p, "x")?,
            get_v(p, "lower")?,
            get_v(p, "upper")?,
        ))),
        "project_simplex" => Ok(RunOutput::Vector(maths::optimization::project_simplex(
            get_v(p, "x")?,
        ))),
        "kkt_violation" => Ok(RunOutput::Scalar(maths::optimization::kkt_violation(
            get_v(p, "grad_f")?,
            get_v(p, "constraints")?,
            get_v(p, "lambdas")?,
            get_m(p, "grad_constraints")?,
        ))),
        "lagrangian" => Ok(RunOutput::Scalar(maths::optimization::lagrangian(
            get_f(p, "f")?,
            get_v(p, "constraints")?,
            get_v(p, "lambdas")?,
        ))),
        "projected_gradient_step" => Ok(RunOutput::Vector(
            maths::optimization::projected_gradient_step(
                get_v(p, "grad")?,
                get_v(p, "x")?,
                get_f(p, "lr")?,
                get_v(p, "lower")?,
                get_v(p, "upper")?,
            ),
        )),
        "frank_wolfe_step" => Ok(RunOutput::Vector(maths::optimization::frank_wolfe_step(
            get_v(p, "grad")?,
            get_v(p, "lower")?,
            get_v(p, "upper")?,
        ))),
        "admm_x_update" => Ok(RunOutput::Vector(maths::optimization::admm_x_update(
            get_m(p, "a")?,
            get_v(p, "b")?,
            get_v(p, "z")?,
            get_v(p, "u")?,
            get_f(p, "rho")?,
        ))),
        "dual_ascent_step" => Ok(RunOutput::Vector(maths::optimization::dual_ascent_step(
            get_v(p, "lambdas")?,
            get_v(p, "constraints")?,
            get_f(p, "step_size")?,
        ))),
        "quadratic_objective" => Ok(RunOutput::Scalar(maths::optimization::quadratic_objective(
            get_m(p, "h")?,
            get_v(p, "c")?,
            get_v(p, "x")?,
        ))),
        "linear_constraint_violation" => Ok(RunOutput::Scalar(
            maths::optimization::linear_constraint_violation(
                get_m(p, "a")?,
                get_v(p, "b")?,
                get_v(p, "x")?,
            ),
        )),
        "merit_function" => Ok(RunOutput::Scalar(maths::optimization::merit_function(
            get_f(p, "f")?,
            get_v(p, "constraints")?,
            get_f(p, "mu")?,
        ))),
        "multi_objective_dominates" => Ok(RunOutput::Boolean(
            maths::optimization::multi_objective_dominates(get_v(p, "a")?, get_v(p, "b")?),
        )),
        "crowding_distance" => Ok(RunOutput::Vector(maths::optimization::crowding_distance(
            get_m(p, "objectives")?,
        ))),
        "nsga2_non_dominated_sort" => Ok(RunOutput::Matrix(
            maths::optimization::nsga2_non_dominated_sort(get_m(p, "objectives")?)
                .into_iter()
                .map(|front| front.into_iter().map(|x| x as f64).collect())
                .collect(),
        )),
        "hypervolume_2d" => {
            let pts: Vec<(f64, f64)> = get_m(p, "points")?.iter().map(|r| (r[0], r[1])).collect();
            Ok(RunOutput::Scalar(maths::optimization::hypervolume_2d(
                &pts,
                (get_f(p, "ref_x")?, get_f(p, "ref_y")?),
            )))
        }

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
