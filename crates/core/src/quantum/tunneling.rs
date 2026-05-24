use sciforge_hub::prelude::physics::quantum as q;

#[derive(Debug, Clone, Copy)]
pub struct RectangularBarrier {
    pub height_j: f64,
    pub width_m: f64,
    pub mass_kg: f64,
}

impl RectangularBarrier {
    pub fn new(height_j: f64, width_m: f64, mass_kg: f64) -> Self {
        Self {
            height_j,
            width_m,
            mass_kg,
        }
    }

    pub fn transmission(&self, energy_j: f64) -> f64 {
        q::tunneling_coefficient(energy_j, self.height_j, self.width_m, self.mass_kg)
    }
}

pub fn wkb_phase<F: Fn(f64) -> f64>(
    potential: F,
    energy_j: f64,
    x1_m: f64,
    x2_m: f64,
    mass_kg: f64,
) -> f64 {
    q::wkb_phase(potential, energy_j, x1_m, x2_m, mass_kg)
}

pub fn wkb_transmission<F: Fn(f64) -> f64>(
    potential: F,
    energy_j: f64,
    x1_m: f64,
    x2_m: f64,
    mass_kg: f64,
) -> f64 {
    q::wkb_transmission(potential, energy_j, x1_m, x2_m, mass_kg)
}
