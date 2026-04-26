//! Dispatch handler for microbiology functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "micro_biofilm_formation_rate" => Ok(RunOutput::Scalar(
            bio::microbiology::biofilm::biofilm_formation_rate(
                get_f(p, "planktonic")?,
                get_f(p, "attachment_rate")?,
                get_f(p, "surface_area")?,
                get_f(p, "detachment_rate")?,
                get_f(p, "biofilm")?,
            ),
        )),
        "biofilm_thickness" => Ok(RunOutput::Scalar(
            bio::microbiology::biofilm::biofilm_thickness(
                get_f(p, "growth_rate")?,
                get_f(p, "nutrient")?,
                get_f(p, "ks")?,
                get_f(p, "detachment")?,
                get_f(p, "t")?,
            ),
        )),
        "micro_extracellular_matrix_production" => Ok(RunOutput::Scalar(
            bio::microbiology::biofilm::extracellular_matrix_production(
                get_f(p, "cell_density")?,
                get_f(p, "signal")?,
                get_f(p, "max_rate")?,
                get_f(p, "threshold")?,
            ),
        )),
        "biofilm_diffusion_limitation" => Ok(RunOutput::Scalar(
            bio::microbiology::biofilm::biofilm_diffusion_limitation(
                get_f(p, "bulk_conc")?,
                get_f(p, "thickness")?,
                get_f(p, "diffusion_biofilm")?,
                get_f(p, "consumption_rate")?,
            ),
        )),
        "persister_fraction" => Ok(RunOutput::Scalar(
            bio::microbiology::biofilm::persister_fraction(
                get_f(p, "antibiotic_conc")?,
                get_f(p, "mic")?,
                get_f(p, "base_fraction")?,
                get_f(p, "max_fraction")?,
            ),
        )),
        "antibiotic_resistance_mutation_rate" => Ok(RunOutput::Scalar(
            bio::microbiology::biofilm::antibiotic_resistance_mutation_rate(
                get_f(p, "population")?,
                get_f(p, "mutation_rate")?,
                get_f(p, "selective_advantage")?,
            ),
        )),
        "minimum_inhibitory_concentration_ratio" => Ok(RunOutput::Scalar(
            bio::microbiology::biofilm::minimum_inhibitory_concentration_ratio(
                get_f(p, "mic_resistant")?,
                get_f(p, "mic_susceptible")?,
            ),
        )),
        "horizontal_gene_transfer" => Ok(RunOutput::Scalar(
            bio::microbiology::biofilm::horizontal_gene_transfer(
                get_f(p, "donor")?,
                get_f(p, "recipient")?,
                get_f(p, "conjugation_rate")?,
            ),
        )),
        "competence_transformation" => Ok(RunOutput::Scalar(
            bio::microbiology::biofilm::competence_transformation(
                get_f(p, "dna_conc")?,
                get_f(p, "competent_cells")?,
                get_f(p, "uptake_rate")?,
                get_f(p, "integration_efficiency")?,
            ),
        )),
        "phage_therapy_lysis" => {
            let (a, b) = bio::microbiology::biofilm::phage_therapy_lysis(
                get_f(p, "phage")?,
                get_f(p, "bacteria")?,
                get_f(p, "adsorption_rate")?,
                get_f(p, "burst_size")?,
                get_f(p, "latent_period")?,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "chemostat_steady_state_biomass" => Ok(RunOutput::Scalar(
            bio::microbiology::culture::chemostat_steady_state_biomass(
                get_f(p, "y")?,
                get_f(p, "s_in")?,
                get_f(p, "ks")?,
                get_f(p, "mu_max")?,
                get_f(p, "d")?,
            ),
        )),
        "chemostat_washout_dilution" => Ok(RunOutput::Scalar(
            bio::microbiology::culture::chemostat_washout_dilution(
                get_f(p, "mu_max")?,
                get_f(p, "s_in")?,
                get_f(p, "ks")?,
            ),
        )),
        "minimum_inhibitory_concentration" => Ok(RunOutput::Scalar(
            bio::microbiology::culture::minimum_inhibitory_concentration(
                get_f(p, "e0")?,
                get_f(p, "emax")?,
                get_f(p, "ec50")?,
                get_f(p, "n")?,
                get_f(p, "target_kill")?,
            ),
        )),
        "competitive_exclusion" => {
            let (a, b, c) = bio::microbiology::culture::competitive_exclusion(
                get_f(p, "x1")?,
                get_f(p, "x2")?,
                get_f(p, "s")?,
                get_f(p, "mu1")?,
                get_f(p, "mu2")?,
                get_f(p, "ks1")?,
                get_f(p, "ks2")?,
                get_f(p, "y1")?,
                get_f(p, "y2")?,
                get_f(p, "d")?,
                get_f(p, "s_in")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "serial_dilution" => Ok(RunOutput::Vector(
            bio::microbiology::culture::serial_dilution(
                get_f(p, "n0")?,
                get_f(p, "dilution_factor")?,
                get_u(p, "transfers")?,
                get_f(p, "growth_per_cycle")?,
            ),
        )),
        "biofilm_formation" => {
            let (a, b) = bio::microbiology::culture::biofilm_formation(
                get_f(p, "planktonic")?,
                get_f(p, "attachment_rate")?,
                get_f(p, "detachment_rate")?,
                get_f(p, "biofilm")?,
                get_f(p, "carrying_capacity")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "quorum_sensing" => Ok(RunOutput::Scalar(
            bio::microbiology::culture::quorum_sensing(
                get_f(p, "cell_density")?,
                get_f(p, "autoinducer_production")?,
                get_f(p, "threshold")?,
                get_f(p, "n_hill")?,
            ),
        )),
        "colony_forming_units" => Ok(RunOutput::Scalar(
            bio::microbiology::culture::colony_forming_units(
                get_f(p, "od600")?,
                get_f(p, "calibration_factor")?,
            ),
        )),
        "turbidostat" => Ok(RunOutput::Scalar(bio::microbiology::culture::turbidostat(
            get_f(p, "biomass")?,
            get_f(p, "target_od")?,
            get_f(p, "mu")?,
            get_f(p, "dt")?,
        ))),
        "ph_growth_response" => Ok(RunOutput::Scalar(
            bio::microbiology::culture::ph_growth_response(
                get_f(p, "ph")?,
                get_f(p, "ph_opt")?,
                get_f(p, "ph_min")?,
                get_f(p, "ph_max")?,
            ),
        )),
        "monod_growth" => Ok(RunOutput::Scalar(bio::microbiology::growth::monod_growth(
            get_f(p, "mu_max")?,
            get_f(p, "s")?,
            get_f(p, "ks")?,
        ))),
        "monod_dynamics" => {
            let (a, b) = bio::microbiology::growth::monod_dynamics(
                get_f(p, "x")?,
                get_f(p, "s")?,
                get_f(p, "mu_max")?,
                get_f(p, "ks")?,
                get_f(p, "y")?,
                get_f(p, "d")?,
                get_f(p, "s_in")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "monod_simulate" => Ok(RunOutput::PairVec(
            bio::microbiology::growth::monod_simulate(
                get_f(p, "x0")?,
                get_f(p, "s0")?,
                get_f(p, "mu_max")?,
                get_f(p, "ks")?,
                get_f(p, "y")?,
                get_f(p, "d")?,
                get_f(p, "s_in")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            ),
        )),
        "bacterial_growth_phases" => Ok(RunOutput::Scalar(
            bio::microbiology::growth::bacterial_growth_phases(
                get_f(p, "n0")?,
                get_f(p, "mu")?,
                get_f(p, "k")?,
                get_f(p, "lag")?,
                get_f(p, "t")?,
            ),
        )),
        "generation_time_bacteria" => Ok(RunOutput::Scalar(
            bio::microbiology::growth::generation_time_bacteria(get_f(p, "mu")?),
        )),
        "death_phase" => Ok(RunOutput::Scalar(bio::microbiology::growth::death_phase(
            get_f(p, "n_peak")?,
            get_f(p, "death_rate")?,
            get_f(p, "t")?,
        ))),
        "diauxic_growth" => Ok(RunOutput::Scalar(
            bio::microbiology::growth::diauxic_growth(
                get_f(p, "s1")?,
                get_f(p, "s2")?,
                get_f(p, "mu1")?,
                get_f(p, "mu2")?,
                get_f(p, "ks1")?,
                get_f(p, "ks2")?,
                get_f(p, "ki")?,
            ),
        )),
        "fermentation_yield" => Ok(RunOutput::Scalar(
            bio::microbiology::metabolism::fermentation_yield(
                get_f(p, "substrate")?,
                get_f(p, "yield_coefficient")?,
            ),
        )),
        "micro_anaerobic_atp_yield" => Ok(RunOutput::Scalar(
            bio::microbiology::metabolism::anaerobic_atp_yield(
                get_f(p, "glucose_moles")?,
                get_f(p, "pathway_efficiency")?,
            ),
        )),
        "chemolithoautotrophy_energy" => Ok(RunOutput::Scalar(
            bio::microbiology::metabolism::chemolithoautotrophy_energy(
                get_f(p, "delta_g")?,
                get_f(p, "moles_substrate")?,
                get_f(p, "efficiency")?,
            ),
        )),
        "nitrogen_fixation_cost" => Ok(RunOutput::Scalar(
            bio::microbiology::metabolism::nitrogen_fixation_cost(
                get_f(p, "n2_fixed")?,
                get_f(p, "atp_per_n2")?,
            ),
        )),
        "denitrification_rate" => Ok(RunOutput::Scalar(
            bio::microbiology::metabolism::denitrification_rate(
                get_f(p, "no3")?,
                get_f(p, "carbon_source")?,
                get_f(p, "max_rate")?,
                get_f(p, "ks_no3")?,
                get_f(p, "ks_c")?,
            ),
        )),
        "sulfate_reduction_rate" => Ok(RunOutput::Scalar(
            bio::microbiology::metabolism::sulfate_reduction_rate(
                get_f(p, "sulfate")?,
                get_f(p, "electron_donor")?,
                get_f(p, "max_rate")?,
                get_f(p, "ks_so4")?,
                get_f(p, "ks_donor")?,
            ),
        )),
        "methanogenesis_rate" => Ok(RunOutput::Scalar(
            bio::microbiology::metabolism::methanogenesis_rate(
                get_f(p, "co2")?,
                get_f(p, "h2")?,
                get_f(p, "max_rate")?,
                get_f(p, "ks_co2")?,
                get_f(p, "ks_h2")?,
                get_f(p, "temperature")?,
            ),
        )),
        "anammox_rate" => Ok(RunOutput::Scalar(
            bio::microbiology::metabolism::anammox_rate(
                get_f(p, "nh4")?,
                get_f(p, "no2")?,
                get_f(p, "max_rate")?,
                get_f(p, "ks_nh4")?,
                get_f(p, "ks_no2")?,
            ),
        )),
        "iron_oxidation_rate" => Ok(RunOutput::Scalar(
            bio::microbiology::metabolism::iron_oxidation_rate(
                get_f(p, "fe2")?,
                get_f(p, "o2")?,
                get_f(p, "ph")?,
                get_f(p, "max_rate")?,
            ),
        )),
        "bioremediation_degradation" => Ok(RunOutput::Scalar(
            bio::microbiology::metabolism::bioremediation_degradation(
                get_f(p, "contaminant")?,
                get_f(p, "biomass")?,
                get_f(p, "max_rate")?,
                get_f(p, "ks")?,
                get_f(p, "inhibition_conc")?,
            ),
        )),
        "quorum_sensing_ahl" => Ok(RunOutput::Scalar(
            bio::microbiology::quorum::quorum_sensing_ahl(
                get_f(p, "n")?,
                get_f(p, "k_prod")?,
                get_f(p, "k_deg")?,
                get_f(p, "diffusion")?,
            ),
        )),
        "quorum_activation" => Ok(RunOutput::Scalar(
            bio::microbiology::quorum::quorum_activation(
                get_f(p, "ahl")?,
                get_f(p, "threshold")?,
                get_f(p, "n")?,
            ),
        )),
        "quorum_bistable" => Ok(RunOutput::Vector(
            bio::microbiology::quorum::quorum_bistable(
                get_f(p, "ahl0")?,
                get_f(p, "n_cells")?,
                get_f(p, "k_prod")?,
                get_f(p, "k_deg")?,
                get_f(p, "k_auto")?,
                get_f(p, "threshold")?,
                get_f(p, "hill_n")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            ),
        )),
        "biofilm_growth" => Ok(RunOutput::Scalar(
            bio::microbiology::quorum::biofilm_growth(
                get_f(p, "b")?,
                get_f(p, "mu")?,
                get_f(p, "k_attach")?,
                get_f(p, "planktonic")?,
                get_f(p, "k_detach")?,
                get_f(p, "k_max")?,
            ),
        )),
        "biofilm_simulate" => Ok(RunOutput::PairVec(
            bio::microbiology::quorum::biofilm_simulate(
                get_f(p, "b0")?,
                get_f(p, "planktonic0")?,
                get_f(p, "mu")?,
                get_f(p, "k_attach")?,
                get_f(p, "k_detach")?,
                get_f(p, "k_max")?,
                get_f(p, "growth_p")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            ),
        )),
        "antibiotic_kill" => Ok(RunOutput::Scalar(
            bio::microbiology::quorum::antibiotic_kill(
                get_f(p, "n")?,
                get_f(p, "mic")?,
                get_f(p, "conc")?,
                get_f(p, "k_max")?,
                get_f(p, "hill")?,
            ),
        )),
        "mutation_resistance_probability" => Ok(RunOutput::Scalar(
            bio::microbiology::quorum::mutation_resistance_probability(
                get_f(p, "mu")?,
                get_f(p, "n")?,
            ),
        )),
        "anaerobic_atp_yield" => Ok(RunOutput::Scalar(
            bio::microbiology::metabolism::anaerobic_atp_yield(
                get_f(p, "glucose_moles")?,
                get_f(p, "pathway_efficiency")?,
            ),
        )),
        "biofilm_formation_rate" => Ok(RunOutput::Scalar(
            bio::microbiology::biofilm::biofilm_formation_rate(
                get_f(p, "planktonic")?,
                get_f(p, "attachment_rate")?,
                get_f(p, "surface_area")?,
                get_f(p, "detachment_rate")?,
                get_f(p, "biofilm")?,
            ),
        )),
        "extracellular_matrix_production" => Ok(RunOutput::Scalar(
            bio::microbiology::biofilm::extracellular_matrix_production(
                get_f(p, "cell_density")?,
                get_f(p, "signal")?,
                get_f(p, "max_rate")?,
                get_f(p, "threshold")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
