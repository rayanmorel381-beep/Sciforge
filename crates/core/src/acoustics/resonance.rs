use crate::moleculars::{Gas, Liquid};
use sciforge_hub::prelude::physics::acoustics::resonance as sf_res;

impl Gas {
    pub fn pipe_open_resonant_frequency_hz(
        &self,
        length_m: f64,
        harmonic: u32,
        temperature_k: f64,
    ) -> f64 {
        sf_res::resonant_frequency_pipe_open(length_m, self.speed_of_sound_m_s(temperature_k), harmonic)
    }

    pub fn pipe_closed_resonant_frequency_hz(
        &self,
        length_m: f64,
        harmonic: u32,
        temperature_k: f64,
    ) -> f64 {
        sf_res::resonant_frequency_pipe_closed(length_m, self.speed_of_sound_m_s(temperature_k), harmonic)
    }

    pub fn helmholtz_resonator_hz(
        &self,
        neck_area_m2: f64,
        cavity_volume_m3: f64,
        neck_length_m: f64,
        temperature_k: f64,
    ) -> f64 {
        sf_res::helmholtz_resonator(
            self.speed_of_sound_m_s(temperature_k),
            neck_area_m2,
            cavity_volume_m3,
            neck_length_m,
        )
    }

    pub fn room_mode_frequency_hz(
        &self,
        lx_m: f64,
        ly_m: f64,
        lz_m: f64,
        nx: u32,
        ny: u32,
        nz: u32,
        temperature_k: f64,
    ) -> f64 {
        sf_res::room_mode_frequency(self.speed_of_sound_m_s(temperature_k), lx_m, ly_m, lz_m, nx, ny, nz)
    }
}

impl Liquid {
    pub fn pipe_open_resonant_frequency_hz(&self, length_m: f64, harmonic: u32) -> f64 {
        sf_res::resonant_frequency_pipe_open(length_m, self.speed_of_sound_m_s(), harmonic)
    }

    pub fn pipe_closed_resonant_frequency_hz(&self, length_m: f64, harmonic: u32) -> f64 {
        sf_res::resonant_frequency_pipe_closed(length_m, self.speed_of_sound_m_s(), harmonic)
    }
}
