use sciforge_hub::prelude::physics::electrodynamics::waves as sf_waves;

#[derive(Debug, Clone, Copy)]
pub struct DipoleAntenna {
    pub length_m: f64,
}

impl DipoleAntenna {
    pub fn new(length_m: f64) -> Self {
        Self { length_m }
    }

    pub fn half_wave(frequency_hz: f64) -> Self {
        Self {
            length_m: 0.5 * sf_waves::wavelength(frequency_hz),
        }
    }

    pub fn radiation_resistance_ohm(&self, frequency_hz: f64) -> f64 {
        let lambda = sf_waves::wavelength(frequency_hz);
        sf_waves::antenna_radiation_resistance_dipole(self.length_m, lambda)
    }

    pub fn ratio_to_wavelength(&self, frequency_hz: f64) -> f64 {
        self.length_m / sf_waves::wavelength(frequency_hz)
    }

    pub fn radiated_power_w(&self, current_rms_a: f64, frequency_hz: f64) -> f64 {
        current_rms_a * current_rms_a * self.radiation_resistance_ohm(frequency_hz)
    }
}
