//! Dispatch handler for nucleosynthesis functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::physics as phys;
use crate::hub::engine::experience::runner::RunOutput;

fn make_process(p: &Params) -> HubResult<phys::nucleosynthesis::ProcessType> {
    let name = get_str(p, "process")?;
    match name {
        "pp_chain" => Ok(phys::nucleosynthesis::ProcessType::PPChain),
        "cno_cycle" => Ok(phys::nucleosynthesis::ProcessType::CNOCycle),
        "triple_alpha" => Ok(phys::nucleosynthesis::ProcessType::TripleAlpha),
        "carbon_burning" => Ok(phys::nucleosynthesis::ProcessType::CarbonBurning),
        "neon_burning" => Ok(phys::nucleosynthesis::ProcessType::NeonBurning),
        "oxygen_burning" => Ok(phys::nucleosynthesis::ProcessType::OxygenBurning),
        "silicon_burning" => Ok(phys::nucleosynthesis::ProcessType::SiliconBurning),
        "s_process" => Ok(phys::nucleosynthesis::ProcessType::SProcess),
        "r_process" => Ok(phys::nucleosynthesis::ProcessType::RProcess),
        "rp_process" => Ok(phys::nucleosynthesis::ProcessType::RPProcess),
        _ => Err(HubError::InvalidInput(format!(
            "unknown process type: {name}"
        ))),
    }
}

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "activity_becquerel" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::activity_becquerel(get_f(p, "n_atoms")?, get_f(p, "half_life")?),
        )),
        "activity_curie" => Ok(RunOutput::Scalar(phys::nucleosynthesis::activity_curie(
            get_f(p, "n_atoms")?,
            get_f(p, "half_life")?,
        ))),
        "alpha_decay_q" => Ok(RunOutput::Scalar(phys::nucleosynthesis::alpha_decay_q(
            get_f(p, "parent_mass_mev")?,
            get_f(p, "daughter_mass_mev")?,
            get_f(p, "alpha_mass_mev")?,
        ))),
        "astrophysical_s_factor" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::astrophysical_s_factor(
                get_f(p, "cross_section_barn")?,
                get_f(p, "energy_kev")?,
                get_i(p, "z1")? as u32,
                get_i(p, "z2")? as u32,
                get_f(p, "mu_amu")?,
            ),
        )),
        "bateman_chain" => Ok(RunOutput::Vector(phys::nucleosynthesis::bateman_chain(
            get_f(p, "n0")?,
            get_v(p, "lambdas")?,
            get_f(p, "time")?,
        ))),
        "beta_minus_q" => Ok(RunOutput::Scalar(phys::nucleosynthesis::beta_minus_q(
            get_f(p, "parent_mass_amu")?,
            get_f(p, "daughter_mass_amu")?,
        ))),
        "beta_plus_q" => Ok(RunOutput::Scalar(phys::nucleosynthesis::beta_plus_q(
            get_f(p, "parent_mass_amu")?,
            get_f(p, "daughter_mass_amu")?,
        ))),
        "binding_energy_per_nucleon_semf" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::binding_energy_per_nucleon_semf(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "branching_ratio_effective" => Ok(RunOutput::Vector(
            phys::nucleosynthesis::branching_ratio_effective(get_v(p, "partial_constants")?),
        )),
        "c12_alpha_rate_estimate" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::c12_alpha_rate_estimate(get_f(p, "temperature_k")?),
        )),
        "cno_cycle_is_dominant" => Ok(RunOutput::Boolean(
            phys::nucleosynthesis::cno_cycle_is_dominant(
                get_f(p, "temperature_k")?,
                get_f(p, "metallicity")?,
            ),
        )),
        "core_collapse_min_mass_solar" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::core_collapse_min_mass_solar(),
        )),
        "coulomb_barrier" => Ok(RunOutput::Scalar(phys::nucleosynthesis::coulomb_barrier(
            get_i(p, "z1")? as u32,
            get_i(p, "z2")? as u32,
            get_i(p, "a1")? as u32,
            get_i(p, "a2")? as u32,
        ))),
        "decay_chain_equilibrium_time" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::decay_chain_equilibrium_time(
                get_f(p, "lambda_parent")?,
                get_f(p, "lambda_daughter")?,
            ),
        )),
        "decay_constant_from_half_life" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::decay_constant_from_half_life(get_f(p, "half_life")?),
        )),
        "decay_remaining" => Ok(RunOutput::Scalar(phys::nucleosynthesis::decay_remaining(
            get_f(p, "n0")?,
            get_f(p, "half_life")?,
            get_f(p, "time")?,
        ))),
        "dose_rate_point_source" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::dose_rate_point_source(
                get_f(p, "activity_bq")?,
                get_f(p, "gamma_constant")?,
                get_f(p, "distance_m")?,
            ),
        )),
        "eddington_luminosity_solar" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::eddington_luminosity_solar(get_f(p, "mass_solar")?),
        )),
        "electron_degeneracy_pressure" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::electron_degeneracy_pressure(get_f(p, "density_kg_m3")?),
        )),
        "fission_barrier_estimate_mev" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::fission_barrier_estimate_mev(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "gamow_peak" => Ok(RunOutput::Scalar(phys::nucleosynthesis::gamow_peak(
            get_i(p, "z1")? as u32,
            get_i(p, "z2")? as u32,
            get_f(p, "reduced_mass_amu")?,
            get_f(p, "temperature_k")?,
        ))),
        "gamow_window_width" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::gamow_window_width(
                get_i(p, "z1")? as u32,
                get_i(p, "z2")? as u32,
                get_f(p, "reduced_mass_amu")?,
                get_f(p, "temperature_k")?,
            ),
        )),
        "geiger_nuttall" => Ok(RunOutput::Scalar(phys::nucleosynthesis::geiger_nuttall(
            get_i(p, "z")? as u32,
            get_f(p, "energy_mev")?,
        ))),
        "half_life_from_constant" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::half_life_from_constant(get_f(p, "decay_constant")?),
        )),
        "iron_peak_binding_energy" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::iron_peak_binding_energy(),
        )),
        "is_doubly_magic" => Ok(RunOutput::Boolean(phys::nucleosynthesis::is_doubly_magic(
            get_i(p, "z")? as u32,
            get_i(p, "n")? as u32,
        ))),
        "isospin_z" => Ok(RunOutput::Scalar(phys::nucleosynthesis::isospin_z(
            get_i(p, "z")? as u32,
            get_i(p, "a")? as u32,
        ))),
        "jeans_mass_solar" => Ok(RunOutput::Scalar(phys::nucleosynthesis::jeans_mass_solar(
            get_f(p, "temperature_k")?,
            get_f(p, "density_kg_m3")?,
        ))),
        "kelvin_helmholtz_timescale_years" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::kelvin_helmholtz_timescale_years(
                get_f(p, "mass_solar")?,
                get_f(p, "radius_solar")?,
                get_f(p, "luminosity_solar")?,
            ),
        )),
        "liquid_drop_fission_parameter" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::liquid_drop_fission_parameter(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "luminosity_radius_temperature" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::luminosity_radius_temperature(
                get_f(p, "radius_solar")?,
                get_f(p, "temperature_k")?,
            ),
        )),
        "magic_number_nearest" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::magic_number_nearest(get_i(p, "n")? as u32) as f64,
        )),
        "maxwell_averaged_velocity" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::maxwell_averaged_velocity(
                get_f(p, "temperature_k")?,
                get_f(p, "mu_amu")?,
            ),
        )),
        "mean_lifetime" => Ok(RunOutput::Scalar(phys::nucleosynthesis::mean_lifetime(
            get_f(p, "half_life")?,
        ))),
        "neutron_capture_rate_estimate" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::neutron_capture_rate_estimate(
                get_f(p, "neutron_density_per_cm3")?,
                get_f(p, "cross_section_barn")?,
                get_f(p, "velocity_cm_s")?,
            ),
        )),
        "neutron_drip_density" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::neutron_drip_density(),
        )),
        "neutron_drip_line_a" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::neutron_drip_line_a(get_i(p, "z")? as u32) as f64,
        )),
        "neutron_excess" => Ok(RunOutput::Scalar(phys::nucleosynthesis::neutron_excess(
            get_i(p, "z")? as u32,
            get_i(p, "a")? as u32,
        ) as f64)),
        "neutron_star_radius_km" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::neutron_star_radius_km(get_f(p, "mass_solar")?),
        )),
        "nuclear_density_kg_m3" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::nuclear_density_kg_m3(),
        )),
        "nuclear_radius_fm" => Ok(RunOutput::Scalar(phys::nucleosynthesis::nuclear_radius_fm(
            get_i(p, "a")? as u32,
        ))),
        "nuclear_reaction_lifetime" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::nuclear_reaction_lifetime(
                get_f(p, "cross_section_barn")?,
                get_f(p, "number_density_per_cm3")?,
                get_f(p, "velocity_cm_s")?,
            ),
        )),
        "nuclear_skin_thickness_fm" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::nuclear_skin_thickness_fm(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "nuclear_statistical_equilibrium_temp" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::nuclear_statistical_equilibrium_temp(),
        )),
        "nuclear_timescale_years" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::nuclear_timescale_years(
                get_f(p, "mass_solar")?,
                get_f(p, "luminosity_solar")?,
                get_f(p, "efficiency")?,
            ),
        )),
        "penetration_factor" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::penetration_factor(
                get_i(p, "z1")? as u32,
                get_i(p, "z2")? as u32,
                get_f(p, "energy_kev")?,
                get_f(p, "mu_amu")?,
            ),
        )),
        "pp_chain_branches" => {
            let r = phys::nucleosynthesis::pp_chain_branches(get_f(p, "temperature_k")?);
            Ok(RunOutput::Triple(r.0, r.1, r.2))
        }
        "pp_rate_estimate" => Ok(RunOutput::Scalar(phys::nucleosynthesis::pp_rate_estimate(
            get_f(p, "temperature_k")?,
            get_f(p, "density_g_cm3")?,
            get_f(p, "x_h")?,
        ))),
        "proton_drip_line_a" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::proton_drip_line_a(get_i(p, "z")? as u32) as f64,
        )),
        "r_process_waiting_point_z" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::r_process_waiting_point_z(
                get_f(p, "neutron_separation_energy_mev")?,
                get_f(p, "temperature_gk")?,
            ),
        )),
        "radioactive_dating_age" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::radioactive_dating_age(
                get_f(p, "ratio_daughter_parent")?,
                get_f(p, "half_life")?,
            ),
        )),
        "reaction_mean_free_path" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::reaction_mean_free_path(
                get_f(p, "cross_section_barn")?,
                get_f(p, "number_density_per_cm3")?,
            ),
        )),
        "reduced_mass_amu" => Ok(RunOutput::Scalar(phys::nucleosynthesis::reduced_mass_amu(
            get_f(p, "m1")?,
            get_f(p, "m2")?,
        ))),
        "s_process_neutron_exposure" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::s_process_neutron_exposure(
                get_f(p, "tau")?,
                get_f(p, "sigma_times_flux")?,
            ),
        )),
        "schwarzschild_radius_km" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::schwarzschild_radius_km(get_f(p, "mass_solar")?),
        )),
        "secular_equilibrium_activity" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::secular_equilibrium_activity(get_f(p, "parent_activity")?),
        )),
        "semi_empirical_mass" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::semi_empirical_mass(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "separation_energy_alpha" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::separation_energy_alpha(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "separation_energy_neutron" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::separation_energy_neutron(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "separation_energy_proton" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::separation_energy_proton(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "sommerfeld_parameter" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::sommerfeld_parameter(
                get_i(p, "z1")? as u32,
                get_i(p, "z2")? as u32,
                get_f(p, "energy_kev")?,
                get_f(p, "mu_amu")?,
            ),
        )),
        "specific_activity" => Ok(RunOutput::Scalar(phys::nucleosynthesis::specific_activity(
            get_f(p, "decay_constant")?,
            get_f(p, "molar_mass")?,
        ))),
        "stellar_wind_mass_loss" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::stellar_wind_mass_loss(
                get_f(p, "luminosity_solar")?,
                get_f(p, "escape_velocity_km_s")?,
            ),
        )),
        "thermonuclear_rate" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::thermonuclear_rate(
                get_f(p, "s_factor_kev_barn")?,
                get_i(p, "z1")? as u32,
                get_i(p, "z2")? as u32,
                get_f(p, "mu_amu")?,
                get_f(p, "temperature_k")?,
            ),
        )),
        "tolman_oppenheimer_volkoff_limit" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::tolman_oppenheimer_volkoff_limit(),
        )),
        "transient_equilibrium_ratio" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::transient_equilibrium_ratio(
                get_f(p, "lambda_parent")?,
                get_f(p, "lambda_daughter")?,
            ),
        )),
        "triple_alpha_rate_estimate" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::triple_alpha_rate_estimate(
                get_f(p, "temperature_k")?,
                get_f(p, "density_g_cm3")?,
                get_f(p, "y_he")?,
            ),
        )),
        "valley_of_stability_z" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::valley_of_stability_z(get_i(p, "a")? as u32),
        )),
        "weizsacker_mass_excess_mev" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::weizsacker_mass_excess_mev(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "white_dwarf_radius_km" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::white_dwarf_radius_km(get_f(p, "mass_solar")?),
        )),
        "active_processes" => {
            let r = phys::nucleosynthesis::active_processes(get_f(p, "temperature_k")?);
            Ok(RunOutput::Text(
                r.iter()
                    .map(|pt| pt.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ))
        }
        "chandrasekhar_limit" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::chandrasekhar_limit(),
        )),
        "dominant_process_at" => {
            let r = phys::nucleosynthesis::dominant_process_at(get_f(p, "temperature_k")?);
            Ok(RunOutput::Text(
                r.map_or_else(|| "none".to_string(), |pt| pt.to_string()),
            ))
        }
        "process_fuel" => {
            let pt = make_process(p)?;
            Ok(RunOutput::Text(
                phys::nucleosynthesis::process_fuel(&pt).to_string(),
            ))
        }
        "process_product" => {
            let pt = make_process(p)?;
            Ok(RunOutput::Text(
                phys::nucleosynthesis::process_product(&pt).to_string(),
            ))
        }
        "process_timescale_years" => {
            let pt = make_process(p)?;
            Ok(RunOutput::Scalar(
                phys::nucleosynthesis::process_timescale_years(&pt, get_f(p, "mass_solar")?),
            ))
        }
        "cross_section_barn_to_si" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::cross_section_barn_to_si(get_f(p, "sigma_barn")?),
        )),
        "nuclear_radius_fermi" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::nuclear_radius_fermi(get_i(p, "a")? as u32),
        )),
        "nuclear_volume" => Ok(RunOutput::Scalar(phys::nucleosynthesis::nuclear_volume(
            get_i(p, "a")? as u32,
        ))),
        "q_value_to_joules" => Ok(RunOutput::Scalar(phys::nucleosynthesis::q_value_to_joules(
            get_f(p, "q_mev")?,
        ))),
        "geometric_cross_section" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::geometric_cross_section(
                get_i(p, "a1")? as u32,
                get_i(p, "a2")? as u32,
            ),
        )),
        "geometric_cross_section_barn" => Ok(RunOutput::Scalar(
            phys::nucleosynthesis::geometric_cross_section_barn(
                get_i(p, "a1")? as u32,
                get_i(p, "a2")? as u32,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
