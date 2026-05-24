use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::solid_mechanics::thermal_expansion;

impl Material {
    pub fn alpha_at_temperature_per_k(&self, temperature_k: f64) -> Option<f64> {
        let entry = thermal_expansion::by_formula(self.formula)?;
        Some(thermal_expansion::alpha_at_temperature(entry, temperature_k))
    }

    pub fn thermal_expansion_valid_range_k(&self) -> Option<(f64, f64)> {
        thermal_expansion::by_formula(self.formula).map(|e| e.valid_range_k)
    }

    pub fn thermal_strain_at(&self, t_k: f64, t_ref_k: f64) -> Option<f64> {
        let alpha = self.alpha_at_temperature_per_k((t_k + t_ref_k) * 0.5)?;
        Some(alpha * (t_k - t_ref_k))
    }

    pub fn thermal_stress_at_pa(&self, t_k: f64, t_ref_k: f64) -> Option<f64> {
        self.thermal_strain_at(t_k, t_ref_k)
            .map(|eps| self.young_modulus_pa * eps)
    }
}
