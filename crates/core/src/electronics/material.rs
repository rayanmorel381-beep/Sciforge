use crate::moleculars::Material;
use sciforge_hub::prelude::constants::MU_0;
use sciforge_hub::prelude::constants::physics::electrodynamics::resistivity::{
    self, CONDUCTOR_MAX_OHM_M, INSULATOR_MIN_OHM_M,
};
use sciforge_hub::prelude::constants::physics::electronics::bandgap;
use sciforge_hub::prelude::physics::electronics::circuits as sf_circuits;
use std::f64::consts::PI;

impl Material {
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

    pub fn bandgap_ev(&self) -> Option<f64> {
        bandgap::by_formula(self.formula).map(|b| b.eg_ev)
    }

    pub fn bandgap_kind(&self) -> Option<&'static str> {
        bandgap::by_formula(self.formula).map(|b| b.kind)
    }

    pub fn is_semiconductor(&self) -> bool {
        bandgap::by_formula(self.formula).is_some()
    }

    pub fn is_conductor(&self) -> bool {
        self.electrical_resistivity_ohm_m()
            .is_some_and(|rho| rho < CONDUCTOR_MAX_OHM_M)
    }

    pub fn is_insulator(&self) -> bool {
        self.electrical_resistivity_ohm_m()
            .is_some_and(|rho| rho > INSULATOR_MIN_OHM_M)
    }

    pub fn conductance_s(&self, length_m: f64, area_m2: f64) -> Option<f64> {
        self.resistance_ohm(length_m, area_m2).map(|r| 1.0 / r)
    }

    pub fn current_density_a_m2(&self, electric_field_v_per_m: f64) -> Option<f64> {
        self.electrical_conductivity_s_per_m()
            .map(|sigma| sigma * electric_field_v_per_m)
    }

    pub fn joule_heat_per_volume_w_m3(&self, electric_field_v_per_m: f64) -> Option<f64> {
        self.electrical_conductivity_s_per_m()
            .map(|sigma| sigma * electric_field_v_per_m * electric_field_v_per_m)
    }

    pub fn skin_depth_m(&self, frequency_hz: f64) -> Option<f64> {
        self.electrical_resistivity_ohm_m()
            .map(|rho| (rho / (PI * frequency_hz * MU_0)).sqrt())
    }

    pub fn skin_depth_at_m(&self, frequency_hz: f64, temperature_k: f64) -> Option<f64> {
        self.electrical_resistivity_at_ohm_m(temperature_k)
            .map(|rho| (rho / (PI * frequency_hz * MU_0)).sqrt())
    }

    pub fn ac_resistance_ohm(
        &self,
        length_m: f64,
        radius_m: f64,
        frequency_hz: f64,
    ) -> Option<f64> {
        let rho = self.electrical_resistivity_ohm_m()?;
        let delta = (rho / (PI * frequency_hz * MU_0)).sqrt();
        let effective_area = if delta < radius_m {
            PI * (radius_m * radius_m - (radius_m - delta).powi(2))
        } else {
            PI * radius_m * radius_m
        };
        Some(rho * length_m / effective_area)
    }

    pub fn temperature_coefficient_per_k(&self) -> Option<f64> {
        resistivity::by_formula(self.formula).map(|r| r.temperature_coeff_per_k)
    }

    pub fn reference_temperature_k(&self) -> Option<f64> {
        resistivity::by_formula(self.formula).map(|r| r.temperature_k)
    }

    pub fn bandgap_reference_temperature_k(&self) -> Option<f64> {
        bandgap::by_formula(self.formula).map(|b| b.temperature_k)
    }
}
