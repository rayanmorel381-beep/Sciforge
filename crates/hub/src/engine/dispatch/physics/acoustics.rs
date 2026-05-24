//! Dispatch handler for acoustics functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::physics as phys;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "a_weighting" => Ok(RunOutput::Scalar(phys::acoustics::absorption::a_weighting(
            get_f(p, "f")?,
        ))),
        "absorption_coefficient" => Ok(RunOutput::Scalar(
            phys::acoustics::absorption::absorption_coefficient(
                get_f(p, "i0")?,
                get_f(p, "i")?,
                get_f(p, "x")?,
            ),
        )),
        "acoustic_impedance" => Ok(RunOutput::Scalar(
            phys::acoustics::propagation::acoustic_impedance(get_f(p, "rho")?, get_f(p, "c")?),
        )),
        "atmospheric_absorption" => Ok(RunOutput::Scalar(
            phys::acoustics::absorption::atmospheric_absorption(
                get_f(p, "f")?,
                get_f(p, "humidity")?,
                get_f(p, "temperature")?,
            ),
        )),
        "beat_frequency" => Ok(RunOutput::Scalar(
            phys::acoustics::resonance::beat_frequency(get_f(p, "f1")?, get_f(p, "f2")?),
        )),
        "decibel_addition" => Ok(RunOutput::Scalar(
            phys::acoustics::absorption::decibel_addition(get_v(p, "levels")?),
        )),
        "doppler_approaching" => Ok(RunOutput::Scalar(
            phys::acoustics::doppler::doppler_approaching(
                get_f(p, "f0")?,
                get_f(p, "c")?,
                get_f(p, "v_source")?,
            ),
        )),
        "doppler_general" => Ok(RunOutput::Scalar(
            phys::acoustics::doppler::doppler_general(
                get_f(p, "f0")?,
                get_f(p, "c")?,
                get_f(p, "v_observer")?,
                get_f(p, "v_source")?,
            ),
        )),
        "doppler_radar_velocity" => Ok(RunOutput::Scalar(
            phys::acoustics::doppler::doppler_radar_velocity(
                get_f(p, "delta_f")?,
                get_f(p, "f0")?,
                get_f(p, "c")?,
            ),
        )),
        "doppler_receding" => Ok(RunOutput::Scalar(
            phys::acoustics::doppler::doppler_receding(
                get_f(p, "f0")?,
                get_f(p, "c")?,
                get_f(p, "v_source")?,
            ),
        )),
        "doppler_shift_wavelength" => Ok(RunOutput::Scalar(
            phys::acoustics::doppler::doppler_shift_wavelength(
                get_f(p, "lambda0")?,
                get_f(p, "v")?,
                get_f(p, "c")?,
            ),
        )),
        "fundamental_frequency_string" => Ok(RunOutput::Scalar(
            phys::acoustics::resonance::fundamental_frequency_string(
                get_f(p, "l")?,
                get_f(p, "tension")?,
                get_f(p, "mu")?,
            ),
        )),
        "harmonic_frequency" => Ok(RunOutput::Scalar(
            phys::acoustics::resonance::harmonic_frequency(
                get_f(p, "fundamental")?,
                get_i(p, "n")? as u32,
            ),
        )),
        "helmholtz_resonator" => Ok(RunOutput::Scalar(
            phys::acoustics::resonance::helmholtz_resonator(
                get_f(p, "c")?,
                get_f(p, "a")?,
                get_f(p, "v")?,
                get_f(p, "l")?,
            ),
        )),
        "intensity" => Ok(RunOutput::Scalar(phys::acoustics::propagation::intensity(
            get_f(p, "power")?,
            get_f(p, "area")?,
        ))),
        "intensity_after_absorption" => Ok(RunOutput::Scalar(
            phys::acoustics::absorption::intensity_after_absorption(
                get_f(p, "i0")?,
                get_f(p, "alpha")?,
                get_f(p, "x")?,
            ),
        )),
        "intensity_level_db" => Ok(RunOutput::Scalar(
            phys::acoustics::propagation::intensity_level_db(
                get_f(p, "intensity")?,
                get_f(p, "i_ref")?,
            ),
        )),
        "inverse_square_law" => Ok(RunOutput::Scalar(
            phys::acoustics::propagation::inverse_square_law(
                get_f(p, "i0")?,
                get_f(p, "r0")?,
                get_f(p, "r")?,
            ),
        )),
        "mach_cone_angle" => Ok(RunOutput::Scalar(
            phys::acoustics::doppler::mach_cone_angle(get_f(p, "v")?, get_f(p, "c")?),
        )),
        "mass_law_transmission_loss" => Ok(RunOutput::Scalar(
            phys::acoustics::absorption::mass_law_transmission_loss(
                get_f(p, "f")?,
                get_f(p, "surface_density")?,
                get_f(p, "rho_c")?,
            ),
        )),
        "noise_reduction_coefficient" => Ok(RunOutput::Scalar(
            phys::acoustics::absorption::noise_reduction_coefficient(get_v(p, "alphas")?),
        )),
        "plane_wave_pressure" => Ok(RunOutput::Scalar(
            phys::acoustics::propagation::plane_wave_pressure(
                get_f(p, "rho")?,
                get_f(p, "c")?,
                get_f(p, "v")?,
            ),
        )),
        "porous_absorber_flow_resistivity" => Ok(RunOutput::Scalar(
            phys::acoustics::absorption::porous_absorber_flow_resistivity(
                get_f(p, "sigma")?,
                get_f(p, "thickness")?,
                get_f(p, "f")?,
                get_f(p, "rho_c")?,
            ),
        )),
        "acoustics::resonance::quality_factor" => Ok(RunOutput::Scalar(
            phys::acoustics::resonance::quality_factor(get_f(p, "f0")?, get_f(p, "bandwidth")?),
        )),
        "reflection_coefficient" => Ok(RunOutput::Scalar(
            phys::acoustics::propagation::reflection_coefficient(get_f(p, "z1")?, get_f(p, "z2")?),
        )),
        "acoustics::doppler::relativistic_doppler" => Ok(RunOutput::Scalar(
            phys::acoustics::doppler::relativistic_doppler(
                get_f(p, "f0")?,
                get_f(p, "v")?,
                get_f(p, "c")?,
            ),
        )),
        "resonant_frequency_pipe_closed" => Ok(RunOutput::Scalar(
            phys::acoustics::resonance::resonant_frequency_pipe_closed(
                get_f(p, "l")?,
                get_f(p, "c")?,
                get_i(p, "n")? as u32,
            ),
        )),
        "resonant_frequency_pipe_open" => Ok(RunOutput::Scalar(
            phys::acoustics::resonance::resonant_frequency_pipe_open(
                get_f(p, "l")?,
                get_f(p, "c")?,
                get_i(p, "n")? as u32,
            ),
        )),
        "reverberation_time_sabine" => Ok(RunOutput::Scalar(
            phys::acoustics::resonance::reverberation_time_sabine(get_f(p, "v")?, get_f(p, "a")?),
        )),
        "room_constant" => Ok(RunOutput::Scalar(
            phys::acoustics::absorption::room_constant(get_f(p, "s")?, get_f(p, "alpha_avg")?),
        )),
        "room_mode_frequency" => Ok(RunOutput::Scalar(
            phys::acoustics::resonance::room_mode_frequency(
                get_f(p, "c")?,
                get_f(p, "lx")?,
                get_f(p, "ly")?,
                get_f(p, "lz")?,
                get_i(p, "nx")? as u32,
                get_i(p, "ny")? as u32,
                get_i(p, "nz")? as u32,
            ),
        )),
        "schroeder_frequency" => Ok(RunOutput::Scalar(
            phys::acoustics::resonance::schroeder_frequency(get_f(p, "rt60")?, get_f(p, "v")?),
        )),
        "sonic_boom_pressure" => Ok(RunOutput::Scalar(
            phys::acoustics::doppler::sonic_boom_pressure(
                get_f(p, "k")?,
                get_f(p, "l")?,
                get_f(p, "d")?,
                get_f(p, "mach")?,
            ),
        )),
        "sound_pressure_level" => Ok(RunOutput::Scalar(
            phys::acoustics::propagation::sound_pressure_level(get_f(p, "p")?, get_f(p, "p_ref")?),
        )),
        "sound_transmission_class" => Ok(RunOutput::Scalar(
            phys::acoustics::absorption::sound_transmission_class(get_v(p, "tl_values")?),
        )),
        "speed_of_sound_gas" => Ok(RunOutput::Scalar(
            phys::acoustics::propagation::speed_of_sound_gas(
                get_f(p, "gamma")?,
                get_f(p, "r")?,
                get_f(p, "t")?,
                get_f(p, "m")?,
            ),
        )),
        "speed_of_sound_solid" => Ok(RunOutput::Scalar(
            phys::acoustics::propagation::speed_of_sound_solid(get_f(p, "e")?, get_f(p, "rho")?),
        )),
        "spherical_spreading" => Ok(RunOutput::Scalar(
            phys::acoustics::propagation::spherical_spreading(
                get_f(p, "p0")?,
                get_f(p, "r0")?,
                get_f(p, "r")?,
            ),
        )),
        "standing_wave_nodes" => Ok(RunOutput::Scalar(
            phys::acoustics::resonance::standing_wave_nodes(get_f(p, "l")?, get_f(p, "wavelength")?)
                as f64,
        )),
        "transmission_coefficient" => Ok(RunOutput::Scalar(
            phys::acoustics::propagation::transmission_coefficient(
                get_f(p, "z1")?,
                get_f(p, "z2")?,
            ),
        )),
        "acoustics::propagation::wavelength" => Ok(RunOutput::Scalar(
            phys::acoustics::propagation::wavelength(get_f(p, "speed")?, get_f(p, "frequency")?),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
