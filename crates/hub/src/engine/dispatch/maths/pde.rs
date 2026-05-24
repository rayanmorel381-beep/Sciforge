//! Dispatch handler for PDE solver functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::maths;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "heat_equation_1d_explicit" => Ok(RunOutput::Vector(
            maths::pde::diffusion::heat_equation_1d_explicit(
                get_v(p, "u")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_f(p, "alpha")?,
            ),
        )),
        "heat_equation_1d_implicit" => Ok(RunOutput::Vector(
            maths::pde::diffusion::heat_equation_1d_implicit(
                get_v(p, "u")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_f(p, "alpha")?,
            ),
        )),
        "heat_equation_1d_crank_nicolson" => Ok(RunOutput::Vector(
            maths::pde::diffusion::heat_equation_1d_crank_nicolson(
                get_v(p, "u")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_f(p, "alpha")?,
            ),
        )),
        "diffusion_2d_explicit" => Ok(RunOutput::Matrix(
            maths::pde::diffusion::diffusion_2d_explicit(
                get_m(p, "u")?,
                get_f(p, "dx")?,
                get_f(p, "dy")?,
                get_f(p, "dt")?,
                get_f(p, "alpha")?,
            ),
        )),
        "stability_criterion_explicit" => Ok(RunOutput::Scalar(
            maths::pde::diffusion::stability_criterion_explicit(
                get_f(p, "dx")?,
                get_f(p, "alpha")?,
            ),
        )),
        "diffusion_green_function" => Ok(RunOutput::Scalar(
            maths::pde::diffusion::diffusion_green_function(
                get_f(p, "x")?,
                get_f(p, "t")?,
                get_f(p, "alpha")?,
            ),
        )),
        "advection_diffusion_1d" => Ok(RunOutput::Vector(
            maths::pde::diffusion::advection_diffusion_1d(
                get_v(p, "u")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_f(p, "alpha")?,
                get_f(p, "velocity")?,
            ),
        )),
        "diffusion_reaction_1d" => Ok(RunOutput::Vector(
            maths::pde::diffusion::diffusion_reaction_1d(
                get_v(p, "u")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_f(p, "alpha")?,
                get_f(p, "reaction_rate")?,
            ),
        )),
        "fisher_kpp_step" => Ok(RunOutput::Vector(maths::pde::diffusion::fisher_kpp_step(
            get_v(p, "u")?,
            get_f(p, "dx")?,
            get_f(p, "dt")?,
            get_f(p, "d")?,
            get_f(p, "r")?,
        ))),
        "nonlinear_diffusion_1d" => Ok(RunOutput::Vector(
            maths::pde::diffusion::nonlinear_diffusion_1d(
                get_v(p, "u")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_f(p, "m")?,
            ),
        )),
        "diffusion_2d_adi" => Ok(RunOutput::Matrix(maths::pde::diffusion::diffusion_2d_adi(
            get_m(p, "u")?,
            get_f(p, "dx")?,
            get_f(p, "dy")?,
            get_f(p, "dt")?,
            get_f(p, "alpha")?,
        ))),
        "stability_criterion_2d" => Ok(RunOutput::Scalar(
            maths::pde::diffusion::stability_criterion_2d(
                get_f(p, "dx")?,
                get_f(p, "dy")?,
                get_f(p, "alpha")?,
            ),
        )),
        "peclet_number" => Ok(RunOutput::Scalar(maths::pde::diffusion::peclet_number(
            get_f(p, "velocity")?,
            get_f(p, "length")?,
            get_f(p, "diffusivity")?,
        ))),
        "diffusion_steady_state_1d" => Ok(RunOutput::Vector(
            maths::pde::diffusion::diffusion_steady_state_1d(
                get_u(p, "n")?,
                get_f(p, "left_bc")?,
                get_f(p, "right_bc")?,
            ),
        )),
        "diffusion_analytical_finite" => Ok(RunOutput::Scalar(
            maths::pde::diffusion::diffusion_analytical_finite(
                get_f(p, "x")?,
                get_f(p, "t")?,
                get_f(p, "alpha")?,
                get_f(p, "length")?,
                get_u(p, "terms")?,
            ),
        )),
        "mass_conservation_check" => Ok(RunOutput::Scalar(
            maths::pde::diffusion::mass_conservation_check(get_v(p, "u")?, get_f(p, "dx")?),
        )),
        "diffusion_coefficient_from_msd" => Ok(RunOutput::Scalar(
            maths::pde::diffusion::diffusion_coefficient_from_msd(
                get_f(p, "msd")?,
                get_f(p, "time")?,
                get_i(p, "dimensions")? as u32,
            ),
        )),

        "wave_equation_1d" => Ok(RunOutput::Vector(maths::pde::wave::wave_equation_1d(
            get_v(p, "u")?,
            get_v(p, "u_prev")?,
            get_f(p, "dx")?,
            get_f(p, "dt")?,
            get_f(p, "c")?,
        ))),
        "wave_initial_step" => Ok(RunOutput::Vector(maths::pde::wave::wave_initial_step(
            get_v(p, "u")?,
            get_v(p, "v0")?,
            get_f(p, "dx")?,
            get_f(p, "dt")?,
            get_f(p, "c")?,
        ))),
        "wave_equation_2d" => Ok(RunOutput::Matrix(maths::pde::wave::wave_equation_2d(
            get_m(p, "u")?,
            get_m(p, "u_prev")?,
            get_f(p, "dx")?,
            get_f(p, "dy")?,
            get_f(p, "dt")?,
            get_f(p, "c")?,
        ))),
        "courant_number" => Ok(RunOutput::Scalar(maths::pde::wave::courant_number(
            get_f(p, "c")?,
            get_f(p, "dt")?,
            get_f(p, "dx")?,
        ))),
        "wave_energy_density" => Ok(RunOutput::Vector(maths::pde::wave::wave_energy_density(
            get_v(p, "u")?,
            get_v(p, "u_prev")?,
            get_f(p, "dx")?,
            get_f(p, "dt")?,
            get_f(p, "c")?,
            get_f(p, "rho")?,
        ))),
        "absorbing_boundary" => {
            let mut u = get_v(p, "u")?.to_vec();
            maths::pde::wave::absorbing_boundary(
                &mut u,
                get_v(p, "u_prev")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_f(p, "c")?,
            );
            Ok(RunOutput::Vector(u))
        }
        "string_vibration_mode" => Ok(RunOutput::Scalar(maths::pde::wave::string_vibration_mode(
            get_f(p, "x")?,
            get_f(p, "length")?,
            get_i(p, "mode")? as u32,
        ))),
        "wave_equation_1d_implicit" => Ok(RunOutput::Vector(
            maths::pde::wave::wave_equation_1d_implicit(
                get_v(p, "u")?,
                get_v(p, "u_prev")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_f(p, "c")?,
            ),
        )),
        "cfl_check" => Ok(RunOutput::Boolean(maths::pde::wave::cfl_check(
            get_f(p, "c")?,
            get_f(p, "dt")?,
            get_f(p, "dx")?,
        ))),
        "wave_reflection_coefficient" => Ok(RunOutput::Scalar(
            maths::pde::wave::wave_reflection_coefficient(get_f(p, "z1")?, get_f(p, "z2")?),
        )),
        "wave_transmission_coefficient" => Ok(RunOutput::Scalar(
            maths::pde::wave::wave_transmission_coefficient(get_f(p, "z1")?, get_f(p, "z2")?),
        )),
        "standing_wave" => Ok(RunOutput::Scalar(maths::pde::wave::standing_wave(
            get_f(p, "x")?,
            get_f(p, "t")?,
            get_f(p, "k")?,
            get_f(p, "omega")?,
            get_f(p, "amplitude")?,
        ))),
        "wave_phase_velocity" => Ok(RunOutput::Scalar(maths::pde::wave::wave_phase_velocity(
            get_f(p, "omega")?,
            get_f(p, "k")?,
        ))),
        "wave_group_velocity" => Ok(RunOutput::Scalar(maths::pde::wave::wave_group_velocity(
            get_f(p, "domega")?,
            get_f(p, "dk")?,
        ))),
        "wave_total_energy" => Ok(RunOutput::Scalar(maths::pde::wave::wave_total_energy(
            get_v(p, "u")?,
            get_v(p, "u_prev")?,
            get_f(p, "dx")?,
            get_f(p, "dt")?,
            get_f(p, "c")?,
            get_f(p, "rho")?,
        ))),
        "spherical_wave_amplitude" => Ok(RunOutput::Scalar(
            maths::pde::wave::spherical_wave_amplitude(
                get_f(p, "r")?,
                get_f(p, "amplitude_0")?,
                get_f(p, "r0")?,
            ),
        )),
        "wave_packet_gaussian" => Ok(RunOutput::Scalar(maths::pde::wave::wave_packet_gaussian(
            get_f(p, "x")?,
            get_f(p, "t")?,
            get_f(p, "k0")?,
            get_f(p, "sigma")?,
            get_f(p, "omega")?,
        ))),
        "wave_superposition" => Ok(RunOutput::Scalar(maths::pde::wave::wave_superposition(
            get_f(p, "x")?,
            get_f(p, "t")?,
            get_v(p, "amplitudes")?,
            get_v(p, "frequencies")?,
            get_v(p, "wave_numbers")?,
        ))),
        "wave_impedance" => Ok(RunOutput::Scalar(maths::pde::wave::wave_impedance(
            get_f(p, "density")?,
            get_f(p, "velocity")?,
        ))),

        "laplace_jacobi" => Ok(RunOutput::Matrix(maths::pde::laplace::laplace_jacobi(
            get_m(p, "u")?,
            get_u(p, "max_iter")?,
            get_f(p, "tol")?,
        ))),
        "laplace_gauss_seidel" => Ok(RunOutput::Matrix(
            maths::pde::laplace::laplace_gauss_seidel(
                get_m(p, "u")?,
                get_u(p, "max_iter")?,
                get_f(p, "tol")?,
            ),
        )),
        "laplace_sor" => Ok(RunOutput::Matrix(maths::pde::laplace::laplace_sor(
            get_m(p, "u")?,
            get_f(p, "omega")?,
            get_u(p, "max_iter")?,
            get_f(p, "tol")?,
        ))),
        "optimal_sor_omega" => Ok(RunOutput::Scalar(maths::pde::laplace::optimal_sor_omega(
            get_u(p, "nx")?,
            get_u(p, "ny")?,
        ))),
        "poisson_jacobi" => Ok(RunOutput::Matrix(maths::pde::laplace::poisson_jacobi(
            get_m(p, "u")?,
            get_m(p, "f_rhs")?,
            get_f(p, "dx")?,
            get_f(p, "dy")?,
            get_u(p, "max_iter")?,
            get_f(p, "tol")?,
        ))),
        "pde_residual_norm" => Ok(RunOutput::Scalar(maths::pde::laplace::residual_norm(
            get_m(p, "u")?,
            get_m(p, "f_rhs")?,
            get_f(p, "dx")?,
            get_f(p, "dy")?,
        ))),
        "harmonic_function_disk" => Ok(RunOutput::Scalar(
            maths::pde::laplace::harmonic_function_disk(
                get_f(p, "r")?,
                get_f(p, "theta")?,
                get_v(p, "a")?,
                get_v(p, "b")?,
                get_f(p, "radius")?,
            ),
        )),
        "poisson_gauss_seidel" => Ok(RunOutput::Matrix(
            maths::pde::laplace::poisson_gauss_seidel(
                get_m(p, "u")?,
                get_m(p, "f_rhs")?,
                get_f(p, "dx")?,
                get_f(p, "dy")?,
                get_u(p, "max_iter")?,
                get_f(p, "tol")?,
            ),
        )),
        "poisson_sor" => Ok(RunOutput::Matrix(maths::pde::laplace::poisson_sor(
            get_m(p, "u")?,
            get_m(p, "f_rhs")?,
            get_f(p, "dx")?,
            get_f(p, "dy")?,
            get_f(p, "omega")?,
            get_u(p, "max_iter")?,
            get_f(p, "tol")?,
        ))),
        "max_principle_check" => Ok(RunOutput::Boolean(
            maths::pde::laplace::max_principle_check(get_m(p, "phi")?),
        )),
        "greens_function_2d_free" => Ok(RunOutput::Scalar(
            maths::pde::laplace::greens_function_2d_free(
                get_f(p, "x")?,
                get_f(p, "y")?,
                get_f(p, "xs")?,
                get_f(p, "ys")?,
            ),
        )),
        "mean_value_property" => Ok(RunOutput::Scalar(maths::pde::laplace::mean_value_property(
            get_m(p, "phi")?,
            get_u(p, "ci")?,
            get_u(p, "cj")?,
            get_u(p, "radius")?,
        ))),
        "laplace_3d_jacobi" => {
            let u3d: Vec<Vec<Vec<f64>>> = get_m(p, "u")?
                .iter()
                .map(|layer| vec![layer.clone()])
                .collect();
            let r = maths::pde::laplace::laplace_3d_jacobi(
                &u3d,
                get_u(p, "max_iter")?,
                get_f(p, "tol")?,
            );
            Ok(RunOutput::Matrix(r.into_iter().flatten().collect()))
        }
        "dirichlet_energy" => Ok(RunOutput::Scalar(maths::pde::laplace::dirichlet_energy(
            get_m(p, "phi")?,
            get_f(p, "dx")?,
            get_f(p, "dy")?,
        ))),
        "laplace_iteration_count" => Ok(RunOutput::Integer(
            maths::pde::laplace::laplace_iteration_count(
                get_m(p, "u")?,
                get_u(p, "max_iter")?,
                get_f(p, "tol")?,
            ) as i64,
        )),

        "first_derivative" => Ok(RunOutput::Vector(
            maths::pde::finite_diff::first_derivative(get_v(p, "f")?, get_f(p, "dx")?),
        )),
        "second_derivative" => Ok(RunOutput::Vector(
            maths::pde::finite_diff::second_derivative(get_v(p, "f")?, get_f(p, "dx")?),
        )),
        "fourth_order_first_derivative" => Ok(RunOutput::Vector(
            maths::pde::finite_diff::fourth_order_first_derivative(get_v(p, "f")?, get_f(p, "dx")?),
        )),
        "laplacian_2d" => Ok(RunOutput::Matrix(maths::pde::finite_diff::laplacian_2d(
            get_m(p, "u")?,
            get_f(p, "dx")?,
            get_f(p, "dy")?,
        ))),
        "gradient_2d" => {
            let (gx, gy) = maths::pde::finite_diff::gradient_2d(
                get_m(p, "u")?,
                get_f(p, "dx")?,
                get_f(p, "dy")?,
            );
            Ok(RunOutput::Matrix(vec![
                gx.into_iter().flatten().collect(),
                gy.into_iter().flatten().collect(),
            ]))
        }
        "divergence_2d" => Ok(RunOutput::Matrix(maths::pde::finite_diff::divergence_2d(
            get_m(p, "fx")?,
            get_m(p, "fy")?,
            get_f(p, "dx")?,
            get_f(p, "dy")?,
        ))),
        "curl_2d" => Ok(RunOutput::Matrix(maths::pde::finite_diff::curl_2d(
            get_m(p, "fx")?,
            get_m(p, "fy")?,
            get_f(p, "dx")?,
            get_f(p, "dy")?,
        ))),
        "upwind_advection" => Ok(RunOutput::Vector(
            maths::pde::finite_diff::upwind_advection(
                get_v(p, "u")?,
                get_f(p, "v")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
            ),
        )),
        "lax_wendroff" => Ok(RunOutput::Vector(maths::pde::finite_diff::lax_wendroff(
            get_v(p, "u")?,
            get_f(p, "c")?,
            get_f(p, "dx")?,
            get_f(p, "dt")?,
        ))),
        "third_derivative" => Ok(RunOutput::Vector(
            maths::pde::finite_diff::third_derivative(get_v(p, "f")?, get_f(p, "dx")?),
        )),
        "mixed_partial_xy" => Ok(RunOutput::Matrix(
            maths::pde::finite_diff::mixed_partial_xy(
                get_m(p, "u")?,
                get_f(p, "dx")?,
                get_f(p, "dy")?,
            ),
        )),
        "compact_fourth_order" => Ok(RunOutput::Vector(
            maths::pde::finite_diff::compact_fourth_order(get_v(p, "f")?, get_f(p, "dx")?),
        )),
        "von_neumann_stability" => Ok(RunOutput::Boolean(
            maths::pde::finite_diff::von_neumann_stability(get_f(p, "amplification_factor")?),
        )),
        "boundary_ghost_extrapolate" => Ok(RunOutput::Vector(
            maths::pde::finite_diff::boundary_ghost_extrapolate(get_v(p, "u")?),
        )),
        "hessian_2d" => {
            let h = maths::pde::finite_diff::hessian_2d(
                get_m(p, "u")?,
                get_f(p, "dx")?,
                get_f(p, "dy")?,
                get_u(p, "i")?,
                get_u(p, "j")?,
            );
            Ok(RunOutput::Matrix(vec![
                vec![h[0][0], h[0][1]],
                vec![h[1][0], h[1][1]],
            ]))
        }
        "biharmonic_2d" => Ok(RunOutput::Matrix(maths::pde::finite_diff::biharmonic_2d(
            get_m(p, "u")?,
            get_f(p, "dx")?,
            get_f(p, "dy")?,
        ))),
        "lax_friedrichs" => Ok(RunOutput::Vector(maths::pde::finite_diff::lax_friedrichs(
            get_v(p, "u")?,
            get_f(p, "c")?,
            get_f(p, "dx")?,
            get_f(p, "dt")?,
        ))),
        "maccormack" => Ok(RunOutput::Vector(maths::pde::finite_diff::maccormack(
            get_v(p, "u")?,
            get_f(p, "c")?,
            get_f(p, "dx")?,
            get_f(p, "dt")?,
        ))),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
