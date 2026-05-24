use crate::moleculars::Liquid;
use sciforge_hub::prelude::constants::physics::electrodynamics::dielectric;
use sciforge_hub::prelude::physics::electrodynamics::waves as sf_waves;

impl Liquid {
    pub fn relative_permittivity(&self) -> Option<f64> {
        dielectric::by_formula(self.formula).map(|d| d.relative_permittivity)
    }

    pub fn loss_tangent(&self) -> Option<f64> {
        dielectric::by_formula(self.formula).map(|d| d.loss_tangent)
    }

    pub fn em_phase_velocity_m_s(&self) -> Option<f64> {
        self.relative_permittivity()
            .map(|eps_r| sf_waves::phase_velocity(eps_r, 1.0))
    }

    pub fn em_refractive_index(&self) -> Option<f64> {
        self.relative_permittivity().map(|eps_r| eps_r.sqrt())
    }

    pub fn em_wave_impedance_ohm(&self) -> Option<f64> {
        let eps_r = self.relative_permittivity()?;
        Some(sf_waves::wave_impedance_free_space() / eps_r.sqrt())
    }

    pub fn em_wavelength_m(&self, frequency_hz: f64) -> Option<f64> {
        self.em_phase_velocity_m_s().map(|v| v / frequency_hz)
    }
}
