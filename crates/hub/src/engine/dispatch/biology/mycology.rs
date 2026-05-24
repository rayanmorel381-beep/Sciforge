//! Dispatch handler for mycology functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "mycology_decomposition_rate" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::decomposition_rate(get_f(p, "k")?, get_f(p, "mass")?),
        )),
        "remaining_mass" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::remaining_mass(
                get_f(p, "initial")?,
                get_f(p, "k")?,
                get_f(p, "t")?,
            ),
        )),
        "lignocellulose_decay" => {
            let (a, b) = bio::mycology::decomposition::lignocellulose_decay(
                get_f(p, "cellulose")?,
                get_f(p, "lignin")?,
                get_f(p, "k_cellulose")?,
                get_f(p, "k_lignin")?,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "enzyme_mediated_decomposition" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::enzyme_mediated_decomposition(
                get_f(p, "substrate")?,
                get_f(p, "enzyme")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
            ),
        )),
        "mycorrhizal_nutrient_exchange" => {
            let (a, b) = bio::mycology::decomposition::mycorrhizal_nutrient_exchange(
                get_f(p, "plant_carbon")?,
                get_f(p, "fungal_phosphorus")?,
                get_f(p, "exchange_coeff")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "saprotrophic_efficiency" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::saprotrophic_efficiency(
                get_f(p, "carbon_assimilated")?,
                get_f(p, "carbon_consumed")?,
            ),
        )),
        "humus_formation_rate" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::humus_formation_rate(
                get_f(p, "recalcitrant_fraction")?,
                get_f(p, "decomposition_rate")?,
                get_f(p, "carbon_input")?,
            ),
        )),
        "mycology_nitrogen_mineralization" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::nitrogen_mineralization(
                get_f(p, "cn_ratio_substrate")?,
                get_f(p, "cn_ratio_microbe")?,
                get_f(p, "carbon_flow")?,
            ),
        )),
        "white_rot_lignin_degradation" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::white_rot_lignin_degradation(
                get_f(p, "lignin")?,
                get_f(p, "laccase_activity")?,
                get_f(p, "peroxidase_activity")?,
                get_f(p, "km")?,
            ),
        )),
        "brown_rot_cellulose_attack" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::brown_rot_cellulose_attack(
                get_f(p, "cellulose")?,
                get_f(p, "hydroxyl_radical")?,
                get_f(p, "rate_constant")?,
            ),
        )),
        "soft_rot_cavity_formation" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::soft_rot_cavity_formation(
                get_f(p, "cellulose")?,
                get_f(p, "moisture")?,
                get_f(p, "enzyme_activity")?,
                get_f(p, "threshold_moisture")?,
            ),
        )),
        "mycorrhizal_carbon_cost" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::mycorrhizal_carbon_cost(
                get_f(p, "plant_npp")?,
                get_f(p, "allocation_fraction")?,
            ),
        )),
        "ectomycorrhizal_hyphal_exploration" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::ectomycorrhizal_hyphal_exploration(
                get_f(p, "biomass")?,
                get_f(p, "soil_volume")?,
                get_f(p, "exploration_type_factor")?,
            ),
        )),
        "arbuscular_mycorrhizal_phosphorus_uptake" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::arbuscular_mycorrhizal_phosphorus_uptake(
                get_f(p, "hyphal_length")?,
                get_f(p, "soil_p")?,
                get_f(p, "uptake_rate")?,
                get_f(p, "km")?,
            ),
        )),
        "wood_decay_mass_loss" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::wood_decay_mass_loss(
                get_f(p, "initial_density")?,
                get_f(p, "fungal_activity")?,
                get_f(p, "moisture_factor")?,
                get_f(p, "temp_factor")?,
                get_f(p, "t")?,
            ),
        )),
        "litter_quality_index" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::litter_quality_index(
                get_f(p, "nitrogen_content")?,
                get_f(p, "lignin_content")?,
            ),
        )),
        "mycology_carbon_use_efficiency" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::carbon_use_efficiency(
                get_f(p, "co2_respired")?,
                get_f(p, "carbon_assimilated")?,
            ),
        )),
        "saprotrophic_decomposition" => Ok(RunOutput::Scalar(
            bio::mycology::ecology::saprotrophic_decomposition(
                get_f(p, "substrate_mass")?,
                get_f(p, "enzyme_activity")?,
                get_f(p, "moisture")?,
                get_f(p, "temperature")?,
                get_f(p, "optimal_temp")?,
            ),
        )),
        "wood_decay_rate" => Ok(RunOutput::Scalar(bio::mycology::ecology::wood_decay_rate(
            get_f(p, "lignin_fraction")?,
            get_f(p, "cellulose_fraction")?,
            get_f(p, "fungal_type_lignin_pref")?,
        ))),
        "fungal_succession_priority" => Ok(RunOutput::Scalar(
            bio::mycology::ecology::fungal_succession_priority(
                get_u(p, "colonization_order")?,
                get_f(p, "competitive_ability")?,
            ),
        )),
        "spore_germination_rate" => Ok(RunOutput::Scalar(
            bio::mycology::ecology::spore_germination_rate(
                get_f(p, "moisture")?,
                get_f(p, "temperature")?,
                get_f(p, "temp_min")?,
                get_f(p, "temp_max")?,
                get_f(p, "temp_opt")?,
            ),
        )),
        "fairy_ring_expansion" => Ok(RunOutput::Scalar(
            bio::mycology::ecology::fairy_ring_expansion(
                get_f(p, "ring_radius")?,
                get_f(p, "growth_rate")?,
                get_f(p, "nutrient_depletion")?,
            ),
        )),
        "ergosterol_biomass_estimate" => Ok(RunOutput::Scalar(
            bio::mycology::ecology::ergosterol_biomass_estimate(
                get_f(p, "ergosterol_conc")?,
                get_f(p, "conversion_factor")?,
            ),
        )),
        "fungal_carbon_mineralization" => Ok(RunOutput::Scalar(
            bio::mycology::ecology::fungal_carbon_mineralization(
                get_f(p, "biomass")?,
                get_f(p, "cue")?,
                get_f(p, "substrate_consumed")?,
            ),
        )),
        "mycelial_network_resilience" => Ok(RunOutput::Scalar(
            bio::mycology::ecology::mycelial_network_resilience(
                get_u(p, "connections")?,
                get_u(p, "nodes")?,
                get_f(p, "redundancy")?,
            ),
        )),
        "hyphal_growth_rate" => Ok(RunOutput::Scalar(
            bio::mycology::growth::hyphal_growth_rate(
                get_f(p, "tip_extension")?,
                get_f(p, "branching_rate")?,
                get_f(p, "tips")?,
            ),
        )),
        "colony_radial_growth" => Ok(RunOutput::Scalar(
            bio::mycology::growth::colony_radial_growth(
                get_f(p, "r0")?,
                get_f(p, "rate")?,
                get_f(p, "t")?,
            ),
        )),
        "spore_germination_probability" => Ok(RunOutput::Scalar(
            bio::mycology::growth::spore_germination_probability(
                get_f(p, "water_activity")?,
                get_f(p, "temperature")?,
                get_f(p, "aw_min")?,
                get_f(p, "t_min")?,
                get_f(p, "t_max")?,
            ),
        )),
        "mycelial_network_transport" => Ok(RunOutput::Scalar(
            bio::mycology::growth::mycelial_network_transport(
                get_f(p, "concentration_source")?,
                get_f(p, "concentration_sink")?,
                get_f(p, "conductance")?,
            ),
        )),
        "chitin_content" => Ok(RunOutput::Scalar(bio::mycology::growth::chitin_content(
            get_f(p, "dry_mass")?,
            get_f(p, "chitin_fraction")?,
        ))),
        "fungal_biomass_from_ergosterol" => Ok(RunOutput::Scalar(
            bio::mycology::growth::fungal_biomass_from_ergosterol(
                get_f(p, "ergosterol_ug")?,
                get_f(p, "conversion_factor")?,
            ),
        )),
        "substrate_colonization_speed" => Ok(RunOutput::Scalar(
            bio::mycology::growth::substrate_colonization_speed(
                get_f(p, "growth_rate")?,
                get_f(p, "nutrient_availability")?,
                get_f(p, "km")?,
            ),
        )),
        "fairy_ring_radius" => Ok(RunOutput::Scalar(bio::mycology::growth::fairy_ring_radius(
            get_f(p, "initial_radius")?,
            get_f(p, "expansion_rate")?,
            get_f(p, "t")?,
        ))),
        "spore_dispersal_distance" => Ok(RunOutput::Scalar(
            bio::mycology::growth::spore_dispersal_distance(
                get_f(p, "wind_speed")?,
                get_f(p, "release_height")?,
                get_f(p, "terminal_velocity")?,
            ),
        )),
        "yeast_budding_rate" => Ok(RunOutput::Scalar(
            bio::mycology::growth::yeast_budding_rate(
                get_f(p, "nutrient")?,
                get_f(p, "temperature")?,
                get_f(p, "optimal_temp")?,
                get_f(p, "temp_width")?,
            ),
        )),
        "mycelial_biomass_logistic" => Ok(RunOutput::Scalar(
            bio::mycology::growth::mycelial_biomass_logistic(
                get_f(p, "biomass")?,
                get_f(p, "max_biomass")?,
                get_f(p, "mu_max")?,
                get_f(p, "dt")?,
            ),
        )),
        "branching_frequency" => Ok(RunOutput::Scalar(
            bio::mycology::growth::branching_frequency(
                get_f(p, "hyphal_length")?,
                get_f(p, "branch_count")?,
            ),
        )),
        "hyphal_tip_vesicle_supply" => Ok(RunOutput::Scalar(
            bio::mycology::growth::hyphal_tip_vesicle_supply(
                get_f(p, "vesicle_production")?,
                get_f(p, "distance_to_tip")?,
                get_f(p, "diffusion")?,
            ),
        )),
        "conidiation_rate" => Ok(RunOutput::Scalar(bio::mycology::growth::conidiation_rate(
            get_f(p, "nutrient_depletion")?,
            get_f(p, "light_signal")?,
            get_f(p, "threshold")?,
        ))),
        "rhizomorph_transport_rate" => Ok(RunOutput::Scalar(
            bio::mycology::growth::rhizomorph_transport_rate(
                get_f(p, "pressure_gradient")?,
                get_f(p, "conductance")?,
                get_f(p, "cross_section")?,
            ),
        )),
        "lichenization_benefit" => Ok(RunOutput::Scalar(
            bio::mycology::growth::lichenization_benefit(
                get_f(p, "algal_photosynthate")?,
                get_f(p, "fungal_protection")?,
                get_f(p, "exchange_rate")?,
            ),
        )),
        "spore_survival_uv" => Ok(RunOutput::Scalar(bio::mycology::growth::spore_survival_uv(
            get_f(p, "initial_viability")?,
            get_f(p, "uv_dose")?,
            get_f(p, "sensitivity")?,
        ))),
        "monod_fungal_growth" => Ok(RunOutput::Scalar(
            bio::mycology::growth::monod_fungal_growth(
                get_f(p, "mu_max")?,
                get_f(p, "substrate")?,
                get_f(p, "ks")?,
            ),
        )),
        "mycology_biofilm_formation_rate" => Ok(RunOutput::Scalar(
            bio::mycology::growth::biofilm_formation_rate(
                get_f(p, "cell_density")?,
                get_f(p, "surface_affinity")?,
                get_f(p, "quorum_signal")?,
                get_f(p, "threshold")?,
            ),
        )),
        "mycorrhizal_nutrient_transfer" => Ok(RunOutput::Scalar(
            bio::mycology::symbiosis::mycorrhizal_nutrient_transfer(
                get_f(p, "root_surface_area")?,
                get_f(p, "hyphal_density")?,
                get_f(p, "nutrient_conc")?,
                get_f(p, "transfer_efficiency")?,
            ),
        )),
        "mycorrhizal_carbon_allocation" => Ok(RunOutput::Scalar(
            bio::mycology::symbiosis::mycorrhizal_carbon_allocation(
                get_f(p, "carbon_to_fungus")?,
                get_f(p, "total_photosynthate")?,
            ),
        )),
        "mycorrhizal_colonization" => Ok(RunOutput::Scalar(
            bio::mycology::symbiosis::mycorrhizal_colonization(
                get_f(p, "inoculum_potential")?,
                get_f(p, "root_growth_rate")?,
                get_f(p, "susceptibility")?,
                get_f(p, "time")?,
            ),
        )),
        "common_mycorrhizal_network_transfer" => Ok(RunOutput::Scalar(
            bio::mycology::symbiosis::common_mycorrhizal_network_transfer(
                get_f(p, "donor_surplus")?,
                get_f(p, "recipient_deficit")?,
                get_f(p, "network_conductance")?,
            ),
        )),
        "lichen_photobiont_contribution" => Ok(RunOutput::Scalar(
            bio::mycology::symbiosis::lichen_photobiont_contribution(
                get_f(p, "photobiont_biomass")?,
                get_f(p, "photosynthesis_rate")?,
                get_f(p, "transfer_fraction")?,
            ),
        )),
        "endophyte_benefit" => Ok(RunOutput::Scalar(
            bio::mycology::symbiosis::endophyte_benefit(
                get_f(p, "plant_growth_base")?,
                get_f(p, "endophyte_effect")?,
                get_f(p, "stress_level")?,
            ),
        )),
        "fungal_network_distance" => Ok(RunOutput::Scalar(
            bio::mycology::symbiosis::fungal_network_distance(
                get_f(p, "hyphal_growth_rate")?,
                get_f(p, "branching_angle")?,
                get_f(p, "time")?,
            ),
        )),
        "truffle_spore_dispersal" => Ok(RunOutput::Scalar(
            bio::mycology::symbiosis::truffle_spore_dispersal(
                get_f(p, "spore_count")?,
                get_f(p, "wind_speed")?,
                get_f(p, "animal_vectors")?,
                get_f(p, "decay_distance")?,
                get_f(p, "distance")?,
            ),
        )),
        "mycobiome_diversity_shannon" => Ok(RunOutput::Scalar(
            bio::mycology::symbiosis::mycobiome_diversity_shannon(get_v(p, "abundances")?),
        )),
        "carbon_use_efficiency" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::carbon_use_efficiency(
                get_f(p, "co2_respired")?,
                get_f(p, "carbon_assimilated")?,
            ),
        )),
        "decomposition_rate" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::decomposition_rate(get_f(p, "k")?, get_f(p, "mass")?),
        )),
        "nitrogen_mineralization" => Ok(RunOutput::Scalar(
            bio::mycology::decomposition::nitrogen_mineralization(
                get_f(p, "cn_ratio_substrate")?,
                get_f(p, "cn_ratio_microbe")?,
                get_f(p, "carbon_flow")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
