//! Dispatch handler for aging biology functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "telomere_shortening" => Ok(RunOutput::Scalar(bio::aging::damage::telomere_shortening(
            get_f(p, "initial_length")?,
            get_f(p, "loss_per_division")?,
            get_f(p, "divisions")?,
        ))),
        "aging_hayflick_limit" => Ok(RunOutput::Scalar(bio::aging::damage::hayflick_limit(
            get_f(p, "initial_length")?,
            get_f(p, "critical_length")?,
            get_f(p, "loss_per_division")?,
        ))),
        "telomerase_equilibrium" => Ok(RunOutput::Scalar(
            bio::aging::damage::telomerase_equilibrium(
                get_f(p, "shortening_rate")?,
                get_f(p, "elongation_rate")?,
                get_f(p, "initial")?,
                get_f(p, "t")?,
            ),
        )),
        "oxidative_damage_accumulation" => Ok(RunOutput::Scalar(
            bio::aging::damage::oxidative_damage_accumulation(
                get_f(p, "production_rate")?,
                get_f(p, "repair_rate")?,
                get_f(p, "t")?,
                get_f(p, "initial_damage")?,
            ),
        )),
        "mitochondrial_damage" => Ok(RunOutput::Scalar(bio::aging::damage::mitochondrial_damage(
            get_f(p, "intact_fraction")?,
            get_f(p, "damage_rate")?,
            get_f(p, "dt")?,
        ))),
        "senescent_cell_fraction" => Ok(RunOutput::Scalar(
            bio::aging::damage::senescent_cell_fraction(
                get_f(p, "division_rate")?,
                get_f(p, "senescence_prob")?,
                get_f(p, "clearance_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "caloric_restriction_lifespan" => Ok(RunOutput::Scalar(
            bio::aging::damage::caloric_restriction_lifespan(
                get_f(p, "base_lifespan")?,
                get_f(p, "restriction_fraction")?,
                get_f(p, "effect_coefficient")?,
            ),
        )),
        "reliability_theory_survival" => Ok(RunOutput::Scalar(
            bio::aging::damage::reliability_theory_survival(
                get_u(p, "n_elements")?,
                get_f(p, "element_failure_rate")?,
                get_u(p, "redundancy")?,
                get_f(p, "t")?,
            ),
        )),
        "ros_steady_state" => Ok(RunOutput::Scalar(bio::aging::damage::ros_steady_state(
            get_f(p, "production_rate")?,
            get_f(p, "sod_activity")?,
            get_f(p, "catalase_activity")?,
        ))),
        "protein_aggregation" => Ok(RunOutput::Scalar(bio::aging::damage::protein_aggregation(
            get_f(p, "misfolded")?,
            get_f(p, "aggregation_rate")?,
            get_f(p, "chaperone_capacity")?,
            get_f(p, "dt")?,
        ))),
        "dna_repair_capacity" => Ok(RunOutput::Scalar(bio::aging::damage::dna_repair_capacity(
            get_f(p, "age")?,
            get_f(p, "base_capacity")?,
            get_f(p, "decline_rate")?,
        ))),
        "somatic_mutation_accumulation" => Ok(RunOutput::Scalar(
            bio::aging::damage::somatic_mutation_accumulation(
                get_f(p, "mutation_rate")?,
                get_f(p, "divisions")?,
                get_f(p, "repair_efficiency")?,
            ),
        )),
        "epigenetic_clock_horvath" => Ok(RunOutput::Scalar(
            bio::aging::damage::epigenetic_clock_horvath(
                get_v(p, "cpg_values")?,
                get_v(p, "coefficients")?,
                get_f(p, "intercept")?,
            ),
        )),
        "nad_decline" => Ok(RunOutput::Scalar(bio::aging::damage::nad_decline(
            get_f(p, "initial_nad")?,
            get_f(p, "decline_rate")?,
            get_f(p, "age")?,
        ))),
        "aging_autophagy_flux" => Ok(RunOutput::Scalar(bio::aging::damage::autophagy_flux(
            get_f(p, "substrate")?,
            get_f(p, "autophagosome_formation")?,
            get_f(p, "lysosomal_activity")?,
            get_f(p, "age_factor")?,
        ))),
        "stem_cell_exhaustion" => Ok(RunOutput::Scalar(bio::aging::damage::stem_cell_exhaustion(
            get_f(p, "initial_pool")?,
            get_f(p, "division_rate")?,
            get_f(p, "senescence_prob")?,
            get_f(p, "age")?,
        ))),
        "inflammaging_cytokine" => Ok(RunOutput::Scalar(
            bio::aging::damage::inflammaging_cytokine(
                get_f(p, "basal")?,
                get_f(p, "senescent_cells")?,
                get_f(p, "amplification")?,
            ),
        )),
        "crosslink_accumulation" => Ok(RunOutput::Scalar(
            bio::aging::damage::crosslink_accumulation(
                get_f(p, "rate")?,
                get_f(p, "turnover")?,
                get_f(p, "t")?,
            ),
        )),
        "lipofuscin_accumulation" => Ok(RunOutput::Scalar(
            bio::aging::damage::lipofuscin_accumulation(
                get_f(p, "production_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "immune_senescence" => Ok(RunOutput::Scalar(bio::aging::damage::immune_senescence(
            get_f(p, "naive_t_cells")?,
            get_f(p, "thymic_output_rate")?,
            get_f(p, "age")?,
            get_f(p, "proliferation_capacity")?,
        ))),
        "gompertz_mortality_rate" => Ok(RunOutput::Scalar(
            bio::aging::mortality::gompertz_mortality_rate(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "age")?,
            ),
        )),
        "gompertz_makeham" => Ok(RunOutput::Scalar(bio::aging::mortality::gompertz_makeham(
            get_f(p, "a")?,
            get_f(p, "b")?,
            get_f(p, "c")?,
            get_f(p, "age")?,
        ))),
        "weibull_mortality_hazard" => Ok(RunOutput::Scalar(
            bio::aging::mortality::weibull_mortality_hazard(
                get_f(p, "lambda")?,
                get_f(p, "k")?,
                get_f(p, "age")?,
            ),
        )),
        "mortality_doubling_time" => Ok(RunOutput::Scalar(
            bio::aging::mortality::mortality_doubling_time(get_f(p, "b")?),
        )),
        "survival_probability" => Ok(RunOutput::Scalar(
            bio::aging::mortality::survival_probability(get_v(p, "hazard_rates")?, get_f(p, "dt")?),
        )),
        "deceleration_plateau" => Ok(RunOutput::Scalar(
            bio::aging::mortality::deceleration_plateau(
                get_f(p, "age")?,
                get_f(p, "plateau_age")?,
                get_f(p, "plateau_rate")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
            ),
        )),
        "frailty_deficit_index" => Ok(RunOutput::Scalar(
            bio::aging::mortality::frailty_deficit_index(
                get_u(p, "deficits")?,
                get_u(p, "total_items")?,
            ),
        )),
        "phenotypic_age_levine" => Ok(RunOutput::Scalar(
            bio::aging::mortality::phenotypic_age_levine(
                get_f(p, "albumin")?,
                get_f(p, "creatinine")?,
                get_f(p, "glucose")?,
                get_f(p, "crp")?,
                get_f(p, "lymphocyte_pct")?,
                get_f(p, "mcv")?,
                get_f(p, "rdw")?,
                get_f(p, "alp")?,
                get_f(p, "wbc")?,
                get_f(p, "chronological_age")?,
            ),
        )),
        "horvath_clock" => Ok(RunOutput::Scalar(bio::aging::mortality::horvath_clock(
            get_v(p, "cpg_betas")?,
            get_v(p, "coefficients")?,
            get_f(p, "intercept")?,
        ))),
        "cr_lifespan_extension" => Ok(RunOutput::Scalar(
            bio::aging::mortality::cr_lifespan_extension(
                get_f(p, "baseline_lifespan")?,
                get_f(p, "restriction_fraction")?,
                get_f(p, "max_extension")?,
            ),
        )),
        "reliability_theory_failure" => Ok(RunOutput::Scalar(
            bio::aging::mortality::reliability_theory_failure(
                get_u(p, "initial_elements")?,
                get_u(p, "redundancy")?,
                get_f(p, "failure_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "ros_production_rate" => Ok(RunOutput::Scalar(
            bio::aging::oxidative_stress::ros_production_rate(
                get_f(p, "metabolic_rate")?,
                get_f(p, "coupling_efficiency")?,
            ),
        )),
        "antioxidant_capacity" => Ok(RunOutput::Scalar(
            bio::aging::oxidative_stress::antioxidant_capacity(
                get_f(p, "sod")?,
                get_f(p, "catalase")?,
                get_f(p, "glutathione")?,
                get_f(p, "k_sod")?,
                get_f(p, "k_cat")?,
                get_f(p, "k_gsh")?,
            ),
        )),
        "oxidative_damage_rate" => Ok(RunOutput::Scalar(
            bio::aging::oxidative_stress::oxidative_damage_rate(
                get_f(p, "ros_level")?,
                get_f(p, "antioxidant")?,
            ),
        )),
        "lipid_peroxidation" => Ok(RunOutput::Scalar(
            bio::aging::oxidative_stress::lipid_peroxidation(
                get_f(p, "pufa_concentration")?,
                get_f(p, "ros_level")?,
                get_f(p, "k_initiation")?,
                get_f(p, "k_propagation")?,
            ),
        )),
        "protein_carbonylation" => Ok(RunOutput::Scalar(
            bio::aging::oxidative_stress::protein_carbonylation(
                get_f(p, "protein_conc")?,
                get_f(p, "ros_level")?,
                get_f(p, "rate_constant")?,
            ),
        )),
        "dna_8oxog_formation" => Ok(RunOutput::Scalar(
            bio::aging::oxidative_stress::dna_8oxog_formation(
                get_f(p, "ros_level")?,
                get_f(p, "guanine_sites")?,
                get_f(p, "k_oxidation")?,
            ),
        )),
        "mitochondrial_ros_vicious_cycle" => Ok(RunOutput::Scalar(
            bio::aging::oxidative_stress::mitochondrial_ros_vicious_cycle(
                get_f(p, "damage")?,
                get_f(p, "ros_base")?,
                get_f(p, "amplification")?,
                get_f(p, "repair_rate")?,
                get_f(p, "dt")?,
            ),
        )),
        "glutathione_ratio" => Ok(RunOutput::Scalar(
            bio::aging::oxidative_stress::glutathione_ratio(get_f(p, "gsh")?, get_f(p, "gssg")?),
        )),
        "fenton_reaction_rate" => Ok(RunOutput::Scalar(
            bio::aging::oxidative_stress::fenton_reaction_rate(
                get_f(p, "fe2")?,
                get_f(p, "h2o2")?,
                get_f(p, "k_fenton")?,
            ),
        )),
        "nrf2_response" => Ok(RunOutput::Scalar(
            bio::aging::oxidative_stress::nrf2_response(
                get_f(p, "ros_level")?,
                get_f(p, "keap1")?,
                get_f(p, "k_activation")?,
            ),
        )),
        "carbonyl_stress" => Ok(RunOutput::Scalar(
            bio::aging::oxidative_stress::carbonyl_stress(
                get_f(p, "methylglyoxal")?,
                get_f(p, "glyoxalase")?,
                get_f(p, "km")?,
            ),
        )),
        "aging_oxidative_stress_index" => Ok(RunOutput::Scalar(
            bio::aging::oxidative_stress::oxidative_stress_index(
                get_f(p, "total_oxidant")?,
                get_f(p, "total_antioxidant")?,
            ),
        )),
        "gompertz_mortality" => Ok(RunOutput::Scalar(
            bio::aging::senescence::gompertz_mortality(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "age")?,
            ),
        )),
        "aging_gompertz_survival" => Ok(RunOutput::Scalar(
            bio::aging::senescence::gompertz_survival(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "age")?,
            ),
        )),
        "weibull_mortality" => Ok(RunOutput::Scalar(
            bio::aging::senescence::weibull_mortality(
                get_f(p, "lambda")?,
                get_f(p, "k")?,
                get_f(p, "age")?,
            ),
        )),
        "aging_weibull_survival" => {
            Ok(RunOutput::Scalar(bio::aging::senescence::weibull_survival(
                get_f(p, "lambda")?,
                get_f(p, "k")?,
                get_f(p, "age")?,
            )))
        }
        "gompertz_makeham_mortality" => Ok(RunOutput::Scalar(
            bio::aging::senescence::gompertz_makeham_mortality(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "c")?,
                get_f(p, "age")?,
            ),
        )),
        "mortality_rate_doubling_time" => Ok(RunOutput::Scalar(
            bio::aging::senescence::mortality_rate_doubling_time(get_f(p, "b")?),
        )),
        "siler_mortality" => Ok(RunOutput::Scalar(bio::aging::senescence::siler_mortality(
            get_f(p, "a1")?,
            get_f(p, "b1")?,
            get_f(p, "a2")?,
            get_f(p, "a3")?,
            get_f(p, "b3")?,
            get_f(p, "age")?,
        ))),
        "logistic_mortality_plateau" => Ok(RunOutput::Scalar(
            bio::aging::senescence::logistic_mortality_plateau(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "c")?,
                get_f(p, "age")?,
            ),
        )),
        "demographic_entropy" => Ok(RunOutput::Scalar(
            bio::aging::senescence::demographic_entropy(get_v(p, "life_table_lx")?),
        )),
        "net_reproduction_rate" => Ok(RunOutput::Scalar(
            bio::aging::senescence::net_reproduction_rate(
                get_v(p, "survivorship")?,
                get_v(p, "fecundity")?,
            ),
        )),
        "aging_generation_time" => Ok(RunOutput::Scalar(bio::aging::senescence::generation_time(
            get_v(p, "survivorship")?,
            get_v(p, "fecundity")?,
        ))),
        "actuarial_senescence_rate" => Ok(RunOutput::Scalar(
            bio::aging::senescence::actuarial_senescence_rate(
                get_f(p, "mortality_young")?,
                get_f(p, "mortality_old")?,
                get_f(p, "age_interval")?,
            ),
        )),
        "proportional_hazards" => Ok(RunOutput::Scalar(
            bio::aging::senescence::proportional_hazards(
                get_f(p, "baseline_hazard")?,
                get_v(p, "covariates")?,
                get_v(p, "coefficients")?,
            ),
        )),
        "biological_age_levine" => Ok(RunOutput::Scalar(
            bio::aging::senescence::biological_age_levine(
                get_f(p, "chronological_age")?,
                get_f(p, "albumin")?,
                get_f(p, "creatinine")?,
                get_f(p, "glucose")?,
                get_f(p, "crp_ln")?,
                get_f(p, "lymphocyte_pct")?,
                get_f(p, "mcv")?,
                get_f(p, "rdw")?,
                get_f(p, "alkaline_phosphatase")?,
                get_f(p, "wbc")?,
            ),
        )),
        "frailty_index" => {
            let deficits_present = get_i(p, "deficits_present")? as u32;
            let deficits_measured = get_i(p, "deficits_measured")? as u32;
            Ok(RunOutput::Scalar(bio::aging::senescence::frailty_index(
                deficits_present,
                deficits_measured,
            )))
        }
        "disability_free_life_expectancy" => Ok(RunOutput::Scalar(
            bio::aging::senescence::disability_free_life_expectancy(
                get_v(p, "survival")?,
                get_v(p, "disability_free")?,
            ),
        )),
        "telomere_attrition" => Ok(RunOutput::Scalar(
            bio::aging::telomeres::telomere_attrition(
                get_f(p, "initial_length")?,
                get_f(p, "loss_per_division")?,
                get_u(p, "divisions")?,
            ),
        )),
        "telomerase_elongation" => Ok(RunOutput::Scalar(
            bio::aging::telomeres::telomerase_elongation(
                get_f(p, "current_length")?,
                get_f(p, "rate")?,
                get_f(p, "telomerase_activity")?,
            ),
        )),
        "replicative_limit" => Ok(RunOutput::Scalar(bio::aging::telomeres::replicative_limit(
            get_f(p, "initial_length")?,
            get_f(p, "loss_per_division")?,
            get_f(p, "critical_length")?,
        ))),
        "telomere_length_distribution" => Ok(RunOutput::Vector(
            bio::aging::telomeres::telomere_length_distribution(
                get_f(p, "mean")?,
                get_f(p, "std_dev")?,
                get_u(p, "n_chromosomes")?,
            ),
        )),
        "critical_shortening_probability" => Ok(RunOutput::Scalar(
            bio::aging::telomeres::critical_shortening_probability(
                get_f(p, "mean_length")?,
                get_f(p, "std_dev")?,
                get_f(p, "critical")?,
            ),
        )),
        "shelterin_protection" => Ok(RunOutput::Scalar(
            bio::aging::telomeres::shelterin_protection(
                get_f(p, "telomere_length")?,
                get_f(p, "shelterin_kd")?,
            ),
        )),
        "end_replication_problem" => Ok(RunOutput::Scalar(
            bio::aging::telomeres::end_replication_problem(
                get_f(p, "lagging_strand_loss")?,
                get_u(p, "divisions")?,
            ),
        )),
        "alt_pathway_elongation" => Ok(RunOutput::Scalar(
            bio::aging::telomeres::alt_pathway_elongation(
                get_f(p, "recombination_rate")?,
                get_f(p, "donor_length")?,
                get_f(p, "recipient_length")?,
            ),
        )),
        "telomere_age_regression" => Ok(RunOutput::Scalar(
            bio::aging::telomeres::telomere_age_regression(
                get_f(p, "age")?,
                get_f(p, "birth_length")?,
                get_f(p, "annual_loss")?,
            ),
        )),
        "gompertz_survival" => Ok(RunOutput::Scalar(
            bio::aging::senescence::gompertz_survival(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "age")?,
            ),
        )),
        "oxidative_stress_index" => Ok(RunOutput::Scalar(
            bio::aging::oxidative_stress::oxidative_stress_index(
                get_f(p, "total_oxidant")?,
                get_f(p, "total_antioxidant")?,
            ),
        )),
        "weibull_survival" => Ok(RunOutput::Scalar(bio::aging::senescence::weibull_survival(
            get_f(p, "lambda")?,
            get_f(p, "k")?,
            get_f(p, "age")?,
        ))),
        "life_expectancy" => {
            let v = get_v(p, "times")?;
            let s = get_v(p, "survivals")?;
            let curve: Vec<(f64, f64)> = v.iter().zip(s.iter()).map(|(&t, &sv)| (t, sv)).collect();
            Ok(RunOutput::Scalar(bio::aging::mortality::life_expectancy(
                &curve,
            )))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
