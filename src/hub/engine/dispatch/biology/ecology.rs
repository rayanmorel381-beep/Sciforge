//! Dispatch handler for ecology functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "shannon_diversity" => Ok(RunOutput::Scalar(
            bio::ecology::diversity::shannon_diversity(get_v(p, "abundances")?),
        )),
        "simpson_diversity" => Ok(RunOutput::Scalar(
            bio::ecology::diversity::simpson_diversity(get_v(p, "abundances")?),
        )),
        "inverse_simpson" => Ok(RunOutput::Scalar(bio::ecology::diversity::inverse_simpson(
            get_v(p, "abundances")?,
        ))),
        "species_richness" => Ok(RunOutput::Integer(
            bio::ecology::diversity::species_richness(get_v(p, "abundances")?) as i64,
        )),
        "pielou_evenness" => Ok(RunOutput::Scalar(bio::ecology::diversity::pielou_evenness(
            get_v(p, "abundances")?,
        ))),
        "berger_parker" => Ok(RunOutput::Scalar(bio::ecology::diversity::berger_parker(
            get_v(p, "abundances")?,
        ))),
        "margalef_richness" => Ok(RunOutput::Scalar(
            bio::ecology::diversity::margalef_richness(
                get_u(p, "species")?,
                get_f(p, "total_individuals")?,
            ),
        )),
        "chao1" => Ok(RunOutput::Scalar(bio::ecology::diversity::chao1(
            get_u(p, "observed")?,
            get_u(p, "singletons")?,
            get_u(p, "doubletons")?,
        ))),
        "hill_number" => Ok(RunOutput::Scalar(bio::ecology::diversity::hill_number(
            get_v(p, "abundances")?,
            get_f(p, "q")?,
        ))),
        "trophic_cascade" => Ok(RunOutput::Matrix(bio::ecology::dynamics::trophic_cascade(
            get_v(p, "levels")?,
            get_v(p, "growth_rates")?,
            get_v(p, "carrying_capacities")?,
            get_v(p, "interaction_strengths")?,
            get_f(p, "dt")?,
            get_u(p, "steps")?,
        ))),
        "species_area" => Ok(RunOutput::Scalar(bio::ecology::dynamics::species_area(
            get_f(p, "c")?,
            get_f(p, "z")?,
            get_f(p, "area")?,
        ))),
        "island_biogeography_equilibrium" => Ok(RunOutput::Scalar(
            bio::ecology::dynamics::island_biogeography_equilibrium(
                get_f(p, "immigration_rate")?,
                get_f(p, "extinction_rate")?,
            ),
        )),
        "carrying_capacity_from_resources" => Ok(RunOutput::Scalar(
            bio::ecology::dynamics::carrying_capacity_from_resources(
                get_f(p, "resource")?,
                get_f(p, "consumption_per_capita")?,
            ),
        )),
        "succession_model" => Ok(RunOutput::Matrix(bio::ecology::dynamics::succession_model(
            get_v(p, "biomass")?,
            get_v(p, "growth_rates")?,
            get_v(p, "capacities")?,
            get_m(p, "competition")?,
            get_f(p, "dt")?,
            get_u(p, "steps")?,
        ))),
        "dispersal_kernel_gaussian" => Ok(RunOutput::Scalar(
            bio::ecology::dynamics::dispersal_kernel_gaussian(
                get_f(p, "distance")?,
                get_f(p, "sigma")?,
            ),
        )),
        "net_primary_productivity" => Ok(RunOutput::Scalar(
            bio::ecology::ecosystem::net_primary_productivity(
                get_f(p, "gpp")?,
                get_f(p, "autotrophic_respiration")?,
            ),
        )),
        "net_ecosystem_productivity" => Ok(RunOutput::Scalar(
            bio::ecology::ecosystem::net_ecosystem_productivity(
                get_f(p, "npp")?,
                get_f(p, "heterotrophic_respiration")?,
            ),
        )),
        "ecology_carbon_use_efficiency" => Ok(RunOutput::Scalar(
            bio::ecology::ecosystem::carbon_use_efficiency(get_f(p, "npp")?, get_f(p, "gpp")?),
        )),
        "ecology_nitrogen_mineralization" => Ok(RunOutput::Scalar(
            bio::ecology::ecosystem::nitrogen_mineralization(
                get_f(p, "organic_n")?,
                get_f(p, "microbial_activity")?,
                get_f(p, "temperature_factor")?,
            ),
        )),
        "nutrient_use_efficiency" => Ok(RunOutput::Scalar(
            bio::ecology::ecosystem::nutrient_use_efficiency(
                get_f(p, "biomass_produced")?,
                get_f(p, "nutrient_absorbed")?,
            ),
        )),
        "liebig_minimum" => Ok(RunOutput::Scalar(bio::ecology::ecosystem::liebig_minimum(
            get_v(p, "nutrients")?,
            get_v(p, "requirements")?,
        ))),
        "ecology_decomposition_rate" => Ok(RunOutput::Scalar(
            bio::ecology::ecosystem::decomposition_rate(
                get_f(p, "initial_mass")?,
                get_f(p, "k")?,
                get_f(p, "t")?,
            ),
        )),
        "soil_respiration" => Ok(RunOutput::Scalar(
            bio::ecology::ecosystem::soil_respiration(
                get_f(p, "temperature")?,
                get_f(p, "moisture")?,
                get_f(p, "q10")?,
                get_f(p, "r_ref")?,
            ),
        )),
        "evapotranspiration_penman_monteith" => Ok(RunOutput::Scalar(
            bio::ecology::ecosystem::evapotranspiration_penman_monteith(
                get_f(p, "net_radiation")?,
                get_f(p, "soil_heat_flux")?,
                get_f(p, "air_temp")?,
                get_f(p, "vpd")?,
                get_f(p, "wind_speed")?,
                get_f(p, "surface_resistance")?,
            ),
        )),
        "ecology_water_use_efficiency" => Ok(RunOutput::Scalar(
            bio::ecology::ecosystem::water_use_efficiency(
                get_f(p, "carbon_assimilated")?,
                get_f(p, "water_transpired")?,
            ),
        )),
        "litter_bag_decomposition" => Ok(RunOutput::Scalar(
            bio::ecology::ecosystem::litter_bag_decomposition(
                get_f(p, "initial_mass")?,
                get_f(p, "remaining_mass")?,
                get_f(p, "time")?,
            ),
        )),
        "lotka_volterra_competition" => {
            let (a, b) = bio::ecology::food_web::lotka_volterra_competition(
                get_f(p, "n1")?,
                get_f(p, "n2")?,
                get_f(p, "r1")?,
                get_f(p, "r2")?,
                get_f(p, "k1")?,
                get_f(p, "k2")?,
                get_f(p, "alpha12")?,
                get_f(p, "alpha21")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "lotka_volterra_predator_prey" => {
            let (a, b) = bio::ecology::food_web::lotka_volterra_predator_prey(
                get_f(p, "prey")?,
                get_f(p, "predator")?,
                get_f(p, "r")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "m")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "ecology_rosenzweig_macarthur" => {
            let (a, b) = bio::ecology::food_web::rosenzweig_macarthur(
                get_f(p, "prey")?,
                get_f(p, "predator")?,
                get_f(p, "r")?,
                get_f(p, "k")?,
                get_f(p, "a")?,
                get_f(p, "h")?,
                get_f(p, "e")?,
                get_f(p, "m")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "type_ii_functional_response" => Ok(RunOutput::Scalar(
            bio::ecology::food_web::type_ii_functional_response(
                get_f(p, "prey_density")?,
                get_f(p, "attack_rate")?,
                get_f(p, "handling_time")?,
            ),
        )),
        "type_iii_functional_response" => Ok(RunOutput::Scalar(
            bio::ecology::food_web::type_iii_functional_response(
                get_f(p, "prey_density")?,
                get_f(p, "attack_rate")?,
                get_f(p, "handling_time")?,
                get_f(p, "exponent")?,
            ),
        )),
        "nutrient_cycling" => {
            let (a, b, c) = bio::ecology::food_web::nutrient_cycling(
                get_f(p, "nutrient")?,
                get_f(p, "producers")?,
                get_f(p, "decomposers")?,
                get_f(p, "uptake_rate")?,
                get_f(p, "mortality_rate")?,
                get_f(p, "decomposition_rate")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "disturbance_regime" => Ok(RunOutput::Scalar(
            bio::ecology::food_web::disturbance_regime(
                get_f(p, "biomass")?,
                get_f(p, "disturbance_intensity")?,
                get_f(p, "return_interval")?,
                get_f(p, "time_since")?,
            ),
        )),
        "intermediate_disturbance_diversity" => Ok(RunOutput::Scalar(
            bio::ecology::food_web::intermediate_disturbance_diversity(
                get_f(p, "disturbance_frequency")?,
                get_f(p, "max_diversity")?,
                get_f(p, "optimal_frequency")?,
            ),
        )),
        "metapopulation_levins" => Ok(RunOutput::Scalar(
            bio::ecology::food_web::metapopulation_levins(
                get_f(p, "p")?,
                get_f(p, "colonization")?,
                get_f(p, "extinction")?,
            ),
        )),
        "source_sink_dynamics" => Ok(RunOutput::Scalar(
            bio::ecology::food_web::source_sink_dynamics(
                get_f(p, "source_emigration")?,
                get_f(p, "sink_mortality")?,
                get_f(p, "sink_immigration")?,
            ),
        )),
        "food_web_connectance" => Ok(RunOutput::Scalar(
            bio::ecology::food_web::food_web_connectance(get_u(p, "links")?, get_u(p, "species")?),
        )),
        "trophic_level" => Ok(RunOutput::Scalar(bio::ecology::food_web::trophic_level(
            get_v(p, "diet_trophic_levels")?,
            get_v(p, "diet_fractions")?,
        ))),
        "lindeman_efficiency" => Ok(RunOutput::Scalar(
            bio::ecology::food_web::lindeman_efficiency(
                get_f(p, "energy_n_plus_1")?,
                get_f(p, "energy_n")?,
            ),
        )),
        "bray_curtis" => Ok(RunOutput::Scalar(bio::ecology::similarity::bray_curtis(
            get_v(p, "a")?,
            get_v(p, "b")?,
        ))),
        "jaccard" => {
            let a_v = get_v(p, "a")?;
            let a: Vec<bool> = a_v.iter().map(|&x| x != 0.0).collect();
            let b_v = get_v(p, "b")?;
            let b: Vec<bool> = b_v.iter().map(|&x| x != 0.0).collect();
            Ok(RunOutput::Scalar(bio::ecology::similarity::jaccard(&a, &b)))
        }
        "sorensen" => {
            let a_v = get_v(p, "a")?;
            let a: Vec<bool> = a_v.iter().map(|&x| x != 0.0).collect();
            let b_v = get_v(p, "b")?;
            let b: Vec<bool> = b_v.iter().map(|&x| x != 0.0).collect();
            Ok(RunOutput::Scalar(bio::ecology::similarity::sorensen(
                &a, &b,
            )))
        }
        "morisita_horn" => Ok(RunOutput::Scalar(bio::ecology::similarity::morisita_horn(
            get_v(p, "a")?,
            get_v(p, "b")?,
        ))),
        "euclidean_distance" => Ok(RunOutput::Scalar(
            bio::ecology::similarity::euclidean_distance(get_v(p, "a")?, get_v(p, "b")?),
        )),
        "whittaker_beta" => Ok(RunOutput::Scalar(bio::ecology::similarity::whittaker_beta(
            get_u(p, "gamma")?,
            get_f(p, "alpha_mean")?,
        ))),
        "horn_overlap" => Ok(RunOutput::Scalar(bio::ecology::similarity::horn_overlap(
            get_v(p, "a")?,
            get_v(p, "b")?,
        ))),
        "chao_jaccard" => Ok(RunOutput::Scalar(bio::ecology::similarity::chao_jaccard(
            get_u(p, "shared")?,
            get_u(p, "a_only")?,
            get_u(p, "b_only")?,
            get_u(p, "n_a")?,
            get_u(p, "n_b")?,
        ))),
        "manhattan_distance" => Ok(RunOutput::Scalar(
            bio::ecology::similarity::manhattan_distance(get_v(p, "a")?, get_v(p, "b")?),
        )),
        "canberra_distance" => Ok(RunOutput::Scalar(
            bio::ecology::similarity::canberra_distance(get_v(p, "a")?, get_v(p, "b")?),
        )),
        "reaction_diffusion_1d" => {
            let mut u = get_v(p, "u")?.to_vec();
            let mut v = get_v(p, "v")?.to_vec();
            bio::ecology::dynamics::reaction_diffusion_1d(
                &mut u,
                &mut v,
                get_f(p, "du")?,
                get_f(p, "dv")?,
                get_f(p, "f_coeff")?,
                get_f(p, "k")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            );
            let mut out = u;
            out.extend(v);
            Ok(RunOutput::Vector(out))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
