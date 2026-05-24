use crate::moleculars::Material;
use sciforge_hub::prelude::physics::electronics::circuits as sf_circuits;

#[derive(Debug, Clone, Copy)]
pub struct Resistor {
    pub material: Material,
    pub length_m: f64,
    pub area_m2: f64,
}

impl Resistor {
    pub fn new(material: Material, length_m: f64, area_m2: f64) -> Self {
        Self { material, length_m, area_m2 }
    }

    pub fn resistance_ohm(&self) -> Option<f64> {
        self.material.resistance_ohm(self.length_m, self.area_m2)
    }

    pub fn resistance_at_ohm(&self, temperature_k: f64) -> Option<f64> {
        self.material.resistance_at_ohm(self.length_m, self.area_m2, temperature_k)
    }

    pub fn voltage_drop_v(&self, current_a: f64) -> Option<f64> {
        self.resistance_ohm().map(|r| sf_circuits::ohm_voltage(current_a, r))
    }

    pub fn current_a(&self, voltage_v: f64) -> Option<f64> {
        self.resistance_ohm().map(|r| sf_circuits::ohm_current(voltage_v, r))
    }

    pub fn power_w(&self, current_a: f64) -> Option<f64> {
        let r = self.resistance_ohm()?;
        Some(sf_circuits::power_dc(sf_circuits::ohm_voltage(current_a, r), current_a))
    }

    pub fn ac_resistance_ohm(&self, radius_m: f64, frequency_hz: f64) -> Option<f64> {
        self.material.ac_resistance_ohm(self.length_m, radius_m, frequency_hz)
    }
}

pub fn series(resistors: &[Resistor]) -> Option<f64> {
    let values: Option<Vec<f64>> = resistors.iter().map(|r| r.resistance_ohm()).collect();
    values.map(|v| sf_circuits::series_resistance(&v))
}

pub fn parallel(resistors: &[Resistor]) -> Option<f64> {
    let values: Option<Vec<f64>> = resistors.iter().map(|r| r.resistance_ohm()).collect();
    values.map(|v| sf_circuits::parallel_resistance(&v))
}

#[derive(Debug, Clone, Copy)]
pub struct Capacitor {
    pub capacitance_f: f64,
}

impl Capacitor {
    pub fn new(capacitance_f: f64) -> Self {
        Self { capacitance_f }
    }

    pub fn impedance(&self, frequency_hz: f64) -> (f64, f64) {
        sf_circuits::impedance_capacitor(self.capacitance_f, frequency_hz)
    }

    pub fn impedance_magnitude_ohm(&self, frequency_hz: f64) -> f64 {
        let (re, im) = self.impedance(frequency_hz);
        sf_circuits::impedance_magnitude(re, im)
    }

    pub fn impedance_phase_rad(&self, frequency_hz: f64) -> f64 {
        let (re, im) = self.impedance(frequency_hz);
        sf_circuits::impedance_phase(re, im)
    }

    pub fn energy_j(&self, voltage_v: f64) -> f64 {
        sf_circuits::capacitor_energy(self.capacitance_f, voltage_v)
    }

    pub fn rc_charging_v(&self, supply_v: f64, time_s: f64, resistance_ohm: f64) -> f64 {
        sf_circuits::rc_charging(supply_v, time_s, resistance_ohm, self.capacitance_f)
    }

    pub fn rc_discharging_v(&self, initial_v: f64, time_s: f64, resistance_ohm: f64) -> f64 {
        sf_circuits::rc_discharging(initial_v, time_s, resistance_ohm, self.capacitance_f)
    }

    pub fn time_constant_s(&self, resistance_ohm: f64) -> f64 {
        resistance_ohm * self.capacitance_f
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Inductor {
    pub inductance_h: f64,
}

impl Inductor {
    pub fn new(inductance_h: f64) -> Self {
        Self { inductance_h }
    }

    pub fn impedance(&self, frequency_hz: f64) -> (f64, f64) {
        sf_circuits::impedance_inductor(self.inductance_h, frequency_hz)
    }

    pub fn impedance_magnitude_ohm(&self, frequency_hz: f64) -> f64 {
        let (re, im) = self.impedance(frequency_hz);
        sf_circuits::impedance_magnitude(re, im)
    }

    pub fn energy_j(&self, current_a: f64) -> f64 {
        sf_circuits::inductor_energy(self.inductance_h, current_a)
    }

    pub fn current_rise_a(&self, voltage_v: f64, resistance_ohm: f64, time_s: f64) -> f64 {
        sf_circuits::rl_current_rise(voltage_v, resistance_ohm, self.inductance_h, time_s)
    }

    pub fn current_decay_a(&self, initial_current_a: f64, resistance_ohm: f64, time_s: f64) -> f64 {
        sf_circuits::rl_current_decay(initial_current_a, resistance_ohm, self.inductance_h, time_s)
    }

    pub fn time_constant_s(&self, resistance_ohm: f64) -> f64 {
        self.inductance_h / resistance_ohm
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RlcCircuit {
    pub resistance_ohm: f64,
    pub inductance_h: f64,
    pub capacitance_f: f64,
}

impl RlcCircuit {
    pub fn new(resistance_ohm: f64, inductance_h: f64, capacitance_f: f64) -> Self {
        Self { resistance_ohm, inductance_h, capacitance_f }
    }

    pub fn resonant_frequency_hz(&self) -> f64 {
        sf_circuits::rlc_resonant_frequency(self.inductance_h, self.capacitance_f)
    }

    pub fn quality_factor(&self) -> f64 {
        sf_circuits::rlc_quality_factor(self.resistance_ohm, self.inductance_h, self.capacitance_f)
    }

    pub fn bandwidth_hz(&self) -> f64 {
        sf_circuits::rlc_bandwidth(self.resonant_frequency_hz(), self.quality_factor())
    }
}
