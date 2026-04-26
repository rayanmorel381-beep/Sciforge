//! Dispatch handler for proteomics functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "b_ion_masses" => Ok(RunOutput::Vector(bio::proteomics::mass_spec::b_ion_masses(
            get_str(p, "sequence")?,
        ))),
        "y_ion_masses" => Ok(RunOutput::Vector(bio::proteomics::mass_spec::y_ion_masses(
            get_str(p, "sequence")?,
        ))),
        "mz_ratio" => Ok(RunOutput::Scalar(bio::proteomics::mass_spec::mz_ratio(
            get_f(p, "mass")?,
            get_u(p, "charge")?,
        ))),
        "mass_from_mz" => Ok(RunOutput::Scalar(bio::proteomics::mass_spec::mass_from_mz(
            get_f(p, "mz")?,
            get_u(p, "charge")?,
        ))),
        "mass_accuracy_ppm" => Ok(RunOutput::Scalar(
            bio::proteomics::mass_spec::mass_accuracy_ppm(
                get_f(p, "theoretical")?,
                get_f(p, "observed")?,
            ),
        )),
        "isotope_pattern_averagine" => Ok(RunOutput::Vector(
            bio::proteomics::mass_spec::isotope_pattern_averagine(
                get_f(p, "mass")?,
                get_u(p, "n_peaks")?,
            ),
        )),
        "ppi_degree" => {
            let adjacency_row_v = get_v(p, "adjacency_row")?;
            let adjacency_row: Vec<bool> = adjacency_row_v.iter().map(|&x| x != 0.0).collect();
            Ok(RunOutput::Integer(
                bio::proteomics::networks::ppi_degree(&adjacency_row) as i64,
            ))
        }
        "clustering_coefficient" => Ok(RunOutput::Scalar(
            bio::proteomics::networks::clustering_coefficient(
                get_u(p, "neighbors_connected")?,
                get_u(p, "degree")?,
            ),
        )),
        "betweenness_centrality_approx" => Ok(RunOutput::Scalar(
            bio::proteomics::networks::betweenness_centrality_approx(
                get_f(p, "shortest_paths_through")?,
                get_f(p, "total_shortest_paths")?,
            ),
        )),
        "network_density" => Ok(RunOutput::Scalar(
            bio::proteomics::networks::network_density(get_u(p, "edges")?, get_u(p, "nodes")?),
        )),
        "scale_free_exponent" => Ok(RunOutput::Scalar(
            bio::proteomics::networks::scale_free_exponent(get_v(p, "degree_distribution")?),
        )),
        "hub_score" => Ok(RunOutput::Scalar(bio::proteomics::networks::hub_score(
            get_u(p, "degree")?,
            get_u(p, "max_degree")?,
        ))),
        "edge_betweenness" => Ok(RunOutput::Scalar(
            bio::proteomics::networks::edge_betweenness(
                get_f(p, "flow_through_edge")?,
                get_f(p, "total_flow")?,
            ),
        )),
        "protein_complex_stoichiometry" => Ok(RunOutput::Vector(
            bio::proteomics::networks::protein_complex_stoichiometry(get_v(p, "abundances")?),
        )),
        "functional_enrichment_odds_ratio" => Ok(RunOutput::Scalar(
            bio::proteomics::networks::functional_enrichment_odds_ratio(
                get_u(p, "hits_in_set")?,
                get_u(p, "set_size")?,
                get_u(p, "hits_total")?,
                get_u(p, "genome_size")?,
            ),
        )),
        "guilt_by_association_score" => {
            let neighbor_annotations_v = get_v(p, "neighbor_annotations")?;
            let neighbor_annotations: Vec<bool> =
                neighbor_annotations_v.iter().map(|&x| x != 0.0).collect();
            Ok(RunOutput::Scalar(
                bio::proteomics::networks::guilt_by_association_score(&neighbor_annotations),
            ))
        }
        "peptide_molecular_weight" => Ok(RunOutput::Scalar(
            bio::proteomics::properties::peptide_molecular_weight(get_str(p, "sequence")?),
        )),
        "isoelectric_point" => Ok(RunOutput::Scalar(
            bio::proteomics::properties::isoelectric_point(get_str(p, "sequence")?),
        )),
        "gravy_index" => Ok(RunOutput::Scalar(bio::proteomics::properties::gravy_index(
            get_str(p, "sequence")?,
        ))),
        "extinction_coefficient_280" => Ok(RunOutput::Scalar(
            bio::proteomics::properties::extinction_coefficient_280(
                get_u(p, "n_trp")?,
                get_u(p, "n_tyr")?,
                get_u(p, "n_cystine")?,
            ),
        )),
        "spectral_count_nsaf" => Ok(RunOutput::Scalar(
            bio::proteomics::quantification::spectral_count_nsaf(
                get_f(p, "spectral_count")?,
                get_f(p, "protein_length")?,
                get_f(p, "total_nsaf")?,
            ),
        )),
        "ibaq" => Ok(RunOutput::Scalar(bio::proteomics::quantification::ibaq(
            get_f(p, "total_intensity")?,
            get_u(p, "num_observable_peptides")?,
        ))),
        "lfq_ratio" => Ok(RunOutput::Scalar(
            bio::proteomics::quantification::lfq_ratio(
                get_f(p, "intensity_sample")?,
                get_f(p, "intensity_reference")?,
            ),
        )),
        "tmt_reporter_ratio" => Ok(RunOutput::Scalar(
            bio::proteomics::quantification::tmt_reporter_ratio(
                get_f(p, "reporter_intensity")?,
                get_f(p, "reference_channel")?,
            ),
        )),
        "silac_ratio" => Ok(RunOutput::Scalar(
            bio::proteomics::quantification::silac_ratio(get_f(p, "heavy")?, get_f(p, "light")?),
        )),
        "protein_fdr" => Ok(RunOutput::Scalar(
            bio::proteomics::quantification::protein_fdr(
                get_f(p, "decoy_hits")?,
                get_f(p, "target_hits")?,
            ),
        )),
        "mascot_ion_score" => Ok(RunOutput::Scalar(
            bio::proteomics::quantification::mascot_ion_score(
                get_f(p, "observed")?,
                get_f(p, "expected")?,
            ),
        )),
        "em_pai" => Ok(RunOutput::Scalar(bio::proteomics::quantification::em_pai(
            get_u(p, "observed_peptides")?,
            get_u(p, "observable_peptides")?,
        ))),
        "protein_coverage" => Ok(RunOutput::Scalar(
            bio::proteomics::quantification::protein_coverage(
                get_u(p, "identified_residues")?,
                get_u(p, "total_residues")?,
            ),
        )),
        "xcorr_normalized" => Ok(RunOutput::Scalar(
            bio::proteomics::quantification::xcorr_normalized(
                get_f(p, "xcorr")?,
                get_u(p, "peptide_length")?,
            ),
        )),
        "missed_cleavage_rate" => Ok(RunOutput::Scalar(
            bio::proteomics::quantification::missed_cleavage_rate(
                get_u(p, "missed")?,
                get_u(p, "total_peptides")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
