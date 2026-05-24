use sciforge_hub::prelude::physics::electrodynamics::fields as sf_fields;

#[derive(Debug, Clone, Copy)]
pub struct Plasma {
    pub electron_number_density_per_m3: f64,
    pub temperature_k: f64,
}

impl Plasma {
    pub fn new(electron_number_density_per_m3: f64, temperature_k: f64) -> Self {
        Self {
            electron_number_density_per_m3,
            temperature_k,
        }
    }

    pub fn plasma_frequency_rad_per_s(&self) -> f64 {
        sf_fields::plasma_frequency(
            self.electron_number_density_per_m3,
            sciforge_hub::prelude::constants::ELECTRON_MASS_KG,
            sciforge_hub::prelude::constants::E_CHARGE,
        )
    }

    pub fn plasma_frequency_hz(&self) -> f64 {
        self.plasma_frequency_rad_per_s() / (2.0 * std::f64::consts::PI)
    }

    pub fn debye_length_m(&self) -> f64 {
        sf_fields::debye_length(
            self.temperature_k,
            self.electron_number_density_per_m3,
            sciforge_hub::prelude::constants::E_CHARGE,
        )
    }

    pub fn is_collective(&self, system_size_m: f64) -> bool {
        system_size_m > self.debye_length_m()
    }

    pub fn reflects_below_hz(&self) -> f64 {
        self.plasma_frequency_hz()
    }

    pub fn is_opaque_to(&self, frequency_hz: f64) -> bool {
        frequency_hz < self.plasma_frequency_hz()
    }
}
