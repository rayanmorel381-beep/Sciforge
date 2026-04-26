//! Dispatch handler for analytical chemistry functions.

use super::super::params::*;
use crate::hub::domain::chemistry as chem;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "retention_factor_rf" => Ok(RunOutput::Scalar(
            chem::analytical::chromatography::retention_factor_rf(
                get_f(p, "distance_solute")?,
                get_f(p, "distance_solvent")?,
            ),
        )),
        "hplc_resolution" => Ok(RunOutput::Scalar(
            chem::analytical::chromatography::hplc_resolution(
                get_f(p, "tr1")?,
                get_f(p, "tr2")?,
                get_f(p, "w1")?,
                get_f(p, "w2")?,
            ),
        )),
        "theoretical_plates" => Ok(RunOutput::Scalar(
            chem::analytical::chromatography::theoretical_plates(get_f(p, "tr")?, get_f(p, "w")?),
        )),
        "height_equivalent_theoretical_plate" => Ok(RunOutput::Scalar(
            chem::analytical::chromatography::height_equivalent_theoretical_plate(
                get_f(p, "column_length")?,
                get_f(p, "n_plates")?,
            ),
        )),
        "van_deemter" => Ok(RunOutput::Scalar(
            chem::analytical::chromatography::van_deemter(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "c")?,
                get_f(p, "u")?,
            ),
        )),
        "selectivity_factor" => Ok(RunOutput::Scalar(
            chem::analytical::chromatography::selectivity_factor(get_f(p, "k2")?, get_f(p, "k1")?),
        )),
        "capacity_factor" => Ok(RunOutput::Scalar(
            chem::analytical::chromatography::capacity_factor(get_f(p, "tr")?, get_f(p, "t0")?),
        )),

        "dilution" => Ok(RunOutput::Scalar(chem::analytical::quantitative::dilution(
            get_f(p, "c1")?,
            get_f(p, "v1")?,
            get_f(p, "v2")?,
        ))),
        "titration_equivalence_volume" => Ok(RunOutput::Scalar(
            chem::analytical::quantitative::titration_equivalence_volume(
                get_f(p, "c_analyte")?,
                get_f(p, "v_analyte")?,
                get_f(p, "c_titrant")?,
                get_f(p, "stoich_ratio")?,
            ),
        )),
        "limit_of_detection" => Ok(RunOutput::Scalar(
            chem::analytical::quantitative::limit_of_detection(
                get_f(p, "blank_std")?,
                get_f(p, "slope")?,
            ),
        )),
        "limit_of_quantitation" => Ok(RunOutput::Scalar(
            chem::analytical::quantitative::limit_of_quantitation(
                get_f(p, "blank_std")?,
                get_f(p, "slope")?,
            ),
        )),
        "percent_recovery" => Ok(RunOutput::Scalar(
            chem::analytical::quantitative::percent_recovery(
                get_f(p, "measured")?,
                get_f(p, "expected")?,
            ),
        )),
        "relative_standard_deviation" => Ok(RunOutput::Scalar(
            chem::analytical::quantitative::relative_standard_deviation(
                get_f(p, "std_dev")?,
                get_f(p, "mean")?,
            ),
        )),
        "gravimetric_factor" => Ok(RunOutput::Scalar(
            chem::analytical::quantitative::gravimetric_factor(
                get_f(p, "mw_analyte")?,
                get_f(p, "mw_precipitate")?,
                get_f(p, "stoich")?,
            ),
        )),
        "karl_fischer_water_content" => Ok(RunOutput::Scalar(
            chem::analytical::quantitative::karl_fischer_water_content(
                get_f(p, "volume_reagent")?,
                get_f(p, "reagent_factor")?,
                get_f(p, "sample_mass")?,
            ),
        )),

        "beer_lambert" => Ok(RunOutput::Scalar(
            chem::analytical::spectrophotometry::beer_lambert(
                get_f(p, "epsilon")?,
                get_f(p, "path_length")?,
                get_f(p, "concentration")?,
            ),
        )),
        "absorbance_to_transmittance" => Ok(RunOutput::Scalar(
            chem::analytical::spectrophotometry::absorbance_to_transmittance(get_f(
                p,
                "absorbance",
            )?),
        )),
        "transmittance_to_absorbance" => Ok(RunOutput::Scalar(
            chem::analytical::spectrophotometry::transmittance_to_absorbance(get_f(
                p,
                "transmittance",
            )?),
        )),
        "concentration_from_absorbance" => Ok(RunOutput::Scalar(
            chem::analytical::spectrophotometry::concentration_from_absorbance(
                get_f(p, "absorbance")?,
                get_f(p, "epsilon")?,
                get_f(p, "path_length")?,
            ),
        )),
        "signal_to_noise" => Ok(RunOutput::Scalar(
            chem::analytical::spectrophotometry::signal_to_noise(
                get_f(p, "signal")?,
                get_f(p, "noise")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
