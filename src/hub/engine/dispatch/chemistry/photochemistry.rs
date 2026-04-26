//! Dispatch handler for photochemistry functions.

use super::super::params::*;
use crate::hub::domain::chemistry as chem;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "photolysis_rate" => Ok(RunOutput::Scalar(
            chem::photochemistry::kinetics::photolysis_rate(
                get_f(p, "quantum_yield")?,
                get_f(p, "absorption_cross_section")?,
                get_f(p, "flux")?,
            ),
        )),
        "stern_volmer" => Ok(RunOutput::Scalar(
            chem::photochemistry::kinetics::stern_volmer(
                get_f(p, "i0")?,
                get_f(p, "ksv")?,
                get_f(p, "quencher")?,
            ),
        )),
        "stern_volmer_ratio" => Ok(RunOutput::Scalar(
            chem::photochemistry::kinetics::stern_volmer_ratio(
                get_f(p, "ksv")?,
                get_f(p, "quencher")?,
            ),
        )),
        "rate_intersystem_crossing" => Ok(RunOutput::Scalar(
            chem::photochemistry::kinetics::rate_intersystem_crossing(
                get_f(p, "total_rate")?,
                get_f(p, "rate_fluorescence")?,
                get_f(p, "rate_internal_conversion")?,
            ),
        )),
        "phosphorescence_lifetime" => Ok(RunOutput::Scalar(
            chem::photochemistry::kinetics::phosphorescence_lifetime(
                get_f(p, "rate_phosphorescence")?,
                get_f(p, "rate_non_radiative")?,
            ),
        )),
        "forster_radius" => Ok(RunOutput::Scalar(
            chem::photochemistry::kinetics::forster_radius(
                get_f(p, "quantum_yield_donor")?,
                get_f(p, "kappa_sq")?,
                get_f(p, "overlap_integral")?,
                get_f(p, "n_refraction")?,
            ),
        )),
        "fret_efficiency" => Ok(RunOutput::Scalar(
            chem::photochemistry::kinetics::fret_efficiency(get_f(p, "r")?, get_f(p, "r0")?),
        )),

        "quantum_yield" => Ok(RunOutput::Scalar(
            chem::photochemistry::quantum_yield::quantum_yield(
                get_f(p, "molecules_reacted")?,
                get_f(p, "photons_absorbed")?,
            ),
        )),
        "photon_energy" => Ok(RunOutput::Scalar(
            chem::photochemistry::quantum_yield::photon_energy(get_f(p, "wavelength_nm")?),
        )),
        "photon_energy_ev" => Ok(RunOutput::Scalar(
            chem::photochemistry::quantum_yield::photon_energy_ev(get_f(p, "wavelength_nm")?),
        )),
        "einstein_energy" => Ok(RunOutput::Scalar(
            chem::photochemistry::quantum_yield::einstein_energy(get_f(p, "wavelength_nm")?),
        )),
        "fluorescence_lifetime" => Ok(RunOutput::Scalar(
            chem::photochemistry::quantum_yield::fluorescence_lifetime(
                get_f(p, "rate_radiative")?,
                get_f(p, "rate_non_radiative")?,
            ),
        )),
        "fluorescence_quantum_yield" => Ok(RunOutput::Scalar(
            chem::photochemistry::quantum_yield::fluorescence_quantum_yield(
                get_f(p, "rate_radiative")?,
                get_f(p, "rate_non_radiative")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
