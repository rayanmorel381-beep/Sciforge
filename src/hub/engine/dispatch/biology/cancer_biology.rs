//! Dispatch handler for cancer biology functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "tumor_immune_ode" => {
            let (a, b) = bio::cancer_biology::immunology::tumor_immune_ode(
                get_f(p, "tumor")?,
                get_f(p, "immune")?,
                get_f(p, "growth_rate")?,
                get_f(p, "carrying_capacity")?,
                get_f(p, "kill_rate")?,
                get_f(p, "stimulation")?,
                get_f(p, "decay_rate")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "tumor_immune_simulate" => Ok(RunOutput::PairVec(
            bio::cancer_biology::immunology::tumor_immune_simulate(
                get_f(p, "tumor0")?,
                get_f(p, "immune0")?,
                get_f(p, "growth_rate")?,
                get_f(p, "carrying_capacity")?,
                get_f(p, "kill_rate")?,
                get_f(p, "stimulation")?,
                get_f(p, "decay_rate")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            ),
        )),
        "immunoediting_escape" => Ok(RunOutput::Scalar(
            bio::cancer_biology::immunology::immunoediting_escape(
                get_f(p, "immunogenic_clones")?,
                get_f(p, "escape_mutation_rate")?,
                get_f(p, "immune_pressure")?,
            ),
        )),
        "checkpoint_blockade_effect" => Ok(RunOutput::Scalar(
            bio::cancer_biology::immunology::checkpoint_blockade_effect(
                get_f(p, "baseline_kill")?,
                get_f(p, "pd1_inhibition")?,
                get_f(p, "ctla4_inhibition")?,
            ),
        )),
        "car_t_cell_expansion" => Ok(RunOutput::Scalar(
            bio::cancer_biology::immunology::car_t_cell_expansion(
                get_f(p, "initial_cells")?,
                get_f(p, "antigen_density")?,
                get_f(p, "expansion_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "cytokine_release_syndrome" => Ok(RunOutput::Scalar(
            bio::cancer_biology::immunology::cytokine_release_syndrome(
                get_f(p, "activated_cells")?,
                get_f(p, "cytokine_per_cell")?,
                get_f(p, "clearance_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "tumor_neoantigen_fitness" => Ok(RunOutput::Scalar(
            bio::cancer_biology::immunology::tumor_neoantigen_fitness(
                get_f(p, "binding_affinity")?,
                get_f(p, "expression_level")?,
                get_f(p, "clonality")?,
            ),
        )),
        "abscopal_effect" => Ok(RunOutput::Scalar(
            bio::cancer_biology::immunology::abscopal_effect(
                get_f(p, "local_dose")?,
                get_f(p, "immune_activation")?,
                get_f(p, "distant_tumor")?,
                get_f(p, "sensitivity")?,
            ),
        )),
        "tumor_angiogenesis_vegf" => Ok(RunOutput::Scalar(
            bio::cancer_biology::microenvironment::tumor_angiogenesis_vegf(
                get_f(p, "vegf")?,
                get_f(p, "endothelial_proliferation_rate")?,
                get_f(p, "kd")?,
            ),
        )),
        "vessel_density" => Ok(RunOutput::Scalar(
            bio::cancer_biology::microenvironment::vessel_density(
                get_f(p, "new_vessels")?,
                get_f(p, "existing_vessels")?,
                get_f(p, "regression_rate")?,
                get_f(p, "dt")?,
            ),
        )),
        "cancer_oxygen_diffusion_krogh" => Ok(RunOutput::Scalar(
            bio::cancer_biology::microenvironment::oxygen_diffusion_krogh(
                get_f(p, "p_vessel")?,
                get_f(p, "consumption_rate")?,
                get_f(p, "diffusion_coeff")?,
                get_f(p, "r")?,
                get_f(p, "r_vessel")?,
            ),
        )),
        "hypoxia_fraction" => Ok(RunOutput::Scalar(
            bio::cancer_biology::microenvironment::hypoxia_fraction(
                get_v(p, "distances")?,
                get_f(p, "diffusion_limit")?,
            ),
        )),
        "microenvironment_tmb" => Ok(RunOutput::Scalar(
            bio::cancer_biology::microenvironment::microenvironment_tmb(
                get_u(p, "mutations")?,
                get_f(p, "megabases_sequenced")?,
            ),
        )),
        "clonal_fitness_advantage" => Ok(RunOutput::Scalar(
            bio::cancer_biology::microenvironment::clonal_fitness_advantage(
                get_v(p, "clone_sizes")?,
                get_v(p, "fitness_values")?,
            ),
        )),
        "tumor_heterogeneity_shannon" => Ok(RunOutput::Scalar(
            bio::cancer_biology::microenvironment::tumor_heterogeneity_shannon(get_v(
                p,
                "clone_fractions",
            )?),
        )),
        "metastatic_probability" => Ok(RunOutput::Scalar(
            bio::cancer_biology::microenvironment::metastatic_probability(
                get_f(p, "invasion_rate")?,
                get_f(p, "survival_fraction")?,
                get_f(p, "colonization_rate")?,
                get_f(p, "time")?,
            ),
        )),
        "emt_score" => Ok(RunOutput::Scalar(
            bio::cancer_biology::microenvironment::emt_score(
                get_v(p, "epithelial_markers")?,
                get_v(p, "mesenchymal_markers")?,
            ),
        )),
        "immune_escape_probability" => Ok(RunOutput::Scalar(
            bio::cancer_biology::microenvironment::immune_escape_probability(
                get_f(p, "mhc_expression")?,
                get_f(p, "pd_l1")?,
                get_f(p, "neoantigen_load")?,
            ),
        )),
        "csc_fraction" => Ok(RunOutput::Scalar(
            bio::cancer_biology::microenvironment::csc_fraction(
                get_f(p, "symmetric_division_rate")?,
                get_f(p, "asymmetric_rate")?,
                get_f(p, "differentiation_rate")?,
            ),
        )),
        "pharmacokinetic_tumor_exposure" => Ok(RunOutput::Scalar(
            bio::cancer_biology::microenvironment::pharmacokinetic_tumor_exposure(
                get_f(p, "dose")?,
                get_f(p, "bioavailability")?,
                get_f(p, "volume_distribution")?,
                get_f(p, "tumor_perfusion_fraction")?,
            ),
        )),
        "cell_kill_log" => {
            let cycles = get_i(p, "cycles")? as u32;
            Ok(RunOutput::Scalar(
                bio::cancer_biology::therapy::cell_kill_log(
                    get_f(p, "initial")?,
                    get_f(p, "surviving_fraction")?,
                    cycles,
                ),
            ))
        }
        "skipper_schabel_log_kill" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::skipper_schabel_log_kill(
                get_f(p, "n")?,
                get_f(p, "dose")?,
                get_f(p, "sensitivity")?,
            ),
        )),
        "drug_resistance_goldie_coldman" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::drug_resistance_goldie_coldman(
                get_f(p, "n")?,
                get_f(p, "mutation_rate")?,
            ),
        )),
        "combination_therapy_survival" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::combination_therapy_survival(
                get_f(p, "sf_a")?,
                get_f(p, "sf_b")?,
                get_f(p, "interaction")?,
            ),
        )),
        "tumor_immune_interaction" => {
            let (a, b) = bio::cancer_biology::therapy::tumor_immune_interaction(
                get_f(p, "tumor")?,
                get_f(p, "immune")?,
                get_f(p, "growth_rate")?,
                get_f(p, "kill_rate")?,
                get_f(p, "stimulation")?,
                get_f(p, "decay")?,
                get_f(p, "k")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "hallmarks_proliferation_index" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::hallmarks_proliferation_index(
                get_f(p, "mitotic_count")?,
                get_f(p, "area")?,
            ),
        )),
        "cancer_stem_cell_fraction" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::cancer_stem_cell_fraction(
                get_f(p, "symmetric_division_rate")?,
                get_f(p, "asymmetric_division_rate")?,
                get_f(p, "differentiation_rate")?,
            ),
        )),
        "cancer_linear_quadratic_survival" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::linear_quadratic_survival(
                get_f(p, "dose")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
            ),
        )),
        "cancer_biologically_effective_dose" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::biologically_effective_dose(
                get_f(p, "dose")?,
                get_f(p, "fractions")?,
                get_f(p, "alpha_beta")?,
            ),
        )),
        "cancer_equivalent_dose_2gy" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::equivalent_dose_2gy(
                get_f(p, "dose")?,
                get_f(p, "dose_per_fraction")?,
                get_f(p, "alpha_beta")?,
            ),
        )),
        "cancer_tumor_control_probability" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::tumor_control_probability(
                get_f(p, "n_cells")?,
                get_f(p, "surviving_fraction")?,
            ),
        )),
        "cancer_normal_tissue_complication_probability" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::normal_tissue_complication_probability(
                get_f(p, "dose")?,
                get_f(p, "td50")?,
                get_f(p, "gamma50")?,
            ),
        )),
        "cancer_therapeutic_ratio" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::therapeutic_ratio(get_f(p, "tcp")?, get_f(p, "ntcp")?),
        )),
        "immunotherapy_checkpoint_response" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::immunotherapy_checkpoint_response(
                get_f(p, "tumor")?,
                get_f(p, "t_cells")?,
                get_f(p, "activation_rate")?,
                get_f(p, "exhaustion_rate")?,
                get_f(p, "checkpoint_blockade")?,
            ),
        )),
        "car_t_expansion" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::car_t_expansion(
                get_f(p, "initial_cells")?,
                get_f(p, "antigen_stimulation")?,
                get_f(p, "expansion_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "antibody_drug_conjugate_kill" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::antibody_drug_conjugate_kill(
                get_f(p, "antibody_conc")?,
                get_f(p, "target_density")?,
                get_f(p, "internalization_rate")?,
                get_f(p, "drug_potency")?,
                get_f(p, "kd")?,
            ),
        )),
        "metronomic_antiangiogenic_effect" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::metronomic_antiangiogenic_effect(
                get_f(p, "dose")?,
                get_f(p, "frequency")?,
                get_f(p, "sensitivity")?,
            ),
        )),
        "fractionation_schedule_bde" => {
            let n_fractions = get_i(p, "n_fractions")? as u32;
            Ok(RunOutput::Scalar(
                bio::cancer_biology::therapy::fractionation_schedule_bde(
                    n_fractions,
                    get_f(p, "dose_per_fraction")?,
                    get_f(p, "alpha_beta")?,
                ),
            ))
        }
        "cell_cycle_specific_kill" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::cell_cycle_specific_kill(
                get_f(p, "drug_conc")?,
                get_f(p, "phase_fraction")?,
                get_f(p, "sensitivity")?,
            ),
        )),
        "combination_index_chou_talalay" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::combination_index_chou_talalay(
                get_f(p, "fa")?,
                get_f(p, "dose_a")?,
                get_f(p, "dose_b")?,
                get_f(p, "dm_a")?,
                get_f(p, "dm_b")?,
                get_f(p, "m_a")?,
                get_f(p, "m_b")?,
            ),
        )),
        "radiation_oxygen_enhancement_ratio" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::radiation_oxygen_enhancement_ratio(
                get_f(p, "dose_hypoxic")?,
                get_f(p, "dose_aerobic")?,
            ),
        )),
        "hyperthermia_enhancement" => Ok(RunOutput::Scalar(
            bio::cancer_biology::therapy::hyperthermia_enhancement(
                get_f(p, "dose")?,
                get_f(p, "thermal_enhancement_ratio")?,
            ),
        )),
        "tumor_growth_gompertz" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::tumor_growth_gompertz(
                get_f(p, "n")?,
                get_f(p, "n_max")?,
                get_f(p, "alpha")?,
                get_f(p, "dt")?,
            ),
        )),
        "tumor_growth_logistic" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::tumor_growth_logistic(
                get_f(p, "n")?,
                get_f(p, "k")?,
                get_f(p, "r")?,
                get_f(p, "dt")?,
            ),
        )),
        "tumor_doubling_time" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::tumor_doubling_time(get_f(p, "growth_rate")?),
        )),
        "clonal_evolution_fitness" => Ok(RunOutput::Vector(
            bio::cancer_biology::tumor::clonal_evolution_fitness(
                get_v(p, "clone_sizes")?,
                get_v(p, "fitness")?,
                get_f(p, "mutation_rate")?,
            ),
        )),
        "metastasis_probability" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::metastasis_probability(
                get_f(p, "tumor_size")?,
                get_f(p, "shedding_rate")?,
                get_f(p, "survival_fraction")?,
                get_f(p, "colonization_rate")?,
            ),
        )),
        "tumor_angiogenesis_rate" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::tumor_angiogenesis_rate(
                get_f(p, "tumor_size")?,
                get_f(p, "vegf_production")?,
                get_f(p, "inhibitor")?,
                get_f(p, "threshold")?,
            ),
        )),
        "norton_simon_regression" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::norton_simon_regression(
                get_f(p, "n")?,
                get_f(p, "kill_fraction")?,
                get_f(p, "gompertz_rate")?,
                get_f(p, "n_max")?,
            ),
        )),
        "tumor_growth_exponential" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::tumor_growth_exponential(
                get_f(p, "n0")?,
                get_f(p, "rate")?,
                get_f(p, "t")?,
            ),
        )),
        "tumor_growth_von_bertalanffy" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::tumor_growth_von_bertalanffy(
                get_f(p, "n")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "dt")?,
            ),
        )),
        "tumor_volume_spherical" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::tumor_volume_spherical(get_f(p, "diameter")?),
        )),
        "tumor_volume_ellipsoid" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::tumor_volume_ellipsoid(
                get_f(p, "length")?,
                get_f(p, "width")?,
                get_f(p, "height")?,
            ),
        )),
        "recist_response" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::recist_response(
                get_f(p, "baseline_diameter")?,
                get_f(p, "current_diameter")?,
            ),
        )),
        "tumor_mutation_burden" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::tumor_mutation_burden(
                get_f(p, "somatic_mutations")?,
                get_f(p, "exome_size_mb")?,
            ),
        )),
        "heterogeneity_shannon" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::heterogeneity_shannon(get_v(p, "clone_fractions")?),
        )),
        "circulating_tumor_cells" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::circulating_tumor_cells(
                get_f(p, "shedding")?,
                get_f(p, "tumor_size")?,
                get_f(p, "half_life")?,
            ),
        )),
        "warburg_glycolysis_rate" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::warburg_glycolysis_rate(
                get_f(p, "glucose")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
                get_f(p, "oxygen_inhibition")?,
                get_f(p, "oxygen")?,
            ),
        )),
        "hypoxia_inducible_factor" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::hypoxia_inducible_factor(
                get_f(p, "po2")?,
                get_f(p, "km_o2")?,
                get_f(p, "max_expression")?,
            ),
        )),
        "necrotic_core_radius" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::necrotic_core_radius(
                get_f(p, "tumor_radius")?,
                get_f(p, "diffusion_length")?,
            ),
        )),
        "viable_rim_fraction" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::viable_rim_fraction(
                get_f(p, "tumor_radius")?,
                get_f(p, "necrotic_radius")?,
            ),
        )),
        "ctc_cluster_survival" => {
            let cluster_size = get_i(p, "cluster_size")? as u32;
            Ok(RunOutput::Scalar(
                bio::cancer_biology::tumor::ctc_cluster_survival(
                    get_f(p, "single_ctc_survival")?,
                    cluster_size,
                ),
            ))
        }
        "invasion_index" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::invasion_index(
                get_f(p, "invaded_distance")?,
                get_f(p, "time")?,
            ),
        )),
        "epithelial_mesenchymal_transition" => Ok(RunOutput::Scalar(
            bio::cancer_biology::tumor::epithelial_mesenchymal_transition(
                get_f(p, "tgf_beta")?,
                get_f(p, "threshold")?,
                get_f(p, "hill")?,
            ),
        )),
        "microsatellite_instability_score" => {
            let unstable_markers = get_i(p, "unstable_markers")? as u32;
            let total_markers = get_i(p, "total_markers")? as u32;
            Ok(RunOutput::Scalar(
                bio::cancer_biology::tumor::microsatellite_instability_score(
                    unstable_markers,
                    total_markers,
                ),
            ))
        }
        "oxygen_diffusion_krogh" => Ok(RunOutput::Scalar(
            bio::cancer_biology::microenvironment::oxygen_diffusion_krogh(
                get_f(p, "p_vessel")?,
                get_f(p, "consumption_rate")?,
                get_f(p, "diffusion_coeff")?,
                get_f(p, "r")?,
                get_f(p, "r_vessel")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
