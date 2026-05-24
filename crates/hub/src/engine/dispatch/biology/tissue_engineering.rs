//! Dispatch handler for tissue engineering functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "perfusion_rate" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::bioreactor::perfusion_rate(
                get_f(p, "flow_ml_min")?,
                get_f(p, "volume_ml")?,
            ),
        )),
        "shear_stress_parallel_plate" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::bioreactor::shear_stress_parallel_plate(
                get_f(p, "viscosity")?,
                get_f(p, "flow_rate")?,
                get_f(p, "width")?,
                get_f(p, "height")?,
            ),
        )),
        "oxygen_transfer_rate" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::bioreactor::oxygen_transfer_rate(
                get_f(p, "kla")?,
                get_f(p, "c_star")?,
                get_f(p, "c_bulk")?,
            ),
        )),
        "residence_time" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::bioreactor::residence_time(
                get_f(p, "volume_ml")?,
                get_f(p, "flow_rate_ml_min")?,
            ),
        )),
        "spinner_flask_shear" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::bioreactor::spinner_flask_shear(
                get_f(p, "viscosity")?,
                get_f(p, "rpm")?,
                get_f(p, "radius")?,
            ),
        )),
        "hollow_fiber_mass_transfer" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::bioreactor::hollow_fiber_mass_transfer(
                get_f(p, "permeability")?,
                get_f(p, "surface_area")?,
                get_f(p, "delta_c")?,
            ),
        )),
        "bioreactor_seeding_efficiency" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::bioreactor::bioreactor_seeding_efficiency(
                get_f(p, "attached")?,
                get_f(p, "seeded")?,
            ),
        )),
        "scaffold_porosity" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::bioreactor::scaffold_porosity(
                get_f(p, "void_volume")?,
                get_f(p, "total_volume")?,
            ),
        )),
        "degradation_rate_first_order" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::bioreactor::degradation_rate_first_order(
                get_f(p, "mass")?,
                get_f(p, "k_deg")?,
                get_f(p, "time")?,
            ),
        )),
        "mechanical_modulus_scaffold" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::bioreactor::mechanical_modulus_scaffold(
                get_f(p, "stress")?,
                get_f(p, "strain")?,
            ),
        )),
        "cell_proliferation_on_scaffold" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::bioreactor::cell_proliferation_on_scaffold(
                get_f(p, "n0")?,
                get_f(p, "doubling_time")?,
                get_f(p, "elapsed")?,
            ),
        )),
        "scaffold_degradation_bulk" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::scaffold_degradation_bulk(
                get_f(p, "m0")?,
                get_f(p, "k")?,
                get_f(p, "t")?,
            ),
        )),
        "scaffold_degradation_surface" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::scaffold_degradation_surface(
                get_f(p, "m0")?,
                get_f(p, "rate")?,
                get_f(p, "surface_area")?,
                get_f(p, "t")?,
            ),
        )),
        "porosity" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::porosity(
                get_f(p, "void_volume")?,
                get_f(p, "total_volume")?,
            ),
        )),
        "pore_interconnectivity" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::pore_interconnectivity(
                get_f(p, "connected_pores")?,
                get_f(p, "total_pores")?,
            ),
        )),
        "mechanical_stiffness_degrading" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::mechanical_stiffness_degrading(
                get_f(p, "e0")?,
                get_f(p, "degradation_fraction")?,
                get_f(p, "n")?,
            ),
        )),
        "scaffold_swelling_ratio" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::scaffold_swelling_ratio(
                get_f(p, "wet_mass")?,
                get_f(p, "dry_mass")?,
            ),
        )),
        "cell_seeding_efficiency" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::cell_seeding_efficiency(
                get_f(p, "attached")?,
                get_f(p, "seeded")?,
            ),
        )),
        "nutrient_diffusion_depth" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::nutrient_diffusion_depth(
                get_f(p, "diffusivity")?,
                get_f(p, "consumption_rate")?,
                get_f(p, "surface_concentration")?,
            ),
        )),
        "construct_viability" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::construct_viability(
                get_f(p, "viable_cells")?,
                get_f(p, "total_cells")?,
            ),
        )),
        "bioreactor_shear_stress" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::bioreactor_shear_stress(
                get_f(p, "flow_rate")?,
                get_f(p, "viscosity")?,
                get_f(p, "channel_height")?,
                get_f(p, "channel_width")?,
            ),
        )),
        "fiber_diameter_electrospinning" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::fiber_diameter_electrospinning(
                get_f(p, "voltage")?,
                get_f(p, "flow_rate")?,
                get_f(p, "concentration")?,
                get_f(p, "distance")?,
            ),
        )),
        "scaffold_compressive_modulus" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::scaffold_compressive_modulus(
                get_f(p, "stress")?,
                get_f(p, "strain")?,
            ),
        )),
        "scaffold_tortuosity" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::scaffold_tortuosity(
                get_f(p, "path_length")?,
                get_f(p, "straight_distance")?,
            ),
        )),
        "specific_surface_area" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::specific_surface_area(
                get_f(p, "surface_area")?,
                get_f(p, "volume")?,
            ),
        )),
        "kozeny_carman_permeability" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::kozeny_carman_permeability(
                get_f(p, "porosity_frac")?,
                get_f(p, "pore_diameter")?,
            ),
        )),
        "hydrogel_crosslink_density" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::hydrogel_crosslink_density(
                get_f(p, "shear_modulus")?,
                get_f(p, "temperature")?,
            ),
        )),
        "hydrogel_mesh_size" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::hydrogel_mesh_size(
                get_f(p, "crosslink_density")?,
                get_f(p, "cn")?,
                get_f(p, "bond_length")?,
                get_f(p, "mr")?,
                get_f(p, "polymer_density")?,
            ),
        )),
        "scaffold_surface_energy" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::scaffold_surface_energy(
                get_f(p, "contact_angle_deg")?,
                get_f(p, "liquid_tension")?,
            ),
        )),
        "drug_release_higuchi" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::drug_release_higuchi(
                get_f(p, "k_h")?,
                get_f(p, "t")?,
            ),
        )),
        "drug_release_korsmeyer_peppas" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::drug_release_korsmeyer_peppas(
                get_f(p, "k")?,
                get_f(p, "t")?,
                get_f(p, "n")?,
            ),
        )),
        "degradation_molecular_weight" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::degradation_molecular_weight(
                get_f(p, "mw0")?,
                get_f(p, "k")?,
                get_f(p, "t")?,
            ),
        )),
        "autocatalytic_degradation" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::scaffold::autocatalytic_degradation(
                get_f(p, "mw0")?,
                get_f(p, "k")?,
                get_f(p, "acid_conc")?,
                get_f(p, "t")?,
            ),
        )),
        "cell_proliferation_3d" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::cell_proliferation_3d(
                get_f(p, "cells")?,
                get_f(p, "growth_rate")?,
                get_f(p, "nutrient_factor")?,
                get_f(p, "carrying_capacity")?,
                get_f(p, "dt")?,
            ),
        )),
        "tissue_oxygen_diffusion_krogh" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::oxygen_diffusion_krogh(
                get_f(p, "r_tissue")?,
                get_f(p, "r_capillary")?,
                get_f(p, "consumption_rate")?,
                get_f(p, "diffusivity")?,
                get_f(p, "c_surface")?,
            ),
        )),
        "nutrient_penetration_depth" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::nutrient_penetration_depth(
                get_f(p, "diffusivity")?,
                get_f(p, "surface_concentration")?,
                get_f(p, "consumption_rate")?,
            ),
        )),
        "perfusion_bioreactor_shear" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::perfusion_bioreactor_shear(
                get_f(p, "viscosity")?,
                get_f(p, "flow_rate")?,
                get_f(p, "channel_height")?,
                get_f(p, "channel_width")?,
            ),
        )),
        "cell_migration_speed" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::cell_migration_speed(
                get_f(p, "traction_force")?,
                get_f(p, "drag_coefficient")?,
            ),
        )),
        "tissue_maturation_index" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::tissue_maturation_index(
                get_f(p, "collagen_content")?,
                get_f(p, "target_collagen")?,
                get_f(p, "mechanical_strength")?,
                get_f(p, "target_strength")?,
            ),
        )),
        "vascularization_density" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::vascularization_density(
                get_f(p, "vessel_length")?,
                get_f(p, "tissue_volume")?,
            ),
        )),
        "tissue_extracellular_matrix_production" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::extracellular_matrix_production(
                get_f(p, "cell_density")?,
                get_f(p, "production_rate")?,
                get_f(p, "stimulus")?,
                get_f(p, "degradation_rate")?,
                get_f(p, "ecm")?,
                get_f(p, "dt")?,
            ),
        )),
        "cell_sheet_contraction" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::cell_sheet_contraction(
                get_f(p, "initial_area")?,
                get_f(p, "contractility")?,
                get_f(p, "cell_density")?,
                get_f(p, "t")?,
            ),
        )),
        "mechanotransduction_response" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::mechanotransduction_response(
                get_f(p, "strain")?,
                get_f(p, "threshold")?,
                get_f(p, "sensitivity")?,
            ),
        )),
        "angiogenic_sprouting_rate" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::angiogenic_sprouting_rate(
                get_f(p, "vegf")?,
                get_f(p, "k_vegf")?,
                get_f(p, "tip_cell_density")?,
            ),
        )),
        "tissue_compaction" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::tissue_compaction(
                get_f(p, "initial_volume")?,
                get_f(p, "cell_traction")?,
                get_f(p, "matrix_stiffness")?,
                get_f(p, "t")?,
            ),
        )),
        "cell_differentiation_rate" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::cell_differentiation_rate(
                get_f(p, "transcription_factor")?,
                get_f(p, "threshold")?,
                get_f(p, "hill")?,
            ),
        )),
        "wound_healing_closure" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::wound_healing_closure(
                get_f(p, "gap_width")?,
                get_f(p, "migration_speed")?,
                get_f(p, "proliferation_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "nutrient_consumption_michaelis" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::nutrient_consumption_michaelis(
                get_f(p, "concentration")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
            ),
        )),
        "cell_viability_hypoxia" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::cell_viability_hypoxia(
                get_f(p, "po2")?,
                get_f(p, "critical_po2")?,
                get_f(p, "sensitivity")?,
            ),
        )),
        "collagen_fiber_alignment" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::collagen_fiber_alignment(
                get_f(p, "strain_direction")?,
                get_f(p, "fiber_angle")?,
            ),
        )),
        "gag_content_cartilage" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::tissue::gag_content_cartilage(
                get_f(p, "cell_density")?,
                get_f(p, "tgf_beta")?,
                get_f(p, "production_rate")?,
                get_f(p, "degradation_rate")?,
                get_f(p, "current")?,
                get_f(p, "dt")?,
            ),
        )),
        "vegf_diffusion_concentration" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::vascularization::vegf_diffusion_concentration(
                get_f(p, "source_concentration")?,
                get_f(p, "distance")?,
                get_f(p, "diffusion_length")?,
            ),
        )),
        "sprouting_probability" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::vascularization::sprouting_probability(
                get_f(p, "vegf_concentration")?,
                get_f(p, "threshold")?,
                get_f(p, "hill_n")?,
            ),
        )),
        "vessel_spacing_optimal" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::vascularization::vessel_spacing_optimal(
                get_f(p, "oxygen_diffusion_coeff")?,
                get_f(p, "consumption_rate")?,
                get_f(p, "po2_arterial")?,
            ),
        )),
        "krogh_cylinder_po2" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::vascularization::krogh_cylinder_po2(
                get_f(p, "po2_arterial")?,
                get_f(p, "consumption_rate")?,
                get_f(p, "diff_coeff")?,
                get_f(p, "r")?,
                get_f(p, "r_capillary")?,
            ),
        )),
        "angiogenic_switch_balance" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::vascularization::angiogenic_switch_balance(
                get_f(p, "pro_angiogenic")?,
                get_f(p, "anti_angiogenic")?,
            ),
        )),
        "capillary_density" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::vascularization::capillary_density(
                get_f(p, "num_capillaries")?,
                get_f(p, "tissue_area_mm2")?,
            ),
        )),
        "endothelial_migration_speed" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::vascularization::endothelial_migration_speed(
                get_f(p, "chemotactic_gradient")?,
                get_f(p, "sensitivity")?,
                get_f(p, "max_speed")?,
            ),
        )),
        "prevascularization_survival" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::vascularization::prevascularization_survival(
                get_f(p, "distance_to_vessel_um")?,
                get_f(p, "max_diffusion_distance")?,
            ),
        )),
        "microvessel_wall_shear_stress" => Ok(RunOutput::Scalar(
            bio::tissue_engineering::vascularization::microvessel_wall_shear_stress(
                get_f(p, "viscosity")?,
                get_f(p, "flow_rate")?,
                get_f(p, "radius")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
