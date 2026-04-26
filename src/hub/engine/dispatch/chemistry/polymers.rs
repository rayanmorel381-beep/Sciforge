//! Dispatch handler for polymer chemistry functions.

use super::super::params::*;
use crate::hub::domain::chemistry as chem;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "number_average_molar_mass" => Ok(RunOutput::Scalar(
            chem::polymers::distributions::number_average_molar_mass(
                get_v(p, "ni")?,
                get_v(p, "mi")?,
            ),
        )),
        "weight_average_molar_mass" => Ok(RunOutput::Scalar(
            chem::polymers::distributions::weight_average_molar_mass(
                get_v(p, "ni")?,
                get_v(p, "mi")?,
            ),
        )),
        "z_average_molar_mass" => Ok(RunOutput::Scalar(
            chem::polymers::distributions::z_average_molar_mass(get_v(p, "ni")?, get_v(p, "mi")?),
        )),
        "schulz_flory_distribution" => Ok(RunOutput::Scalar(
            chem::polymers::distributions::schulz_flory_distribution(
                get_f(p, "p")?,
                get_i(p, "x")? as u32,
            ),
        )),
        "most_probable_chain_length" => Ok(RunOutput::Scalar(
            chem::polymers::distributions::most_probable_chain_length(get_f(p, "p")?),
        )),
        "flory_huggins_free_energy" => Ok(RunOutput::Scalar(
            chem::polymers::distributions::flory_huggins_free_energy(
                get_f(p, "phi")?,
                get_f(p, "n1")?,
                get_f(p, "n2")?,
                get_f(p, "chi")?,
                get_f(p, "temperature")?,
            ),
        )),

        "degree_of_polymerization_number" => Ok(RunOutput::Scalar(
            chem::polymers::properties::degree_of_polymerization_number(
                get_f(p, "mn")?,
                get_f(p, "m0")?,
            ),
        )),
        "degree_of_polymerization_weight" => Ok(RunOutput::Scalar(
            chem::polymers::properties::degree_of_polymerization_weight(
                get_f(p, "mw")?,
                get_f(p, "m0")?,
            ),
        )),
        "polydispersity_index" => Ok(RunOutput::Scalar(
            chem::polymers::properties::polydispersity_index(get_f(p, "mw")?, get_f(p, "mn")?),
        )),
        "intrinsic_viscosity_mark_houwink" => Ok(RunOutput::Scalar(
            chem::polymers::properties::intrinsic_viscosity_mark_houwink(
                get_f(p, "k")?,
                get_f(p, "m")?,
                get_f(p, "a")?,
            ),
        )),
        "end_to_end_distance_freely_jointed" => Ok(RunOutput::Scalar(
            chem::polymers::properties::end_to_end_distance_freely_jointed(
                get_f(p, "n")?,
                get_f(p, "l")?,
            ),
        )),
        "radius_of_gyration" => Ok(RunOutput::Scalar(
            chem::polymers::properties::radius_of_gyration(get_f(p, "end_to_end")?),
        )),
        "glass_transition_fox" => Ok(RunOutput::Scalar(
            chem::polymers::properties::glass_transition_fox(
                get_f(p, "w1")?,
                get_f(p, "tg1")?,
                get_f(p, "w2")?,
                get_f(p, "tg2")?,
            ),
        )),
        "carothers_equation" => Ok(RunOutput::Scalar(
            chem::polymers::properties::carothers_equation(get_f(p, "p")?, get_f(p, "f_avg")?),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
