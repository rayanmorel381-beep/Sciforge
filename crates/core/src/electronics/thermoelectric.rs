use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::electrodynamics::thermoelectric;
use sciforge_hub::prelude::physics::electronics::devices as sf_dev;

impl Material {
    pub fn seebeck_coefficient_v_per_k(&self) -> Option<f64> {
        thermoelectric::by_formula(self.formula).map(|t| t.seebeck_coefficient_v_per_k)
    }

    pub fn figure_of_merit_zt(&self) -> Option<f64> {
        thermoelectric::by_formula(self.formula).map(|t| t.figure_of_merit_zt)
    }

    pub fn seebeck_voltage_v(&self, t_hot_k: f64, t_cold_k: f64) -> Option<f64> {
        let s = self.seebeck_coefficient_v_per_k()?;
        Some(s * (t_hot_k - t_cold_k))
    }

    pub fn peltier_heat_w(&self, temperature_k: f64, current_a: f64) -> Option<f64> {
        let s = self.seebeck_coefficient_v_per_k()?;
        Some(sf_dev::peltier_heat(s, temperature_k, current_a))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Thermocouple {
    pub material_a: Material,
    pub material_b: Material,
}

impl Thermocouple {
    pub fn new(material_a: Material, material_b: Material) -> Self {
        Self { material_a, material_b }
    }

    pub fn voltage_v(&self, t_hot_k: f64, t_cold_k: f64) -> Option<f64> {
        let s_a = self.material_a.seebeck_coefficient_v_per_k()?;
        let s_b = self.material_b.seebeck_coefficient_v_per_k()?;
        Some(sf_dev::seebeck_voltage(s_a, s_b, t_hot_k, t_cold_k))
    }

    pub fn sensitivity_v_per_k(&self) -> Option<f64> {
        let s_a = self.material_a.seebeck_coefficient_v_per_k()?;
        let s_b = self.material_b.seebeck_coefficient_v_per_k()?;
        Some(s_a - s_b)
    }
}
