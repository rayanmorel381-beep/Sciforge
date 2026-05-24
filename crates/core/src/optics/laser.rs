use sciforge_hub::prelude::constants::physics::optics::laser_media;

#[derive(Debug, Clone, Copy)]
pub struct GainMedium {
    pub formula: &'static str,
}

impl GainMedium {
    pub fn new(formula: &'static str) -> Option<Self> {
        laser_media::by_formula(formula)?;
        Some(Self { formula })
    }

    pub fn wavelength_nm(&self) -> f64 {
        laser_media::by_formula(self.formula).map(|l| l.wavelength_nm).unwrap_or(0.0)
    }

    pub fn emission_cross_section_m2(&self) -> f64 {
        laser_media::by_formula(self.formula)
            .map(|l| l.emission_cross_section_m2)
            .unwrap_or(0.0)
    }

    pub fn upper_state_lifetime_s(&self) -> f64 {
        laser_media::by_formula(self.formula)
            .map(|l| l.upper_state_lifetime_s)
            .unwrap_or(0.0)
    }

    pub fn host(&self) -> &'static str {
        laser_media::by_formula(self.formula).map(|l| l.host).unwrap_or("")
    }

    pub fn saturation_intensity_w_per_m2(&self, wavelength_m: f64) -> f64 {
        let sigma = self.emission_cross_section_m2();
        let tau = self.upper_state_lifetime_s();
        let h = sciforge_hub::prelude::constants::H;
        let c = sciforge_hub::prelude::constants::C;
        if sigma == 0.0 || tau == 0.0 || wavelength_m == 0.0 {
            return 0.0;
        }
        h * c / (sigma * tau * wavelength_m)
    }

    pub fn small_signal_gain(&self, population_inversion_per_m3: f64, length_m: f64) -> f64 {
        self.emission_cross_section_m2() * population_inversion_per_m3 * length_m
    }
}
