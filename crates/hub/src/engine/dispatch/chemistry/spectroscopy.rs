//! Dispatch handler for spectroscopy functions.

use super::super::params::*;
use crate::domain::chemistry as chem;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "wavenumber_to_wavelength" => Ok(RunOutput::Scalar(
            chem::spectroscopy::ir::wavenumber_to_wavelength(get_f(p, "wavenumber_cm")?),
        )),
        "wavelength_to_wavenumber" => Ok(RunOutput::Scalar(
            chem::spectroscopy::ir::wavelength_to_wavenumber(get_f(p, "wavelength_um")?),
        )),
        "wavenumber_to_frequency" => Ok(RunOutput::Scalar(
            chem::spectroscopy::ir::wavenumber_to_frequency(get_f(p, "wavenumber_cm")?),
        )),
        "force_constant_from_frequency" => Ok(RunOutput::Scalar(
            chem::spectroscopy::ir::force_constant_from_frequency(
                get_f(p, "wavenumber")?,
                get_f(p, "reduced_mass_amu")?,
            ),
        )),
        "reduced_mass" => Ok(RunOutput::Scalar(chem::spectroscopy::ir::reduced_mass(
            get_f(p, "m1")?,
            get_f(p, "m2")?,
        ))),
        "ir_intensity_ratio" => Ok(RunOutput::Scalar(
            chem::spectroscopy::ir::ir_intensity_ratio(
                get_f(p, "absorbance_sample")?,
                get_f(p, "absorbance_reference")?,
            ),
        )),

        "mass_to_charge" => Ok(RunOutput::Scalar(
            chem::spectroscopy::mass_spec::mass_to_charge(
                get_f(p, "mass")?,
                get_i(p, "charge")? as u32,
            ),
        )),
        "exact_mass_difference" => Ok(RunOutput::Scalar(
            chem::spectroscopy::mass_spec::exact_mass_difference(
                get_f(p, "theoretical")?,
                get_f(p, "experimental")?,
            ),
        )),
        "nitrogen_rule" => Ok(RunOutput::Boolean(
            chem::spectroscopy::mass_spec::nitrogen_rule(get_i(p, "nominal_mass")? as u32),
        )),
        "rings_plus_double_bonds" => Ok(RunOutput::Scalar(
            chem::spectroscopy::mass_spec::rings_plus_double_bonds(
                get_i(p, "c")? as u32,
                get_i(p, "h")? as u32,
                get_i(p, "n")? as u32,
                get_i(p, "halogens")? as u32,
            ),
        )),
        "isotope_pattern_monoisotopic" => Ok(RunOutput::Scalar(
            chem::spectroscopy::mass_spec::isotope_pattern_monoisotopic(get_v(p, "abundances")?),
        )),
        "resolving_power" => Ok(RunOutput::Scalar(
            chem::spectroscopy::mass_spec::resolving_power(get_f(p, "m")?, get_f(p, "delta_m")?),
        )),

        "chemical_shift_ppm" => Ok(RunOutput::Scalar(
            chem::spectroscopy::nmr::chemical_shift_ppm(
                get_f(p, "frequency_sample")?,
                get_f(p, "frequency_reference")?,
                get_f(p, "spectrometer")?,
            ),
        )),
        "coupling_constant_first_order" => Ok(RunOutput::Scalar(
            chem::spectroscopy::nmr::coupling_constant_first_order(get_f(p, "line_separation_hz")?),
        )),
        "nmr_multiplicity" => Ok(RunOutput::Integer(
            chem::spectroscopy::nmr::multiplicity(get_i(p, "n_neighbors")? as u32) as i64,
        )),
        "larmor_frequency" => Ok(RunOutput::Scalar(
            chem::spectroscopy::nmr::larmor_frequency(get_f(p, "gamma")?, get_f(p, "b0")?),
        )),
        "t1_inversion_recovery" => Ok(RunOutput::Scalar(
            chem::spectroscopy::nmr::t1_inversion_recovery(
                get_f(p, "m0")?,
                get_f(p, "t1")?,
                get_f(p, "tau")?,
            ),
        )),
        "t2_spin_echo" => Ok(RunOutput::Scalar(chem::spectroscopy::nmr::t2_spin_echo(
            get_f(p, "m0")?,
            get_f(p, "t2")?,
            get_f(p, "tau")?,
        ))),
        "linewidth_from_t2" => Ok(RunOutput::Scalar(
            chem::spectroscopy::nmr::linewidth_from_t2(get_f(p, "t2")?),
        )),
        "nuclear_overhauser_enhancement" => Ok(RunOutput::Scalar(
            chem::spectroscopy::nmr::nuclear_overhauser_enhancement(
                get_f(p, "gamma_irradiated")?,
                get_f(p, "gamma_observed")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
