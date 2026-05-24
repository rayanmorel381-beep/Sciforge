use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::optics::transmission_window;

impl Material {
    pub fn uv_cutoff_nm(&self) -> Option<f64> {
        transmission_window::by_formula(self.formula).map(|t| t.uv_cutoff_nm)
    }

    pub fn ir_cutoff_nm(&self) -> Option<f64> {
        transmission_window::by_formula(self.formula).map(|t| t.ir_cutoff_nm)
    }

    pub fn typical_transmittance(&self) -> Option<f64> {
        transmission_window::by_formula(self.formula).map(|t| t.typical_transmittance)
    }

    pub fn is_transparent_at_nm(&self, wavelength_nm: f64) -> Option<bool> {
        let w = transmission_window::by_formula(self.formula)?;
        Some((w.uv_cutoff_nm..=w.ir_cutoff_nm).contains(&wavelength_nm))
    }

    pub fn transparency_window_nm(&self) -> Option<(f64, f64)> {
        transmission_window::by_formula(self.formula).map(|t| (t.uv_cutoff_nm, t.ir_cutoff_nm))
    }
}
