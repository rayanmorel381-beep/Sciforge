//! Dispatch handler for solution chemistry functions.

use super::super::params::*;
use crate::hub::domain::chemistry as chem;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "boiling_point_elevation" => Ok(RunOutput::Scalar(
            chem::solutions::colligative::boiling_point_elevation(
                get_f(p, "kb")?,
                get_f(p, "molality")?,
                get_f(p, "vant_hoff_factor")?,
            ),
        )),
        "freezing_point_depression" => Ok(RunOutput::Scalar(
            chem::solutions::colligative::freezing_point_depression(
                get_f(p, "kf")?,
                get_f(p, "molality")?,
                get_f(p, "vant_hoff_factor")?,
            ),
        )),
        "osmotic_pressure" => Ok(RunOutput::Scalar(
            chem::solutions::colligative::osmotic_pressure(
                get_f(p, "molarity")?,
                get_f(p, "temperature")?,
                get_f(p, "vant_hoff_factor")?,
            ),
        )),
        "vapor_pressure_lowering" => Ok(RunOutput::Scalar(
            chem::solutions::colligative::vapor_pressure_lowering(
                get_f(p, "x_solvent")?,
                get_f(p, "p0_solvent")?,
            ),
        )),
        "molar_mass_from_ebullioscopy" => Ok(RunOutput::Scalar(
            chem::solutions::colligative::molar_mass_from_ebullioscopy(
                get_f(p, "kb")?,
                get_f(p, "mass_solute")?,
                get_f(p, "mass_solvent_kg")?,
                get_f(p, "delta_t")?,
            ),
        )),
        "molar_mass_from_cryoscopy" => Ok(RunOutput::Scalar(
            chem::solutions::colligative::molar_mass_from_cryoscopy(
                get_f(p, "kf")?,
                get_f(p, "mass_solute")?,
                get_f(p, "mass_solvent_kg")?,
                get_f(p, "delta_t")?,
            ),
        )),

        "raoult_law" => Ok(RunOutput::Scalar(chem::solutions::mixtures::raoult_law(
            get_f(p, "x_solvent")?,
            get_f(p, "p0_solvent")?,
        ))),
        "henrys_law" => Ok(RunOutput::Scalar(chem::solutions::mixtures::henrys_law(
            get_f(p, "kh")?,
            get_f(p, "partial_pressure")?,
        ))),
        "mole_fraction" => Ok(RunOutput::Scalar(chem::solutions::mixtures::mole_fraction(
            get_f(p, "moles_component")?,
            get_f(p, "total_moles")?,
        ))),
        "molality" => Ok(RunOutput::Scalar(chem::solutions::mixtures::molality(
            get_f(p, "moles_solute")?,
            get_f(p, "mass_solvent_kg")?,
        ))),
        "molarity" => Ok(RunOutput::Scalar(chem::solutions::mixtures::molarity(
            get_f(p, "moles_solute")?,
            get_f(p, "volume_liters")?,
        ))),
        "gibbs_duhem_check" => Ok(RunOutput::Scalar(
            chem::solutions::mixtures::gibbs_duhem_check(
                get_f(p, "x1")?,
                get_f(p, "d_mu1")?,
                get_f(p, "x2")?,
                get_f(p, "d_mu2")?,
            ),
        )),
        "activity_from_mole_fraction" => Ok(RunOutput::Scalar(
            chem::solutions::mixtures::activity_from_mole_fraction(
                get_f(p, "gamma")?,
                get_f(p, "x")?,
            ),
        )),
        "margules_one_suffix" => Ok(RunOutput::Scalar(
            chem::solutions::mixtures::margules_one_suffix(get_f(p, "a")?, get_f(p, "x1")?),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
