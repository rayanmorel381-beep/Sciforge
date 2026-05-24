use sciforge_hub::prelude::physics::quantum as q;

#[derive(Debug, Clone, Copy)]
pub struct HarmonicOscillator {
    pub mass_kg: f64,
    pub angular_frequency_rad_per_s: f64,
}

impl HarmonicOscillator {
    pub fn new(mass_kg: f64, angular_frequency_rad_per_s: f64) -> Self {
        Self {
            mass_kg,
            angular_frequency_rad_per_s,
        }
    }

    pub fn energy_level_j(&self, n: u32) -> f64 {
        q::harmonic_oscillator_energy(n, self.angular_frequency_rad_per_s)
    }

    pub fn zero_point_energy_j(&self) -> f64 {
        self.energy_level_j(0)
    }

    pub fn wavefunction(&self, n: u32, x_m: f64) -> f64 {
        q::harmonic_oscillator_wf(n, x_m, self.mass_kg, self.angular_frequency_rad_per_s)
    }
}
