//! Dispatch handler for bioenergetics functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "atp_hydrolysis_free_energy" => Ok(RunOutput::Scalar(
            bio::bioenergetics::atp::atp_hydrolysis_free_energy(
                get_f(p, "delta_g0")?,
                get_f(p, "atp")?,
                get_f(p, "adp")?,
                get_f(p, "pi")?,
                get_f(p, "t")?,
            ),
        )),
        "p_o_ratio" => Ok(RunOutput::Scalar(bio::bioenergetics::atp::p_o_ratio(
            get_f(p, "atp_produced")?,
            get_f(p, "oxygen_consumed")?,
        ))),
        "respiratory_control_index" => Ok(RunOutput::Scalar(
            bio::bioenergetics::atp::respiratory_control_index(
                get_f(p, "state3_rate")?,
                get_f(p, "state4_rate")?,
            ),
        )),
        "uncoupling_heat" => Ok(RunOutput::Scalar(bio::bioenergetics::atp::uncoupling_heat(
            get_f(p, "pmf")?,
            get_f(p, "proton_leak")?,
        ))),
        "chemiosmotic_atp_rate" => Ok(RunOutput::Scalar(
            bio::bioenergetics::atp::chemiosmotic_atp_rate(
                get_f(p, "pmf")?,
                get_f(p, "atp_synthase_activity")?,
                get_f(p, "h_per_atp")?,
            ),
        )),
        "shuttle_efficiency_malate_aspartate" => Ok(RunOutput::Scalar(
            bio::bioenergetics::atp::shuttle_efficiency_malate_aspartate(
                get_f(p, "nadh_cytoplasmic")?,
                get_f(p, "transfer_rate")?,
            ),
        )),
        "shuttle_efficiency_glycerol_3p" => Ok(RunOutput::Scalar(
            bio::bioenergetics::atp::shuttle_efficiency_glycerol_3p(
                get_f(p, "nadh_cytoplasmic")?,
                get_f(p, "transfer_rate")?,
            ),
        )),
        "metabolic_water" => Ok(RunOutput::Scalar(bio::bioenergetics::atp::metabolic_water(
            get_f(p, "glucose_oxidized")?,
        ))),
        "adenylate_energy_charge" => Ok(RunOutput::Scalar(
            bio::bioenergetics::atp::adenylate_energy_charge(
                get_f(p, "atp")?,
                get_f(p, "adp")?,
                get_f(p, "amp")?,
            ),
        )),
        "phosphocreatine_equilibrium" => Ok(RunOutput::Scalar(
            bio::bioenergetics::atp::phosphocreatine_equilibrium(
                get_f(p, "creatine")?,
                get_f(p, "atp")?,
                get_f(p, "k_eq")?,
            ),
        )),
        "myosin_atpase_rate" => Ok(RunOutput::Scalar(
            bio::bioenergetics::atp::myosin_atpase_rate(
                get_f(p, "load_fraction")?,
                get_f(p, "vmax")?,
            ),
        )),
        "ionic_gradient_energy" => Ok(RunOutput::Scalar(
            bio::bioenergetics::atp::ionic_gradient_energy(
                get_f(p, "z")?,
                get_f(p, "vm")?,
                get_f(p, "c_out")?,
                get_f(p, "c_in")?,
                get_f(p, "t")?,
            ),
        )),
        "glycolysis_net_atp" => Ok(RunOutput::Scalar(
            bio::bioenergetics::metabolism::glycolysis_net_atp(get_f(p, "glucose")?),
        )),
        "glycolysis_pyruvate_yield" => Ok(RunOutput::Scalar(
            bio::bioenergetics::metabolism::glycolysis_pyruvate_yield(get_f(p, "glucose")?),
        )),
        "gluconeogenesis_cost" => Ok(RunOutput::Scalar(
            bio::bioenergetics::metabolism::gluconeogenesis_cost(get_f(p, "glucose")?),
        )),
        "pentose_phosphate_nadph" => Ok(RunOutput::Scalar(
            bio::bioenergetics::metabolism::pentose_phosphate_nadph(get_f(p, "glucose_6p")?),
        )),
        "fatty_acid_synthesis_cost" => Ok(RunOutput::Scalar(
            bio::bioenergetics::metabolism::fatty_acid_synthesis_cost(get_f(
                p,
                "acetyl_coa_units",
            )?),
        )),
        "urea_cycle_cost" => Ok(RunOutput::Scalar(
            bio::bioenergetics::metabolism::urea_cycle_cost(get_f(p, "amino_acids")?),
        )),
        "glycogen_storage_efficiency" => Ok(RunOutput::Scalar(
            bio::bioenergetics::metabolism::glycogen_storage_efficiency(get_f(p, "glucose_units")?),
        )),
        "warburg_effect" => Ok(RunOutput::Scalar(
            bio::bioenergetics::metabolism::warburg_effect(
                get_f(p, "aerobic_glycolysis_rate")?,
                get_f(p, "oxidative_rate")?,
            ),
        )),
        "ketogenesis_yield" => Ok(RunOutput::Scalar(
            bio::bioenergetics::metabolism::ketogenesis_yield(get_f(p, "acetyl_coa")?),
        )),
        "amino_acid_catabolism_atp" => Ok(RunOutput::Scalar(
            bio::bioenergetics::metabolism::amino_acid_catabolism_atp(
                get_u(p, "carbon_count")?,
                get_b(p, "is_glucogenic")?,
                get_b(p, "is_ketogenic")?,
            ),
        )),
        "cori_cycle_cost" => Ok(RunOutput::Scalar(
            bio::bioenergetics::metabolism::cori_cycle_cost(get_f(p, "lactate")?),
        )),
        "bioen_respiratory_quotient" => Ok(RunOutput::Scalar(
            bio::bioenergetics::metabolism::respiratory_quotient(
                get_f(p, "co2_produced")?,
                get_f(p, "o2_consumed")?,
            ),
        )),
        "metabolic_flux_control_coefficient" => Ok(RunOutput::Scalar(
            bio::bioenergetics::metabolism::metabolic_flux_control_coefficient(
                get_f(p, "v_enzyme")?,
                get_f(p, "v_pathway")?,
                get_f(p, "elasticity")?,
            ),
        )),
        "farquhar_model" => Ok(RunOutput::Scalar(
            bio::bioenergetics::photosynthesis::farquhar_model(
                get_f(p, "vcmax")?,
                get_f(p, "jmax")?,
                get_f(p, "ci")?,
                get_f(p, "gamma_star")?,
                get_f(p, "kc")?,
                get_f(p, "ko")?,
                get_f(p, "o")?,
                get_f(p, "rd")?,
                get_f(p, "par")?,
            ),
        )),
        "bioen_electron_transport_rate" => Ok(RunOutput::Scalar(
            bio::bioenergetics::photosynthesis::electron_transport_rate(
                get_f(p, "jmax")?,
                get_f(p, "par")?,
            ),
        )),
        "bioen_light_response_curve" => Ok(RunOutput::Scalar(
            bio::bioenergetics::photosynthesis::light_response_curve(
                get_f(p, "amax")?,
                get_f(p, "phi")?,
                get_f(p, "par")?,
                get_f(p, "rd")?,
            ),
        )),
        "light_compensation_point" => Ok(RunOutput::Scalar(
            bio::bioenergetics::photosynthesis::light_compensation_point(
                get_f(p, "amax")?,
                get_f(p, "phi")?,
                get_f(p, "rd")?,
            ),
        )),
        "bioen_water_use_efficiency" => Ok(RunOutput::Scalar(
            bio::bioenergetics::photosynthesis::water_use_efficiency(
                get_f(p, "assimilation")?,
                get_f(p, "transpiration")?,
            ),
        )),
        "bioen_rubisco_specificity" => Ok(RunOutput::Scalar(
            bio::bioenergetics::photosynthesis::rubisco_specificity(
                get_f(p, "vcmax")?,
                get_f(p, "kc")?,
                get_f(p, "vomax")?,
                get_f(p, "ko")?,
            ),
        )),
        "bioen_photorespiration_rate" => Ok(RunOutput::Scalar(
            bio::bioenergetics::photosynthesis::photorespiration_rate(
                get_f(p, "vomax")?,
                get_f(p, "o")?,
                get_f(p, "ko")?,
                get_f(p, "ci")?,
                get_f(p, "kc")?,
            ),
        )),
        "quantum_yield" => Ok(RunOutput::Scalar(
            bio::bioenergetics::photosynthesis::quantum_yield(
                get_f(p, "assimilation_rate")?,
                get_f(p, "photon_flux")?,
            ),
        )),
        "co2_compensation_point_photo" => Ok(RunOutput::Scalar(
            bio::bioenergetics::photosynthesis::co2_compensation_point_photo(
                get_f(p, "gamma_star")?,
                get_f(p, "rd")?,
                get_f(p, "vcmax")?,
                get_f(p, "kc")?,
                get_f(p, "ko")?,
                get_f(p, "o")?,
            ),
        )),
        "bioen_stomatal_conductance_ball_berry" => Ok(RunOutput::Scalar(
            bio::bioenergetics::photosynthesis::stomatal_conductance_ball_berry(
                get_f(p, "assimilation")?,
                get_f(p, "rh")?,
                get_f(p, "cs")?,
                get_f(p, "g0")?,
                get_f(p, "g1")?,
            ),
        )),
        "mesophyll_conductance_photo" => Ok(RunOutput::Scalar(
            bio::bioenergetics::photosynthesis::mesophyll_conductance_photo(
                get_f(p, "assimilation")?,
                get_f(p, "ci")?,
                get_f(p, "cc")?,
            ),
        )),
        "triose_phosphate_utilization" => Ok(RunOutput::Scalar(
            bio::bioenergetics::photosynthesis::triose_phosphate_utilization(
                get_f(p, "tpu")?,
                get_f(p, "ci")?,
                get_f(p, "gamma_star")?,
            ),
        )),
        "light_inhibition_photoinhibition" => Ok(RunOutput::Scalar(
            bio::bioenergetics::photosynthesis::light_inhibition_photoinhibition(
                get_f(p, "fv_fm_initial")?,
                get_f(p, "light_excess")?,
                get_f(p, "ki")?,
            ),
        )),
        "canopy_photosynthesis_sun_shade" => Ok(RunOutput::Scalar(
            bio::bioenergetics::photosynthesis::canopy_photosynthesis_sun_shade(
                get_f(p, "lai")?,
                get_f(p, "k_ext")?,
                get_f(p, "a_sun")?,
                get_f(p, "a_shade")?,
            ),
        )),
        "carbon_concentrating_mechanism_benefit" => Ok(RunOutput::Scalar(
            bio::bioenergetics::photosynthesis::carbon_concentrating_mechanism_benefit(
                get_f(p, "ci_c3")?,
                get_f(p, "ci_c4")?,
                get_f(p, "vcmax")?,
                get_f(p, "kc")?,
            ),
        )),
        "atp_free_energy" => Ok(RunOutput::Scalar(
            bio::bioenergetics::respiration::atp_free_energy(
                get_f(p, "delta_g0")?,
                get_f(p, "atp")?,
                get_f(p, "adp")?,
                get_f(p, "pi")?,
                get_f(p, "t")?,
            ),
        )),
        "atp_synthase_rate" => Ok(RunOutput::Scalar(
            bio::bioenergetics::respiration::atp_synthase_rate(
                get_f(p, "proton_gradient")?,
                get_f(p, "n_protons")?,
                get_f(p, "delta_g_atp")?,
                get_f(p, "t")?,
            ),
        )),
        "proton_motive_force" => Ok(RunOutput::Scalar(
            bio::bioenergetics::respiration::proton_motive_force(
                get_f(p, "delta_psi")?,
                get_f(p, "delta_ph")?,
                get_f(p, "t")?,
            ),
        )),
        "p_to_o_ratio" => Ok(RunOutput::Scalar(
            bio::bioenergetics::respiration::p_to_o_ratio(
                get_f(p, "atp_produced")?,
                get_f(p, "oxygen_consumed")?,
            ),
        )),
        "respiratory_control_ratio" => Ok(RunOutput::Scalar(
            bio::bioenergetics::respiration::respiratory_control_ratio(
                get_f(p, "state3")?,
                get_f(p, "state4")?,
            ),
        )),
        "membrane_potential_nernst" => Ok(RunOutput::Scalar(
            bio::bioenergetics::respiration::membrane_potential_nernst(
                get_f(p, "z")?,
                get_f(p, "c_out")?,
                get_f(p, "c_in")?,
                get_f(p, "t")?,
            ),
        )),
        "uncoupler_effect" => Ok(RunOutput::Scalar(
            bio::bioenergetics::respiration::uncoupler_effect(
                get_f(p, "pmf")?,
                get_f(p, "permeability")?,
                get_f(p, "concentration")?,
            ),
        )),
        "citric_acid_cycle_nadh_yield" => Ok(RunOutput::Scalar(
            bio::bioenergetics::respiration::citric_acid_cycle_nadh_yield(get_f(
                p,
                "acetyl_coa_flux",
            )?),
        )),
        "citric_acid_cycle_fadh2_yield" => Ok(RunOutput::Scalar(
            bio::bioenergetics::respiration::citric_acid_cycle_fadh2_yield(get_f(
                p,
                "acetyl_coa_flux",
            )?),
        )),
        "electron_transport_efficiency" => Ok(RunOutput::Scalar(
            bio::bioenergetics::respiration::electron_transport_efficiency(
                get_f(p, "n_electrons")?,
                get_f(p, "delta_e")?,
                get_f(p, "delta_g_atp")?,
                get_f(p, "n_atp")?,
            ),
        )),
        "substrate_level_phosphorylation" => Ok(RunOutput::Scalar(
            bio::bioenergetics::respiration::substrate_level_phosphorylation(
                get_f(p, "n_reactions")?,
                get_f(p, "delta_g_per_reaction")?,
            ),
        )),
        "bioen_anaerobic_atp_yield" => Ok(RunOutput::Scalar(
            bio::bioenergetics::respiration::anaerobic_atp_yield(get_f(p, "glucose_flux")?),
        )),
        "aerobic_atp_yield" => Ok(RunOutput::Scalar(
            bio::bioenergetics::respiration::aerobic_atp_yield(get_f(p, "glucose_flux")?),
        )),
        "lactate_production_rate" => Ok(RunOutput::Scalar(
            bio::bioenergetics::respiration::lactate_production_rate(
                get_f(p, "pyruvate_flux")?,
                get_f(p, "nad_ratio")?,
            ),
        )),
        "beta_oxidation_atp_yield" => Ok(RunOutput::Scalar(
            bio::bioenergetics::respiration::beta_oxidation_atp_yield(get_f(
                p,
                "carbon_chain_length",
            )?),
        )),
        "creatine_phosphate_buffer" => Ok(RunOutput::Scalar(
            bio::bioenergetics::respiration::creatine_phosphate_buffer(
                get_f(p, "atp")?,
                get_f(p, "adp")?,
                get_f(p, "cr_p")?,
                get_f(p, "keq")?,
            ),
        )),
        "gibbs_free_energy_reaction" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::gibbs_free_energy_reaction(
                get_f(p, "delta_h")?,
                get_f(p, "t")?,
                get_f(p, "delta_s")?,
            ),
        )),
        "equilibrium_constant" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::equilibrium_constant(
                get_f(p, "delta_g0")?,
                get_f(p, "t")?,
            ),
        )),
        "redox_potential" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::redox_potential(
                get_f(p, "e0")?,
                get_f(p, "n")?,
                get_f(p, "oxidized")?,
                get_f(p, "reduced")?,
                get_f(p, "t")?,
            ),
        )),
        "energy_charge" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::energy_charge(
                get_f(p, "atp")?,
                get_f(p, "adp")?,
                get_f(p, "amp")?,
            ),
        )),
        "metabolic_rate_kleiber" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::metabolic_rate_kleiber(get_f(p, "mass")?),
        )),
        "oxygen_consumption_rate" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::oxygen_consumption_rate(
                get_f(p, "metabolic_rate")?,
                get_f(p, "oxycaloric_equivalent")?,
            ),
        )),
        "coupling_efficiency" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::coupling_efficiency(
                get_f(p, "delta_g_atp")?,
                get_f(p, "delta_g_substrate")?,
                get_f(p, "n_atp")?,
            ),
        )),
        "heat_dissipation" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::heat_dissipation(
                get_f(p, "delta_g_reaction")?,
                get_f(p, "useful_work")?,
            ),
        )),
        "bioen_metabolic_rate_q10" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::metabolic_rate_q10(
                get_f(p, "rate_ref")?,
                get_f(p, "t")?,
                get_f(p, "t_ref")?,
                get_f(p, "q10")?,
            ),
        )),
        "arrhenius_metabolic" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::arrhenius_metabolic(
                get_f(p, "rate_ref")?,
                get_f(p, "ea")?,
                get_f(p, "t")?,
                get_f(p, "t_ref")?,
            ),
        )),
        "thermogenic_cost" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::thermogenic_cost(
                get_f(p, "delta_h")?,
                get_f(p, "efficiency")?,
            ),
        )),
        "proton_gradient_energy" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::proton_gradient_energy(
                get_f(p, "n_protons")?,
                get_f(p, "delta_mu")?,
            ),
        )),
        "nad_redox_potential" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::nad_redox_potential(
                get_f(p, "nad_ox")?,
                get_f(p, "nad_red")?,
                get_f(p, "e0")?,
                get_f(p, "t")?,
            ),
        )),
        "entropy_production_rate" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::entropy_production_rate(
                get_f(p, "heat_flux")?,
                get_f(p, "temperature")?,
            ),
        )),
        "exergy_content" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::exergy_content(
                get_f(p, "delta_h")?,
                get_f(p, "t0")?,
                get_f(p, "delta_s")?,
            ),
        )),
        "muscle_mechanical_efficiency" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::muscle_mechanical_efficiency(
                get_f(p, "work_output")?,
                get_f(p, "metabolic_input")?,
            ),
        )),
        "basal_metabolic_scaling" => Ok(RunOutput::Scalar(
            bio::bioenergetics::thermodynamics::basal_metabolic_scaling(
                get_f(p, "m0")?,
                get_f(p, "mass")?,
                get_f(p, "exponent")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
