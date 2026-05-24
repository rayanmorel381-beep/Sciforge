//! Dispatch handler for biophysics functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "membrane_bending_energy" => Ok(RunOutput::Scalar(
            bio::biophysics::membrane::membrane_bending_energy(
                get_f(p, "kappa")?,
                get_f(p, "curvature")?,
                get_f(p, "spontaneous_curvature")?,
            ),
        )),
        "helfrich_energy" => Ok(RunOutput::Scalar(
            bio::biophysics::membrane::helfrich_energy(
                get_f(p, "kappa")?,
                get_f(p, "kappa_bar")?,
                get_f(p, "c1")?,
                get_f(p, "c2")?,
                get_f(p, "c0")?,
            ),
        )),
        "membrane_tension" => Ok(RunOutput::Scalar(
            bio::biophysics::membrane::membrane_tension(
                get_f(p, "area_strain")?,
                get_f(p, "stretch_modulus")?,
            ),
        )),
        "lipid_diffusion_saffman_delbruck" => Ok(RunOutput::Scalar(
            bio::biophysics::membrane::lipid_diffusion_saffman_delbruck(
                get_f(p, "viscosity_membrane")?,
                get_f(p, "viscosity_water")?,
                get_f(p, "membrane_thickness")?,
                get_f(p, "radius")?,
                get_f(p, "t")?,
            ),
        )),
        "osmotic_lysis_threshold" => Ok(RunOutput::Scalar(
            bio::biophysics::membrane::osmotic_lysis_threshold(
                get_f(p, "internal_osmolarity")?,
                get_f(p, "membrane_tension_max")?,
                get_f(p, "radius")?,
            ),
        )),
        "vesicle_budding_energy" => Ok(RunOutput::Scalar(
            bio::biophysics::membrane::vesicle_budding_energy(
                get_f(p, "kappa")?,
                get_f(p, "radius")?,
            ),
        )),
        "flip_flop_rate" => Ok(RunOutput::Scalar(
            bio::biophysics::membrane::flip_flop_rate(
                get_f(p, "activation_energy")?,
                get_f(p, "t")?,
            ),
        )),
        "lateral_pressure_profile" => Ok(RunOutput::Scalar(
            bio::biophysics::membrane::lateral_pressure_profile(
                get_f(p, "depth")?,
                get_f(p, "head_pressure")?,
                get_f(p, "tail_pressure")?,
                get_f(p, "thickness")?,
            ),
        )),
        "line_tension_domain" => Ok(RunOutput::Scalar(
            bio::biophysics::membrane::line_tension_domain(
                get_f(p, "length")?,
                get_f(p, "lambda")?,
            ),
        )),
        "lennard_jones" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::lennard_jones(
                get_f(p, "r")?,
                get_f(p, "epsilon")?,
                get_f(p, "sigma")?,
            ),
        )),
        "lennard_jones_force" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::lennard_jones_force(
                get_f(p, "r")?,
                get_f(p, "epsilon")?,
                get_f(p, "sigma")?,
            ),
        )),
        "biophys_coulomb_interaction" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::coulomb_interaction(
                get_f(p, "q1")?,
                get_f(p, "q2")?,
                get_f(p, "r")?,
                get_f(p, "epsilon_r")?,
            ),
        )),
        "debye_huckel" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::debye_huckel(
                get_f(p, "q")?,
                get_f(p, "r")?,
                get_f(p, "kappa")?,
                get_f(p, "epsilon_r")?,
            ),
        )),
        "kinetic_energy" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::kinetic_energy(
                get_v(p, "velocities")?,
                get_v(p, "masses")?,
            ),
        )),
        "temperature_from_ke" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::temperature_from_ke(
                get_f(p, "ke")?,
                get_u(p, "n_particles")?,
                get_u(p, "n_dim")?,
            ),
        )),
        "morse_potential" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::morse_potential(
                get_f(p, "r")?,
                get_f(p, "d_e")?,
                get_f(p, "a")?,
                get_f(p, "r_e")?,
            ),
        )),
        "harmonic_bond" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::harmonic_bond(
                get_f(p, "r")?,
                get_f(p, "k")?,
                get_f(p, "r0")?,
            ),
        )),
        "harmonic_angle" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::harmonic_angle(
                get_f(p, "theta")?,
                get_f(p, "k")?,
                get_f(p, "theta0")?,
            ),
        )),
        "dihedral_potential" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::dihedral_potential(
                get_f(p, "phi")?,
                get_f(p, "k")?,
                get_f(p, "n")?,
                get_f(p, "delta")?,
            ),
        )),
        "nose_hoover_friction" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::nose_hoover_friction(
                get_f(p, "ke")?,
                get_f(p, "target_ke")?,
                get_f(p, "q")?,
            ),
        )),
        "switching_function" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::switching_function(
                get_f(p, "r")?,
                get_f(p, "r_on")?,
                get_f(p, "r_off")?,
            ),
        )),
        "pair_correlation_bin" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::pair_correlation_bin(
                get_v(p, "distances")?,
                get_f(p, "r_min")?,
                get_f(p, "r_max")?,
                get_u(p, "n_particles")?,
                get_f(p, "volume")?,
            ),
        )),
        "pressure_virial" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::pressure_virial(
                get_u(p, "n")?,
                get_f(p, "volume")?,
                get_f(p, "temperature")?,
                get_f(p, "virial_sum")?,
            ),
        )),
        "mean_free_path" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::mean_free_path(
                get_f(p, "density")?,
                get_f(p, "cross_section")?,
            ),
        )),
        "born_mayer_repulsion" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::born_mayer_repulsion(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "r")?,
            ),
        )),
        "buckingham_potential" => Ok(RunOutput::Scalar(
            bio::biophysics::molecular_dynamics::buckingham_potential(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "c")?,
                get_f(p, "r")?,
            ),
        )),
        "worm_like_chain" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::worm_like_chain(
                get_f(p, "l")?,
                get_f(p, "lp")?,
                get_f(p, "lc")?,
            ),
        )),
        "freely_jointed_chain" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::freely_jointed_chain(
                get_f(p, "l")?,
                get_u(p, "n")?,
                get_f(p, "b")?,
            ),
        )),
        "end_to_end_distance_rms" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::end_to_end_distance_rms(get_u(p, "n")?, get_f(p, "b")?),
        )),
        "biophys_radius_of_gyration" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::radius_of_gyration(get_u(p, "n")?, get_f(p, "b")?),
        )),
        "persistence_length_from_tangent" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::persistence_length_from_tangent(
                get_f(p, "cos_theta")?,
                get_f(p, "segment_length")?,
            ),
        )),
        "kratky_porod_energy" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::kratky_porod_energy(
                get_f(p, "kappa")?,
                get_f(p, "ds")?,
                get_f(p, "curvature")?,
            ),
        )),
        "dna_twist_energy" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::dna_twist_energy(
                get_f(p, "c_twist")?,
                get_f(p, "delta_twist")?,
                get_f(p, "length")?,
            ),
        )),
        "stokes_einstein_diffusion" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::stokes_einstein_diffusion(
                get_f(p, "t")?,
                get_f(p, "viscosity")?,
                get_f(p, "radius")?,
            ),
        )),
        "mean_squared_displacement" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::mean_squared_displacement(
                get_f(p, "d")?,
                get_f(p, "t")?,
                get_u(p, "n_dim")?,
            ),
        )),
        "sedimentation_coefficient" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::sedimentation_coefficient(
                get_f(p, "mass")?,
                get_f(p, "partial_specific_vol")?,
                get_f(p, "rho_solvent")?,
                get_f(p, "friction")?,
            ),
        )),
        "flory_radius" => Ok(RunOutput::Scalar(bio::biophysics::polymers::flory_radius(
            get_u(p, "n")?,
            get_f(p, "b")?,
            get_f(p, "nu")?,
        ))),
        "kuhn_length" => Ok(RunOutput::Scalar(bio::biophysics::polymers::kuhn_length(
            get_f(p, "persistence_length")?,
        ))),
        "contour_length" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::contour_length(get_u(p, "n")?, get_f(p, "b")?),
        )),
        "extensible_wlc" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::extensible_wlc(
                get_f(p, "force")?,
                get_f(p, "lp")?,
                get_f(p, "lc")?,
                get_f(p, "stretch_modulus")?,
                get_f(p, "t")?,
            ),
        )),
        "odijk_deflection_length" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::odijk_deflection_length(get_f(p, "lp")?, get_f(p, "d")?),
        )),
        "blob_size" => Ok(RunOutput::Scalar(bio::biophysics::polymers::blob_size(
            get_f(p, "kbt")?,
            get_f(p, "force")?,
        ))),
        "zimm_relaxation_time" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::zimm_relaxation_time(
                get_f(p, "viscosity")?,
                get_f(p, "rg")?,
                get_f(p, "kbt")?,
            ),
        )),
        "rouse_relaxation_time" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::rouse_relaxation_time(
                get_f(p, "friction")?,
                get_u(p, "n")?,
                get_f(p, "b")?,
                get_f(p, "kbt")?,
            ),
        )),
        "intrinsic_viscosity" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::intrinsic_viscosity(get_f(p, "rg")?, get_f(p, "mw")?),
        )),
        "overlap_concentration" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::overlap_concentration(get_f(p, "mw")?, get_f(p, "rg")?),
        )),
        "debye_scattering" => Ok(RunOutput::Scalar(
            bio::biophysics::polymers::debye_scattering(get_f(p, "q")?, get_f(p, "rg")?),
        )),
        "biophys_ramachandran_energy" => Ok(RunOutput::Scalar(
            bio::biophysics::protein::ramachandran_energy(get_f(p, "phi")?, get_f(p, "psi")?),
        )),
        "hydrophobic_free_energy" => Ok(RunOutput::Scalar(
            bio::biophysics::protein::hydrophobic_free_energy(
                get_f(p, "sasa_nonpolar")?,
                get_f(p, "gamma")?,
            ),
        )),
        "biophys_hydrogen_bond_energy" => Ok(RunOutput::Scalar(
            bio::biophysics::protein::hydrogen_bond_energy(
                get_f(p, "r")?,
                get_f(p, "theta")?,
                get_f(p, "epsilon")?,
                get_f(p, "r0")?,
            ),
        )),
        "electrostatic_solvation" => Ok(RunOutput::Scalar(
            bio::biophysics::protein::electrostatic_solvation(
                get_f(p, "charge")?,
                get_f(p, "radius")?,
                get_f(p, "epsilon_solvent")?,
            ),
        )),
        "fold_stability" => Ok(RunOutput::Scalar(bio::biophysics::protein::fold_stability(
            get_f(p, "delta_h")?,
            get_f(p, "delta_s")?,
            get_f(p, "delta_cp")?,
            get_f(p, "t")?,
            get_f(p, "t_ref")?,
        ))),
        "fraction_folded" => Ok(RunOutput::Scalar(
            bio::biophysics::protein::fraction_folded(get_f(p, "delta_g")?, get_f(p, "t")?),
        )),
        "two_state_folding_rate" => Ok(RunOutput::Scalar(
            bio::biophysics::protein::two_state_folding_rate(
                get_f(p, "k0")?,
                get_f(p, "delta_g_dagger")?,
                get_f(p, "t")?,
            ),
        )),
        "zimm_bragg_helix_coil" => Ok(RunOutput::Scalar(
            bio::biophysics::protein::zimm_bragg_helix_coil(
                get_f(p, "s")?,
                get_f(p, "sigma")?,
                get_u(p, "n")?,
            ),
        )),
        "phi_value" => Ok(RunOutput::Scalar(bio::biophysics::protein::phi_value(
            get_f(p, "delta_g_mut_folding")?,
            get_f(p, "delta_g_wt_folding")?,
            get_f(p, "delta_g_mut_ts")?,
            get_f(p, "delta_g_wt_ts")?,
        ))),
        "kauzmann_hydrophobic" => Ok(RunOutput::Scalar(
            bio::biophysics::protein::kauzmann_hydrophobic(
                get_f(p, "delta_cp")?,
                get_f(p, "t")?,
                get_f(p, "t_s")?,
                get_f(p, "t_h")?,
                get_f(p, "delta_h_h")?,
            ),
        )),
        "native_contact_fraction" => Ok(RunOutput::Scalar(
            bio::biophysics::protein::native_contact_fraction(
                get_v(p, "current_distances")?,
                get_v(p, "native_distances")?,
                get_f(p, "cutoff")?,
            ),
        )),
        "denaturation_midpoint" => Ok(RunOutput::Scalar(
            bio::biophysics::protein::denaturation_midpoint(
                get_f(p, "delta_h")?,
                get_f(p, "delta_s")?,
            ),
        )),
        "chevron_plot_folding" => Ok(RunOutput::Scalar(
            bio::biophysics::protein::chevron_plot_folding(
                get_f(p, "k_f_water")?,
                get_f(p, "m_f")?,
                get_f(p, "denaturant")?,
            ),
        )),
        "chevron_plot_unfolding" => Ok(RunOutput::Scalar(
            bio::biophysics::protein::chevron_plot_unfolding(
                get_f(p, "k_u_water")?,
                get_f(p, "m_u")?,
                get_f(p, "denaturant")?,
            ),
        )),
        "optical_trap_force" => Ok(RunOutput::Scalar(
            bio::biophysics::single_molecule::optical_trap_force(
                get_f(p, "laser_power")?,
                get_f(p, "n_medium")?,
                get_f(p, "trap_efficiency")?,
            ),
        )),
        "fret_efficiency" => Ok(RunOutput::Scalar(
            bio::biophysics::single_molecule::fret_efficiency(get_f(p, "r")?, get_f(p, "r0")?),
        )),
        "fret_distance" => Ok(RunOutput::Scalar(
            bio::biophysics::single_molecule::fret_distance(
                get_f(p, "efficiency")?,
                get_f(p, "r0")?,
            ),
        )),
        "fluorescence_lifetime" => Ok(RunOutput::Scalar(
            bio::biophysics::single_molecule::fluorescence_lifetime(
                get_f(p, "quantum_yield")?,
                get_f(p, "radiative_rate")?,
            ),
        )),
        "photobleaching_rate" => Ok(RunOutput::Scalar(
            bio::biophysics::single_molecule::photobleaching_rate(
                get_f(p, "intensity")?,
                get_f(p, "cross_section")?,
                get_f(p, "quantum_yield_bleach")?,
            ),
        )),
        "fluorescence_recovery_half_time" => Ok(RunOutput::Scalar(
            bio::biophysics::single_molecule::fluorescence_recovery_half_time(
                get_f(p, "beam_radius")?,
                get_f(p, "diffusion_coeff")?,
            ),
        )),
        "single_molecule_diffusion_msd" => Ok(RunOutput::Scalar(
            bio::biophysics::single_molecule::single_molecule_diffusion_msd(
                get_f(p, "d")?,
                get_f(p, "t")?,
                get_f(p, "localization_error")?,
            ),
        )),
        "afm_cantilever_force" => Ok(RunOutput::Scalar(
            bio::biophysics::single_molecule::afm_cantilever_force(
                get_f(p, "spring_constant")?,
                get_f(p, "deflection")?,
            ),
        )),
        "hertz_contact_indentation" => Ok(RunOutput::Scalar(
            bio::biophysics::single_molecule::hertz_contact_indentation(
                get_f(p, "force")?,
                get_f(p, "radius")?,
                get_f(p, "youngs_modulus")?,
                get_f(p, "poisson")?,
            ),
        )),
        "micropipette_aspiration_tension" => Ok(RunOutput::Scalar(
            bio::biophysics::single_molecule::micropipette_aspiration_tension(
                get_f(p, "pressure")?,
                get_f(p, "pipette_radius")?,
            ),
        )),
        "youngs_modulus_from_hertz" => Ok(RunOutput::Scalar(
            bio::biophysics::single_molecule::youngs_modulus_from_hertz(
                get_f(p, "force")?,
                get_f(p, "indentation")?,
                get_f(p, "tip_radius")?,
                get_f(p, "poisson")?,
            ),
        )),
        "traction_force" => Ok(RunOutput::Scalar(
            bio::biophysics::single_molecule::traction_force(
                get_f(p, "displacement")?,
                get_f(p, "substrate_stiffness")?,
            ),
        )),
        "contact_order" => {
            let v = get_v(p, "contacts")?;
            let contacts: Vec<(usize, usize)> = v
                .chunks(2)
                .map(|c| (c[0] as usize, c[1] as usize))
                .collect();
            Ok(RunOutput::Scalar(bio::biophysics::protein::contact_order(
                &contacts,
                get_u(p, "chain_length")?,
            )))
        }
        "go_model_energy" => {
            let v = get_v(p, "contacts")?;
            let contacts: Vec<(usize, usize)> = v
                .chunks(2)
                .map(|c| (c[0] as usize, c[1] as usize))
                .collect();
            Ok(RunOutput::Scalar(
                bio::biophysics::protein::go_model_energy(
                    &contacts,
                    get_v(p, "distances")?,
                    get_v(p, "native_distances")?,
                    get_f(p, "epsilon")?,
                ),
            ))
        }
        "radius_of_gyration_3d" => {
            let v = get_v(p, "coords")?;
            let coords: Vec<(f64, f64, f64)> = v.chunks(3).map(|c| (c[0], c[1], c[2])).collect();
            Ok(RunOutput::Scalar(
                bio::biophysics::protein::radius_of_gyration_3d(&coords),
            ))
        }
        "verlet_step" => {
            let mut pos = get_v(p, "positions")?.to_vec();
            let mut vel = get_v(p, "velocities")?.to_vec();
            let forces = get_v(p, "forces")?;
            let masses = get_v(p, "masses")?;
            bio::biophysics::molecular_dynamics::verlet_step(
                &mut pos,
                &mut vel,
                forces,
                masses,
                get_f(p, "dt")?,
            );
            let mut out = pos;
            out.extend(vel);
            Ok(RunOutput::Vector(out))
        }
        "velocity_verlet_step" => {
            let mut pos = get_v(p, "positions")?.to_vec();
            let mut vel = get_v(p, "velocities")?.to_vec();
            let fo = get_v(p, "forces_old")?;
            let fn_ = get_v(p, "forces_new")?;
            let masses = get_v(p, "masses")?;
            bio::biophysics::molecular_dynamics::velocity_verlet_step(
                &mut pos,
                &mut vel,
                fo,
                fn_,
                masses,
                get_f(p, "dt")?,
            );
            let mut out = pos;
            out.extend(vel);
            Ok(RunOutput::Vector(out))
        }
        "berendsen_thermostat" => {
            let mut vel = get_v(p, "velocities")?.to_vec();
            bio::biophysics::molecular_dynamics::berendsen_thermostat(
                &mut vel,
                get_f(p, "current_temp")?,
                get_f(p, "target_temp")?,
                get_f(p, "tau")?,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Vector(vel))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
