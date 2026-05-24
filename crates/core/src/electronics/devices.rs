use crate::moleculars::Material;
use sciforge_hub::prelude::physics::electronics::amplifiers as sf_amp;
use sciforge_hub::prelude::physics::electronics::semiconductor_devices as sf_semi;

#[derive(Debug, Clone, Copy)]
pub struct Bjt {
    pub material: Material,
    pub beta: f64,
    pub early_voltage_v: f64,
}

impl Bjt {
    pub fn new(material: Material, beta: f64, early_voltage_v: f64) -> Self {
        Self { material, beta, early_voltage_v }
    }

    pub fn collector_current_a(&self, base_current_a: f64) -> f64 {
        sf_semi::bjt_ic_active(self.beta, base_current_a)
    }

    pub fn emitter_current_a(&self, base_current_a: f64) -> f64 {
        let ic = self.collector_current_a(base_current_a);
        sf_semi::bjt_ie(ic, base_current_a)
    }

    pub fn alpha(&self) -> f64 {
        sf_semi::bjt_alpha(self.beta)
    }

    pub fn early_effect_a(&self, ic0: f64, vce: f64) -> f64 {
        sf_semi::early_effect(ic0, vce, self.early_voltage_v)
    }

    pub fn transconductance_s(&self, collector_current_a: f64, temperature_k: f64) -> f64 {
        sf_amp::transconductance(collector_current_a, sf_amp::thermal_voltage(temperature_k))
    }

    pub fn common_emitter_gain(&self, collector_current_a: f64, load_ohm: f64, temperature_k: f64) -> f64 {
        let gm = self.transconductance_s(collector_current_a, temperature_k);
        sf_amp::common_emitter_voltage_gain(gm, load_ohm)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Mosfet {
    pub material: Material,
    pub kn: f64,
    pub vth_v: f64,
}

impl Mosfet {
    pub fn new(material: Material, kn: f64, vth_v: f64) -> Self {
        Self { material, kn, vth_v }
    }

    pub fn drain_current_saturation_a(&self, vgs_v: f64) -> f64 {
        sf_semi::mosfet_drain_current_saturation(self.kn, vgs_v, self.vth_v)
    }

    pub fn drain_current_linear_a(&self, vgs_v: f64, vds_v: f64) -> f64 {
        sf_semi::mosfet_drain_current_linear(self.kn, vgs_v, self.vth_v, vds_v)
    }

    pub fn threshold_with_body_effect_v(&self, gamma: f64, vsb_v: f64, phi_v: f64) -> f64 {
        sf_semi::mosfet_threshold_body_effect(self.vth_v, gamma, vsb_v, phi_v)
    }

    pub fn dibl_threshold_v(&self, sigma: f64, vds_v: f64) -> f64 {
        sf_semi::drain_induced_barrier_lowering(self.vth_v, sigma, vds_v)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OpAmp {
    pub gain_bandwidth_hz: f64,
}

impl OpAmp {
    pub fn new(gain_bandwidth_hz: f64) -> Self {
        Self { gain_bandwidth_hz }
    }

    pub fn inverting_gain(&self, feedback_ohm: f64, input_ohm: f64) -> f64 {
        sf_amp::inverting_gain(feedback_ohm, input_ohm)
    }

    pub fn non_inverting_gain(&self, feedback_ohm: f64, input_ohm: f64) -> f64 {
        sf_amp::non_inverting_gain(feedback_ohm, input_ohm)
    }

    pub fn differential_gain(&self, feedback_ohm: f64, input_ohm: f64) -> f64 {
        sf_amp::differential_gain(feedback_ohm, input_ohm)
    }

    pub fn summing_output_v(&self, inputs_v: &[f64], input_resistors_ohm: &[f64], feedback_ohm: f64) -> f64 {
        sf_amp::summing_amplifier(inputs_v, input_resistors_ohm, feedback_ohm)
    }

    pub fn integrator_output_v(&self, input_v: f64, resistance_ohm: f64, capacitance_f: f64, time_s: f64) -> f64 {
        sf_amp::integrator_output(input_v, resistance_ohm, capacitance_f, time_s)
    }

    pub fn differentiator_output_v(&self, dv_dt: f64, resistance_ohm: f64, capacitance_f: f64) -> f64 {
        sf_amp::differentiator_output(dv_dt, resistance_ohm, capacitance_f)
    }

    pub fn bandwidth_hz(&self, gain: f64) -> f64 {
        self.gain_bandwidth_hz / gain
    }

    pub fn gain_at_bandwidth(&self, bandwidth_hz: f64) -> f64 {
        sf_amp::gain_bandwidth_product(self.gain_bandwidth_hz / bandwidth_hz, bandwidth_hz)
            / bandwidth_hz
    }
}

pub fn decibel_voltage(v_out: f64, v_in: f64) -> f64 {
    sf_amp::decibel_voltage(v_out, v_in)
}

pub fn decibel_power(p_out: f64, p_in: f64) -> f64 {
    sf_amp::decibel_power(p_out, p_in)
}

pub fn cascaded_gain_db(gains_db: &[f64]) -> f64 {
    sf_amp::cascaded_gain(gains_db)
}

pub fn noise_figure_db(snr_in: f64, snr_out: f64) -> f64 {
    sf_amp::noise_figure(snr_in, snr_out)
}

pub fn friis_noise_factor(factors: &[f64], gains: &[f64]) -> f64 {
    sf_amp::friis_noise_factor(factors, gains)
}
