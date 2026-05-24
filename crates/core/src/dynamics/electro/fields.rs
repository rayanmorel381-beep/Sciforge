use sciforge_hub::prelude::physics::electrodynamics::fields as sf_fields;
use sciforge_hub::prelude::physics::electrodynamics::waves as sf_waves;

#[derive(Debug, Clone, Copy)]
pub struct ChargedParticle {
    pub charge_c: f64,
    pub mass_kg: f64,
}

impl ChargedParticle {
    pub fn new(charge_c: f64, mass_kg: f64) -> Self {
        Self { charge_c, mass_kg }
    }

    pub fn electron() -> Self {
        Self {
            charge_c: -sciforge_hub::prelude::constants::E_CHARGE,
            mass_kg: sciforge_hub::prelude::constants::ELECTRON_MASS_KG,
        }
    }

    pub fn proton() -> Self {
        Self {
            charge_c: sciforge_hub::prelude::constants::E_CHARGE,
            mass_kg: sciforge_hub::prelude::constants::PROTON_MASS_KG,
        }
    }

    pub fn cyclotron_frequency_rad_per_s(&self, b_t: f64) -> f64 {
        sf_fields::cyclotron_frequency(self.charge_c, self.mass_kg, b_t)
    }

    pub fn cyclotron_frequency_hz(&self, b_t: f64) -> f64 {
        self.cyclotron_frequency_rad_per_s(b_t) / (2.0 * std::f64::consts::PI)
    }

    pub fn larmor_radius_m(&self, v_perp_m_s: f64, b_t: f64) -> f64 {
        sf_fields::larmor_radius(self.mass_kg, v_perp_m_s, self.charge_c, b_t)
    }

    pub fn lorentz_force_n(
        &self,
        velocity_m_s: [f64; 3],
        e_v_per_m: [f64; 3],
        b_t: [f64; 3],
    ) -> [f64; 3] {
        sf_fields::lorentz_force(self.charge_c, velocity_m_s, e_v_per_m, b_t)
    }

    pub fn larmor_radiation_power_w(&self, acceleration_m_s2: f64) -> f64 {
        sf_waves::larmor_radiation_power(self.charge_c, acceleration_m_s2)
    }

    pub fn electric_field_v_per_m(&self, position_m: [f64; 3]) -> [f64; 3] {
        sf_fields::electric_field_point_charge(self.charge_c, position_m)
    }

    pub fn electric_potential_v(&self, distance_m: f64) -> f64 {
        sf_fields::electric_potential_point(self.charge_c, distance_m)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CurrentLoop {
    pub current_a: f64,
    pub radius_m: f64,
}

impl CurrentLoop {
    pub fn new(current_a: f64, radius_m: f64) -> Self {
        Self { current_a, radius_m }
    }

    pub fn axial_field_t(&self, z_m: f64) -> f64 {
        sf_fields::magnetic_field_loop(self.current_a, self.radius_m, z_m)
    }

    pub fn center_field_t(&self) -> f64 {
        self.axial_field_t(0.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StraightWire {
    pub current_a: f64,
}

impl StraightWire {
    pub fn new(current_a: f64) -> Self {
        Self { current_a }
    }

    pub fn field_t_at_distance(&self, r_m: f64) -> f64 {
        sf_fields::magnetic_field_wire(self.current_a, r_m)
    }
}
