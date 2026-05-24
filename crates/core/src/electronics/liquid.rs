use crate::moleculars::Liquid;
use sciforge_hub::prelude::constants::physics::electrodynamics::resistivity::{
    self, ELECTROLYTE_MAX_OHM_M, ELECTROLYTE_MIN_OHM_M,
};
use sciforge_hub::prelude::physics::electronics::circuits as sf_circuits;

impl Liquid {
    pub fn electrical_resistivity_ohm_m(&self) -> Option<f64> {
        resistivity::by_formula(self.formula).map(|r| r.rho_ohm_m)
    }

    pub fn electrical_resistivity_at_ohm_m(&self, temperature_k: f64) -> Option<f64> {
        resistivity::by_formula(self.formula).map(|r| r.rho_at(temperature_k))
    }

    pub fn electrical_conductivity_s_per_m(&self) -> Option<f64> {
        resistivity::by_formula(self.formula).map(|r| r.conductivity_s_per_m())
    }

    pub fn resistance_ohm(&self, length_m: f64, area_m2: f64) -> Option<f64> {
        self.electrical_resistivity_ohm_m()
            .map(|rho| rho * length_m / area_m2)
    }

    pub fn resistance_at_ohm(
        &self,
        length_m: f64,
        area_m2: f64,
        temperature_k: f64,
    ) -> Option<f64> {
        self.electrical_resistivity_at_ohm_m(temperature_k)
            .map(|rho| rho * length_m / area_m2)
    }

    pub fn conductance_s(&self, length_m: f64, area_m2: f64) -> Option<f64> {
        self.resistance_ohm(length_m, area_m2).map(|r| 1.0 / r)
    }

    pub fn ohmic_voltage_v(
        &self,
        current_a: f64,
        length_m: f64,
        area_m2: f64,
    ) -> Option<f64> {
        self.resistance_ohm(length_m, area_m2)
            .map(|r| sf_circuits::ohm_voltage(current_a, r))
    }

    pub fn ohmic_current_a(
        &self,
        voltage_v: f64,
        length_m: f64,
        area_m2: f64,
    ) -> Option<f64> {
        self.resistance_ohm(length_m, area_m2)
            .map(|r| sf_circuits::ohm_current(voltage_v, r))
    }

    pub fn ohmic_power_w(
        &self,
        current_a: f64,
        length_m: f64,
        area_m2: f64,
    ) -> Option<f64> {
        let r = self.resistance_ohm(length_m, area_m2)?;
        Some(sf_circuits::power_dc(sf_circuits::ohm_voltage(current_a, r), current_a))
    }

    pub fn current_density_a_m2(&self, electric_field_v_per_m: f64) -> Option<f64> {
        self.electrical_conductivity_s_per_m()
            .map(|sigma| sigma * electric_field_v_per_m)
    }

    pub fn joule_heat_per_volume_w_m3(&self, electric_field_v_per_m: f64) -> Option<f64> {
        self.electrical_conductivity_s_per_m()
            .map(|sigma| sigma * electric_field_v_per_m * electric_field_v_per_m)
    }

    pub fn is_electrolyte(&self) -> bool {
        self.electrical_resistivity_ohm_m()
            .is_some_and(|rho| (ELECTROLYTE_MIN_OHM_M..ELECTROLYTE_MAX_OHM_M).contains(&rho))
    }
}
