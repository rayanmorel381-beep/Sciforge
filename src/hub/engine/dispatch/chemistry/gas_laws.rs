//! Dispatch handler for gas law functions.

use super::super::params::*;
use crate::hub::domain::chemistry as chem;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "ideal_gas_pressure" => Ok(RunOutput::Scalar(
            chem::gas_laws::ideal::ideal_gas_pressure(
                get_f(p, "n")?,
                get_f(p, "t")?,
                get_f(p, "v")?,
            ),
        )),
        "ideal_gas_volume" => Ok(RunOutput::Scalar(chem::gas_laws::ideal::ideal_gas_volume(
            get_f(p, "n")?,
            get_f(p, "t")?,
            get_f(p, "p")?,
        ))),
        "ideal_gas_temperature" => Ok(RunOutput::Scalar(
            chem::gas_laws::ideal::ideal_gas_temperature(
                get_f(p, "p")?,
                get_f(p, "v")?,
                get_f(p, "n")?,
            ),
        )),
        "boyles_law" => Ok(RunOutput::Scalar(chem::gas_laws::ideal::boyles_law(
            get_f(p, "p1")?,
            get_f(p, "v1")?,
            get_f(p, "v2")?,
        ))),
        "charles_law" => Ok(RunOutput::Scalar(chem::gas_laws::ideal::charles_law(
            get_f(p, "v1")?,
            get_f(p, "t1")?,
            get_f(p, "t2")?,
        ))),
        "gay_lussac_law" => Ok(RunOutput::Scalar(chem::gas_laws::ideal::gay_lussac_law(
            get_f(p, "p1")?,
            get_f(p, "t1")?,
            get_f(p, "t2")?,
        ))),
        "combined_gas_law" => Ok(RunOutput::Scalar(chem::gas_laws::ideal::combined_gas_law(
            get_f(p, "p1")?,
            get_f(p, "v1")?,
            get_f(p, "t1")?,
            get_f(p, "t2")?,
            get_f(p, "p2")?,
        ))),
        "dalton_partial_pressure" => Ok(RunOutput::Scalar(
            chem::gas_laws::ideal::dalton_partial_pressure(
                get_f(p, "mole_fraction")?,
                get_f(p, "total_pressure")?,
            ),
        )),
        "grahams_law_effusion" => Ok(RunOutput::Scalar(
            chem::gas_laws::ideal::grahams_law_effusion(get_f(p, "m1")?, get_f(p, "m2")?),
        )),
        "gas_density" => Ok(RunOutput::Scalar(chem::gas_laws::ideal::gas_density(
            get_f(p, "p")?,
            get_f(p, "mw")?,
            get_f(p, "t")?,
        ))),
        "rms_speed" => Ok(RunOutput::Scalar(chem::gas_laws::ideal::rms_speed(
            get_f(p, "t")?,
            get_f(p, "mw")?,
        ))),
        "mean_speed" => Ok(RunOutput::Scalar(chem::gas_laws::ideal::mean_speed(
            get_f(p, "t")?,
            get_f(p, "mw")?,
        ))),
        "most_probable_speed" => Ok(RunOutput::Scalar(
            chem::gas_laws::ideal::most_probable_speed(get_f(p, "t")?, get_f(p, "mw")?),
        )),
        "mean_free_path" => Ok(RunOutput::Scalar(chem::gas_laws::ideal::mean_free_path(
            get_f(p, "d")?,
            get_f(p, "n_over_v")?,
        ))),

        "van_der_waals_pressure" => Ok(RunOutput::Scalar(
            chem::gas_laws::real::van_der_waals_pressure(
                get_f(p, "n")?,
                get_f(p, "t")?,
                get_f(p, "v")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
            ),
        )),
        "redlich_kwong_pressure" => Ok(RunOutput::Scalar(
            chem::gas_laws::real::redlich_kwong_pressure(
                get_f(p, "n")?,
                get_f(p, "t")?,
                get_f(p, "v")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
            ),
        )),
        "compressibility_factor" => Ok(RunOutput::Scalar(
            chem::gas_laws::real::compressibility_factor(
                get_f(p, "p")?,
                get_f(p, "v")?,
                get_f(p, "n")?,
                get_f(p, "t")?,
            ),
        )),
        "virial_equation_b" => Ok(RunOutput::Scalar(chem::gas_laws::real::virial_equation_b(
            get_f(p, "p")?,
            get_f(p, "t")?,
            get_f(p, "b")?,
        ))),
        "boyle_temperature" => Ok(RunOutput::Scalar(chem::gas_laws::real::boyle_temperature(
            get_f(p, "a")?,
            get_f(p, "b")?,
        ))),
        "critical_temperature" => Ok(RunOutput::Scalar(
            chem::gas_laws::real::critical_temperature(get_f(p, "a")?, get_f(p, "b")?),
        )),
        "critical_pressure" => Ok(RunOutput::Scalar(chem::gas_laws::real::critical_pressure(
            get_f(p, "a")?,
            get_f(p, "b")?,
        ))),
        "critical_volume" => Ok(RunOutput::Scalar(chem::gas_laws::real::critical_volume(
            get_f(p, "b")?,
        ))),
        "reduced_temperature" => Ok(RunOutput::Scalar(
            chem::gas_laws::real::reduced_temperature(get_f(p, "t")?, get_f(p, "tc")?),
        )),
        "reduced_pressure" => Ok(RunOutput::Scalar(chem::gas_laws::real::reduced_pressure(
            get_f(p, "p")?,
            get_f(p, "pc")?,
        ))),
        "peng_robinson_pressure" => Ok(RunOutput::Scalar(
            chem::gas_laws::real::peng_robinson_pressure(
                get_f(p, "t")?,
                get_f(p, "vm")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
            ),
        )),
        "fugacity_coefficient" => Ok(RunOutput::Scalar(
            chem::gas_laws::real::fugacity_coefficient(
                get_f(p, "z")?,
                get_f(p, "b_prime")?,
                get_f(p, "p")?,
            ),
        )),
        "acentric_factor" => Ok(RunOutput::Scalar(chem::gas_laws::real::acentric_factor(
            get_f(p, "p_sat")?,
            get_f(p, "pc")?,
        ))),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
