use crate::moleculars::Material;
use sciforge_hub::prelude::constants::C;
use sciforge_hub::prelude::physics::electrodynamics::waves as sf_waves;

impl Material {
    pub fn em_phase_velocity_m_s(&self) -> Option<f64> {
        let eps_r = self.relative_permittivity()?;
        let mu_r = self.relative_permeability().unwrap_or(1.0);
        Some(sf_waves::phase_velocity(eps_r, mu_r))
    }

    pub fn em_refractive_index(&self) -> Option<f64> {
        let eps_r = self.relative_permittivity()?;
        let mu_r = self.relative_permeability().unwrap_or(1.0);
        Some((eps_r * mu_r).sqrt())
    }

    pub fn em_wave_impedance_ohm(&self) -> Option<f64> {
        let eps_r = self.relative_permittivity()?;
        let mu_r = self.relative_permeability().unwrap_or(1.0);
        let eta_0 = sf_waves::wave_impedance_free_space();
        Some(eta_0 * (mu_r / eps_r).sqrt())
    }

    pub fn em_wavelength_m(&self, frequency_hz: f64) -> Option<f64> {
        self.em_phase_velocity_m_s().map(|v| v / frequency_hz)
    }

    pub fn em_wave_number_per_m(&self, frequency_hz: f64) -> Option<f64> {
        let v = self.em_phase_velocity_m_s()?;
        Some(2.0 * std::f64::consts::PI * frequency_hz / v)
    }

    pub fn em_attenuation_per_m(&self, frequency_hz: f64) -> Option<f64> {
        let sigma = self.electrical_conductivity_s_per_m()?;
        let mu_r = self.relative_permeability().unwrap_or(1.0);
        let omega = 2.0 * std::f64::consts::PI * frequency_hz;
        let mu = mu_r * sciforge_hub::prelude::constants::MU_0;
        Some((omega * mu * sigma / 2.0).sqrt())
    }

    pub fn em_dielectric_attenuation_neper_per_m(&self, frequency_hz: f64) -> Option<f64> {
        let tan_delta = self.loss_tangent()?;
        let k = self.em_wave_number_per_m(frequency_hz)?;
        Some(0.5 * k * tan_delta)
    }

    pub fn fresnel_reflection_normal_from_vacuum(&self) -> Option<f64> {
        let n = self.em_refractive_index()?;
        let r = (n - 1.0) / (n + 1.0);
        Some(r * r)
    }

    pub fn vacuum_speed_ratio(&self) -> Option<f64> {
        self.em_phase_velocity_m_s().map(|v| v / C)
    }
}
