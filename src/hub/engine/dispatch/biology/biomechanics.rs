//! Dispatch handler for biomechanics functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "biomech_poiseuille_flow" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::poiseuille_flow(
                get_f(p, "delta_p")?,
                get_f(p, "radius")?,
                get_f(p, "length")?,
                get_f(p, "viscosity")?,
            ),
        )),
        "biomech_wall_shear_stress" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::wall_shear_stress(
                get_f(p, "flow_rate")?,
                get_f(p, "radius")?,
                get_f(p, "viscosity")?,
            ),
        )),
        "reynolds_number" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::reynolds_number(
                get_f(p, "density")?,
                get_f(p, "velocity")?,
                get_f(p, "diameter")?,
                get_f(p, "viscosity")?,
            ),
        )),
        "murrays_law_radius" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::murrays_law_radius(
                get_f(p, "parent_radius")?,
                get_u(p, "n_children")?,
            ),
        )),
        "biomech_pulse_wave_velocity" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::pulse_wave_velocity(
                get_f(p, "elastic_modulus")?,
                get_f(p, "wall_thickness")?,
                get_f(p, "radius")?,
                get_f(p, "density")?,
            ),
        )),
        "casson_viscosity" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::casson_viscosity(
                get_f(p, "tau_y")?,
                get_f(p, "eta_inf")?,
                get_f(p, "shear_rate")?,
            ),
        )),
        "oxygen_dissociation_hill" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::oxygen_dissociation_hill(
                get_f(p, "po2")?,
                get_f(p, "p50")?,
                get_f(p, "n")?,
            ),
        )),
        "biomech_cardiac_output" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::cardiac_output(
                get_f(p, "stroke_volume")?,
                get_f(p, "heart_rate")?,
            ),
        )),
        "biomech_mean_arterial_pressure" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::mean_arterial_pressure(
                get_f(p, "systolic")?,
                get_f(p, "diastolic")?,
            ),
        )),
        "biomech_total_peripheral_resistance" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::total_peripheral_resistance(
                get_f(p, "map")?,
                get_f(p, "cvp")?,
                get_f(p, "cardiac_output")?,
            ),
        )),
        "womersley_number" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::womersley_number(
                get_f(p, "radius")?,
                get_f(p, "angular_freq")?,
                get_f(p, "kinematic_viscosity")?,
            ),
        )),
        "fahraeus_lindqvist" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::fahraeus_lindqvist(
                get_f(p, "viscosity_plasma")?,
                get_f(p, "hematocrit")?,
                get_f(p, "diameter_um")?,
            ),
        )),
        "compliance" => Ok(RunOutput::Scalar(bio::biomechanics::fluids::compliance(
            get_f(p, "delta_v")?,
            get_f(p, "delta_p")?,
        ))),
        "laplace_law_sphere" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::laplace_law_sphere(
                get_f(p, "pressure")?,
                get_f(p, "radius")?,
                get_f(p, "wall_thickness")?,
            ),
        )),
        "laplace_law_cylinder" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::laplace_law_cylinder(
                get_f(p, "pressure")?,
                get_f(p, "radius")?,
                get_f(p, "wall_thickness")?,
            ),
        )),
        "bernoulli_velocity" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::bernoulli_velocity(
                get_f(p, "delta_p")?,
                get_f(p, "density")?,
            ),
        )),
        "biomech_systemic_vascular_resistance" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::systemic_vascular_resistance(
                get_f(p, "map")?,
                get_f(p, "rap")?,
                get_f(p, "co")?,
            ),
        )),
        "biomech_ejection_fraction" => Ok(RunOutput::Scalar(
            bio::biomechanics::fluids::ejection_fraction(get_f(p, "edv")?, get_f(p, "esv")?),
        )),
        "gait_stride_length" => Ok(RunOutput::Scalar(
            bio::biomechanics::locomotion::gait_stride_length(
                get_f(p, "velocity")?,
                get_f(p, "cadence")?,
            ),
        )),
        "ground_reaction_force" => Ok(RunOutput::Scalar(
            bio::biomechanics::locomotion::ground_reaction_force(
                get_f(p, "mass")?,
                get_f(p, "acceleration")?,
            ),
        )),
        "joint_moment" => Ok(RunOutput::Scalar(
            bio::biomechanics::locomotion::joint_moment(
                get_f(p, "force")?,
                get_f(p, "moment_arm")?,
            ),
        )),
        "joint_power" => Ok(RunOutput::Scalar(
            bio::biomechanics::locomotion::joint_power(
                get_f(p, "moment")?,
                get_f(p, "angular_velocity")?,
            ),
        )),
        "inverse_dynamics_moment" => Ok(RunOutput::Scalar(
            bio::biomechanics::locomotion::inverse_dynamics_moment(
                get_f(p, "i_segment")?,
                get_f(p, "alpha")?,
                get_f(p, "proximal_force")?,
                get_f(p, "proximal_arm")?,
                get_f(p, "distal_force")?,
                get_f(p, "distal_arm")?,
            ),
        )),
        "metabolic_cost_of_transport" => Ok(RunOutput::Scalar(
            bio::biomechanics::locomotion::metabolic_cost_of_transport(
                get_f(p, "metabolic_rate")?,
                get_f(p, "mass")?,
                get_f(p, "velocity")?,
            ),
        )),
        "froude_number" => Ok(RunOutput::Scalar(
            bio::biomechanics::locomotion::froude_number(
                get_f(p, "velocity")?,
                get_f(p, "leg_length")?,
            ),
        )),
        "work_loop_area" => Ok(RunOutput::Scalar(
            bio::biomechanics::locomotion::work_loop_area(get_v(p, "force")?, get_v(p, "length")?),
        )),
        "pendulum_energy_recovery" => Ok(RunOutput::Scalar(
            bio::biomechanics::locomotion::pendulum_energy_recovery(
                get_f(p, "ek_change")?,
                get_f(p, "ep_change")?,
            ),
        )),
        "cross_bridge_huxley" => Ok(RunOutput::Scalar(
            bio::biomechanics::muscle::cross_bridge_huxley(
                get_f(p, "x")?,
                get_f(p, "f_rate")?,
                get_f(p, "g_rate")?,
                get_f(p, "dt")?,
                get_f(p, "n")?,
            ),
        )),
        "pennation_angle_force" => Ok(RunOutput::Scalar(
            bio::biomechanics::muscle::pennation_angle_force(
                get_f(p, "f_tendon")?,
                get_f(p, "angle_rad")?,
            ),
        )),
        "joint_torque" => Ok(RunOutput::Scalar(bio::biomechanics::muscle::joint_torque(
            get_f(p, "force")?,
            get_f(p, "moment_arm")?,
        ))),
        "angular_impulse" => Ok(RunOutput::Scalar(
            bio::biomechanics::muscle::angular_impulse(get_f(p, "torque")?, get_f(p, "dt")?),
        )),
        "muscle_power" => Ok(RunOutput::Scalar(bio::biomechanics::muscle::muscle_power(
            get_f(p, "force")?,
            get_f(p, "velocity")?,
        ))),
        "work" => Ok(RunOutput::Scalar(bio::biomechanics::muscle::work(
            get_f(p, "force")?,
            get_f(p, "displacement")?,
        ))),
        "tendon_force" => Ok(RunOutput::Scalar(bio::biomechanics::muscle::tendon_force(
            get_f(p, "stiffness")?,
            get_f(p, "strain")?,
        ))),
        "excitation_contraction_coupling" => Ok(RunOutput::Scalar(
            bio::biomechanics::muscle::excitation_contraction_coupling(
                get_f(p, "calcium")?,
                get_f(p, "k_half")?,
                get_f(p, "n")?,
            ),
        )),
        "fatigue_model" => Ok(RunOutput::Scalar(bio::biomechanics::muscle::fatigue_model(
            get_f(p, "force_max")?,
            get_f(p, "time")?,
            get_f(p, "fatigue_rate")?,
        ))),
        "muscle_stiffness" => Ok(RunOutput::Scalar(
            bio::biomechanics::muscle::muscle_stiffness(
                get_f(p, "force")?,
                get_f(p, "length")?,
                get_f(p, "l_opt")?,
            ),
        )),
        "isometric_twitch" => Ok(RunOutput::Scalar(
            bio::biomechanics::muscle::isometric_twitch(
                get_f(p, "f_max")?,
                get_f(p, "t")?,
                get_f(p, "tp")?,
            ),
        )),
        "tetanus_fusion" => Ok(RunOutput::Scalar(
            bio::biomechanics::muscle::tetanus_fusion(
                get_f(p, "f_twitch")?,
                get_f(p, "frequency")?,
                get_f(p, "fusion_freq")?,
            ),
        )),
        "muscle_metabolic_rate" => Ok(RunOutput::Scalar(
            bio::biomechanics::muscle::muscle_metabolic_rate(
                get_f(p, "force")?,
                get_f(p, "velocity")?,
                get_f(p, "activation")?,
                get_f(p, "basal")?,
            ),
        )),
        "fiber_type_recruitment" => {
            let (a, b) = bio::biomechanics::muscle::fiber_type_recruitment(
                get_f(p, "excitation")?,
                get_f(p, "threshold_slow")?,
                get_f(p, "threshold_fast")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "sarcomere_force_length" => Ok(RunOutput::Scalar(
            bio::biomechanics::muscle::sarcomere_force_length(get_f(p, "sl")?),
        )),
        "linear_elastic_stress" => Ok(RunOutput::Scalar(
            bio::biomechanics::tissue::linear_elastic_stress(
                get_f(p, "modulus")?,
                get_f(p, "strain")?,
            ),
        )),
        "kelvin_voigt" => Ok(RunOutput::Scalar(bio::biomechanics::tissue::kelvin_voigt(
            get_f(p, "modulus")?,
            get_f(p, "viscosity")?,
            get_f(p, "strain")?,
            get_f(p, "strain_rate")?,
        ))),
        "maxwell_stress_relaxation" => Ok(RunOutput::Scalar(
            bio::biomechanics::tissue::maxwell_stress_relaxation(
                get_f(p, "sigma0")?,
                get_f(p, "modulus")?,
                get_f(p, "viscosity")?,
                get_f(p, "t")?,
            ),
        )),
        "standard_linear_solid" => Ok(RunOutput::Scalar(
            bio::biomechanics::tissue::standard_linear_solid(
                get_f(p, "e1")?,
                get_f(p, "e2")?,
                get_f(p, "eta")?,
                get_f(p, "strain")?,
                get_f(p, "strain_rate")?,
                get_f(p, "stress")?,
            ),
        )),
        "hyperelastic_neo_hookean" => Ok(RunOutput::Scalar(
            bio::biomechanics::tissue::hyperelastic_neo_hookean(
                get_f(p, "c1")?,
                get_f(p, "lambda")?,
            ),
        )),
        "mooney_rivlin" => Ok(RunOutput::Scalar(bio::biomechanics::tissue::mooney_rivlin(
            get_f(p, "c1")?,
            get_f(p, "c2")?,
            get_f(p, "lambda")?,
        ))),
        "poroelastic_consolidation" => Ok(RunOutput::Scalar(
            bio::biomechanics::tissue::poroelastic_consolidation(
                get_f(p, "stress")?,
                get_f(p, "modulus")?,
                get_f(p, "permeability")?,
                get_f(p, "viscosity")?,
                get_f(p, "thickness")?,
                get_f(p, "t")?,
            ),
        )),
        "strain_energy_density_linear" => Ok(RunOutput::Scalar(
            bio::biomechanics::tissue::strain_energy_density_linear(
                get_f(p, "modulus")?,
                get_f(p, "strain")?,
            ),
        )),
        "creep_power_law" => Ok(RunOutput::Scalar(
            bio::biomechanics::tissue::creep_power_law(
                get_f(p, "a")?,
                get_f(p, "sigma")?,
                get_f(p, "n")?,
                get_f(p, "t")?,
            ),
        )),
        "bone_density_wolff" => Ok(RunOutput::Scalar(
            bio::biomechanics::tissue::bone_density_wolff(
                get_f(p, "rho0")?,
                get_f(p, "stimulus")?,
                get_f(p, "reference_stimulus")?,
                get_f(p, "rate")?,
                get_f(p, "dt")?,
            ),
        )),
        "ogden_model" => Ok(RunOutput::Scalar(bio::biomechanics::tissue::ogden_model(
            get_f(p, "mu")?,
            get_f(p, "alpha")?,
            get_f(p, "lambda")?,
        ))),
        "fracture_toughness" => Ok(RunOutput::Scalar(
            bio::biomechanics::tissue::fracture_toughness(
                get_f(p, "force")?,
                get_f(p, "crack_length")?,
                get_f(p, "width")?,
                get_f(p, "thickness")?,
            ),
        )),
        "viscoelastic_prony" => Ok(RunOutput::Scalar(
            bio::biomechanics::tissue::viscoelastic_prony(
                get_v(p, "moduli")?,
                get_v(p, "taus")?,
                get_f(p, "t")?,
            ),
        )),
        "tissue_hydration_swelling" => Ok(RunOutput::Scalar(
            bio::biomechanics::tissue::tissue_hydration_swelling(
                get_f(p, "phi_0")?,
                get_f(p, "pi_ext")?,
                get_f(p, "bulk_modulus")?,
            ),
        )),
        "biphasic_permeability" => Ok(RunOutput::Scalar(
            bio::biomechanics::tissue::biphasic_permeability(
                get_f(p, "k0")?,
                get_f(p, "strain")?,
                get_f(p, "m")?,
            ),
        )),
        "stress_fiber_remodeling" => Ok(RunOutput::Scalar(
            bio::biomechanics::tissue::stress_fiber_remodeling(
                get_f(p, "sigma_old")?,
                get_f(p, "reference")?,
                get_f(p, "rate")?,
                get_f(p, "dt")?,
            ),
        )),
        "damage_accumulation" => Ok(RunOutput::Scalar(
            bio::biomechanics::tissue::damage_accumulation(
                get_f(p, "d")?,
                get_f(p, "stress")?,
                get_f(p, "threshold")?,
                get_f(p, "rate")?,
                get_f(p, "dt")?,
            ),
        )),
        "elastic_modulus_density" => Ok(RunOutput::Scalar(
            bio::biomechanics::tissue::elastic_modulus_density(
                get_f(p, "rho")?,
                get_f(p, "c")?,
                get_f(p, "exponent")?,
            ),
        )),
        "center_of_pressure" => {
            let fv = get_v(p, "forces")?;
            let pv = get_v(p, "positions")?;
            let forces: Vec<(f64, f64, f64)> = fv.chunks(3).map(|c| (c[0], c[1], c[2])).collect();
            let positions: Vec<(f64, f64)> = pv.chunks(2).map(|c| (c[0], c[1])).collect();
            let (x, y) = bio::biomechanics::locomotion::center_of_pressure(&forces, &positions);
            Ok(RunOutput::Pair(x, y))
        }
        "dynamic_stability_margin" => {
            let v = get_v(p, "base_of_support")?;
            let base: Vec<(f64, f64)> = v.chunks(2).map(|c| (c[0], c[1])).collect();
            Ok(RunOutput::Scalar(
                bio::biomechanics::locomotion::dynamic_stability_margin(
                    &base,
                    (get_f(p, "com_x")?, get_f(p, "com_y")?),
                ),
            ))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
