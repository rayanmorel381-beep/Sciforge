use crate::moleculars::{Gas, Liquid};
use sciforge_hub::prelude::constants::EARTH_GRAVITY;
use sciforge_hub::prelude::physics::fluid_mechanics::waves as sf_waves;

impl Gas {
    pub fn mach_number(&self, velocity_m_s: f64, temperature_k: f64) -> f64 {
        sf_waves::mach_number(velocity_m_s, self.speed_of_sound_m_s(temperature_k))
    }
}

impl Liquid {
    pub fn mach_number(&self, velocity_m_s: f64) -> f64 {
        sf_waves::mach_number(velocity_m_s, self.speed_of_sound_m_s())
    }

    pub fn water_hammer_pressure_pa(&self, delta_velocity_m_s: f64) -> f64 {
        sf_waves::water_hammer_pressure(
            self.density_kg_m3_ref,
            self.speed_of_sound_m_s(),
            delta_velocity_m_s,
        )
    }

    pub fn weber_number(&self, velocity_m_s: f64, length_m: f64) -> f64 {
        sf_waves::weber_number(
            self.density_kg_m3_ref,
            velocity_m_s,
            length_m,
            self.surface_tension_n_m,
        )
    }

    pub fn capillary_number(&self, velocity_m_s: f64) -> f64 {
        sf_waves::capillary_number(
            self.dynamic_viscosity_pa_s_ref,
            velocity_m_s,
            self.surface_tension_n_m,
        )
    }

    pub fn froude_number(&self, velocity_m_s: f64, depth_m: f64) -> f64 {
        sf_waves::froude_number(velocity_m_s, EARTH_GRAVITY, depth_m)
    }

    pub fn shallow_water_speed_m_s(&self, depth_m: f64) -> f64 {
        sf_waves::shallow_water_speed(EARTH_GRAVITY, depth_m)
    }

    pub fn deep_water_speed_m_s(&self, wavelength_m: f64) -> f64 {
        sf_waves::deep_water_speed(EARTH_GRAVITY, wavelength_m)
    }

    pub fn wave_energy_density_j_m2(&self, amplitude_m: f64) -> f64 {
        sf_waves::wave_energy_density(self.density_kg_m3_ref, EARTH_GRAVITY, amplitude_m)
    }
}
