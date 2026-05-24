use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::electrodynamics::superconductor;

impl Material {
    pub fn critical_temperature_k(&self) -> Option<f64> {
        superconductor::by_formula(self.formula).map(|s| s.critical_temperature_k)
    }

    pub fn critical_field_t(&self) -> Option<f64> {
        superconductor::by_formula(self.formula).map(|s| s.critical_field_t)
    }

    pub fn critical_current_density_a_m2(&self) -> Option<f64> {
        superconductor::by_formula(self.formula).map(|s| s.critical_current_density_a_m2)
    }

    pub fn superconductor_kind(&self) -> Option<&'static str> {
        superconductor::by_formula(self.formula).map(|s| s.kind)
    }

    pub fn is_superconductor(&self) -> bool {
        superconductor::by_formula(self.formula).is_some()
    }

    pub fn is_superconducting_at(&self, temperature_k: f64, field_t: f64) -> Option<bool> {
        let s = superconductor::by_formula(self.formula)?;
        Some(temperature_k < s.critical_temperature_k && field_t < s.critical_field_t)
    }

    pub fn critical_current_a(&self, area_m2: f64) -> Option<f64> {
        self.critical_current_density_a_m2().map(|jc| jc * area_m2)
    }

    pub fn temperature_dependent_critical_field_t(&self, temperature_k: f64) -> Option<f64> {
        let s = superconductor::by_formula(self.formula)?;
        let ratio = (temperature_k / s.critical_temperature_k).powi(2);
        if ratio >= 1.0 {
            Some(0.0)
        } else {
            Some(s.critical_field_t * (1.0 - ratio))
        }
    }
}
