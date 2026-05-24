use crate::moleculars::{Liquid, Material};

impl Material {
    pub fn joule_temperature_rise_k(
        &self,
        current_a: f64,
        length_m: f64,
        area_m2: f64,
        time_s: f64,
    ) -> Option<f64> {
        let power = self.ohmic_power_w(current_a, length_m, area_m2)?;
        let mass = self.density_kg_m3 * length_m * area_m2;
        Some(power * time_s / (mass * self.specific_heat_j_kgk))
    }

    pub fn joule_temperature_rise_rate_k_s(
        &self,
        current_a: f64,
        length_m: f64,
        area_m2: f64,
    ) -> Option<f64> {
        let power = self.ohmic_power_w(current_a, length_m, area_m2)?;
        let mass = self.density_kg_m3 * length_m * area_m2;
        Some(power / (mass * self.specific_heat_j_kgk))
    }

    pub fn final_temperature_under_current_k(
        &self,
        initial_temperature_k: f64,
        current_a: f64,
        length_m: f64,
        area_m2: f64,
        time_s: f64,
    ) -> Option<f64> {
        self.joule_temperature_rise_k(current_a, length_m, area_m2, time_s)
            .map(|dt| initial_temperature_k + dt)
    }

    pub fn fusing_current_a(&self, area_m2: f64, ambient_k: f64) -> Option<f64> {
        let rho = self.electrical_resistivity_ohm_m()?;
        let dt = (self.max_service_temp_k - ambient_k).max(0.0);
        Some((dt * self.thermal_conductivity_w_mk * area_m2 * area_m2 / rho).sqrt())
    }
}

impl Liquid {
    pub fn joule_temperature_rise_k(
        &self,
        current_a: f64,
        length_m: f64,
        area_m2: f64,
        time_s: f64,
    ) -> Option<f64> {
        let power = self.ohmic_power_w(current_a, length_m, area_m2)?;
        let mass = self.density_kg_m3_ref * length_m * area_m2;
        Some(power * time_s / (mass * self.specific_heat_j_kgk))
    }

    pub fn joule_temperature_rise_rate_k_s(
        &self,
        current_a: f64,
        length_m: f64,
        area_m2: f64,
    ) -> Option<f64> {
        let power = self.ohmic_power_w(current_a, length_m, area_m2)?;
        let mass = self.density_kg_m3_ref * length_m * area_m2;
        Some(power / (mass * self.specific_heat_j_kgk))
    }
}
