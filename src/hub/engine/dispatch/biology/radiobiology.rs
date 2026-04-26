//! Dispatch handler for radiobiology functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "dna_strand_break_probability" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::dna_strand_break_probability(
                get_f(p, "dose")?,
                get_f(p, "target_size")?,
                get_f(p, "repair_efficiency")?,
            ),
        )),
        "base_excision_repair_rate" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::base_excision_repair_rate(
                get_f(p, "damage_sites")?,
                get_f(p, "enzyme_concentration")?,
                get_f(p, "km")?,
            ),
        )),
        "misrepair_probability" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::misrepair_probability(
                get_f(p, "damage_density")?,
                get_f(p, "complexity_factor")?,
            ),
        )),
        "chromosome_aberration_yield" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::chromosome_aberration_yield(
                get_f(p, "dose")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
            ),
        )),
        "lethal_aberration_fraction" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::lethal_aberration_fraction(get_f(p, "aberrations")?),
        )),
        "mutation_frequency" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::mutation_frequency(
                get_f(p, "dose")?,
                get_f(p, "spontaneous_rate")?,
                get_f(p, "induced_rate_per_gy")?,
            ),
        )),
        "double_strand_break_yield" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::double_strand_break_yield(
                get_f(p, "dose")?,
                get_f(p, "let_factor")?,
            ),
        )),
        "nhej_repair_kinetics" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::nhej_repair_kinetics(
                get_f(p, "breaks")?,
                get_f(p, "fast_rate")?,
                get_f(p, "slow_rate")?,
                get_f(p, "fast_fraction")?,
                get_f(p, "t")?,
            ),
        )),
        "homologous_recombination_probability" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::homologous_recombination_probability(
                get_f(p, "cell_cycle_s_g2_fraction")?,
                get_b(p, "sister_chromatid_available")?,
            ),
        )),
        "clustered_damage_probability" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::clustered_damage_probability(
                get_f(p, "dose")?,
                get_f(p, "let_val")?,
                get_f(p, "target_radius")?,
            ),
        )),
        "single_strand_break_yield" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::single_strand_break_yield(get_f(p, "dose")?),
        )),
        "oxidative_base_damage_yield" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::oxidative_base_damage_yield(
                get_f(p, "dose")?,
                get_f(p, "oxygen_concentration")?,
            ),
        )),
        "dna_damage_complexity_score" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::dna_damage_complexity_score(
                get_f(p, "ssb")?,
                get_f(p, "dsb")?,
                get_f(p, "base_damage")?,
            ),
        )),
        "foci_formation_kinetics" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::foci_formation_kinetics(
                get_f(p, "dsb")?,
                get_f(p, "recruitment_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "foci_resolution_kinetics" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::foci_resolution_kinetics(
                get_f(p, "foci_max")?,
                get_f(p, "repair_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "micronucleus_formation" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::micronucleus_formation(
                get_f(p, "dose")?,
                get_f(p, "alpha_mn")?,
                get_f(p, "beta_mn")?,
            ),
        )),
        "comet_tail_moment" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::comet_tail_moment(
                get_f(p, "tail_length")?,
                get_f(p, "tail_dna_fraction")?,
            ),
        )),
        "gamma_h2ax_signal" => Ok(RunOutput::Scalar(
            bio::radiobiology::dna_damage::gamma_h2ax_signal(
                get_f(p, "dsb")?,
                get_f(p, "spreading_factor")?,
                get_f(p, "background")?,
            ),
        )),
        "repair_pathway_choice" => {
            let (a, b) = bio::radiobiology::dna_damage::repair_pathway_choice(
                get_f(p, "dsb")?,
                get_f(p, "cell_cycle_phase")?,
                get_f(p, "brca_status")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "radio_linear_quadratic_survival" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::linear_quadratic_survival(
                get_f(p, "dose")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
            ),
        )),
        "radio_biologically_effective_dose" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::biologically_effective_dose(
                get_f(p, "n")?,
                get_f(p, "d")?,
                get_f(p, "alpha_beta")?,
            ),
        )),
        "radio_equivalent_dose_2gy" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::equivalent_dose_2gy(
                get_f(p, "bed")?,
                get_f(p, "alpha_beta")?,
            ),
        )),
        "tcp" => Ok(RunOutput::Scalar(bio::radiobiology::dose_response::tcp(
            get_f(p, "n_cells")?,
            get_f(p, "survival_fraction")?,
        ))),
        "ntcp_lyman" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::ntcp_lyman(
                get_f(p, "dose")?,
                get_f(p, "td50")?,
                get_f(p, "m")?,
            ),
        )),
        "oxygen_enhancement_ratio" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::oxygen_enhancement_ratio(
                get_f(p, "dose_hypoxic")?,
                get_f(p, "dose_oxic")?,
            ),
        )),
        "dna_dsb_yield" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::dna_dsb_yield(
                get_f(p, "dose")?,
                get_f(p, "yield_per_gray")?,
            ),
        )),
        "repair_kinetics" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::repair_kinetics(
                get_f(p, "dsb0")?,
                get_f(p, "repair_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "fractionation_survival" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::fractionation_survival(
                get_u(p, "n_fractions")?,
                get_f(p, "dose_per_fraction")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "repair_factor")?,
            ),
        )),
        "relative_biological_effectiveness" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::relative_biological_effectiveness(
                get_f(p, "dose_ref")?,
                get_f(p, "dose_test")?,
            ),
        )),
        "let_to_rbe" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::let_to_rbe(
                get_f(p, "let_val")?,
                get_f(p, "rbe_max")?,
                get_f(p, "let_half")?,
            ),
        )),
        "protraction_factor" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::protraction_factor(
                get_f(p, "dose_rate")?,
                get_f(p, "repair_half_time")?,
                get_f(p, "total_time")?,
            ),
        )),
        "bystander_effect" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::bystander_effect(
                get_f(p, "dose")?,
                get_f(p, "max_effect")?,
                get_f(p, "dose_half")?,
            ),
        )),
        "adaptive_response" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::adaptive_response(
                get_f(p, "priming_dose")?,
                get_f(p, "challenge_dose")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "reduction_factor")?,
            ),
        )),
        "low_dose_hypersensitivity" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::low_dose_hypersensitivity(
                get_f(p, "dose")?,
                get_f(p, "alpha_r")?,
                get_f(p, "alpha_s")?,
                get_f(p, "dc")?,
                get_f(p, "beta")?,
            ),
        )),
        "tumor_growth_delay" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::tumor_growth_delay(
                get_f(p, "dose")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "doubling_time")?,
            ),
        )),
        "complication_free_cure" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::complication_free_cure(
                get_f(p, "tcp_val")?,
                get_f(p, "ntcp_val")?,
            ),
        )),
        "isoeffect_dose" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::isoeffect_dose(
                get_f(p, "n1")?,
                get_f(p, "d1")?,
                get_f(p, "alpha_beta")?,
                get_f(p, "n2")?,
            ),
        )),
        "bed_biologically_effective_dose" => Ok(RunOutput::Scalar(
            bio::radiobiology::fractionation::bed_biologically_effective_dose(
                get_f(p, "n")?,
                get_f(p, "d")?,
                get_f(p, "alpha_beta")?,
            ),
        )),
        "eqd2" => Ok(RunOutput::Scalar(bio::radiobiology::fractionation::eqd2(
            get_f(p, "n")?,
            get_f(p, "d")?,
            get_f(p, "alpha_beta")?,
        ))),
        "radio_tumor_control_probability" => Ok(RunOutput::Scalar(
            bio::radiobiology::fractionation::tumor_control_probability(
                get_f(p, "n_clonogens")?,
                get_f(p, "surviving_fraction")?,
            ),
        )),
        "radio_normal_tissue_complication_probability" => Ok(RunOutput::Scalar(
            bio::radiobiology::fractionation::normal_tissue_complication_probability(
                get_f(p, "dose")?,
                get_f(p, "td50")?,
                get_f(p, "gamma50")?,
            ),
        )),
        "incomplete_repair_factor" => Ok(RunOutput::Scalar(
            bio::radiobiology::fractionation::incomplete_repair_factor(
                get_f(p, "delta_t")?,
                get_f(p, "repair_half_time")?,
            ),
        )),
        "repopulation_dose_equivalent" => Ok(RunOutput::Scalar(
            bio::radiobiology::fractionation::repopulation_dose_equivalent(
                get_f(p, "doubling_time")?,
                get_f(p, "treatment_duration")?,
                get_f(p, "alpha")?,
            ),
        )),
        "lq_with_repopulation" => Ok(RunOutput::Scalar(
            bio::radiobiology::fractionation::lq_with_repopulation(
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "dose")?,
                get_f(p, "n_fractions")?,
                get_f(p, "treatment_days")?,
                get_f(p, "tp")?,
                get_f(p, "tk")?,
            ),
        )),
        "radio_therapeutic_ratio" => Ok(RunOutput::Scalar(
            bio::radiobiology::fractionation::therapeutic_ratio(
                get_f(p, "tcp")?,
                get_f(p, "ntcp")?,
            ),
        )),
        "fraction_size_optimization" => Ok(RunOutput::Scalar(
            bio::radiobiology::fractionation::fraction_size_optimization(
                get_f(p, "alpha_beta_tumor")?,
                get_f(p, "alpha_beta_normal")?,
            ),
        )),
        "hyperfractionation_advantage" => Ok(RunOutput::Scalar(
            bio::radiobiology::fractionation::hyperfractionation_advantage(
                get_f(p, "d_conventional")?,
                get_f(p, "d_hyper")?,
                get_f(p, "alpha_beta")?,
            ),
        )),
        "radiation_shielding_half_value" => Ok(RunOutput::Scalar(
            bio::radiobiology::shielding::radiation_shielding_half_value(
                get_f(p, "initial_intensity")?,
                get_f(p, "hvl")?,
                get_f(p, "thickness")?,
            ),
        )),
        "shielding_tenth_value" => Ok(RunOutput::Scalar(
            bio::radiobiology::shielding::shielding_tenth_value(
                get_f(p, "initial_intensity")?,
                get_f(p, "tvl")?,
                get_f(p, "thickness")?,
            ),
        )),
        "mass_attenuation" => Ok(RunOutput::Scalar(
            bio::radiobiology::shielding::mass_attenuation(
                get_f(p, "intensity")?,
                get_f(p, "mu_over_rho")?,
                get_f(p, "density")?,
                get_f(p, "thickness")?,
            ),
        )),
        "buildup_factor" => Ok(RunOutput::Scalar(
            bio::radiobiology::shielding::buildup_factor(
                get_f(p, "beam_layers")?,
                get_f(p, "mu")?,
                get_f(p, "thickness")?,
            ),
        )),
        "concrete_shielding_thickness" => Ok(RunOutput::Scalar(
            bio::radiobiology::shielding::concrete_shielding_thickness(
                get_f(p, "dose_rate")?,
                get_f(p, "dose_limit")?,
                get_f(p, "hvl")?,
            ),
        )),
        "lead_equivalent_thickness" => Ok(RunOutput::Scalar(
            bio::radiobiology::shielding::lead_equivalent_thickness(
                get_f(p, "mu_material")?,
                get_f(p, "mu_lead")?,
                get_f(p, "thickness_material")?,
            ),
        )),
        "inverse_square_distance" => Ok(RunOutput::Scalar(
            bio::radiobiology::shielding::inverse_square_distance(
                get_f(p, "dose_at_d1")?,
                get_f(p, "d1")?,
                get_f(p, "d2")?,
            ),
        )),
        "occupancy_factor_dose" => Ok(RunOutput::Scalar(
            bio::radiobiology::shielding::occupancy_factor_dose(
                get_f(p, "dose_unshielded")?,
                get_f(p, "occupancy")?,
                get_f(p, "use_factor")?,
            ),
        )),
        "neutron_shielding_hydrogen" => Ok(RunOutput::Scalar(
            bio::radiobiology::shielding::neutron_shielding_hydrogen(
                get_f(p, "thickness_cm")?,
                get_f(p, "cross_section")?,
                get_f(p, "density_h")?,
            ),
        )),
        "annual_dose_limit_check" => Ok(RunOutput::Scalar(
            bio::radiobiology::shielding::annual_dose_limit_check(
                get_f(p, "dose_received")?,
                get_f(p, "dose_limit")?,
            ),
        )),
        "biologically_effective_dose" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::biologically_effective_dose(
                get_f(p, "n")?,
                get_f(p, "d")?,
                get_f(p, "alpha_beta")?,
            ),
        )),
        "equivalent_dose_2gy" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::equivalent_dose_2gy(
                get_f(p, "bed")?,
                get_f(p, "alpha_beta")?,
            ),
        )),
        "linear_quadratic_survival" => Ok(RunOutput::Scalar(
            bio::radiobiology::dose_response::linear_quadratic_survival(
                get_f(p, "dose")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
            ),
        )),
        "normal_tissue_complication_probability" => Ok(RunOutput::Scalar(
            bio::radiobiology::fractionation::normal_tissue_complication_probability(
                get_f(p, "dose")?,
                get_f(p, "td50")?,
                get_f(p, "gamma50")?,
            ),
        )),
        "therapeutic_ratio" => Ok(RunOutput::Scalar(
            bio::radiobiology::fractionation::therapeutic_ratio(
                get_f(p, "tcp")?,
                get_f(p, "ntcp")?,
            ),
        )),
        "tumor_control_probability" => Ok(RunOutput::Scalar(
            bio::radiobiology::fractionation::tumor_control_probability(
                get_f(p, "n_clonogens")?,
                get_f(p, "surviving_fraction")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
