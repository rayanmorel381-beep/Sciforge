//! Dispatch handler for neuroscience functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "firing_rate" => {
            let spikes_v = get_v(p, "spikes")?;
            let spikes: Vec<usize> = spikes_v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Scalar(bio::neuroscience::analysis::firing_rate(
                &spikes,
                get_f(p, "dt")?,
                get_u(p, "total_steps")?,
            )))
        }
        "interspike_intervals" => {
            let spikes_v = get_v(p, "spikes")?;
            let spikes: Vec<usize> = spikes_v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Vector(
                bio::neuroscience::analysis::interspike_intervals(&spikes, get_f(p, "dt")?),
            ))
        }
        "coefficient_of_variation" => Ok(RunOutput::Scalar(
            bio::neuroscience::analysis::coefficient_of_variation(get_v(p, "intervals")?),
        )),
        "neuro_fano_factor" => {
            let spike_counts_v = get_v(p, "spike_counts")?;
            let spike_counts: Vec<usize> = spike_counts_v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Scalar(bio::neuroscience::analysis::fano_factor(
                &spike_counts,
            )))
        }
        "spike_count_correlation" => {
            let spikes_a_v = get_v(p, "spikes_a")?;
            let spikes_a: Vec<usize> = spikes_a_v.iter().map(|&x| x as usize).collect();
            let spikes_b_v = get_v(p, "spikes_b")?;
            let spikes_b: Vec<usize> = spikes_b_v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Scalar(
                bio::neuroscience::analysis::spike_count_correlation(&spikes_a, &spikes_b),
            ))
        }
        "cross_correlogram" => Ok(RunOutput::PairVec(
            bio::neuroscience::analysis::cross_correlogram(
                get_v(p, "spikes_a")?,
                get_v(p, "spikes_b")?,
                get_f(p, "bin_width")?,
                get_f(p, "max_lag")?,
            )
            .into_iter()
            .map(|(a, b)| (a, b as f64))
            .collect(),
        )),
        "local_field_potential_power" => Ok(RunOutput::Scalar(
            bio::neuroscience::analysis::local_field_potential_power(
                get_v(p, "signal")?,
                get_f(p, "freq")?,
                get_f(p, "dt")?,
            ),
        )),
        "spike_triggered_average" => {
            let spike_times_v = get_v(p, "spike_times")?;
            let spike_times: Vec<usize> = spike_times_v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Vector(
                bio::neuroscience::analysis::spike_triggered_average(
                    get_v(p, "stimulus")?,
                    &spike_times,
                    get_u(p, "window")?,
                ),
            ))
        }
        "burst_detection" => Ok(RunOutput::PairVec(
            bio::neuroscience::analysis::burst_detection(get_v(p, "isi")?, get_f(p, "threshold")?)
                .into_iter()
                .map(|(a, b)| (a as f64, b as f64))
                .collect(),
        )),
        "information_rate" => {
            let spike_counts_v = get_v(p, "spike_counts")?;
            let spike_counts: Vec<usize> = spike_counts_v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Scalar(
                bio::neuroscience::analysis::information_rate(
                    &spike_counts,
                    get_u(p, "stimulus_repeats")?,
                ),
            ))
        }
        "drift_diffusion_decision" => {
            let (a, b) = bio::neuroscience::cognition::drift_diffusion_decision(
                get_f(p, "drift_rate")?,
                get_f(p, "noise")?,
                get_f(p, "threshold")?,
                get_f(p, "bias")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            );
            Ok(RunOutput::Pair(a, b as f64))
        }
        "softmax_choice" => Ok(RunOutput::Vector(
            bio::neuroscience::cognition::softmax_choice(
                get_v(p, "values")?,
                get_f(p, "temperature")?,
            ),
        )),
        "rescorla_wagner_update" => Ok(RunOutput::Scalar(
            bio::neuroscience::cognition::rescorla_wagner_update(
                get_f(p, "value")?,
                get_f(p, "reward")?,
                get_f(p, "alpha")?,
            ),
        )),
        "td_learning_update" => Ok(RunOutput::Scalar(
            bio::neuroscience::cognition::td_learning_update(
                get_f(p, "value_current")?,
                get_f(p, "value_next")?,
                get_f(p, "reward")?,
                get_f(p, "alpha")?,
                get_f(p, "gamma")?,
            ),
        )),
        "reward_prediction_error" => Ok(RunOutput::Scalar(
            bio::neuroscience::cognition::reward_prediction_error(
                get_f(p, "reward")?,
                get_f(p, "expected")?,
            ),
        )),
        "weber_fraction" => Ok(RunOutput::Scalar(
            bio::neuroscience::cognition::weber_fraction(
                get_f(p, "jnd")?,
                get_f(p, "stimulus_intensity")?,
            ),
        )),
        "signal_to_noise_neural" => Ok(RunOutput::Scalar(
            bio::neuroscience::cognition::signal_to_noise_neural(
                get_f(p, "signal_mean")?,
                get_f(p, "noise_std")?,
            ),
        )),
        "attentional_gain" => Ok(RunOutput::Scalar(
            bio::neuroscience::cognition::attentional_gain(
                get_f(p, "stimulus")?,
                get_f(p, "attention")?,
                get_f(p, "baseline")?,
                get_f(p, "gain")?,
            ),
        )),
        "working_memory_decay" => Ok(RunOutput::Scalar(
            bio::neuroscience::cognition::working_memory_decay(
                get_f(p, "strength")?,
                get_f(p, "decay_rate")?,
                get_u(p, "interference_count")?,
                get_f(p, "dt")?,
            ),
        )),
        "neural_efficiency" => Ok(RunOutput::Scalar(
            bio::neuroscience::cognition::neural_efficiency(
                get_f(p, "performance")?,
                get_f(p, "metabolic_cost")?,
            ),
        )),
        "bayesian_integration" => {
            let (a, b) = bio::neuroscience::cognition::bayesian_integration(
                get_f(p, "prior_mean")?,
                get_f(p, "prior_var")?,
                get_f(p, "likelihood_mean")?,
                get_f(p, "likelihood_var")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "stdp_update" => Ok(RunOutput::Scalar(bio::neuroscience::networks::stdp_update(
            get_f(p, "delta_t")?,
            get_f(p, "a_plus")?,
            get_f(p, "a_minus")?,
            get_f(p, "tau_plus")?,
            get_f(p, "tau_minus")?,
        ))),
        "simulate_network" => Ok(RunOutput::Matrix(
            bio::neuroscience::networks::simulate_network(
                get_u(p, "n_neurons")?,
                get_m(p, "weights")?,
                get_f(p, "external_current")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
                get_f(p, "threshold")?,
                get_f(p, "reset")?,
                get_f(p, "tau")?,
                get_f(p, "resistance")?,
                get_f(p, "rest")?,
            ),
        )),
        "mean_field_rate" => Ok(RunOutput::Scalar(
            bio::neuroscience::networks::mean_field_rate(
                get_f(p, "mu")?,
                get_f(p, "sigma")?,
                get_f(p, "threshold")?,
                get_f(p, "reset")?,
                get_f(p, "tau")?,
            ),
        )),
        "ltp_magnitude" => Ok(RunOutput::Scalar(
            bio::neuroscience::synapses::ltp_magnitude(
                get_f(p, "stimulus_frequency")?,
                get_f(p, "calcium_influx")?,
                get_f(p, "threshold")?,
                get_f(p, "max_potentiation")?,
            ),
        )),
        "ltd_magnitude" => Ok(RunOutput::Scalar(
            bio::neuroscience::synapses::ltd_magnitude(
                get_f(p, "calcium_level")?,
                get_f(p, "low_threshold")?,
                get_f(p, "high_threshold")?,
                get_f(p, "max_depression")?,
            ),
        )),
        "stdp_weight_change" => Ok(RunOutput::Scalar(
            bio::neuroscience::synapses::stdp_weight_change(
                get_f(p, "delta_t_ms")?,
                get_f(p, "a_plus")?,
                get_f(p, "a_minus")?,
                get_f(p, "tau_plus")?,
                get_f(p, "tau_minus")?,
            ),
        )),
        "synaptic_vesicle_release_probability" => Ok(RunOutput::Scalar(
            bio::neuroscience::synapses::synaptic_vesicle_release_probability(
                get_f(p, "calcium")?,
                get_u(p, "n_vesicles")?,
                get_f(p, "p_single")?,
            ),
        )),
        "short_term_facilitation" => Ok(RunOutput::Scalar(
            bio::neuroscience::synapses::short_term_facilitation(
                get_f(p, "baseline_p")?,
                get_f(p, "facilitation")?,
                get_f(p, "tau")?,
                get_f(p, "isi")?,
            ),
        )),
        "short_term_depression" => Ok(RunOutput::Scalar(
            bio::neuroscience::synapses::short_term_depression(
                get_f(p, "available")?,
                get_f(p, "release_p")?,
                get_f(p, "recovery_tau")?,
                get_f(p, "isi")?,
            ),
        )),
        "miniature_epsp_amplitude" => Ok(RunOutput::Scalar(
            bio::neuroscience::synapses::miniature_epsp_amplitude(
                get_f(p, "quantal_size")?,
                get_f(p, "n_receptors")?,
                get_f(p, "receptor_conductance")?,
            ),
        )),
        "nmda_voltage_dependence" => Ok(RunOutput::Scalar(
            bio::neuroscience::synapses::nmda_voltage_dependence(
                get_f(p, "voltage")?,
                get_f(p, "mg_conc")?,
            ),
        )),
        "dendritic_spine_volume_change" => Ok(RunOutput::Scalar(
            bio::neuroscience::synapses::dendritic_spine_volume_change(
                get_f(p, "calcium")?,
                get_f(p, "actin_polymerization")?,
                get_f(p, "spine_volume")?,
                get_f(p, "max_growth")?,
            ),
        )),
        "homeostatic_synaptic_scaling" => Ok(RunOutput::Scalar(
            bio::neuroscience::synapses::homeostatic_synaptic_scaling(
                get_f(p, "target_rate")?,
                get_f(p, "current_rate")?,
                get_f(p, "scaling_tau")?,
                get_f(p, "dt")?,
            ),
        )),
        "glutamate_clearance" => Ok(RunOutput::Scalar(
            bio::neuroscience::synapses::glutamate_clearance(
                get_f(p, "released")?,
                get_f(p, "transporter_density")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
            ),
        )),
        "fano_factor" => {
            let v = get_v(p, "spike_counts")?;
            let counts: Vec<usize> = v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Scalar(bio::neuroscience::analysis::fano_factor(
                &counts,
            )))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
