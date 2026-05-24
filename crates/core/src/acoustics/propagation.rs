use crate::moleculars::{Gas, Liquid, Material};
use sciforge_hub::prelude::physics::acoustics::propagation as sf_prop;

impl Gas {
    pub fn wavelength_m(&self, frequency_hz: f64, temperature_k: f64) -> f64 {
        sf_prop::wavelength(self.speed_of_sound_m_s(temperature_k), frequency_hz)
    }

    pub fn acoustic_impedance_pa_s_m(&self, pressure_pa: f64, temperature_k: f64) -> f64 {
        sf_prop::acoustic_impedance(
            self.density_ideal_kg_m3(pressure_pa, temperature_k),
            self.speed_of_sound_m_s(temperature_k),
        )
    }

    pub fn plane_wave_pressure_pa(
        &self,
        particle_velocity_m_s: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        sf_prop::plane_wave_pressure(
            self.density_ideal_kg_m3(pressure_pa, temperature_k),
            self.speed_of_sound_m_s(temperature_k),
            particle_velocity_m_s,
        )
    }

    pub fn sound_pressure_level_db(&self, pressure_pa: f64) -> f64 {
        sf_prop::sound_pressure_level_air(pressure_pa)
    }
}

impl Liquid {
    pub fn wavelength_m(&self, frequency_hz: f64) -> f64 {
        sf_prop::wavelength(self.speed_of_sound_m_s(), frequency_hz)
    }

    pub fn acoustic_impedance_pa_s_m(&self) -> f64 {
        sf_prop::acoustic_impedance(self.density_kg_m3_ref, self.speed_of_sound_m_s())
    }

    pub fn plane_wave_pressure_pa(&self, particle_velocity_m_s: f64) -> f64 {
        sf_prop::plane_wave_pressure(
            self.density_kg_m3_ref,
            self.speed_of_sound_m_s(),
            particle_velocity_m_s,
        )
    }

    pub fn sound_pressure_level_db(&self, pressure_pa: f64) -> f64 {
        sf_prop::sound_pressure_level_water(pressure_pa)
    }
}

impl Material {
    pub fn speed_of_sound_m_s(&self) -> f64 {
        sf_prop::speed_of_sound_solid(self.young_modulus_pa, self.density_kg_m3)
    }

    pub fn longitudinal_speed_m_s(&self) -> f64 {
        sf_prop::speed_of_sound_solid_longitudinal(
            self.young_modulus_pa,
            self.poisson_ratio,
            self.density_kg_m3,
        )
    }

    pub fn shear_speed_m_s(&self) -> f64 {
        sf_prop::speed_of_sound_solid_shear(
            self.young_modulus_pa,
            self.poisson_ratio,
            self.density_kg_m3,
        )
    }

    pub fn wavelength_m(&self, frequency_hz: f64) -> f64 {
        sf_prop::wavelength(self.speed_of_sound_m_s(), frequency_hz)
    }

    pub fn acoustic_impedance_pa_s_m(&self) -> f64 {
        sf_prop::acoustic_impedance(self.density_kg_m3, self.speed_of_sound_m_s())
    }

    pub fn longitudinal_acoustic_impedance_pa_s_m(&self) -> f64 {
        sf_prop::acoustic_impedance(self.density_kg_m3, self.longitudinal_speed_m_s())
    }

    pub fn shear_acoustic_impedance_pa_s_m(&self) -> f64 {
        sf_prop::acoustic_impedance(self.density_kg_m3, self.shear_speed_m_s())
    }

    pub fn plane_wave_pressure_pa(&self, particle_velocity_m_s: f64) -> f64 {
        sf_prop::plane_wave_pressure(
            self.density_kg_m3,
            self.speed_of_sound_m_s(),
            particle_velocity_m_s,
        )
    }
}
