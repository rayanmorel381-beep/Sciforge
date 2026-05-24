use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::solid_mechanics::weibull;
use sciforge_hub::prelude::constants::physics::solid_mechanics::ductile_brittle;

impl Material {
    pub fn weibull_modulus(&self) -> Option<f64> {
        weibull::by_formula(self.formula).map(|w| w.m_modulus)
    }

    pub fn weibull_failure_probability(&self, sigma_pa: f64, volume_m3: f64) -> Option<f64> {
        let w = weibull::by_formula(self.formula)?;
        Some(weibull::failure_probability(w, sigma_pa, volume_m3))
    }

    pub fn ductile_brittle_transition_k(&self) -> Option<f64> {
        ductile_brittle::by_formula(self.formula).map(|d| d.transition_k)
    }

    pub fn is_brittle_at(&self, temperature_k: f64) -> Option<bool> {
        ductile_brittle::is_brittle_at(self.formula, temperature_k)
    }
}
