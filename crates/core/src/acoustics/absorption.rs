use crate::moleculars::{Gas, Liquid, Material};
use sciforge_hub::prelude::physics::acoustics::absorption as sf_abs;

impl Gas {
    pub fn mass_law_transmission_loss_db(
        &self,
        frequency_hz: f64,
        surface_density_kg_m2: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        let rho = self.density_ideal_kg_m3(pressure_pa, temperature_k);
        let c = self.speed_of_sound_m_s(temperature_k);
        sf_abs::mass_law_transmission_loss(frequency_hz, surface_density_kg_m2, rho * c)
    }

    pub fn porous_absorber_flow_resistivity(
        &self,
        sigma_pa_s_m2: f64,
        thickness_m: f64,
        frequency_hz: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        let rho = self.density_ideal_kg_m3(pressure_pa, temperature_k);
        let c = self.speed_of_sound_m_s(temperature_k);
        sf_abs::porous_absorber_flow_resistivity(sigma_pa_s_m2, thickness_m, frequency_hz, rho * c)
    }
}

impl Liquid {
    pub fn stokes_absorption_np_m(&self, frequency_hz: f64) -> f64 {
        sf_abs::stokes_absorption_liquid(
            self.dynamic_viscosity_pa_s_ref,
            self.density_kg_m3_ref,
            self.speed_of_sound_m_s(),
            frequency_hz,
        )
    }

    pub fn mass_law_transmission_loss_db(
        &self,
        frequency_hz: f64,
        surface_density_kg_m2: f64,
    ) -> f64 {
        sf_abs::mass_law_transmission_loss(
            frequency_hz,
            surface_density_kg_m2,
            self.density_kg_m3_ref * self.speed_of_sound_m_s(),
        )
    }
}

impl Material {
    pub fn classical_absorption_np_m(
        &self,
        shear_viscosity_pa_s: f64,
        bulk_viscosity_pa_s: f64,
        frequency_hz: f64,
    ) -> f64 {
        sf_abs::classical_absorption_solid(
            shear_viscosity_pa_s,
            bulk_viscosity_pa_s,
            self.density_kg_m3,
            self.longitudinal_speed_m_s(),
            frequency_hz,
        )
    }

    pub fn mass_law_transmission_loss_db(
        &self,
        frequency_hz: f64,
        surface_density_kg_m2: f64,
    ) -> f64 {
        sf_abs::mass_law_transmission_loss(
            frequency_hz,
            surface_density_kg_m2,
            self.density_kg_m3 * self.longitudinal_speed_m_s(),
        )
    }
}
