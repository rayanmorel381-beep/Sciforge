//! Dispatch handler for plate tectonics functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::geology as geo;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "plate_velocity" => Ok(RunOutput::Scalar(geo::tectonics::plate_velocity(
            get_f(p, "distance")?,
            get_f(p, "time")?,
        ))),
        "euler_pole_velocity" => Ok(RunOutput::Scalar(geo::tectonics::euler_pole_velocity(
            get_f(p, "omega")?,
            get_f(p, "radius")?,
            get_f(p, "colatitude")?,
        ))),
        "isostatic_equilibrium" => Ok(RunOutput::Scalar(geo::tectonics::isostatic_equilibrium(
            get_f(p, "rho_crust")?,
            get_f(p, "thickness")?,
            get_f(p, "rho_mantle")?,
        ))),
        "pratt_isostasy" => Ok(RunOutput::Scalar(geo::tectonics::pratt_isostasy(
            get_f(p, "rho_ref")?,
            get_f(p, "d_ref")?,
            get_f(p, "elevation")?,
        ))),
        "airy_root" => Ok(RunOutput::Scalar(geo::tectonics::airy_root(
            get_f(p, "elevation")?,
            get_f(p, "rho_crust")?,
            get_f(p, "rho_mantle")?,
        ))),
        "thermal_subsidence" => Ok(RunOutput::Scalar(geo::tectonics::thermal_subsidence(
            get_f(p, "e0")?,
            get_f(p, "t")?,
            get_f(p, "tau")?,
        ))),
        "mckenzie_stretching" => Ok(RunOutput::Scalar(geo::tectonics::mckenzie_stretching(
            get_f(p, "beta")?,
            get_f(p, "rho_m")?,
            get_f(p, "rho_c")?,
            get_f(p, "alpha")?,
            get_f(p, "tl")?,
            get_f(p, "tc")?,
        ))),
        "heat_flow" => Ok(RunOutput::Scalar(geo::tectonics::heat_flow(
            get_f(p, "k")?,
            get_f(p, "dt_dz")?,
        ))),
        "geothermal_gradient" => Ok(RunOutput::Scalar(geo::tectonics::geothermal_gradient(
            get_f(p, "surface_temp")?,
            get_f(p, "depth")?,
            get_f(p, "gradient")?,
        ))),
        "flexural_rigidity" => Ok(RunOutput::Scalar(geo::tectonics::flexural_rigidity(
            get_f(p, "e")?,
            get_f(p, "te")?,
            get_f(p, "nu")?,
        ))),
        "elastic_thickness_from_rigidity" => Ok(RunOutput::Scalar(
            geo::tectonics::elastic_thickness_from_rigidity(
                get_f(p, "d")?,
                get_f(p, "e")?,
                get_f(p, "nu")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
