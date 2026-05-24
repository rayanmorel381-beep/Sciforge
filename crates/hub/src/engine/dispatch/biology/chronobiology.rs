//! Dispatch handler for chronobiology functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "zeitgeber_strength" => Ok(RunOutput::Scalar(
            bio::chronobiology::entrainment::zeitgeber_strength(
                get_f(p, "light_intensity")?,
                get_f(p, "threshold")?,
                get_f(p, "saturation")?,
            ),
        )),
        "phase_response_curve" => Ok(RunOutput::Scalar(
            bio::chronobiology::entrainment::phase_response_curve(
                get_f(p, "phase")?,
                get_f(p, "light_pulse_phase")?,
                get_f(p, "sensitivity")?,
            ),
        )),
        "jet_lag_recovery" => Ok(RunOutput::Scalar(
            bio::chronobiology::entrainment::jet_lag_recovery(
                get_f(p, "timezone_shift")?,
                get_f(p, "adaptation_rate")?,
                get_f(p, "days")?,
            ),
        )),
        "shift_work_desynchrony" => Ok(RunOutput::Scalar(
            bio::chronobiology::entrainment::shift_work_desynchrony(
                get_f(p, "internal_phase")?,
                get_f(p, "external_phase")?,
            ),
        )),
        "seasonal_photoperiod" => Ok(RunOutput::Scalar(
            bio::chronobiology::entrainment::seasonal_photoperiod(
                get_u(p, "day_of_year")?,
                get_f(p, "latitude")?,
            ),
        )),
        "melatonin_suppression" => Ok(RunOutput::Scalar(
            bio::chronobiology::entrainment::melatonin_suppression(
                get_f(p, "light_intensity")?,
                get_f(p, "ic50")?,
                get_f(p, "hill_n")?,
            ),
        )),
        "social_zeitgeber_strength" => Ok(RunOutput::Scalar(
            bio::chronobiology::entrainment::social_zeitgeber_strength(
                get_f(p, "regularity")?,
                get_f(p, "social_contacts")?,
            ),
        )),
        "food_entrainment" => Ok(RunOutput::Scalar(
            bio::chronobiology::entrainment::food_entrainment(
                get_f(p, "feeding_time")?,
                get_f(p, "clock_phase")?,
                get_f(p, "coupling")?,
            ),
        )),
        "chronotype_score" => Ok(RunOutput::Scalar(
            bio::chronobiology::entrainment::chronotype_score(get_f(p, "midpoint_sleep")?),
        )),
        "circadian_amplitude_damping" => Ok(RunOutput::Scalar(
            bio::chronobiology::entrainment::circadian_amplitude_damping(
                get_f(p, "initial_amplitude")?,
                get_f(p, "damping_rate")?,
                get_f(p, "t")?,
            ),
        )),
        "goodwin_oscillator" => {
            let (a, b, c) = bio::chronobiology::oscillators::goodwin_oscillator(
                get_f(p, "x")?,
                get_f(p, "y")?,
                get_f(p, "z")?,
                get_f(p, "k1")?,
                get_f(p, "k2")?,
                get_f(p, "k3")?,
                get_f(p, "ki")?,
                get_f(p, "n")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "van_der_pol_circadian" => {
            let (a, b) = bio::chronobiology::oscillators::van_der_pol_circadian(
                get_f(p, "x")?,
                get_f(p, "y")?,
                get_f(p, "mu")?,
                get_f(p, "tau")?,
                get_f(p, "light")?,
                get_f(p, "alpha")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "phase_response" => Ok(RunOutput::Scalar(
            bio::chronobiology::oscillators::phase_response(
                get_f(p, "phase")?,
                get_f(p, "light_intensity")?,
                get_f(p, "sensitivity")?,
                get_f(p, "tau")?,
            ),
        )),
        "entrainment_range" => {
            let (a, b) = bio::chronobiology::oscillators::entrainment_range(
                get_f(p, "coupling_strength")?,
                get_f(p, "intrinsic_period")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "melatonin_profile" => Ok(RunOutput::Scalar(
            bio::chronobiology::oscillators::melatonin_profile(
                get_f(p, "t")?,
                get_f(p, "onset")?,
                get_f(p, "offset")?,
                get_f(p, "amplitude")?,
            ),
        )),
        "desynchrony_index" => Ok(RunOutput::Scalar(
            bio::chronobiology::oscillators::desynchrony_index(
                get_f(p, "observed_period")?,
                get_f(p, "zeitgeber_period")?,
            ),
        )),
        "goodwin_simulate" => Ok(RunOutput::Matrix(
            bio::chronobiology::oscillators::goodwin_simulate(
                get_f(p, "x0")?,
                get_f(p, "y0")?,
                get_f(p, "z0")?,
                get_f(p, "k1")?,
                get_f(p, "k2")?,
                get_f(p, "k3")?,
                get_f(p, "ki")?,
                get_f(p, "n")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            )
            .into_iter()
            .map(|(a, b, c)| vec![a, b, c])
            .collect(),
        )),
        "kuramoto_order_parameter" => {
            let (a, b) =
                bio::chronobiology::oscillators::kuramoto_order_parameter(get_v(p, "phases")?);
            Ok(RunOutput::Pair(a, b))
        }
        "arnolds_tongue_boundary" => Ok(RunOutput::Boolean(
            bio::chronobiology::oscillators::arnolds_tongue_boundary(
                get_f(p, "coupling")?,
                get_f(p, "detuning")?,
            ),
        )),
        "chrono_repressilator" => {
            let (a, b, c) = bio::chronobiology::oscillators::repressilator(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "c")?,
                get_f(p, "alpha")?,
                get_f(p, "alpha0")?,
                get_f(p, "n")?,
                get_f(p, "beta")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "amplitude_phase_from_timeseries" => {
            let (a, b) = bio::chronobiology::oscillators::amplitude_phase_from_timeseries(
                get_v(p, "values")?,
                get_f(p, "period")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "phase_diffusion_coefficient" => Ok(RunOutput::Scalar(
            bio::chronobiology::oscillators::phase_diffusion_coefficient(
                get_f(p, "phase_variance")?,
                get_f(p, "time")?,
            ),
        )),
        "limit_cycle_stability" => Ok(RunOutput::Boolean(
            bio::chronobiology::oscillators::limit_cycle_stability(get_f(p, "floquet_exponent")?),
        )),
        "poincare_section_period" => Ok(RunOutput::Scalar(
            bio::chronobiology::oscillators::poincare_section_period(get_v(p, "crossing_times")?),
        )),
        "detrend_moving_average" => Ok(RunOutput::Vector(
            bio::chronobiology::oscillators::detrend_moving_average(
                get_v(p, "data")?,
                get_u(p, "window")?,
            ),
        )),
        "instantaneous_frequency" => Ok(RunOutput::Scalar(
            bio::chronobiology::oscillators::instantaneous_frequency(
                get_f(p, "phase_prev")?,
                get_f(p, "phase_curr")?,
                get_f(p, "dt")?,
            ),
        )),
        "mutual_information_phase" => Ok(RunOutput::Scalar(
            bio::chronobiology::oscillators::mutual_information_phase(
                get_v(p, "phases1")?,
                get_v(p, "phases2")?,
                get_u(p, "n_bins")?,
            ),
        )),
        "stochastic_resonance_snr" => Ok(RunOutput::Scalar(
            bio::chronobiology::oscillators::stochastic_resonance_snr(
                get_f(p, "signal_power")?,
                get_f(p, "noise_intensity")?,
                get_f(p, "threshold")?,
            ),
        )),
        "jet_lag_resync_time" => Ok(RunOutput::Scalar(
            bio::chronobiology::rhythms::jet_lag_resync_time(
                get_f(p, "time_zones_crossed")?,
                get_f(p, "resync_rate")?,
            ),
        )),
        "sleep_pressure" => Ok(RunOutput::Scalar(
            bio::chronobiology::rhythms::sleep_pressure(
                get_f(p, "wake_duration")?,
                get_f(p, "buildup_rate")?,
                get_f(p, "max_pressure")?,
            ),
        )),
        "two_process_model" => Ok(RunOutput::Scalar(
            bio::chronobiology::rhythms::two_process_model(
                get_f(p, "sleep_pressure")?,
                get_f(p, "circadian_amplitude")?,
                get_f(p, "phase")?,
            ),
        )),
        "photoperiod" => Ok(RunOutput::Scalar(bio::chronobiology::rhythms::photoperiod(
            get_f(p, "latitude_rad")?,
            get_f(p, "declination_rad")?,
        ))),
        "ultradian_rhythm" => Ok(RunOutput::Scalar(
            bio::chronobiology::rhythms::ultradian_rhythm(
                get_v(p, "amplitudes")?,
                get_v(p, "periods")?,
                get_f(p, "t")?,
            ),
        )),
        "chronotype_shift" => Ok(RunOutput::Scalar(
            bio::chronobiology::rhythms::chronotype_shift(
                get_f(p, "mid_sleep_free")?,
                get_f(p, "sleep_debt_correction")?,
            ),
        )),
        "circadian_acrophase" => Ok(RunOutput::Scalar(
            bio::chronobiology::rhythms::circadian_acrophase(
                get_v(p, "data")?,
                get_f(p, "period")?,
            ),
        )),
        "cosinor_amplitude" => Ok(RunOutput::Scalar(
            bio::chronobiology::rhythms::cosinor_amplitude(get_v(p, "data")?, get_f(p, "period")?),
        )),
        "social_jet_lag" => Ok(RunOutput::Scalar(
            bio::chronobiology::rhythms::social_jet_lag(
                get_f(p, "weekday_midsleep")?,
                get_f(p, "weekend_midsleep")?,
            ),
        )),
        "mesor" => Ok(RunOutput::Scalar(bio::chronobiology::rhythms::mesor(
            get_v(p, "data")?,
        ))),
        "sleep_debt" => Ok(RunOutput::Scalar(bio::chronobiology::rhythms::sleep_debt(
            get_f(p, "wake_hours")?,
            get_f(p, "optimal_sleep")?,
            get_f(p, "actual_sleep")?,
        ))),
        "circadian_phase_estimate" => Ok(RunOutput::Scalar(
            bio::chronobiology::rhythms::circadian_phase_estimate(get_f(
                p,
                "core_body_temp_min_time",
            )?),
        )),
        "light_phase_advance" => Ok(RunOutput::Scalar(
            bio::chronobiology::rhythms::light_phase_advance(
                get_f(p, "lux")?,
                get_f(p, "sensitivity")?,
                get_f(p, "timing_factor")?,
            ),
        )),
        "dim_light_melatonin_onset" => Ok(RunOutput::Integer(
            bio::chronobiology::rhythms::dim_light_melatonin_onset(
                get_v(p, "melatonin_levels")?,
                get_f(p, "threshold")?,
            )
            .map(|x| x as i64)
            .unwrap_or(-1),
        )),
        "infradian_cycle" => Ok(RunOutput::Scalar(
            bio::chronobiology::rhythms::infradian_cycle(
                get_f(p, "base_amplitude")?,
                get_f(p, "period_days")?,
                get_f(p, "day")?,
            ),
        )),
        "temperature_compensation_q10" => Ok(RunOutput::Scalar(
            bio::chronobiology::rhythms::temperature_compensation_q10(
                get_f(p, "rate_t1")?,
                get_f(p, "rate_t2")?,
                get_f(p, "t1")?,
                get_f(p, "t2")?,
            ),
        )),
        "masking_effect" => Ok(RunOutput::Scalar(
            bio::chronobiology::rhythms::masking_effect(
                get_f(p, "endogenous")?,
                get_f(p, "exogenous_signal")?,
                get_f(p, "masking_gain")?,
            ),
        )),
        "relative_amplitude" => Ok(RunOutput::Scalar(
            bio::chronobiology::rhythms::relative_amplitude(
                get_f(p, "max_val")?,
                get_f(p, "min_val")?,
            ),
        )),
        "interdaily_stability" => Ok(RunOutput::Scalar(
            bio::chronobiology::rhythms::interdaily_stability(
                get_v(p, "data")?,
                get_u(p, "period")?,
            ),
        )),
        "intradaily_variability" => Ok(RunOutput::Scalar(
            bio::chronobiology::rhythms::intradaily_variability(get_v(p, "data")?),
        )),
        "kuramoto_step" => {
            let mut phases = get_v(p, "phases")?.to_vec();
            let freqs = get_v(p, "frequencies")?;
            bio::chronobiology::oscillators::kuramoto_step(
                &mut phases,
                freqs,
                get_f(p, "coupling")?,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Vector(phases))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
