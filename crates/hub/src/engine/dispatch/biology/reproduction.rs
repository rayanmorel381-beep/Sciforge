//! Dispatch handler for reproduction biology functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "cleavage_timing" => {
            let stage = get_i(p, "stage")? as u32;
            Ok(RunOutput::Scalar(
                bio::reproduction::embryogenesis::cleavage_timing(
                    stage,
                    get_f(p, "base_interval")?,
                    get_f(p, "temperature_factor")?,
                ),
            ))
        }
        "blastocyst_cell_count" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::blastocyst_cell_count(
                get_f(p, "initial_cells")?,
                get_f(p, "division_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "morphogen_gradient_embryo" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::morphogen_gradient_embryo(
                get_f(p, "source")?,
                get_f(p, "diffusion")?,
                get_f(p, "degradation")?,
                get_f(p, "x")?,
            ),
        )),
        "gastrulation_cell_migration" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::gastrulation_cell_migration(
                get_f(p, "chemotactic_sensitivity")?,
                get_f(p, "gradient")?,
                get_f(p, "random_motility")?,
            ),
        )),
        "reprod_somitogenesis_clock" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::somitogenesis_clock(
                get_f(p, "frequency")?,
                get_f(p, "wavefront_speed")?,
                get_f(p, "position")?,
                get_f(p, "t")?,
            ),
        )),
        "fetal_weight_hadlock" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::fetal_weight_hadlock(get_f(
                p,
                "gestational_age_weeks",
            )?),
        )),
        "placental_transfer_rate" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::placental_transfer_rate(
                get_f(p, "maternal_conc")?,
                get_f(p, "fetal_conc")?,
                get_f(p, "permeability")?,
                get_f(p, "surface_area")?,
            ),
        )),
        "crown_rump_length" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::crown_rump_length(get_f(p, "gestational_age_weeks")?),
        )),
        "biparietal_diameter" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::biparietal_diameter(get_f(
                p,
                "gestational_age_weeks",
            )?),
        )),
        "amniotic_fluid_index" => {
            let quadrants_v = get_v(p, "quadrants")?;
            let quadrants = [
                quadrants_v[0],
                quadrants_v[1],
                quadrants_v[2],
                quadrants_v[3],
            ];
            Ok(RunOutput::Scalar(
                bio::reproduction::embryogenesis::amniotic_fluid_index(&quadrants),
            ))
        }
        "neural_tube_closure_progress" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::neural_tube_closure_progress(
                get_f(p, "t")?,
                get_f(p, "rate")?,
                get_f(p, "max_closure")?,
            ),
        )),
        "organogenesis_differentiation_rate" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::organogenesis_differentiation_rate(
                get_f(p, "morphogen_conc")?,
                get_f(p, "threshold")?,
                get_f(p, "hill_coefficient")?,
            ),
        )),
        "turing_activator_inhibitor" => {
            let (a, b) = bio::reproduction::embryogenesis::turing_activator_inhibitor(
                get_f(p, "activator")?,
                get_f(p, "inhibitor")?,
                get_f(p, "rho_a")?,
                get_f(p, "rho_i")?,
                get_f(p, "mu_a")?,
                get_f(p, "mu_i")?,
                get_f(p, "kappa")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "fetal_lung_maturity_ls_ratio" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::fetal_lung_maturity_ls_ratio(
                get_f(p, "lecithin")?,
                get_f(p, "sphingomyelin")?,
            ),
        )),
        "apgar_component" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::apgar_component(
                get_f(p, "heart_rate")?,
                get_f(p, "respiration")?,
                get_f(p, "muscle_tone")?,
                get_f(p, "reflex")?,
                get_f(p, "color")?,
            ),
        )),
        "fetal_heart_rate_baseline" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::fetal_heart_rate_baseline(get_f(
                p,
                "gestational_age_weeks",
            )?),
        )),
        "umbilical_artery_pi" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::umbilical_artery_pi(
                get_f(p, "systolic")?,
                get_f(p, "diastolic")?,
                get_f(p, "mean")?,
            ),
        )),
        "placental_oxygen_delivery" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::placental_oxygen_delivery(
                get_f(p, "blood_flow")?,
                get_f(p, "hemoglobin")?,
                get_f(p, "saturation")?,
                get_f(p, "o2_binding_capacity")?,
            ),
        )),
        "trophoblast_invasion_depth" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::trophoblast_invasion_depth(
                get_f(p, "migration_rate")?,
                get_f(p, "protease_activity")?,
                get_f(p, "resistance")?,
                get_f(p, "t")?,
            ),
        )),
        "gestational_sac_diameter" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::gestational_sac_diameter(get_f(
                p,
                "gestational_age_days",
            )?),
        )),
        "yolk_sac_regression" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::yolk_sac_regression(
                get_f(p, "initial_size")?,
                get_f(p, "regression_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "limb_bud_outgrowth" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::limb_bud_outgrowth(
                get_f(p, "fgf_conc")?,
                get_f(p, "shh_conc")?,
                get_f(p, "growth_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "reprod_cell_fate_probability" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::cell_fate_probability(
                get_f(p, "signal_strength")?,
                get_f(p, "noise")?,
                get_f(p, "threshold")?,
            ),
        )),
        "ovarian_cycle_hormone" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::ovarian_cycle_hormone(
                get_f(p, "t")?,
                get_f(p, "amplitude")?,
                get_f(p, "peak_day")?,
                get_f(p, "width")?,
                get_f(p, "baseline")?,
            ),
        )),
        "follicle_growth" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::follicle_growth(
                get_f(p, "diameter")?,
                get_f(p, "fsh")?,
                get_f(p, "growth_rate")?,
                get_f(p, "max_diameter")?,
            ),
        )),
        "sperm_motility_fraction" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::sperm_motility_fraction(
                get_f(p, "velocity")?,
                get_f(p, "threshold")?,
                get_f(p, "concentration")?,
            ),
        )),
        "sperm_capacitation_rate" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::sperm_capacitation_rate(
                get_f(p, "t")?,
                get_f(p, "half_time")?,
            ),
        )),
        "fertilization_probability" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::fertilization_probability(
                get_f(p, "sperm_count")?,
                get_f(p, "half_max")?,
            ),
        )),
        "implantation_window" => Ok(RunOutput::Boolean(
            bio::reproduction::fertility::implantation_window(
                get_f(p, "progesterone")?,
                get_f(p, "threshold")?,
                get_f(p, "estrogen_ratio")?,
            ),
        )),
        "hcg_doubling" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::hcg_doubling(
                get_f(p, "initial")?,
                get_f(p, "doubling_time")?,
                get_f(p, "t")?,
            ),
        )),
        "lh_surge_model" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::lh_surge_model(
                get_f(p, "t")?,
                get_f(p, "t_peak")?,
                get_f(p, "amplitude")?,
                get_f(p, "rise_rate")?,
                get_f(p, "fall_rate")?,
            ),
        )),
        "estradiol_follicular" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::estradiol_follicular(
                get_f(p, "follicle_diameter")?,
                get_f(p, "num_follicles")?,
                get_f(p, "sensitivity")?,
            ),
        )),
        "progesterone_luteal" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::progesterone_luteal(
                get_f(p, "t_post_ovulation")?,
                get_f(p, "peak")?,
                get_f(p, "rise_rate")?,
                get_f(p, "fall_rate")?,
            ),
        )),
        "oocyte_quality_age" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::oocyte_quality_age(
                get_f(p, "base_quality")?,
                get_f(p, "age")?,
                get_f(p, "decline_start")?,
                get_f(p, "decline_rate")?,
            ),
        )),
        "antral_follicle_count" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::antral_follicle_count(
                get_f(p, "age")?,
                get_f(p, "initial_pool")?,
                get_f(p, "depletion_rate")?,
            ),
        )),
        "anti_mullerian_hormone" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::anti_mullerian_hormone(
                get_f(p, "follicle_count")?,
                get_f(p, "sensitivity")?,
            ),
        )),
        "ivf_success_rate" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::ivf_success_rate(
                get_f(p, "age")?,
                get_f(p, "embryo_quality")?,
                get_f(p, "endometrial_thickness")?,
            ),
        )),
        "menstrual_cycle_length" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::menstrual_cycle_length(
                get_f(p, "lh_peak_day")?,
                get_f(p, "luteal_phase_length")?,
            ),
        )),
        "sperm_concentration_fertility" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::sperm_concentration_fertility(
                get_f(p, "concentration")?,
                get_f(p, "motility")?,
                get_f(p, "morphology")?,
            ),
        )),
        "cumulative_pregnancy_rate" => {
            let months = get_i(p, "months")? as u32;
            Ok(RunOutput::Scalar(
                bio::reproduction::fertility::cumulative_pregnancy_rate(
                    get_f(p, "monthly_fecundability")?,
                    months,
                ),
            ))
        }
        "zona_pellucida_binding" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::zona_pellucida_binding(
                get_f(p, "receptors")?,
                get_f(p, "sperm_conc")?,
                get_f(p, "kd")?,
            ),
        )),
        "acrosome_reaction_rate" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::acrosome_reaction_rate(
                get_f(p, "capacitated_fraction")?,
                get_f(p, "zona_signal")?,
                get_f(p, "k")?,
            ),
        )),
        "endometrial_receptivity" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::endometrial_receptivity(
                get_f(p, "p4")?,
                get_f(p, "lif")?,
                get_f(p, "integrin")?,
                get_f(p, "threshold_p4")?,
            ),
        )),
        "twin_probability_dizygotic" => Ok(RunOutput::Scalar(
            bio::reproduction::fertility::twin_probability_dizygotic(
                get_f(p, "age")?,
                get_f(p, "fsh_level")?,
            ),
        )),
        "menstrual_cycle_hormone" => Ok(RunOutput::Scalar(
            bio::reproduction::hormonal_cycles::menstrual_cycle_hormone(
                get_f(p, "day")?,
                get_str(p, "hormone")?,
            ),
        )),
        "ovulation_probability" => Ok(RunOutput::Scalar(
            bio::reproduction::hormonal_cycles::ovulation_probability(
                get_f(p, "lh_surge")?,
                get_f(p, "follicle_maturity")?,
                get_f(p, "threshold")?,
            ),
        )),
        "endometrial_thickness" => Ok(RunOutput::Scalar(
            bio::reproduction::hormonal_cycles::endometrial_thickness(
                get_f(p, "day")?,
                get_f(p, "estrogen")?,
            ),
        )),
        "fertility_window" => Ok(RunOutput::Scalar(
            bio::reproduction::hormonal_cycles::fertility_window(
                get_f(p, "cycle_day")?,
                get_f(p, "cycle_length")?,
            ),
        )),
        "hcg_doubling_time" => Ok(RunOutput::Scalar(
            bio::reproduction::hormonal_cycles::hcg_doubling_time(
                get_f(p, "initial_hcg")?,
                get_f(p, "days")?,
                get_f(p, "doubling_time")?,
            ),
        )),
        "implantation_probability" => Ok(RunOutput::Scalar(
            bio::reproduction::hormonal_cycles::implantation_probability(
                get_f(p, "embryo_quality")?,
                get_f(p, "endometrial_receptivity")?,
                get_f(p, "age_factor")?,
            ),
        )),
        "spermatogenesis_duration_days" => Ok(RunOutput::Scalar(
            bio::reproduction::hormonal_cycles::spermatogenesis_duration_days(),
        )),
        "sperm_motility_score" => Ok(RunOutput::Scalar(
            bio::reproduction::hormonal_cycles::sperm_motility_score(
                get_f(p, "progressive")?,
                get_f(p, "non_progressive")?,
                get_f(p, "immotile")?,
            ),
        )),
        "testosterone_circadian" => Ok(RunOutput::Scalar(
            bio::reproduction::hormonal_cycles::testosterone_circadian(
                get_f(p, "hour")?,
                get_f(p, "peak_level")?,
                get_f(p, "trough_level")?,
            ),
        )),
        "ivf_cycle_success_rate" => Ok(RunOutput::Scalar(
            bio::reproduction::ivf::ivf_cycle_success_rate(
                get_f(p, "age")?,
                get_f(p, "embryo_quality")?,
                get_f(p, "endometrial_thickness")?,
            ),
        )),
        "ovarian_reserve_amh" => Ok(RunOutput::Text(
            bio::reproduction::ivf::ovarian_reserve_amh(get_f(p, "amh_ng_ml")?).to_string(),
        )),
        "antral_follicle_response" => Ok(RunOutput::Scalar(
            bio::reproduction::ivf::antral_follicle_response(
                get_f(p, "fsh_dose")?,
                get_f(p, "sensitivity")?,
                get_f(p, "max_follicles")?,
            ),
        )),
        "ohss_risk" => Ok(RunOutput::Scalar(bio::reproduction::ivf::ohss_risk(
            get_f(p, "estradiol")?,
            get_u(p, "follicle_count")?,
            get_f(p, "bmi")?,
        ))),
        "embryo_grading_score" => Ok(RunOutput::Scalar(
            bio::reproduction::ivf::embryo_grading_score(
                get_u(p, "cell_count")?,
                get_f(p, "fragmentation_pct")?,
                get_f(p, "symmetry")?,
            ),
        )),
        "blastocyst_expansion_rate" => Ok(RunOutput::Scalar(
            bio::reproduction::ivf::blastocyst_expansion_rate(get_f(
                p,
                "hours_post_fertilization",
            )?),
        )),
        "cryopreservation_survival" => Ok(RunOutput::Scalar(
            bio::reproduction::ivf::cryopreservation_survival(
                get_f(p, "cooling_rate")?,
                get_f(p, "optimal_rate")?,
                get_f(p, "cpa_conc")?,
            ),
        )),
        "cumulative_ivf_pregnancy_rate" => Ok(RunOutput::Scalar(
            bio::reproduction::ivf::cumulative_ivf_pregnancy_rate(
                get_f(p, "cycle_rate")?,
                get_u(p, "cycles")?,
            ),
        )),
        "sperm_dna_fragmentation_impact" => Ok(RunOutput::Scalar(
            bio::reproduction::ivf::sperm_dna_fragmentation_impact(
                get_f(p, "dfi")?,
                get_f(p, "baseline_fertility")?,
            ),
        )),
        "cell_fate_probability" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::cell_fate_probability(
                get_f(p, "signal_strength")?,
                get_f(p, "noise")?,
                get_f(p, "threshold")?,
            ),
        )),
        "somitogenesis_clock" => Ok(RunOutput::Scalar(
            bio::reproduction::embryogenesis::somitogenesis_clock(
                get_f(p, "frequency")?,
                get_f(p, "wavefront_speed")?,
                get_f(p, "position")?,
                get_f(p, "t")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
