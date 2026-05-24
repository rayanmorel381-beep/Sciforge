use sciforge_hub::prelude::physics::quantum as q;

#[derive(Debug, Clone, Copy)]
pub struct HydrogenAtom {
    pub principal_quantum_number: u32,
    pub effective_charge: f64,
}

impl HydrogenAtom {
    pub fn new(principal_quantum_number: u32) -> Self {
        Self {
            principal_quantum_number,
            effective_charge: 1.0,
        }
    }

    pub fn hydrogenic(principal_quantum_number: u32, effective_charge: f64) -> Self {
        Self {
            principal_quantum_number,
            effective_charge,
        }
    }

    pub fn energy_j(&self) -> f64 {
        q::hydrogen_energy(self.principal_quantum_number)
    }

    pub fn energy_ev(&self) -> f64 {
        q::hydrogen_energy_level_z(self.principal_quantum_number, self.effective_charge)
    }

    pub fn bohr_radius_m(&self) -> f64 {
        q::bohr_radius_nth(self.principal_quantum_number, self.effective_charge)
    }
}

pub fn ground_state() -> HydrogenAtom {
    HydrogenAtom::new(1)
}

pub fn transition_wavelength_m(n_initial: u32, n_final: u32) -> f64 {
    q::rydberg_wavelength(n_initial, n_final)
}

pub fn radial_r10(r_m: f64) -> f64 {
    q::hydrogen_radial_r10(r_m)
}

pub fn radial_r20(r_m: f64) -> f64 {
    q::hydrogen_radial_r20(r_m)
}

pub fn radial_r21(r_m: f64) -> f64 {
    q::hydrogen_radial_r21(r_m)
}
