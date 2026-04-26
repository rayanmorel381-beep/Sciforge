//! Dispatch handler for virology functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "target_cell_model" => {
            let (a, b, c) = bio::virology::dynamics::target_cell_model(
                get_f(p, "v")?,
                get_f(p, "x")?,
                get_f(p, "y")?,
                get_f(p, "beta")?,
                get_f(p, "delta")?,
                get_f(p, "p")?,
                get_f(p, "c")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "basic_reproduction_number_viral" => Ok(RunOutput::Scalar(
            bio::virology::dynamics::basic_reproduction_number_viral(
                get_f(p, "beta")?,
                get_f(p, "p")?,
                get_f(p, "x0")?,
                get_f(p, "delta")?,
                get_f(p, "c")?,
            ),
        )),
        "viral_fitness" => Ok(RunOutput::Scalar(bio::virology::dynamics::viral_fitness(
            get_f(p, "replication_rate")?,
            get_f(p, "clearance_rate")?,
        ))),
        "burst_size" => Ok(RunOutput::Scalar(bio::virology::dynamics::burst_size(
            get_f(p, "p")?,
            get_f(p, "delta")?,
        ))),
        "viral_load_decay" => Ok(RunOutput::Scalar(
            bio::virology::dynamics::viral_load_decay(
                get_f(p, "v0")?,
                get_f(p, "clearance")?,
                get_f(p, "t")?,
            ),
        )),
        "eclipse_phase_model" => {
            let r = bio::virology::dynamics::eclipse_phase_model(
                get_f(p, "v")?,
                get_f(p, "x")?,
                get_f(p, "y1")?,
                get_f(p, "y2")?,
                get_f(p, "beta")?,
                get_f(p, "k")?,
                get_f(p, "delta")?,
                get_f(p, "p")?,
                get_f(p, "c")?,
            );
            Ok(RunOutput::Vector(vec![r.0, r.1, r.2, r.3]))
        }
        "effective_reproduction_number" => Ok(RunOutput::Scalar(
            bio::virology::dynamics::effective_reproduction_number(
                get_f(p, "r0")?,
                get_f(p, "susceptible_fraction")?,
            ),
        )),
        "viral_load_biphasic_decay" => Ok(RunOutput::Scalar(
            bio::virology::dynamics::viral_load_biphasic_decay(
                get_f(p, "v0")?,
                get_f(p, "f_fast")?,
                get_f(p, "delta1")?,
                get_f(p, "delta2")?,
                get_f(p, "t")?,
            ),
        )),
        "within_host_r0" => Ok(RunOutput::Scalar(bio::virology::dynamics::within_host_r0(
            get_f(p, "beta")?,
            get_f(p, "s0")?,
            get_f(p, "p")?,
            get_f(p, "delta")?,
            get_f(p, "c")?,
        ))),
        "latently_infected_dynamics" => {
            let r = bio::virology::dynamics::latently_infected_dynamics(
                get_f(p, "x")?,
                get_f(p, "y_l")?,
                get_f(p, "y_a")?,
                get_f(p, "v")?,
                get_f(p, "beta")?,
                get_f(p, "alpha")?,
                get_f(p, "delta_l")?,
                get_f(p, "delta_a")?,
                get_f(p, "p")?,
                get_f(p, "c")?,
                get_f(p, "f_latent")?,
            );
            Ok(RunOutput::Vector(vec![r.0, r.1, r.2, r.3]))
        }
        "immune_response_ctl" => {
            let (a, b) = bio::virology::dynamics::immune_response_ctl(
                get_f(p, "y")?,
                get_f(p, "z")?,
                get_f(p, "k_kill")?,
                get_f(p, "k_expand")?,
                get_f(p, "d_z")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "antibody_neutralization" => Ok(RunOutput::Scalar(
            bio::virology::dynamics::antibody_neutralization(
                get_f(p, "v")?,
                get_f(p, "ab")?,
                get_f(p, "k_neut")?,
            ),
        )),
        "viral_replication_logistic" => Ok(RunOutput::Scalar(
            bio::virology::dynamics::viral_replication_logistic(
                get_f(p, "v")?,
                get_f(p, "r")?,
                get_f(p, "k_capacity")?,
            ),
        )),
        "cell_to_cell_transmission" => Ok(RunOutput::Scalar(
            bio::virology::dynamics::cell_to_cell_transmission(
                get_f(p, "y")?,
                get_f(p, "x")?,
                get_f(p, "gamma")?,
            ),
        )),
        "defective_interfering_particles" => {
            let (a, b) = bio::virology::dynamics::defective_interfering_particles(
                get_f(p, "v")?,
                get_f(p, "d")?,
                get_f(p, "beta_v")?,
                get_f(p, "beta_d")?,
                get_f(p, "p_v")?,
                get_f(p, "p_d")?,
                get_f(p, "c")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "superinfection_exclusion" => {
            let (a, b) = bio::virology::dynamics::superinfection_exclusion(
                get_f(p, "v1")?,
                get_f(p, "v2")?,
                get_f(p, "beta1")?,
                get_f(p, "beta2")?,
                get_f(p, "exclusion_factor")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "time_to_peak_viremia" => Ok(RunOutput::Scalar(
            bio::virology::dynamics::time_to_peak_viremia(
                get_f(p, "beta")?,
                get_f(p, "x0")?,
                get_f(p, "p")?,
                get_f(p, "delta")?,
                get_f(p, "c")?,
            ),
        )),
        "generation_time_viral" => Ok(RunOutput::Scalar(
            bio::virology::dynamics::generation_time_viral(
                get_f(p, "eclipse")?,
                get_f(p, "infectious_lifespan")?,
            ),
        )),
        "sir_step" => {
            let (a, b, c) = bio::virology::epidemiology::sir_step(
                get_f(p, "s")?,
                get_f(p, "i")?,
                get_f(p, "r")?,
                get_f(p, "beta")?,
                get_f(p, "gamma")?,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "viro_seir_step" => {
            let r = bio::virology::epidemiology::seir_step(
                get_f(p, "s")?,
                get_f(p, "e")?,
                get_f(p, "i")?,
                get_f(p, "r")?,
                get_f(p, "beta")?,
                get_f(p, "sigma")?,
                get_f(p, "gamma")?,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Vector(vec![r.0, r.1, r.2, r.3]))
        }
        "viro_basic_reproduction_number" => Ok(RunOutput::Scalar(
            bio::virology::epidemiology::basic_reproduction_number(
                get_f(p, "beta")?,
                get_f(p, "gamma")?,
            ),
        )),
        "viro_herd_immunity_threshold" => Ok(RunOutput::Scalar(
            bio::virology::epidemiology::herd_immunity_threshold(get_f(p, "r0")?),
        )),
        "serial_interval_mean" => Ok(RunOutput::Scalar(
            bio::virology::epidemiology::serial_interval_mean(
                get_f(p, "incubation_mean")?,
                get_f(p, "infectious_offset")?,
            ),
        )),
        "rt_effective" => Ok(RunOutput::Scalar(
            bio::virology::epidemiology::rt_effective(
                get_f(p, "r0")?,
                get_f(p, "fraction_susceptible")?,
            ),
        )),
        "epidemic_growth_rate" => Ok(RunOutput::Scalar(
            bio::virology::epidemiology::epidemic_growth_rate(
                get_f(p, "r0")?,
                get_f(p, "generation_time")?,
            ),
        )),
        "viro_doubling_time" => Ok(RunOutput::Scalar(
            bio::virology::epidemiology::doubling_time(get_f(p, "growth_rate")?),
        )),
        "attack_rate" => Ok(RunOutput::Scalar(bio::virology::epidemiology::attack_rate(
            get_f(p, "r0")?,
        ))),
        "viro_case_fatality_rate" => Ok(RunOutput::Scalar(
            bio::virology::epidemiology::case_fatality_rate(
                get_f(p, "deaths")?,
                get_f(p, "cases")?,
            ),
        )),
        "viral_load_dynamics" => Ok(RunOutput::Scalar(
            bio::virology::epidemiology::viral_load_dynamics(
                get_f(p, "v0")?,
                get_f(p, "growth_rate")?,
                get_f(p, "clearance_rate")?,
                get_f(p, "time")?,
            ),
        )),
        "mutation_rate_per_replication" => Ok(RunOutput::Scalar(
            bio::virology::evolution::mutation_rate_per_replication(
                get_f(p, "mutations_observed")?,
                get_f(p, "genome_length")?,
                get_f(p, "replications")?,
            ),
        )),
        "quasispecies_fitness" => Ok(RunOutput::Scalar(
            bio::virology::evolution::quasispecies_fitness(
                get_f(p, "master_fitness")?,
                get_f(p, "mutation_rate")?,
                get_f(p, "genome_length")?,
            ),
        )),
        "error_threshold" => Ok(RunOutput::Scalar(
            bio::virology::evolution::error_threshold(
                get_f(p, "genome_length")?,
                get_f(p, "superiority")?,
            ),
        )),
        "antigenic_distance" => Ok(RunOutput::Scalar(
            bio::virology::evolution::antigenic_distance(
                get_v(p, "epitope_a")?,
                get_v(p, "epitope_b")?,
            ),
        )),
        "recombination_probability" => Ok(RunOutput::Scalar(
            bio::virology::evolution::recombination_probability(
                get_f(p, "co_infection_rate")?,
                get_f(p, "recombination_rate")?,
            ),
        )),
        "viral_diversity_shannon" => Ok(RunOutput::Scalar(
            bio::virology::evolution::viral_diversity_shannon(get_v(p, "frequencies")?),
        )),
        "selection_coefficient_viral" => Ok(RunOutput::Scalar(
            bio::virology::evolution::selection_coefficient_viral(
                get_f(p, "fitness_mutant")?,
                get_f(p, "fitness_wildtype")?,
            ),
        )),
        "molecular_clock_substitutions" => Ok(RunOutput::Scalar(
            bio::virology::evolution::molecular_clock_substitutions(
                get_f(p, "rate")?,
                get_f(p, "time")?,
            ),
        )),
        "coalescent_time_two_lineages" => Ok(RunOutput::Scalar(
            bio::virology::evolution::coalescent_time_two_lineages(get_f(p, "ne")?),
        )),
        "expected_pairwise_differences" => Ok(RunOutput::Scalar(
            bio::virology::evolution::expected_pairwise_differences(get_f(p, "theta")?),
        )),
        "reassortment_probability" => {
            let segments = get_i(p, "segments")? as u32;
            Ok(RunOutput::Scalar(
                bio::virology::evolution::reassortment_probability(
                    segments,
                    get_f(p, "co_infection_rate")?,
                ),
            ))
        }
        "immune_escape_rate" => Ok(RunOutput::Scalar(
            bio::virology::evolution::immune_escape_rate(
                get_f(p, "mutation_rate")?,
                get_f(p, "epitopes")?,
                get_f(p, "fitness_cost")?,
            ),
        )),
        "antigenic_cartography_distance" => Ok(RunOutput::Scalar(
            bio::virology::evolution::antigenic_cartography_distance(
                get_f(p, "titer_ref")?,
                get_f(p, "titer_cross")?,
            ),
        )),
        "phylogenetic_diversity" => Ok(RunOutput::Scalar(
            bio::virology::evolution::phylogenetic_diversity(get_v(p, "branch_lengths")?),
        )),
        "viro_dn_ds_ratio" => Ok(RunOutput::Scalar(bio::virology::evolution::dn_ds_ratio(
            get_f(p, "nonsynonymous")?,
            get_f(p, "synonymous")?,
            get_f(p, "sites_n")?,
            get_f(p, "sites_s")?,
        ))),
        "bottleneck_drift" => Ok(RunOutput::Scalar(
            bio::virology::evolution::bottleneck_drift(
                get_f(p, "allele_freq")?,
                get_f(p, "bottleneck_size")?,
            ),
        )),
        "lethal_mutagenesis_threshold" => Ok(RunOutput::Scalar(
            bio::virology::evolution::lethal_mutagenesis_threshold(
                get_f(p, "replication_fidelity")?,
                get_f(p, "genome_length")?,
            ),
        )),
        "fitness_landscape_epistasis" => Ok(RunOutput::Scalar(
            bio::virology::evolution::fitness_landscape_epistasis(
                get_f(p, "w_ab")?,
                get_f(p, "w_a")?,
                get_f(p, "w_b")?,
                get_f(p, "w_wt")?,
            ),
        )),
        "zoonotic_spillover_rate" => Ok(RunOutput::Scalar(
            bio::virology::evolution::zoonotic_spillover_rate(
                get_f(p, "contact_rate")?,
                get_f(p, "cross_species_infectivity")?,
                get_f(p, "prevalence")?,
            ),
        )),
        "antigenic_drift_rate" => Ok(RunOutput::Scalar(
            bio::virology::evolution::antigenic_drift_rate(
                get_f(p, "substitution_rate")?,
                get_f(p, "epitope_fraction")?,
            ),
        )),
        "capsid_triangulation_subunits" => {
            let t_number = get_i(p, "t_number")? as u32;
            Ok(RunOutput::Integer(
                bio::virology::structure::capsid_triangulation_subunits(t_number) as i64,
            ))
        }
        "capsid_radius_from_subunits" => Ok(RunOutput::Scalar(
            bio::virology::structure::capsid_radius_from_subunits(
                get_f(p, "n_subunits")?,
                get_f(p, "subunit_area")?,
            ),
        )),
        "genome_packaging_density" => Ok(RunOutput::Scalar(
            bio::virology::structure::genome_packaging_density(
                get_f(p, "genome_length_bp")?,
                get_f(p, "capsid_volume_nm3")?,
            ),
        )),
        "viral_burst_size" => Ok(RunOutput::Scalar(
            bio::virology::structure::viral_burst_size(
                get_f(p, "total_virions")?,
                get_f(p, "infected_cells")?,
            ),
        )),
        "viral_decay_rate" => Ok(RunOutput::Scalar(
            bio::virology::structure::viral_decay_rate(
                get_f(p, "initial_titer")?,
                get_f(p, "final_titer")?,
                get_f(p, "time")?,
            ),
        )),
        "mutation_rate_per_site" => Ok(RunOutput::Scalar(
            bio::virology::structure::mutation_rate_per_site(
                get_f(p, "mutations")?,
                get_f(p, "genome_length")?,
                get_f(p, "replications")?,
            ),
        )),
        "quasispecies_error_threshold" => Ok(RunOutput::Scalar(
            bio::virology::structure::quasispecies_error_threshold(
                get_f(p, "genome_length")?,
                get_f(p, "mu_per_site")?,
            ),
        )),
        "receptor_binding_affinity" => Ok(RunOutput::Scalar(
            bio::virology::structure::receptor_binding_affinity(
                get_f(p, "kon")?,
                get_f(p, "koff")?,
            ),
        )),
        "plaque_forming_units" => Ok(RunOutput::Scalar(
            bio::virology::structure::plaque_forming_units(
                get_f(p, "plaques")?,
                get_f(p, "dilution_factor")?,
                get_f(p, "volume_ml")?,
            ),
        )),
        "epitope_distance" => Ok(RunOutput::Scalar(
            bio::virology::structure::epitope_distance(
                get_f(p, "mismatches")?,
                get_f(p, "total_epitope_sites")?,
            ),
        )),

        "case_fatality_rate" => Ok(RunOutput::Scalar(
            bio::virology::epidemiology::case_fatality_rate(
                get_f(p, "deaths")?,
                get_f(p, "cases")?,
            ),
        )),
        "seir_step" => {
            let (s, e, i, r) = bio::virology::epidemiology::seir_step(
                get_f(p, "s")?,
                get_f(p, "e")?,
                get_f(p, "i")?,
                get_f(p, "r")?,
                get_f(p, "beta")?,
                get_f(p, "sigma")?,
                get_f(p, "gamma")?,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Vector(vec![s, e, i, r]))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
