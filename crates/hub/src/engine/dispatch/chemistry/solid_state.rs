//! Dispatch handler for solid-state chemistry functions.

use super::super::params::*;
use crate::domain::chemistry as chem;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "fermi_dirac" => Ok(RunOutput::Scalar(
            chem::solid_state::band_theory::fermi_dirac(
                get_f(p, "energy")?,
                get_f(p, "fermi_level")?,
                get_f(p, "t")?,
            ),
        )),
        "band_gap_from_absorption" => Ok(RunOutput::Scalar(
            chem::solid_state::band_theory::band_gap_from_absorption(get_f(
                p,
                "wavelength_edge_nm",
            )?),
        )),
        "intrinsic_carrier_concentration" => Ok(RunOutput::Scalar(
            chem::solid_state::band_theory::intrinsic_carrier_concentration(
                get_f(p, "nc")?,
                get_f(p, "nv")?,
                get_f(p, "eg")?,
                get_f(p, "t")?,
            ),
        )),
        "conductivity_semiconductor" => Ok(RunOutput::Scalar(
            chem::solid_state::band_theory::conductivity_semiconductor(
                get_f(p, "n")?,
                get_f(p, "mu_e")?,
                get_f(p, "p")?,
                get_f(p, "mu_h")?,
            ),
        )),
        "hall_coefficient" => Ok(RunOutput::Scalar(
            chem::solid_state::band_theory::hall_coefficient(get_f(p, "n")?, get_f(p, "p")?),
        )),
        "doping_carrier_concentration" => {
            let (n, pv) = chem::solid_state::band_theory::doping_carrier_concentration(
                get_f(p, "nd")?,
                get_f(p, "ni")?,
            );
            Ok(RunOutput::Pair(n, pv))
        }
        "resistivity_from_conductivity" => Ok(RunOutput::Scalar(
            chem::solid_state::band_theory::resistivity_from_conductivity(get_f(p, "sigma")?),
        )),
        "seebeck_coefficient" => Ok(RunOutput::Scalar(
            chem::solid_state::band_theory::seebeck_coefficient(
                get_f(p, "delta_v")?,
                get_f(p, "delta_t")?,
            ),
        )),
        "ionic_conductivity_arrhenius" => Ok(RunOutput::Scalar(
            chem::solid_state::band_theory::ionic_conductivity_arrhenius(
                get_f(p, "sigma0")?,
                get_f(p, "ea")?,
                get_f(p, "t")?,
            ),
        )),

        "schottky_defect_concentration" => Ok(RunOutput::Scalar(
            chem::solid_state::defects::schottky_defect_concentration(
                get_f(p, "n_sites")?,
                get_f(p, "e_formation")?,
                get_f(p, "t")?,
            ),
        )),
        "frenkel_defect_concentration" => Ok(RunOutput::Scalar(
            chem::solid_state::defects::frenkel_defect_concentration(
                get_f(p, "n_sites")?,
                get_f(p, "n_interstitial")?,
                get_f(p, "e_formation")?,
                get_f(p, "t")?,
            ),
        )),
        "vacancy_diffusion_coefficient" => Ok(RunOutput::Scalar(
            chem::solid_state::defects::vacancy_diffusion_coefficient(
                get_f(p, "d0")?,
                get_f(p, "q")?,
                get_f(p, "t")?,
            ),
        )),
        "color_center_absorption" => Ok(RunOutput::Scalar(
            chem::solid_state::defects::color_center_absorption(get_f(p, "wavelength_nm")?),
        )),
        "kroger_vink_equilibrium" => Ok(RunOutput::Scalar(
            chem::solid_state::defects::kroger_vink_equilibrium(
                get_f(p, "k")?,
                get_f(p, "oxygen_pressure")?,
                get_f(p, "exponent")?,
            ),
        )),
        "nonstoichiometry_delta" => Ok(RunOutput::Scalar(
            chem::solid_state::defects::nonstoichiometry_delta(
                get_f(p, "mass_change")?,
                get_f(p, "molar_mass_oxygen")?,
                get_f(p, "sample_mass")?,
                get_f(p, "molar_mass_sample")?,
            ),
        )),
        "defect_formation_volume" => Ok(RunOutput::Scalar(
            chem::solid_state::defects::defect_formation_volume(
                get_f(p, "lattice_param_defect")?,
                get_f(p, "lattice_param_perfect")?,
            ),
        )),
        "burgers_vector_magnitude" => Ok(RunOutput::Scalar(
            chem::solid_state::defects::burgers_vector_magnitude(
                get_f(p, "a")?,
                get_i(p, "h")? as i32,
                get_i(p, "k")? as i32,
                get_i(p, "l")? as i32,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
