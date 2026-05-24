use crate::moleculars::Material;
use sciforge_hub::prelude::physics::electrodynamics::waves as sf_waves;

#[derive(Debug, Clone, Copy)]
pub struct ElectromagneticWave {
    pub medium: Material,
    pub frequency_hz: f64,
}

impl ElectromagneticWave {
    pub fn new(medium: Material, frequency_hz: f64) -> Self {
        Self { medium, frequency_hz }
    }

    pub fn phase_velocity_m_s(&self) -> Option<f64> {
        self.medium.em_phase_velocity_m_s()
    }

    pub fn wavelength_m(&self) -> Option<f64> {
        self.medium.em_wavelength_m(self.frequency_hz)
    }

    pub fn wave_number_per_m(&self) -> Option<f64> {
        self.medium.em_wave_number_per_m(self.frequency_hz)
    }

    pub fn wave_impedance_ohm(&self) -> Option<f64> {
        self.medium.em_wave_impedance_ohm()
    }

    pub fn skin_depth_m(&self) -> Option<f64> {
        self.medium.skin_depth_with_permeability_m(self.frequency_hz)
    }

    pub fn attenuation_per_m(&self) -> Option<f64> {
        self.medium.em_attenuation_per_m(self.frequency_hz)
    }

    pub fn intensity_after_m(&self, incident_intensity_w_m2: f64, distance_m: f64) -> Option<f64> {
        let alpha = self.attenuation_per_m()?;
        Some(incident_intensity_w_m2 * (-2.0 * alpha * distance_m).exp())
    }
}

pub fn radiation_pressure_absorbed_pa(intensity_w_m2: f64) -> f64 {
    sf_waves::radiation_pressure_absorbed(intensity_w_m2)
}

pub fn radiation_pressure_reflected_pa(intensity_w_m2: f64) -> f64 {
    sf_waves::radiation_pressure_reflected(intensity_w_m2)
}
