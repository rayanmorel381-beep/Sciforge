//! Dispatch handler for computational chemistry functions.

use super::super::params::*;
use crate::hub::domain::chemistry as chem;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "gaussian_primitive" => Ok(RunOutput::Scalar(
            chem::computational::basis_sets::gaussian_primitive(
                get_f(p, "alpha")?,
                get_f(p, "r_sq")?,
            ),
        )),
        "normalization_s_orbital" => Ok(RunOutput::Scalar(
            chem::computational::basis_sets::normalization_s_orbital(get_f(p, "alpha")?),
        )),
        "normalization_p_orbital" => Ok(RunOutput::Scalar(
            chem::computational::basis_sets::normalization_p_orbital(get_f(p, "alpha")?),
        )),
        "slater_exponent" => Ok(RunOutput::Scalar(
            chem::computational::basis_sets::slater_exponent(get_f(p, "z_eff")?, get_f(p, "n")?),
        )),
        "boys_function_zero" => Ok(RunOutput::Scalar(
            chem::computational::basis_sets::boys_function_zero(get_f(p, "t")?),
        )),
        "basis_overlap_integral_1s" => Ok(RunOutput::Scalar(
            chem::computational::basis_sets::overlap_integral_1s(
                get_f(p, "alpha1")?,
                get_f(p, "alpha2")?,
                get_f(p, "r_sq")?,
            ),
        )),
        "kinetic_integral_1s" => Ok(RunOutput::Scalar(
            chem::computational::basis_sets::kinetic_integral_1s(
                get_f(p, "alpha1")?,
                get_f(p, "alpha2")?,
                get_f(p, "r_sq")?,
            ),
        )),
        "sto_ng_coefficients" => Ok(RunOutput::PairVec(
            chem::computational::basis_sets::sto_ng_coefficients(get_u(p, "n")?),
        )),
        "contracted_gaussian" => {
            let m = get_m(p, "coeffs")?;
            let coeffs: Vec<(f64, f64)> = m.iter().map(|r| (r[0], r[1])).collect();
            Ok(RunOutput::Scalar(
                chem::computational::basis_sets::contracted_gaussian(&coeffs, get_f(p, "r_sq")?),
            ))
        }

        "thomas_fermi_kinetic_energy" => Ok(RunOutput::Scalar(
            chem::computational::dft::thomas_fermi_kinetic_energy(
                get_v(p, "density")?,
                get_f(p, "volume_element")?,
            ),
        )),
        "exchange_energy_lda" => Ok(RunOutput::Scalar(
            chem::computational::dft::exchange_energy_lda(
                get_v(p, "density")?,
                get_f(p, "volume_element")?,
            ),
        )),
        "dft_hartree_energy" => Ok(RunOutput::Scalar(chem::computational::dft::hartree_energy(
            get_v(p, "density")?,
            get_v(p, "potential")?,
            get_f(p, "volume_element")?,
        ))),
        "nuclear_attraction_energy" => Ok(RunOutput::Scalar(
            chem::computational::dft::nuclear_attraction_energy(get_f(p, "z")?, get_f(p, "r")?),
        )),
        "electron_nuclear_energy" => Ok(RunOutput::Scalar(
            chem::computational::dft::electron_nuclear_energy(
                get_v(p, "density")?,
                get_v(p, "distances")?,
                get_f(p, "z")?,
                get_f(p, "volume_element")?,
            ),
        )),
        "xc_potential_lda" => Ok(RunOutput::Scalar(
            chem::computational::dft::xc_potential_lda(get_f(p, "density")?),
        )),
        "correlation_energy_vwn" => Ok(RunOutput::Scalar(
            chem::computational::dft::correlation_energy_vwn(get_f(p, "rs")?),
        )),
        "wigner_seitz_radius" => Ok(RunOutput::Scalar(
            chem::computational::dft::wigner_seitz_radius(get_f(p, "density")?),
        )),
        "kohn_sham_total_energy" => Ok(RunOutput::Scalar(
            chem::computational::dft::kohn_sham_total_energy(
                get_f(p, "eigenvalue_sum")?,
                get_f(p, "hartree")?,
                get_f(p, "xc_energy")?,
                get_f(p, "xc_potential_integral")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
