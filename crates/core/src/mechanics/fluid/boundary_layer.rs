use crate::moleculars::{Gas, Liquid};
use sciforge_hub::prelude::physics::fluid_mechanics::boundary_layer as sf_bl;

impl Gas {
    pub fn blasius_thickness_m(
        &self,
        x_m: f64,
        velocity_m_s: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        let re_x = self.reynolds_number(velocity_m_s, x_m, pressure_pa, temperature_k);
        sf_bl::blasius_thickness(x_m, re_x)
    }

    pub fn displacement_thickness_m(
        &self,
        x_m: f64,
        velocity_m_s: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        let re_x = self.reynolds_number(velocity_m_s, x_m, pressure_pa, temperature_k);
        sf_bl::displacement_thickness(x_m, re_x)
    }

    pub fn momentum_thickness_m(
        &self,
        x_m: f64,
        velocity_m_s: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        let re_x = self.reynolds_number(velocity_m_s, x_m, pressure_pa, temperature_k);
        sf_bl::momentum_thickness(x_m, re_x)
    }

    pub fn skin_friction_laminar(
        &self,
        x_m: f64,
        velocity_m_s: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        sf_bl::skin_friction_laminar(self.reynolds_number(velocity_m_s, x_m, pressure_pa, temperature_k))
    }

    pub fn skin_friction_turbulent(
        &self,
        x_m: f64,
        velocity_m_s: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        sf_bl::skin_friction_turbulent(self.reynolds_number(velocity_m_s, x_m, pressure_pa, temperature_k))
    }

    pub fn turbulent_bl_thickness_m(
        &self,
        x_m: f64,
        velocity_m_s: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        let re_x = self.reynolds_number(velocity_m_s, x_m, pressure_pa, temperature_k);
        sf_bl::turbulent_bl_thickness(x_m, re_x)
    }
}

impl Liquid {
    pub fn blasius_thickness_m(&self, x_m: f64, velocity_m_s: f64) -> f64 {
        sf_bl::blasius_thickness(x_m, self.reynolds_number(velocity_m_s, x_m))
    }

    pub fn displacement_thickness_m(&self, x_m: f64, velocity_m_s: f64) -> f64 {
        sf_bl::displacement_thickness(x_m, self.reynolds_number(velocity_m_s, x_m))
    }

    pub fn momentum_thickness_m(&self, x_m: f64, velocity_m_s: f64) -> f64 {
        sf_bl::momentum_thickness(x_m, self.reynolds_number(velocity_m_s, x_m))
    }

    pub fn skin_friction_laminar(&self, x_m: f64, velocity_m_s: f64) -> f64 {
        sf_bl::skin_friction_laminar(self.reynolds_number(velocity_m_s, x_m))
    }

    pub fn skin_friction_turbulent(&self, x_m: f64, velocity_m_s: f64) -> f64 {
        sf_bl::skin_friction_turbulent(self.reynolds_number(velocity_m_s, x_m))
    }

    pub fn turbulent_bl_thickness_m(&self, x_m: f64, velocity_m_s: f64) -> f64 {
        sf_bl::turbulent_bl_thickness(x_m, self.reynolds_number(velocity_m_s, x_m))
    }

    pub fn nusselt_flat_plate_laminar(&self, x_m: f64, velocity_m_s: f64) -> f64 {
        sf_bl::nusselt_flat_plate_laminar(self.reynolds_number(velocity_m_s, x_m), self.prandtl_number())
    }

    pub fn thermal_bl_thickness_m(&self, velocity_thickness_m: f64) -> f64 {
        sf_bl::thermal_bl_thickness(velocity_thickness_m, self.prandtl_number())
    }
}
