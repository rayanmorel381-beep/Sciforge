use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::solid_mechanics::debye_temperature;
use sciforge_hub::prelude::physics::solid_mechanics::elasticity as sf_el;

impl Material {
    pub fn debye_temperature_k(&self) -> Option<f64> {
        debye_temperature::by_formula(self.formula).map(|d| d.theta_d_k)
    }

    pub fn thermal_strain(&self, delta_temperature_k: f64) -> f64 {
        sf_el::thermal_strain(self.thermal_expansion_per_k, delta_temperature_k)
    }

    pub fn thermal_stress_pa(&self, delta_temperature_k: f64) -> f64 {
        sf_el::thermal_stress(
            self.young_modulus_pa,
            self.thermal_expansion_per_k,
            delta_temperature_k,
        )
    }

    pub fn thermal_shock_resistance(&self) -> Option<f64> {
        if self.thermal_expansion_per_k == 0.0 || self.young_modulus_pa == 0.0 {
            return None;
        }
        Some(
            self.yield_strength_pa * self.thermal_conductivity_w_mk
                / (self.young_modulus_pa * self.thermal_expansion_per_k),
        )
    }

    pub fn is_above_max_service_temp(&self, temperature_k: f64) -> bool {
        self.max_service_temp_k > 0.0 && temperature_k > self.max_service_temp_k
    }
}
