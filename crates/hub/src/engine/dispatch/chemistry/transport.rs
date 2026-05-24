//! Dispatch handler for transport phenomena functions.

use super::super::params::*;
use crate::domain::chemistry as chem;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "fick_first_law" => Ok(RunOutput::Scalar(
            chem::transport::diffusion::fick_first_law(get_f(p, "d")?, get_f(p, "dc_dx")?),
        )),
        "fick_second_law_solution" => Ok(RunOutput::Scalar(
            chem::transport::diffusion::fick_second_law_solution(
                get_f(p, "c0")?,
                get_f(p, "cs")?,
                get_f(p, "x")?,
                get_f(p, "d")?,
                get_f(p, "t")?,
            ),
        )),
        "diffusion_coefficient_stokes_einstein" => Ok(RunOutput::Scalar(
            chem::transport::diffusion::diffusion_coefficient_stokes_einstein(
                get_f(p, "t")?,
                get_f(p, "viscosity")?,
                get_f(p, "r")?,
            ),
        )),
        "wilke_chang" => Ok(RunOutput::Scalar(chem::transport::diffusion::wilke_chang(
            get_f(p, "t")?,
            get_f(p, "viscosity")?,
            get_f(p, "mw_solvent")?,
            get_f(p, "phi")?,
            get_f(p, "v_solute")?,
        ))),
        "knudsen_diffusivity" => Ok(RunOutput::Scalar(
            chem::transport::diffusion::knudsen_diffusivity(
                get_f(p, "r_pore")?,
                get_f(p, "t")?,
                get_f(p, "mw")?,
            ),
        )),
        "effective_diffusivity" => Ok(RunOutput::Scalar(
            chem::transport::diffusion::effective_diffusivity(
                get_f(p, "d_bulk")?,
                get_f(p, "porosity")?,
                get_f(p, "tortuosity")?,
            ),
        )),
        "diffusion_time_estimate" => Ok(RunOutput::Scalar(
            chem::transport::diffusion::diffusion_time_estimate(
                get_f(p, "length")?,
                get_f(p, "d")?,
            ),
        )),

        "mass_transfer_coefficient_film" => Ok(RunOutput::Scalar(
            chem::transport::mass_transfer::mass_transfer_coefficient_film(
                get_f(p, "d")?,
                get_f(p, "delta")?,
            ),
        )),
        "mass_flux" => Ok(RunOutput::Scalar(
            chem::transport::mass_transfer::mass_flux(
                get_f(p, "k")?,
                get_f(p, "c_bulk")?,
                get_f(p, "c_surface")?,
            ),
        )),
        "sherwood_number" => Ok(RunOutput::Scalar(
            chem::transport::mass_transfer::sherwood_number(
                get_f(p, "k")?,
                get_f(p, "l")?,
                get_f(p, "d")?,
            ),
        )),
        "schmidt_number" => Ok(RunOutput::Scalar(
            chem::transport::mass_transfer::schmidt_number(
                get_f(p, "viscosity")?,
                get_f(p, "density")?,
                get_f(p, "d")?,
            ),
        )),
        "penetration_theory_coefficient" => Ok(RunOutput::Scalar(
            chem::transport::mass_transfer::penetration_theory_coefficient(
                get_f(p, "d")?,
                get_f(p, "t_contact")?,
            ),
        )),
        "surface_renewal_coefficient" => Ok(RunOutput::Scalar(
            chem::transport::mass_transfer::surface_renewal_coefficient(
                get_f(p, "d")?,
                get_f(p, "s")?,
            ),
        )),
        "two_film_theory_overall" => Ok(RunOutput::Scalar(
            chem::transport::mass_transfer::two_film_theory_overall(
                get_f(p, "k_g")?,
                get_f(p, "k_l")?,
                get_f(p, "henry")?,
            ),
        )),
        "mass_transfer_biot" => Ok(RunOutput::Scalar(
            chem::transport::mass_transfer::mass_transfer_biot(
                get_f(p, "k_ext")?,
                get_f(p, "r")?,
                get_f(p, "d_eff")?,
            ),
        )),
        "overall_mass_transfer_resistance" => Ok(RunOutput::Scalar(
            chem::transport::mass_transfer::overall_mass_transfer_resistance(get_v(
                p,
                "resistances",
            )?),
        )),
        "ntu_mass_transfer" => Ok(RunOutput::Scalar(
            chem::transport::mass_transfer::ntu_mass_transfer(
                get_f(p, "k")?,
                get_f(p, "a")?,
                get_f(p, "flow")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
