//! Dispatch handler for biogeography functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "range_size_latitude" => Ok(RunOutput::Scalar(
            bio::biogeography::climate::range_size_latitude(get_f(p, "area")?),
        )),
        "elevational_diversity_gradient" => Ok(RunOutput::Scalar(
            bio::biogeography::climate::elevational_diversity_gradient(
                get_f(p, "elevation")?,
                get_f(p, "peak_elevation")?,
                get_f(p, "max_richness")?,
            ),
        )),
        "biome_niche_model" => Ok(RunOutput::Scalar(
            bio::biogeography::climate::biome_niche_model(
                get_f(p, "temperature")?,
                get_f(p, "precipitation")?,
                get_f(p, "t_opt")?,
                get_f(p, "p_opt")?,
                get_f(p, "t_width")?,
                get_f(p, "p_width")?,
            ),
        )),
        "regional_endemism_index" => Ok(RunOutput::Scalar(
            bio::biogeography::climate::regional_endemism_index(
                get_u(p, "endemic_species")?,
                get_u(p, "total_species")?,
            ),
        )),
        "latitudinal_diversity_gradient" => Ok(RunOutput::Scalar(
            bio::biogeography::climate::latitudinal_diversity_gradient(
                get_f(p, "latitude")?,
                get_f(p, "max_richness")?,
                get_f(p, "steepness")?,
            ),
        )),
        "range_shift_velocity" => Ok(RunOutput::Scalar(
            bio::biogeography::climate::range_shift_velocity(
                get_f(p, "temp_change_rate")?,
                get_f(p, "spatial_temp_gradient")?,
            ),
        )),
        "climate_envelope_suitability" => Ok(RunOutput::Scalar(
            bio::biogeography::climate::climate_envelope_suitability(
                get_f(p, "temp")?,
                get_f(p, "precip")?,
                get_f(p, "temp_min")?,
                get_f(p, "temp_max")?,
                get_f(p, "precip_min")?,
                get_f(p, "precip_max")?,
            ),
        )),
        "refugia_persistence" => Ok(RunOutput::Scalar(
            bio::biogeography::climate::refugia_persistence(
                get_f(p, "area")?,
                get_f(p, "min_viable_area")?,
                get_f(p, "climate_stability")?,
            ),
        )),
        "wallace_line_effect" => Ok(RunOutput::Scalar(
            bio::biogeography::climate::wallace_line_effect(
                get_f(p, "dispersal_ability")?,
                get_f(p, "barrier_width")?,
            ),
        )),
        "biogeo_isolation_by_distance" => Ok(RunOutput::Scalar(
            bio::biogeography::connectivity::isolation_by_distance(
                get_v(p, "genetic_dist")?,
                get_v(p, "geographic_dist")?,
            ),
        )),
        "connectivity_index" => Ok(RunOutput::Vector(
            bio::biogeography::connectivity::connectivity_index(
                get_v(p, "patch_areas")?,
                get_m(p, "distances")?,
                get_f(p, "alpha")?,
            ),
        )),
        "metapopulation_incidence" => Ok(RunOutput::Scalar(
            bio::biogeography::connectivity::metapopulation_incidence(
                get_f(p, "connectivity")?,
                get_f(p, "e")?,
                get_f(p, "colonization_coeff")?,
            ),
        )),
        "habitat_fragmentation_index" => Ok(RunOutput::Scalar(
            bio::biogeography::connectivity::habitat_fragmentation_index(
                get_f(p, "total_area")?,
                get_u(p, "n_patches")?,
                get_f(p, "perimeter_sum")?,
            ),
        )),
        "effective_mesh_size" => Ok(RunOutput::Scalar(
            bio::biogeography::connectivity::effective_mesh_size(
                get_v(p, "patch_areas")?,
                get_f(p, "total_area")?,
            ),
        )),
        "proximity_index" => Ok(RunOutput::Scalar(
            bio::biogeography::connectivity::proximity_index(
                get_f(p, "focal_area")?,
                get_v(p, "neighbor_areas")?,
                get_v(p, "distances")?,
            ),
        )),
        "biogeo_corridor_effectiveness" => Ok(RunOutput::Scalar(
            bio::biogeography::connectivity::corridor_effectiveness(
                get_f(p, "corridor_length")?,
                get_f(p, "corridor_width")?,
                get_f(p, "matrix_resistance")?,
            ),
        )),
        "graph_connectivity" => {
            let adjacency_m = get_m(p, "adjacency")?;
            let adjacency: Vec<Vec<bool>> = adjacency_m
                .iter()
                .map(|r| r.iter().map(|&x| x != 0.0).collect())
                .collect();
            Ok(RunOutput::Scalar(
                bio::biogeography::connectivity::graph_connectivity(&adjacency),
            ))
        }
        "biogeo_stepping_stone_migration" => Ok(RunOutput::Vector(
            bio::biogeography::connectivity::stepping_stone_migration(
                get_v(p, "populations")?,
                get_f(p, "migration_rate")?,
            ),
        )),
        "resistance_distance" => Ok(RunOutput::Scalar(
            bio::biogeography::connectivity::resistance_distance(
                get_m(p, "conductances")?,
                get_u(p, "node_a")?,
                get_u(p, "node_b")?,
            ),
        )),
        "patch_cohesion" => Ok(RunOutput::Scalar(
            bio::biogeography::connectivity::patch_cohesion(
                get_v(p, "patch_perimeters")?,
                get_v(p, "patch_areas")?,
                get_f(p, "total_cells")?,
            ),
        )),
        "circuitscape_effective_resistance" => Ok(RunOutput::Scalar(
            bio::biogeography::connectivity::circuitscape_effective_resistance(get_v(
                p,
                "node_conductances",
            )?),
        )),
        "dispersal_kernel_exponential" => Ok(RunOutput::Scalar(
            bio::biogeography::distribution::dispersal_kernel_exponential(
                get_f(p, "distance")?,
                get_f(p, "mean_dispersal")?,
            ),
        )),
        "dispersal_kernel_2dt" => Ok(RunOutput::Scalar(
            bio::biogeography::distribution::dispersal_kernel_2dt(
                get_f(p, "distance")?,
                get_f(p, "a")?,
                get_f(p, "p")?,
            ),
        )),
        "range_shift_rate" => Ok(RunOutput::Scalar(
            bio::biogeography::distribution::range_shift_rate(
                get_f(p, "warming_rate")?,
                get_f(p, "lapse_rate")?,
            ),
        )),
        "latitudinal_gradient" => Ok(RunOutput::Scalar(
            bio::biogeography::distribution::latitudinal_gradient(
                get_f(p, "species_tropical")?,
                get_f(p, "species_polar")?,
                get_f(p, "lat_range")?,
            ),
        )),
        "altitudinal_gradient" => Ok(RunOutput::Scalar(
            bio::biogeography::distribution::altitudinal_gradient(
                get_f(p, "species_low")?,
                get_f(p, "species_high")?,
                get_f(p, "alt_range")?,
            ),
        )),
        "bioclimatic_envelope" => Ok(RunOutput::Scalar(
            bio::biogeography::distribution::bioclimatic_envelope(
                get_f(p, "temp")?,
                get_f(p, "precip")?,
                get_f(p, "temp_min")?,
                get_f(p, "temp_max")?,
                get_f(p, "precip_min")?,
                get_f(p, "precip_max")?,
            ),
        )),
        "species_area_relationship" => Ok(RunOutput::Scalar(
            bio::biogeography::distribution::species_area_relationship(
                get_f(p, "c")?,
                get_f(p, "z")?,
                get_f(p, "area")?,
            ),
        )),
        "endemism_index" => Ok(RunOutput::Scalar(
            bio::biogeography::distribution::endemism_index(
                get_u(p, "endemic_species")?,
                get_u(p, "total_species")?,
            ),
        )),
        "climate_velocity" => Ok(RunOutput::Scalar(
            bio::biogeography::distribution::climate_velocity(
                get_f(p, "temp_change_rate")?,
                get_f(p, "spatial_gradient")?,
            ),
        )),
        "habitat_suitability_index" => Ok(RunOutput::Scalar(
            bio::biogeography::distribution::habitat_suitability_index(
                get_v(p, "variables")?,
                get_v(p, "optima")?,
                get_v(p, "tolerances")?,
            ),
        )),
        "island_equilibrium_richness" => Ok(RunOutput::Scalar(
            bio::biogeography::distribution::island_equilibrium_richness(
                get_f(p, "immigration_max")?,
                get_f(p, "extinction_max")?,
                get_f(p, "area")?,
                get_f(p, "distance")?,
            ),
        )),
        "nestedness_temperature" => {
            let presence_matrix_m = get_m(p, "presence_matrix")?;
            let presence_matrix: Vec<Vec<bool>> = presence_matrix_m
                .iter()
                .map(|r| r.iter().map(|&x| x != 0.0).collect())
                .collect();
            Ok(RunOutput::Scalar(
                bio::biogeography::distribution::nestedness_temperature(&presence_matrix),
            ))
        }
        "mid_domain_effect" => Ok(RunOutput::Scalar(
            bio::biogeography::distribution::mid_domain_effect(
                get_f(p, "domain_size")?,
                get_f(p, "range_size")?,
            ),
        )),
        "beta_diversity_whittaker" => Ok(RunOutput::Scalar(
            bio::biogeography::distribution::beta_diversity_whittaker(
                get_f(p, "gamma")?,
                get_f(p, "alpha_mean")?,
            ),
        )),
        "beta_diversity_sorensen" => Ok(RunOutput::Scalar(
            bio::biogeography::distribution::beta_diversity_sorensen(
                get_f(p, "shared")?,
                get_f(p, "unique_a")?,
                get_f(p, "unique_b")?,
            ),
        )),
        "rapoport_rule" => Ok(RunOutput::Scalar(
            bio::biogeography::distribution::rapoport_rule(
                get_v(p, "range_sizes")?,
                get_v(p, "latitudes")?,
            ),
        )),
        "occupancy_frequency" => {
            let presences_v = get_v(p, "presences")?;
            let presences: Vec<bool> = presences_v.iter().map(|&x| x != 0.0).collect();
            Ok(RunOutput::Scalar(
                bio::biogeography::distribution::occupancy_frequency(&presences),
            ))
        }
        "island_species_area" => Ok(RunOutput::Scalar(
            bio::biogeography::island::island_species_area(
                get_f(p, "c")?,
                get_f(p, "z")?,
                get_f(p, "area")?,
            ),
        )),
        "island_immigration_rate" => Ok(RunOutput::Scalar(
            bio::biogeography::island::island_immigration_rate(
                get_f(p, "s")?,
                get_f(p, "p")?,
                get_f(p, "i_max")?,
            ),
        )),
        "island_extinction_rate" => Ok(RunOutput::Scalar(
            bio::biogeography::island::island_extinction_rate(get_f(p, "s")?, get_f(p, "e_max")?),
        )),
        "macarthur_wilson_equilibrium" => Ok(RunOutput::Scalar(
            bio::biogeography::island::macarthur_wilson_equilibrium(
                get_f(p, "i_max")?,
                get_f(p, "e_max")?,
                get_f(p, "p")?,
            ),
        )),
        "macarthur_wilson_turnover" => Ok(RunOutput::Scalar(
            bio::biogeography::island::macarthur_wilson_turnover(
                get_f(p, "i_max")?,
                get_f(p, "e_max")?,
                get_f(p, "p")?,
            ),
        )),
        "distance_decay" => Ok(RunOutput::Scalar(
            bio::biogeography::island::distance_decay(
                get_f(p, "similarity_0")?,
                get_f(p, "decay_rate")?,
                get_f(p, "distance")?,
            ),
        )),
        "rescue_effect" => Ok(RunOutput::Scalar(bio::biogeography::island::rescue_effect(
            get_f(p, "extinction_base")?,
            get_f(p, "immigration")?,
        ))),
        "target_effect" => Ok(RunOutput::Scalar(bio::biogeography::island::target_effect(
            get_f(p, "immigration_base")?,
            get_f(p, "area")?,
            get_f(p, "area_ref")?,
        ))),
        "species_isolation_index" => Ok(RunOutput::Scalar(
            bio::biogeography::island::species_isolation_index(get_v(p, "distances")?),
        )),
        "area_effect_on_extinction" => Ok(RunOutput::Scalar(
            bio::biogeography::island::area_effect_on_extinction(
                get_f(p, "e_base")?,
                get_f(p, "area")?,
                get_f(p, "z")?,
            ),
        )),
        "habitat_diversity" => Ok(RunOutput::Scalar(
            bio::biogeography::island::habitat_diversity(get_f(p, "area")?, get_f(p, "k")?),
        )),
        "species_range_overlap" => Ok(RunOutput::Scalar(
            bio::biogeography::climate::species_range_overlap(
                (get_f(p, "a_min")?, get_f(p, "a_max")?),
                (get_f(p, "b_min")?, get_f(p, "b_max")?),
            ),
        )),
        "least_cost_distance" => {
            let m = get_m(p, "cost_surface")?;
            let (sx, sy, ex, ey) = (
                get_u(p, "start_x")?,
                get_u(p, "start_y")?,
                get_u(p, "end_x")?,
                get_u(p, "end_y")?,
            );
            Ok(RunOutput::Scalar(
                bio::biogeography::connectivity::least_cost_distance(m, (sx, sy), (ex, ey)),
            ))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
