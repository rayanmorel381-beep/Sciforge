use crate::moleculars::{Gas, Liquid, Material};
use sciforge_hub::prelude::physics::acoustics::doppler as sf_dop;

impl Gas {
    pub fn doppler_approaching_hz(
        &self,
        emitted_frequency_hz: f64,
        source_velocity_m_s: f64,
        temperature_k: f64,
    ) -> f64 {
        sf_dop::doppler_approaching(
            emitted_frequency_hz,
            self.speed_of_sound_m_s(temperature_k),
            source_velocity_m_s,
        )
    }

    pub fn doppler_receding_hz(
        &self,
        emitted_frequency_hz: f64,
        source_velocity_m_s: f64,
        temperature_k: f64,
    ) -> f64 {
        sf_dop::doppler_receding(
            emitted_frequency_hz,
            self.speed_of_sound_m_s(temperature_k),
            source_velocity_m_s,
        )
    }

    pub fn doppler_general_hz(
        &self,
        emitted_frequency_hz: f64,
        observer_velocity_m_s: f64,
        source_velocity_m_s: f64,
        temperature_k: f64,
    ) -> f64 {
        sf_dop::doppler_general(
            emitted_frequency_hz,
            self.speed_of_sound_m_s(temperature_k),
            observer_velocity_m_s,
            source_velocity_m_s,
        )
    }

    pub fn mach_cone_angle_rad(&self, velocity_m_s: f64, temperature_k: f64) -> f64 {
        sf_dop::mach_cone_angle(velocity_m_s, self.speed_of_sound_m_s(temperature_k))
    }
}

impl Liquid {
    pub fn doppler_approaching_hz(&self, emitted_frequency_hz: f64, source_velocity_m_s: f64) -> f64 {
        sf_dop::doppler_approaching(emitted_frequency_hz, self.speed_of_sound_m_s(), source_velocity_m_s)
    }

    pub fn doppler_receding_hz(&self, emitted_frequency_hz: f64, source_velocity_m_s: f64) -> f64 {
        sf_dop::doppler_receding(emitted_frequency_hz, self.speed_of_sound_m_s(), source_velocity_m_s)
    }

    pub fn doppler_general_hz(
        &self,
        emitted_frequency_hz: f64,
        observer_velocity_m_s: f64,
        source_velocity_m_s: f64,
    ) -> f64 {
        sf_dop::doppler_general(
            emitted_frequency_hz,
            self.speed_of_sound_m_s(),
            observer_velocity_m_s,
            source_velocity_m_s,
        )
    }
}

impl Material {
    pub fn doppler_approaching_hz(&self, emitted_frequency_hz: f64, source_velocity_m_s: f64) -> f64 {
        sf_dop::doppler_approaching(emitted_frequency_hz, self.speed_of_sound_m_s(), source_velocity_m_s)
    }

    pub fn doppler_receding_hz(&self, emitted_frequency_hz: f64, source_velocity_m_s: f64) -> f64 {
        sf_dop::doppler_receding(emitted_frequency_hz, self.speed_of_sound_m_s(), source_velocity_m_s)
    }
}
