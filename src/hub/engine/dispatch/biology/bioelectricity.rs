//! Dispatch handler for bioelectricity functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "action_potential_shape" => Ok(RunOutput::Scalar(
            bio::bioelectricity::action_potential::action_potential_shape(
                get_f(p, "t")?,
                get_f(p, "v_rest")?,
                get_f(p, "v_peak")?,
                get_f(p, "tau_rise")?,
                get_f(p, "tau_fall")?,
            ),
        )),
        "cable_equation_steady" => Ok(RunOutput::Scalar(
            bio::bioelectricity::action_potential::cable_equation_steady(
                get_f(p, "v0")?,
                get_f(p, "x")?,
                get_f(p, "lambda")?,
            ),
        )),
        "electrotonic_length" => Ok(RunOutput::Scalar(
            bio::bioelectricity::action_potential::electrotonic_length(
                get_f(p, "physical_length")?,
                get_f(p, "space_constant")?,
            ),
        )),
        "input_resistance_cylinder" => Ok(RunOutput::Scalar(
            bio::bioelectricity::action_potential::input_resistance_cylinder(
                get_f(p, "rm")?,
                get_f(p, "ri")?,
                get_f(p, "diameter")?,
            ),
        )),
        "strength_duration_weiss" => Ok(RunOutput::Scalar(
            bio::bioelectricity::action_potential::strength_duration_weiss(
                get_f(p, "rheobase")?,
                get_f(p, "chronaxie")?,
                get_f(p, "duration")?,
            ),
        )),
        "strength_duration_lapicque" => Ok(RunOutput::Scalar(
            bio::bioelectricity::action_potential::strength_duration_lapicque(
                get_f(p, "rheobase")?,
                get_f(p, "chronaxie")?,
                get_f(p, "duration")?,
            ),
        )),
        "local_field_potential" => Ok(RunOutput::Scalar(
            bio::bioelectricity::action_potential::local_field_potential(
                get_v(p, "currents")?,
                get_v(p, "distances")?,
                get_f(p, "sigma")?,
            ),
        )),
        "extracellular_spike_amplitude" => Ok(RunOutput::Scalar(
            bio::bioelectricity::action_potential::extracellular_spike_amplitude(
                get_f(p, "transmembrane_current")?,
                get_f(p, "distance")?,
                get_f(p, "sigma")?,
            ),
        )),
        "impedance_tissue" => Ok(RunOutput::Scalar(
            bio::bioelectricity::action_potential::impedance_tissue(
                get_f(p, "resistance")?,
                get_f(p, "capacitance")?,
                get_f(p, "frequency")?,
            ),
        )),
        "defibrillation_threshold" => Ok(RunOutput::Scalar(
            bio::bioelectricity::action_potential::defibrillation_threshold(
                get_f(p, "body_mass")?,
                get_f(p, "transthoracic_impedance")?,
            ),
        )),
        "bioimpedance_body_composition" => Ok(RunOutput::Scalar(
            bio::bioelectricity::action_potential::bioimpedance_body_composition(
                get_f(p, "impedance")?,
                get_f(p, "height")?,
                get_f(p, "weight")?,
                get_f(p, "age")?,
                get_f(p, "sex_factor")?,
            ),
        )),
        "coulter_counter_volume" => Ok(RunOutput::Scalar(
            bio::bioelectricity::impedance::coulter_counter_volume(
                get_f(p, "baseline_impedance")?,
                get_f(p, "pulse_amplitude")?,
                get_f(p, "orifice_volume")?,
            ),
        )),
        "dielectrophoresis_force" => Ok(RunOutput::Scalar(
            bio::bioelectricity::impedance::dielectrophoresis_force(
                get_f(p, "radius")?,
                get_f(p, "epsilon_m")?,
                get_f(p, "cm_factor")?,
                get_f(p, "grad_e2")?,
            ),
        )),
        "clausius_mossotti" => Ok(RunOutput::Scalar(
            bio::bioelectricity::impedance::clausius_mossotti(
                get_f(p, "epsilon_p")?,
                get_f(p, "epsilon_m")?,
            ),
        )),
        "electroporation_threshold" => Ok(RunOutput::Scalar(
            bio::bioelectricity::impedance::electroporation_threshold(
                get_f(p, "membrane_thickness")?,
                get_f(p, "breakdown_voltage")?,
            ),
        )),
        "electroporation_pore_density" => Ok(RunOutput::Scalar(
            bio::bioelectricity::impedance::electroporation_pore_density(
                get_f(p, "v_m")?,
                get_f(p, "v_ep")?,
                get_f(p, "n0")?,
                get_f(p, "alpha")?,
            ),
        )),
        "joule_heating" => Ok(RunOutput::Scalar(
            bio::bioelectricity::impedance::joule_heating(
                get_f(p, "current_density")?,
                get_f(p, "conductivity")?,
                get_f(p, "duration")?,
            ),
        )),
        "electrode_double_layer_capacitance" => Ok(RunOutput::Scalar(
            bio::bioelectricity::impedance::electrode_double_layer_capacitance(
                get_f(p, "epsilon")?,
                get_f(p, "debye_length")?,
            ),
        )),
        "iontophoresis_flux" => Ok(RunOutput::Scalar(
            bio::bioelectricity::impedance::iontophoresis_flux(
                get_f(p, "current")?,
                get_f(p, "z")?,
                get_f(p, "transport_number")?,
            ),
        )),
        "skin_impedance_model" => Ok(RunOutput::Scalar(
            bio::bioelectricity::impedance::skin_impedance_model(
                get_f(p, "r_stratum")?,
                get_f(p, "c_stratum")?,
                get_f(p, "r_deep")?,
                get_f(p, "frequency")?,
            ),
        )),
        "eeg_dipole_potential" => Ok(RunOutput::Scalar(
            bio::bioelectricity::impedance::eeg_dipole_potential(
                get_f(p, "dipole_moment")?,
                get_f(p, "cos_angle")?,
                get_f(p, "distance")?,
                get_f(p, "conductivity")?,
            ),
        )),
        "nerve_conduction_velocity_temperature" => Ok(RunOutput::Scalar(
            bio::bioelectricity::impedance::nerve_conduction_velocity_temperature(
                get_f(p, "v_ref")?,
                get_f(p, "q10")?,
                get_f(p, "t")?,
                get_f(p, "t_ref")?,
            ),
        )),
        "bioelec_nernst_potential" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::nernst_potential(
                get_f(p, "z")?,
                get_f(p, "temperature")?,
                get_f(p, "concentration_out")?,
                get_f(p, "concentration_in")?,
            ),
        )),
        "goldman_hodgkin_katz" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::goldman_hodgkin_katz(
                get_f(p, "pk")?,
                get_f(p, "k_out")?,
                get_f(p, "k_in")?,
                get_f(p, "pna")?,
                get_f(p, "na_out")?,
                get_f(p, "na_in")?,
                get_f(p, "pcl")?,
                get_f(p, "cl_out")?,
                get_f(p, "cl_in")?,
                get_f(p, "temperature")?,
            ),
        )),
        "cable_equation_steady_state" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::cable_equation_steady_state(
                get_f(p, "v0")?,
                get_f(p, "x")?,
                get_f(p, "lambda")?,
            ),
        )),
        "membrane_time_constant" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::membrane_time_constant(get_f(p, "rm")?, get_f(p, "cm")?),
        )),
        "length_constant" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::length_constant(get_f(p, "rm")?, get_f(p, "ri")?),
        )),
        "gap_junction_conductance" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::gap_junction_conductance(
                get_f(p, "n_channels")?,
                get_f(p, "single_channel_conductance")?,
                get_f(p, "open_probability")?,
            ),
        )),
        "electrodiffusion_flux" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::electrodiffusion_flux(
                get_f(p, "permeability")?,
                get_f(p, "z")?,
                get_f(p, "vm")?,
                get_f(p, "c_out")?,
                get_f(p, "c_in")?,
                get_f(p, "temperature")?,
            ),
        )),
        "hodgkin_huxley_sodium_current" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::hodgkin_huxley_sodium_current(
                get_f(p, "g_na")?,
                get_f(p, "m")?,
                get_f(p, "h")?,
                get_f(p, "v")?,
                get_f(p, "e_na")?,
            ),
        )),
        "hodgkin_huxley_potassium_current" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::hodgkin_huxley_potassium_current(
                get_f(p, "g_k")?,
                get_f(p, "n")?,
                get_f(p, "v")?,
                get_f(p, "e_k")?,
            ),
        )),
        "hodgkin_huxley_leak_current" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::hodgkin_huxley_leak_current(
                get_f(p, "g_l")?,
                get_f(p, "v")?,
                get_f(p, "e_l")?,
            ),
        )),
        "hodgkin_huxley_dv_dt" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::hodgkin_huxley_dv_dt(
                get_f(p, "cm")?,
                get_f(p, "i_ext")?,
                get_f(p, "i_na")?,
                get_f(p, "i_k")?,
                get_f(p, "i_l")?,
            ),
        )),
        "gating_alpha_n" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::gating_alpha_n(get_f(p, "v")?),
        )),
        "gating_beta_n" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::gating_beta_n(get_f(p, "v")?),
        )),
        "gating_alpha_m" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::gating_alpha_m(get_f(p, "v")?),
        )),
        "gating_beta_m" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::gating_beta_m(get_f(p, "v")?),
        )),
        "gating_alpha_h" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::gating_alpha_h(get_f(p, "v")?),
        )),
        "gating_beta_h" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::gating_beta_h(get_f(p, "v")?),
        )),
        "gating_steady_state" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::gating_steady_state(
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
            ),
        )),
        "gating_time_constant" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::gating_time_constant(
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
            ),
        )),
        "reversal_potential_two_ion" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::reversal_potential_two_ion(
                get_f(p, "g1")?,
                get_f(p, "e1")?,
                get_f(p, "g2")?,
                get_f(p, "e2")?,
            ),
        )),
        "bioelec_membrane_capacitance_current" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::membrane_capacitance_current(
                get_f(p, "cm")?,
                get_f(p, "dv_dt")?,
            ),
        )),
        "ion_channel_open_probability" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::ion_channel_open_probability(
                get_f(p, "v")?,
                get_f(p, "v_half")?,
                get_f(p, "slope")?,
            ),
        )),
        "synaptic_conductance_alpha" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::synaptic_conductance_alpha(
                get_f(p, "g_max")?,
                get_f(p, "t")?,
                get_f(p, "tau")?,
            ),
        )),
        "synaptic_current" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::synaptic_current(
                get_f(p, "g_syn")?,
                get_f(p, "v_post")?,
                get_f(p, "e_syn")?,
            ),
        )),
        "calcium_nernst" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::calcium_nernst(
                get_f(p, "temperature")?,
                get_f(p, "ca_out")?,
                get_f(p, "ca_in")?,
            ),
        )),
        "chloride_equilibrium" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::chloride_equilibrium(
                get_f(p, "temperature")?,
                get_f(p, "cl_out")?,
                get_f(p, "cl_in")?,
            ),
        )),
        "resting_potential_contribution" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::resting_potential_contribution(
                get_f(p, "conductance")?,
                get_f(p, "reversal")?,
                get_f(p, "total_conductance")?,
            ),
        )),
        "space_clamp_error" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::space_clamp_error(
                get_f(p, "distance")?,
                get_f(p, "lambda")?,
            ),
        )),
        "action_potential_threshold_estimate" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::action_potential_threshold_estimate(
                get_f(p, "v_rest")?,
                get_f(p, "depolarization")?,
            ),
        )),
        "conduction_velocity" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::conduction_velocity(
                get_f(p, "diameter")?,
                get_b(p, "myelinated")?,
            ),
        )),
        "saltatory_conduction_delay" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::saltatory_conduction_delay(
                get_f(p, "internode_distance")?,
                get_f(p, "velocity")?,
            ),
        )),
        "membrane_resistance_per_area" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::membrane_resistance_per_area(
                get_f(p, "resistivity")?,
                get_f(p, "thickness")?,
            ),
        )),
        "specific_membrane_capacitance" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::specific_membrane_capacitance(
                get_f(p, "epsilon_r")?,
                get_f(p, "thickness")?,
            ),
        )),
        "defibrillation_energy" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::defibrillation_energy(
                get_f(p, "capacitance")?,
                get_f(p, "voltage")?,
            ),
        )),
        "electrode_impedance" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::electrode_impedance(
                get_f(p, "resistance")?,
                get_f(p, "capacitance")?,
                get_f(p, "frequency")?,
            ),
        )),
        "stimulation_strength_duration" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::stimulation_strength_duration(
                get_f(p, "rheobase")?,
                get_f(p, "chronaxie")?,
                get_f(p, "pulse_width")?,
            ),
        )),
        "bioimpedance_cole_model" => {
            let (a, b) = bio::bioelectricity::stimulation::bioimpedance_cole_model(
                get_f(p, "r_inf")?,
                get_f(p, "r_0")?,
                get_f(p, "tau")?,
                get_f(p, "alpha")?,
                get_f(p, "frequency")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "transcranial_current_density" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::transcranial_current_density(
                get_f(p, "current")?,
                get_f(p, "electrode_area")?,
            ),
        )),
        "neural_recruitment_curve" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::neural_recruitment_curve(
                get_f(p, "stimulus")?,
                get_f(p, "threshold")?,
                get_f(p, "saturation")?,
                get_f(p, "steepness")?,
            ),
        )),
        "charge_density" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::charge_density(
                get_f(p, "charge")?,
                get_f(p, "electrode_area")?,
            ),
        )),
        "cathodic_charge_balanced" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::cathodic_charge_balanced(
                get_f(p, "anodic_amplitude")?,
                get_f(p, "anodic_duration")?,
                get_f(p, "cathodic_duration")?,
            ),
        )),
        "pulse_train_energy" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::pulse_train_energy(
                get_f(p, "amplitude")?,
                get_f(p, "pulse_width")?,
                get_f(p, "frequency")?,
                get_f(p, "duration")?,
                get_f(p, "impedance")?,
            ),
        )),
        "tissue_heating" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::tissue_heating(
                get_f(p, "current_density")?,
                get_f(p, "conductivity")?,
                get_f(p, "duration")?,
                get_f(p, "specific_heat")?,
                get_f(p, "density")?,
            ),
        )),
        "tms_induced_efield" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::tms_induced_efield(
                get_f(p, "di_dt")?,
                get_f(p, "coil_inductance")?,
                get_f(p, "distance")?,
            ),
        )),
        "dbs_volume_tissue_activated" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::dbs_volume_tissue_activated(
                get_f(p, "current")?,
                get_f(p, "impedance")?,
                get_f(p, "threshold_efield")?,
            ),
        )),
        "cochlear_implant_spread" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::cochlear_implant_spread(
                get_f(p, "current")?,
                get_f(p, "distance")?,
                get_f(p, "sigma")?,
            ),
        )),
        "fes_fatigue_index" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::fes_fatigue_index(
                get_f(p, "initial_force")?,
                get_f(p, "final_force")?,
            ),
        )),
        "shannon_safety_limit" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::shannon_safety_limit(
                get_f(p, "charge_per_phase_uc")?,
                get_f(p, "electrode_area_cm2")?,
            ),
        )),
        "biphasic_pulse_charge" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::biphasic_pulse_charge(
                get_f(p, "amplitude")?,
                get_f(p, "phase_duration")?,
            ),
        )),
        "interphase_gap_effect" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::interphase_gap_effect(
                get_f(p, "threshold_no_gap")?,
                get_f(p, "gap_duration")?,
                get_f(p, "time_constant")?,
            ),
        )),
        "electrochemical_safety_margin" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::electrochemical_safety_margin(
                get_f(p, "water_window")?,
                get_f(p, "electrode_potential")?,
            ),
        )),
        "warburg_impedance" => {
            let (a, b) = bio::bioelectricity::stimulation::warburg_impedance(
                get_f(p, "sigma")?,
                get_f(p, "frequency")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "constant_phase_element" => {
            let (a, b) = bio::bioelectricity::stimulation::constant_phase_element(
                get_f(p, "q")?,
                get_f(p, "alpha")?,
                get_f(p, "frequency")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "chronaxie_from_strength_duration" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::chronaxie_from_strength_duration(
                get_f(p, "rheobase")?,
                get_f(p, "threshold_at_pw")?,
                get_f(p, "pulse_width")?,
            ),
        )),
        "galvanic_skin_response" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::galvanic_skin_response(
                get_f(p, "baseline_conductance")?,
                get_f(p, "peak_conductance")?,
                get_f(p, "t")?,
                get_f(p, "tau_rise")?,
                get_f(p, "tau_decay")?,
            ),
        )),
        "total_charge_delivered" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::total_charge_delivered(
                get_f(p, "amplitude")?,
                get_f(p, "pulse_width")?,
                get_f(p, "frequency")?,
                get_f(p, "duration")?,
            ),
        )),
        "electrode_polarization_voltage" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::electrode_polarization_voltage(
                get_f(p, "charge")?,
                get_f(p, "capacitance")?,
            ),
        )),
        "anodal_break_excitation_threshold" => Ok(RunOutput::Scalar(
            bio::bioelectricity::stimulation::anodal_break_excitation_threshold(
                get_f(p, "membrane_tau")?,
                get_f(p, "pulse_duration")?,
                get_f(p, "rheobase")?,
            ),
        )),
        "ecg_lead_vector" => Ok(RunOutput::Scalar(
            bio::bioelectricity::impedance::ecg_lead_vector(
                (get_f(p, "dx")?, get_f(p, "dy")?, get_f(p, "dz")?),
                (get_f(p, "lx")?, get_f(p, "ly")?, get_f(p, "lz")?),
            ),
        )),
        "membrane_capacitance_current" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::membrane_capacitance_current(
                get_f(p, "cm")?,
                get_f(p, "dv_dt")?,
            ),
        )),
        "nernst_potential" => Ok(RunOutput::Scalar(
            bio::bioelectricity::membrane::nernst_potential(
                get_f(p, "z")?,
                get_f(p, "temperature")?,
                get_f(p, "concentration_out")?,
                get_f(p, "concentration_in")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
