//! Dispatch handler for acid-base chemistry functions.

use super::super::params::*;
use crate::domain::chemistry as chem;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "henderson_hasselbalch" => Ok(RunOutput::Scalar(
            chem::acid_base::equilibria::henderson_hasselbalch(
                get_f(p, "pka")?,
                get_f(p, "base")?,
                get_f(p, "acid")?,
            ),
        )),
        "pka_from_ka" => Ok(RunOutput::Scalar(chem::acid_base::equilibria::pka_from_ka(
            get_f(p, "ka")?,
        ))),
        "ka_from_pka" => Ok(RunOutput::Scalar(chem::acid_base::equilibria::ka_from_pka(
            get_f(p, "pka")?,
        ))),
        "pkb_from_pka" => Ok(RunOutput::Scalar(
            chem::acid_base::equilibria::pkb_from_pka(get_f(p, "pka")?, get_f(p, "pkw")?),
        )),
        "ph_strong_acid" => Ok(RunOutput::Scalar(
            chem::acid_base::equilibria::ph_strong_acid(get_f(p, "concentration")?),
        )),
        "ph_strong_base" => Ok(RunOutput::Scalar(
            chem::acid_base::equilibria::ph_strong_base(get_f(p, "concentration")?),
        )),
        "ph_weak_acid" => Ok(RunOutput::Scalar(
            chem::acid_base::equilibria::ph_weak_acid(get_f(p, "ka")?, get_f(p, "c")?),
        )),
        "ph_weak_base" => Ok(RunOutput::Scalar(
            chem::acid_base::equilibria::ph_weak_base(get_f(p, "kb")?, get_f(p, "c")?),
        )),
        "alpha_fraction" => Ok(RunOutput::Scalar(
            chem::acid_base::equilibria::alpha_fraction(
                get_f(p, "h")?,
                get_v(p, "ka_values")?,
                get_u(p, "species_index")?,
            ),
        )),
        "amphiprotic_ph" => Ok(RunOutput::Scalar(
            chem::acid_base::equilibria::amphiprotic_ph(get_f(p, "pka1")?, get_f(p, "pka2")?),
        )),
        "ionic_product_water" => Ok(RunOutput::Scalar(
            chem::acid_base::equilibria::ionic_product_water(get_f(p, "t")?),
        )),

        "strong_acid_strong_base_ph" => Ok(RunOutput::Scalar(
            chem::acid_base::titrations::strong_acid_strong_base_ph(
                get_f(p, "c_acid")?,
                get_f(p, "v_acid")?,
                get_f(p, "c_base")?,
                get_f(p, "v_base")?,
            ),
        )),
        "weak_acid_strong_base_ph" => Ok(RunOutput::Scalar(
            chem::acid_base::titrations::weak_acid_strong_base_ph(
                get_f(p, "c_acid")?,
                get_f(p, "v_acid")?,
                get_f(p, "ka")?,
                get_f(p, "c_base")?,
                get_f(p, "v_base")?,
            ),
        )),
        "equivalence_point_volume" => Ok(RunOutput::Scalar(
            chem::acid_base::titrations::equivalence_point_volume(
                get_f(p, "c_analyte")?,
                get_f(p, "v_analyte")?,
                get_f(p, "c_titrant")?,
            ),
        )),
        "half_equivalence_ph" => Ok(RunOutput::Scalar(
            chem::acid_base::titrations::half_equivalence_ph(get_f(p, "pka")?),
        )),
        "buffer_range" => {
            let (lo, hi) = chem::acid_base::titrations::buffer_range(get_f(p, "pka")?);
            Ok(RunOutput::Pair(lo, hi))
        }
        "buffer_capacity_vanslyke" => Ok(RunOutput::Scalar(
            chem::acid_base::titrations::buffer_capacity_vanslyke(
                get_f(p, "c_total")?,
                get_f(p, "ka")?,
                get_f(p, "h")?,
            ),
        )),
        "diprotic_ph_first_equiv" => Ok(RunOutput::Scalar(
            chem::acid_base::titrations::diprotic_ph_first_equiv(
                get_f(p, "pka1")?,
                get_f(p, "pka2")?,
            ),
        )),
        "back_titration_moles" => Ok(RunOutput::Scalar(
            chem::acid_base::titrations::back_titration_moles(
                get_f(p, "mol_excess_added")?,
                get_f(p, "c_back_titrant")?,
                get_f(p, "v_back_titrant")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
