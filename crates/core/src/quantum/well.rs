use sciforge_hub::prelude::physics::quantum as q;

#[derive(Debug, Clone, Copy)]
pub struct InfiniteWell {
    pub length_m: f64,
    pub mass_kg: f64,
}

impl InfiniteWell {
    pub fn new(length_m: f64, mass_kg: f64) -> Self {
        Self { length_m, mass_kg }
    }

    pub fn energy_level_j(&self, n: u32) -> f64 {
        q::infinite_well_energy(n, self.length_m, self.mass_kg)
    }

    pub fn ground_state_energy_j(&self) -> f64 {
        self.energy_level_j(1)
    }

    pub fn wavefunction(&self, n: u32, x_m: f64) -> f64 {
        q::infinite_well_wf(n, x_m, self.length_m)
    }
}
