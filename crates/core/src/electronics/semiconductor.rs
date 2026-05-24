use crate::moleculars::Material;
use sciforge_hub::prelude::constants::E_CHARGE;
use sciforge_hub::prelude::constants::physics::electronics::{bandgap, carrier_concentration};
use sciforge_hub::prelude::physics::electronics::amplifiers as sf_amp;
use sciforge_hub::prelude::physics::electronics::devices as sf_dev;
use sciforge_hub::prelude::physics::electronics::semiconductor_devices as sf_semi;

impl Material {
    pub fn thermal_voltage_v(&self, temperature_k: f64) -> f64 {
        sf_amp::thermal_voltage(temperature_k)
    }

    pub fn diode_shockley_current_a(
        &self,
        saturation_current_a: f64,
        voltage_v: f64,
        ideality_factor: f64,
        temperature_k: f64,
    ) -> f64 {
        sf_semi::diode_shockley(
            saturation_current_a,
            voltage_v,
            ideality_factor,
            sf_amp::thermal_voltage(temperature_k),
        )
    }

    pub fn solar_cell_current_a(
        &self,
        photo_current_a: f64,
        saturation_current_a: f64,
        voltage_v: f64,
        ideality_factor: f64,
        temperature_k: f64,
        series_resistance_ohm: f64,
    ) -> f64 {
        sf_semi::solar_cell_iv(
            photo_current_a,
            saturation_current_a,
            voltage_v,
            ideality_factor,
            sf_amp::thermal_voltage(temperature_k),
            series_resistance_ohm,
        )
    }

    pub fn pn_junction_capacitance_f(
        &self,
        zero_bias_capacitance_f: f64,
        voltage_v: f64,
        grading_coefficient: f64,
    ) -> Option<f64> {
        let v_bi = bandgap::by_formula(self.formula)?.eg_ev;
        Some(sf_semi::pn_junction_capacitance(
            zero_bias_capacitance_f,
            voltage_v,
            v_bi,
            grading_coefficient,
        ))
    }

    pub fn led_series_resistor_ohm(
        &self,
        supply_voltage_v: f64,
        led_current_a: f64,
    ) -> Option<f64> {
        let v_led = bandgap::by_formula(self.formula)?.eg_ev;
        Some(sf_semi::led_resistor(supply_voltage_v, v_led, led_current_a))
    }

    pub fn is_direct_bandgap(&self) -> bool {
        bandgap::by_formula(self.formula).is_some_and(|b| b.kind == "direct")
    }

    pub fn is_indirect_bandgap(&self) -> bool {
        bandgap::by_formula(self.formula).is_some_and(|b| b.kind == "indirect")
    }

    pub fn intrinsic_carrier_concentration_per_m3(&self) -> Option<f64> {
        carrier_concentration::by_formula(self.formula).map(|c| c.ni_300k_per_m3)
    }

    pub fn intrinsic_carrier_concentration_at_per_m3(&self, temperature_k: f64) -> Option<f64> {
        let c = carrier_concentration::by_formula(self.formula)?;
        let eg = bandgap::by_formula(self.formula)?.eg_ev;
        Some(sf_dev::intrinsic_carrier_concentration(
            c.ni_300k_per_m3,
            eg,
            temperature_k,
            300.0,
        ))
    }

    pub fn electron_mobility_m2_per_vs(&self) -> Option<f64> {
        carrier_concentration::by_formula(self.formula).map(|c| c.electron_mobility_m2_per_vs)
    }

    pub fn hole_mobility_m2_per_vs(&self) -> Option<f64> {
        carrier_concentration::by_formula(self.formula).map(|c| c.hole_mobility_m2_per_vs)
    }

    pub fn intrinsic_conductivity_s_per_m(&self) -> Option<f64> {
        let c = carrier_concentration::by_formula(self.formula)?;
        Some(E_CHARGE * c.ni_300k_per_m3 * (c.electron_mobility_m2_per_vs + c.hole_mobility_m2_per_vs))
    }

    pub fn intrinsic_conductivity_at_s_per_m(&self, temperature_k: f64) -> Option<f64> {
        let c = carrier_concentration::by_formula(self.formula)?;
        let ni = self.intrinsic_carrier_concentration_at_per_m3(temperature_k)?;
        Some(E_CHARGE * ni * (c.electron_mobility_m2_per_vs + c.hole_mobility_m2_per_vs))
    }

    pub fn effective_mass_electron(&self) -> Option<f64> {
        carrier_concentration::by_formula(self.formula).map(|c| c.effective_mass_electron)
    }

    pub fn effective_mass_hole(&self) -> Option<f64> {
        carrier_concentration::by_formula(self.formula).map(|c| c.effective_mass_hole)
    }

    pub fn built_in_voltage_v(
        &self,
        n_acceptor_per_m3: f64,
        n_donor_per_m3: f64,
        temperature_k: f64,
    ) -> Option<f64> {
        let ni = self.intrinsic_carrier_concentration_at_per_m3(temperature_k)?;
        let vt = sf_amp::thermal_voltage(temperature_k);
        Some(sf_dev::built_in_voltage(vt, n_acceptor_per_m3, n_donor_per_m3, ni))
    }

    pub fn depletion_width_m(
        &self,
        v_bi_v: f64,
        v_applied_v: f64,
        n_doping_per_m3: f64,
    ) -> Option<f64> {
        let eps_r = self.relative_permittivity()?;
        Some(sf_dev::depletion_width(eps_r, v_bi_v, v_applied_v, n_doping_per_m3))
    }

    pub fn hall_voltage_v(
        &self,
        magnetic_field_t: f64,
        current_a: f64,
        thickness_m: f64,
    ) -> Option<f64> {
        let n = self.intrinsic_carrier_concentration_per_m3()?;
        Some(sf_dev::hall_voltage(magnetic_field_t, current_a, thickness_m, n, E_CHARGE))
    }
}
