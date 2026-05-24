//! Dispatch handler for vector operation functions.

use super::super::params::*;
use super::helpers::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::maths;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "vec_lerp" => Ok(out_vec3(maths::vector::lerp(
            &mk_vec3(p, "a")?,
            &mk_vec3(p, "b")?,
            get_f(p, "t")?,
        ))),
        "vec_angle_between" => Ok(RunOutput::Scalar(maths::vector::angle_between(
            &mk_vec3(p, "a")?,
            &mk_vec3(p, "b")?,
        ))),
        "vec_project" => Ok(out_vec3(maths::vector::project(
            &mk_vec3(p, "v")?,
            &mk_vec3(p, "onto")?,
        ))),
        "vec_reject" => Ok(out_vec3(maths::vector::reject(
            &mk_vec3(p, "v")?,
            &mk_vec3(p, "from")?,
        ))),
        "vec_reflect" => Ok(out_vec3(maths::vector::reflect(
            &mk_vec3(p, "v")?,
            &mk_vec3(p, "normal")?,
        ))),
        "vec_triple_scalar" => Ok(RunOutput::Scalar(maths::vector::triple_scalar(
            &mk_vec3(p, "a")?,
            &mk_vec3(p, "b")?,
            &mk_vec3(p, "c")?,
        ))),
        "vec_triple_vector" => Ok(out_vec3(maths::vector::triple_vector(
            &mk_vec3(p, "a")?,
            &mk_vec3(p, "b")?,
            &mk_vec3(p, "c")?,
        ))),
        "vec_slerp" => Ok(out_vec3(maths::vector::slerp(
            &mk_vec3(p, "a")?,
            &mk_vec3(p, "b")?,
            get_f(p, "t")?,
        ))),
        "vec_distance" => Ok(RunOutput::Scalar(maths::vector::distance(
            &mk_vec3(p, "a")?,
            &mk_vec3(p, "b")?,
        ))),
        "vec_midpoint" => Ok(out_vec3(maths::vector::midpoint(
            &mk_vec3(p, "a")?,
            &mk_vec3(p, "b")?,
        ))),
        "vec_centroid" => {
            let pts: Vec<Vec3> = get_m(p, "points")?
                .iter()
                .map(|r| Vec3::new(r[0], r[1], r[2]))
                .collect();
            Ok(out_vec3(maths::vector::centroid(&pts)))
        }
        "vec_area_triangle" => Ok(RunOutput::Scalar(maths::vector::area_triangle(
            &mk_vec3(p, "a")?,
            &mk_vec3(p, "b")?,
            &mk_vec3(p, "c")?,
        ))),
        "vec_normal_triangle" => Ok(out_vec3(maths::vector::normal_triangle(
            &mk_vec3(p, "a")?,
            &mk_vec3(p, "b")?,
            &mk_vec3(p, "c")?,
        ))),
        "vec_decompose_parallel_perpendicular" => {
            let (par, perp) = maths::vector::decompose_parallel_perpendicular(
                &mk_vec3(p, "v")?,
                &mk_vec3(p, "direction")?,
            );
            Ok(RunOutput::Matrix(vec![
                vec![par.x, par.y, par.z],
                vec![perp.x, perp.y, perp.z],
            ]))
        }
        "vec_rotate_around_axis" => Ok(out_vec3(maths::vector::rotate_around_axis(
            &mk_vec3(p, "v")?,
            &mk_vec3(p, "axis")?,
            get_f(p, "angle")?,
        ))),
        "spherical_to_cartesian" => Ok(out_vec3(maths::vector::spherical_to_cartesian(
            get_f(p, "r")?,
            get_f(p, "theta")?,
            get_f(p, "phi")?,
        ))),
        "cartesian_to_spherical" => {
            let (r, t, ph) = maths::vector::cartesian_to_spherical(&mk_vec3(p, "v")?);
            Ok(RunOutput::Triple(r, t, ph))
        }
        "cylindrical_to_cartesian" => Ok(out_vec3(maths::vector::cylindrical_to_cartesian(
            get_f(p, "rho")?,
            get_f(p, "phi")?,
            get_f(p, "z")?,
        ))),
        "cartesian_to_cylindrical" => {
            let (rho, phi, z) = maths::vector::cartesian_to_cylindrical(&mk_vec3(p, "v")?);
            Ok(RunOutput::Triple(rho, phi, z))
        }
        "lorentz_force" => Ok(out_vec3(maths::vector::lorentz_force(
            get_f(p, "charge")?,
            &mk_vec3(p, "velocity")?,
            &mk_vec3(p, "e_field")?,
            &mk_vec3(p, "b_field")?,
        ))),
        "spring_force" => Ok(out_vec3(maths::vector::spring_force(
            get_f(p, "k")?,
            get_f(p, "rest_length")?,
            &mk_vec3(p, "p1")?,
            &mk_vec3(p, "p2")?,
        ))),
        "drag_force" => Ok(out_vec3(maths::vector::drag_force(
            get_f(p, "cd")?,
            get_f(p, "rho")?,
            get_f(p, "area")?,
            &mk_vec3(p, "velocity")?,
        ))),
        "dipole_field" => Ok(out_vec3(maths::vector::dipole_field(
            &mk_vec3(p, "moment")?,
            &mk_vec3(p, "position")?,
        ))),
        "euler_step" => {
            let (pos, vel) = maths::vector::euler_step(
                &mk_vec3(p, "pos")?,
                &mk_vec3(p, "vel")?,
                &mk_vec3(p, "accel")?,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Matrix(vec![
                vec![pos.x, pos.y, pos.z],
                vec![vel.x, vel.y, vel.z],
            ]))
        }

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
