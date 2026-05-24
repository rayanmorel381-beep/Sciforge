//! Dispatch handler for nuclear functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::physics as phys;
use crate::engine::experience::runner::RunOutput;

fn make_process(p: &Params) -> HubResult<phys::nuclear::ProcessType> {
    let name = get_str(p, "process")?;
    match name {
        "pp_chain" => Ok(phys::nuclear::ProcessType::PPChain),
        "cno_cycle" => Ok(phys::nuclear::ProcessType::CNOCycle),
        "triple_alpha" => Ok(phys::nuclear::ProcessType::TripleAlpha),
        "carbon_burning" => Ok(phys::nuclear::ProcessType::CarbonBurning),
        "neon_burning" => Ok(phys::nuclear::ProcessType::NeonBurning),
        "oxygen_burning" => Ok(phys::nuclear::ProcessType::OxygenBurning),
        "silicon_burning" => Ok(phys::nuclear::ProcessType::SiliconBurning),
        "s_process" => Ok(phys::nuclear::ProcessType::SProcess),
        "r_process" => Ok(phys::nuclear::ProcessType::RProcess),
        "rp_process" => Ok(phys::nuclear::ProcessType::RPProcess),
        _ => Err(HubError::InvalidInput(format!(
            "unknown process type: {name}"
        ))),
    }
}

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "activity_becquerel" => Ok(RunOutput::Scalar(
            phys::nuclear::activity_becquerel(get_f(p, "n_atoms")?, get_f(p, "half_life")?),
        )),
        "activity_curie" => Ok(RunOutput::Scalar(phys::nuclear::activity_curie(
            get_f(p, "n_atoms")?,
            get_f(p, "half_life")?,
        ))),
        "alpha_decay_q" => Ok(RunOutput::Scalar(phys::nuclear::alpha_decay_q(
            get_f(p, "parent_mass_mev")?,
            get_f(p, "daughter_mass_mev")?,
            get_f(p, "alpha_mass_mev")?,
        ))),
        "astrophysical_s_factor" => Ok(RunOutput::Scalar(
            phys::nuclear::astrophysical_s_factor(
                get_f(p, "cross_section_barn")?,
                get_f(p, "energy_kev")?,
                get_i(p, "z1")? as u32,
                get_i(p, "z2")? as u32,
                get_f(p, "mu_amu")?,
            ),
        )),
        "bateman_chain" => Ok(RunOutput::Vector(phys::nuclear::bateman_chain(
            get_f(p, "n0")?,
            get_v(p, "lambdas")?,
            get_f(p, "time")?,
        ))),
        "beta_minus_q" => Ok(RunOutput::Scalar(phys::nuclear::beta_minus_q(
            get_f(p, "parent_mass_amu")?,
            get_f(p, "daughter_mass_amu")?,
        ))),
        "beta_plus_q" => Ok(RunOutput::Scalar(phys::nuclear::beta_plus_q(
            get_f(p, "parent_mass_amu")?,
            get_f(p, "daughter_mass_amu")?,
        ))),
        "binding_energy_per_nucleon_semf" => Ok(RunOutput::Scalar(
            phys::nuclear::binding_energy_per_nucleon_semf(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "branching_ratio_effective" => Ok(RunOutput::Vector(
            phys::nuclear::branching_ratio_effective(get_v(p, "partial_constants")?),
        )),
        "c12_alpha_rate_estimate" => Ok(RunOutput::Scalar(
            phys::nuclear::c12_alpha_rate_estimate(get_f(p, "temperature_k")?),
        )),
        "cno_cycle_is_dominant" => Ok(RunOutput::Boolean(
            phys::nuclear::cno_cycle_is_dominant(
                get_f(p, "temperature_k")?,
                get_f(p, "metallicity")?,
            ),
        )),
        "core_collapse_min_mass_solar" => Ok(RunOutput::Scalar(
            phys::nuclear::core_collapse_min_mass_solar(),
        )),
        "coulomb_barrier" => Ok(RunOutput::Scalar(phys::nuclear::coulomb_barrier(
            get_i(p, "z1")? as u32,
            get_i(p, "z2")? as u32,
            get_i(p, "a1")? as u32,
            get_i(p, "a2")? as u32,
        ))),
        "decay_chain_equilibrium_time" => Ok(RunOutput::Scalar(
            phys::nuclear::decay_chain_equilibrium_time(
                get_f(p, "lambda_parent")?,
                get_f(p, "lambda_daughter")?,
            ),
        )),
        "decay_constant_from_half_life" => Ok(RunOutput::Scalar(
            phys::nuclear::decay_constant_from_half_life(get_f(p, "half_life")?),
        )),
        "decay_remaining" => Ok(RunOutput::Scalar(phys::nuclear::decay_remaining(
            get_f(p, "n0")?,
            get_f(p, "half_life")?,
            get_f(p, "time")?,
        ))),
        "dose_rate_point_source" => Ok(RunOutput::Scalar(
            phys::nuclear::dose_rate_point_source(
                get_f(p, "activity_bq")?,
                get_f(p, "gamma_constant")?,
                get_f(p, "distance_m")?,
            ),
        )),
        "eddington_luminosity_solar" => Ok(RunOutput::Scalar(
            phys::nuclear::eddington_luminosity_solar(get_f(p, "mass_solar")?),
        )),
        "electron_degeneracy_pressure" => Ok(RunOutput::Scalar(
            phys::nuclear::electron_degeneracy_pressure(get_f(p, "density_kg_m3")?),
        )),
        "fission_barrier_estimate_mev" => Ok(RunOutput::Scalar(
            phys::nuclear::fission_barrier_estimate_mev(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "gamow_peak" => Ok(RunOutput::Scalar(phys::nuclear::gamow_peak(
            get_i(p, "z1")? as u32,
            get_i(p, "z2")? as u32,
            get_f(p, "reduced_mass_amu")?,
            get_f(p, "temperature_k")?,
        ))),
        "gamow_window_width" => Ok(RunOutput::Scalar(
            phys::nuclear::gamow_window_width(
                get_i(p, "z1")? as u32,
                get_i(p, "z2")? as u32,
                get_f(p, "reduced_mass_amu")?,
                get_f(p, "temperature_k")?,
            ),
        )),
        "geiger_nuttall" => Ok(RunOutput::Scalar(phys::nuclear::geiger_nuttall(
            get_i(p, "z")? as u32,
            get_f(p, "energy_mev")?,
        ))),
        "half_life_from_constant" => Ok(RunOutput::Scalar(
            phys::nuclear::half_life_from_constant(get_f(p, "decay_constant")?),
        )),
        "iron_peak_binding_energy" => Ok(RunOutput::Scalar(
            phys::nuclear::iron_peak_binding_energy(),
        )),
        "is_doubly_magic" => Ok(RunOutput::Boolean(phys::nuclear::is_doubly_magic(
            get_i(p, "z")? as u32,
            get_i(p, "n")? as u32,
        ))),
        "isospin_z" => Ok(RunOutput::Scalar(phys::nuclear::isospin_z(
            get_i(p, "z")? as u32,
            get_i(p, "a")? as u32,
        ))),
        "jeans_mass_solar" => Ok(RunOutput::Scalar(phys::nuclear::jeans_mass_solar(
            get_f(p, "temperature_k")?,
            get_f(p, "density_kg_m3")?,
        ))),
        "kelvin_helmholtz_timescale_years" => Ok(RunOutput::Scalar(
            phys::nuclear::kelvin_helmholtz_timescale_years(
                get_f(p, "mass_solar")?,
                get_f(p, "radius_solar")?,
                get_f(p, "luminosity_solar")?,
            ),
        )),
        "liquid_drop_fission_parameter" => Ok(RunOutput::Scalar(
            phys::nuclear::liquid_drop_fission_parameter(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "luminosity_radius_temperature" => Ok(RunOutput::Scalar(
            phys::nuclear::luminosity_radius_temperature(
                get_f(p, "radius_solar")?,
                get_f(p, "temperature_k")?,
            ),
        )),
        "magic_number_nearest" => Ok(RunOutput::Scalar(
            phys::nuclear::magic_number_nearest(get_i(p, "n")? as u32) as f64,
        )),
        "maxwell_averaged_velocity" => Ok(RunOutput::Scalar(
            phys::nuclear::maxwell_averaged_velocity(
                get_f(p, "temperature_k")?,
                get_f(p, "mu_amu")?,
            ),
        )),
        "mean_lifetime" => Ok(RunOutput::Scalar(phys::nuclear::mean_lifetime(
            get_f(p, "half_life")?,
        ))),
        "neutron_capture_rate_estimate" => Ok(RunOutput::Scalar(
            phys::nuclear::neutron_capture_rate_estimate(
                get_f(p, "neutron_density_per_cm3")?,
                get_f(p, "cross_section_barn")?,
                get_f(p, "velocity_cm_s")?,
            ),
        )),
        "neutron_drip_density" => Ok(RunOutput::Scalar(
            phys::nuclear::neutron_drip_density(),
        )),
        "neutron_drip_line_a" => Ok(RunOutput::Scalar(
            phys::nuclear::neutron_drip_line_a(get_i(p, "z")? as u32) as f64,
        )),
        "neutron_excess" => Ok(RunOutput::Scalar(phys::nuclear::neutron_excess(
            get_i(p, "z")? as u32,
            get_i(p, "a")? as u32,
        ) as f64)),
        "neutron_star_radius_km" => Ok(RunOutput::Scalar(
            phys::nuclear::neutron_star_radius_km(get_f(p, "mass_solar")?),
        )),
        "nuclear_density_kg_m3" => Ok(RunOutput::Scalar(
            phys::nuclear::nuclear_density_kg_m3(),
        )),
        "nuclear_radius_fm" => Ok(RunOutput::Scalar(phys::nuclear::nuclear_radius_fm(
            get_i(p, "a")? as u32,
        ))),
        "nuclear_reaction_lifetime" => Ok(RunOutput::Scalar(
            phys::nuclear::nuclear_reaction_lifetime(
                get_f(p, "cross_section_barn")?,
                get_f(p, "number_density_per_cm3")?,
                get_f(p, "velocity_cm_s")?,
            ),
        )),
        "nuclear_skin_thickness_fm" => Ok(RunOutput::Scalar(
            phys::nuclear::nuclear_skin_thickness_fm(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "nuclear_statistical_equilibrium_temp" => Ok(RunOutput::Scalar(
            phys::nuclear::nuclear_statistical_equilibrium_temp(),
        )),
        "nuclear_timescale_years" => Ok(RunOutput::Scalar(
            phys::nuclear::nuclear_timescale_years(
                get_f(p, "mass_solar")?,
                get_f(p, "luminosity_solar")?,
                get_f(p, "efficiency")?,
            ),
        )),
        "penetration_factor" => Ok(RunOutput::Scalar(
            phys::nuclear::penetration_factor(
                get_i(p, "z1")? as u32,
                get_i(p, "z2")? as u32,
                get_f(p, "energy_kev")?,
                get_f(p, "mu_amu")?,
            ),
        )),
        "pp_chain_branches" => {
            let r = phys::nuclear::pp_chain_branches(get_f(p, "temperature_k")?);
            Ok(RunOutput::Triple(r.0, r.1, r.2))
        }
        "pp_rate_estimate" => Ok(RunOutput::Scalar(phys::nuclear::pp_rate_estimate(
            get_f(p, "temperature_k")?,
            get_f(p, "density_g_cm3")?,
            get_f(p, "x_h")?,
        ))),
        "proton_drip_line_a" => Ok(RunOutput::Scalar(
            phys::nuclear::proton_drip_line_a(get_i(p, "z")? as u32) as f64,
        )),
        "r_process_waiting_point_z" => Ok(RunOutput::Scalar(
            phys::nuclear::r_process_waiting_point_z(
                get_f(p, "neutron_separation_energy_mev")?,
                get_f(p, "temperature_gk")?,
            ),
        )),
        "radioactive_dating_age" => Ok(RunOutput::Scalar(
            phys::nuclear::radioactive_dating_age(
                get_f(p, "ratio_daughter_parent")?,
                get_f(p, "half_life")?,
            ),
        )),
        "reaction_mean_free_path" => Ok(RunOutput::Scalar(
            phys::nuclear::reaction_mean_free_path(
                get_f(p, "cross_section_barn")?,
                get_f(p, "number_density_per_cm3")?,
            ),
        )),
        "reduced_mass_amu" => Ok(RunOutput::Scalar(phys::nuclear::reduced_mass_amu(
            get_f(p, "m1")?,
            get_f(p, "m2")?,
        ))),
        "s_process_neutron_exposure" => Ok(RunOutput::Scalar(
            phys::nuclear::s_process_neutron_exposure(
                get_f(p, "tau")?,
                get_f(p, "sigma_times_flux")?,
            ),
        )),
        "schwarzschild_radius_km" => Ok(RunOutput::Scalar(
            phys::nuclear::schwarzschild_radius_km(get_f(p, "mass_solar")?),
        )),
        "secular_equilibrium_activity" => Ok(RunOutput::Scalar(
            phys::nuclear::secular_equilibrium_activity(get_f(p, "parent_activity")?),
        )),
        "semi_empirical_mass" => Ok(RunOutput::Scalar(
            phys::nuclear::semi_empirical_mass(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "separation_energy_alpha" => Ok(RunOutput::Scalar(
            phys::nuclear::separation_energy_alpha(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "separation_energy_neutron" => Ok(RunOutput::Scalar(
            phys::nuclear::separation_energy_neutron(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "separation_energy_proton" => Ok(RunOutput::Scalar(
            phys::nuclear::separation_energy_proton(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "sommerfeld_parameter" => Ok(RunOutput::Scalar(
            phys::nuclear::sommerfeld_parameter(
                get_i(p, "z1")? as u32,
                get_i(p, "z2")? as u32,
                get_f(p, "energy_kev")?,
                get_f(p, "mu_amu")?,
            ),
        )),
        "specific_activity" => Ok(RunOutput::Scalar(phys::nuclear::specific_activity(
            get_f(p, "decay_constant")?,
            get_f(p, "molar_mass")?,
        ))),
        "stellar_wind_mass_loss" => Ok(RunOutput::Scalar(
            phys::nuclear::stellar_wind_mass_loss(
                get_f(p, "luminosity_solar")?,
                get_f(p, "escape_velocity_km_s")?,
            ),
        )),
        "thermonuclear_rate" => Ok(RunOutput::Scalar(
            phys::nuclear::thermonuclear_rate(
                get_f(p, "s_factor_kev_barn")?,
                get_i(p, "z1")? as u32,
                get_i(p, "z2")? as u32,
                get_f(p, "mu_amu")?,
                get_f(p, "temperature_k")?,
            ),
        )),
        "tolman_oppenheimer_volkoff_limit" => Ok(RunOutput::Scalar(
            phys::nuclear::tolman_oppenheimer_volkoff_limit(),
        )),
        "transient_equilibrium_ratio" => Ok(RunOutput::Scalar(
            phys::nuclear::transient_equilibrium_ratio(
                get_f(p, "lambda_parent")?,
                get_f(p, "lambda_daughter")?,
            ),
        )),
        "triple_alpha_rate_estimate" => Ok(RunOutput::Scalar(
            phys::nuclear::triple_alpha_rate_estimate(
                get_f(p, "temperature_k")?,
                get_f(p, "density_g_cm3")?,
                get_f(p, "y_he")?,
            ),
        )),
        "valley_of_stability_z" => Ok(RunOutput::Scalar(
            phys::nuclear::valley_of_stability_z(get_i(p, "a")? as u32),
        )),
        "weizsacker_mass_excess_mev" => Ok(RunOutput::Scalar(
            phys::nuclear::weizsacker_mass_excess_mev(
                get_i(p, "z")? as u32,
                get_i(p, "a")? as u32,
            ),
        )),
        "white_dwarf_radius_km" => Ok(RunOutput::Scalar(
            phys::nuclear::white_dwarf_radius_km(get_f(p, "mass_solar")?),
        )),
        "active_processes" => {
            let r = phys::nuclear::active_processes(get_f(p, "temperature_k")?);
            Ok(RunOutput::Text(
                r.iter()
                    .map(|pt: &phys::nuclear::ProcessType| pt.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ))
        }
        "chandrasekhar_limit" => Ok(RunOutput::Scalar(
            phys::nuclear::chandrasekhar_limit(),
        )),
        "dominant_process_at" => {
            let r = phys::nuclear::dominant_process_at(get_f(p, "temperature_k")?);
            Ok(RunOutput::Text(
                r.map_or_else(|| "none".to_string(), |pt: phys::nuclear::ProcessType| pt.to_string()),
            ))
        }
        "process_fuel" => {
            let pt = make_process(p)?;
            Ok(RunOutput::Text(
                phys::nuclear::process_fuel(&pt).to_string(),
            ))
        }
        "process_product" => {
            let pt = make_process(p)?;
            Ok(RunOutput::Text(
                phys::nuclear::process_product(&pt).to_string(),
            ))
        }
        "process_timescale_years" => {
            let pt = make_process(p)?;
            Ok(RunOutput::Scalar(
                phys::nuclear::process_timescale_years(&pt, get_f(p, "mass_solar")?),
            ))
        }
        "cross_section_barn_to_si" => Ok(RunOutput::Scalar(
            phys::nuclear::cross_section_barn_to_si(get_f(p, "sigma_barn")?),
        )),
        "nuclear_radius_fermi" => Ok(RunOutput::Scalar(
            phys::nuclear::nuclear_radius_fermi(get_i(p, "a")? as u32),
        )),
        "nuclear_volume" => Ok(RunOutput::Scalar(phys::nuclear::nuclear_volume(
            get_i(p, "a")? as u32,
        ))),
        "q_value_to_joules" => Ok(RunOutput::Scalar(phys::nuclear::q_value_to_joules(
            get_f(p, "q_mev")?,
        ))),
        "geometric_cross_section" => Ok(RunOutput::Scalar(
            phys::nuclear::geometric_cross_section(
                get_i(p, "a1")? as u32,
                get_i(p, "a2")? as u32,
            ),
        )),
        "geometric_cross_section_barn" => Ok(RunOutput::Scalar(
            phys::nuclear::geometric_cross_section_barn(
                get_i(p, "a1")? as u32,
                get_i(p, "a2")? as u32,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
