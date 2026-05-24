use sciforge_hub::prelude::physics::electrodynamics::waves as sf_waves;

#[derive(Debug, Clone, Copy)]
pub struct RectangularWaveguide {
    pub width_a_m: f64,
    pub height_b_m: f64,
}

impl RectangularWaveguide {
    pub fn new(width_a_m: f64, height_b_m: f64) -> Self {
        Self { width_a_m, height_b_m }
    }

    pub fn cutoff_frequency_te_hz(&self, m: u32, n: u32) -> f64 {
        sf_waves::waveguide_cutoff_te(m, n, self.width_a_m, self.height_b_m)
    }

    pub fn cutoff_frequency_te10_hz(&self) -> f64 {
        self.cutoff_frequency_te_hz(1, 0)
    }

    pub fn is_propagating(&self, frequency_hz: f64, m: u32, n: u32) -> bool {
        frequency_hz > self.cutoff_frequency_te_hz(m, n)
    }

    pub fn guide_wavelength_m(&self, frequency_hz: f64, m: u32, n: u32) -> Option<f64> {
        let fc = self.cutoff_frequency_te_hz(m, n);
        if frequency_hz <= fc {
            return None;
        }
        let lambda_0 = sf_waves::wavelength(frequency_hz);
        Some(lambda_0 / (1.0 - (fc / frequency_hz).powi(2)).sqrt())
    }
}
