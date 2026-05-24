use crate::moleculars::Material;
use sciforge_hub::prelude::physics::optics::devices as sf_dev;

#[derive(Debug, Clone, Copy)]
pub struct OpticalFiber {
    pub core: Material,
    pub cladding: Material,
    pub core_radius_m: f64,
}

impl OpticalFiber {
    pub fn new(core: Material, cladding: Material, core_radius_m: f64) -> Self {
        Self { core, cladding, core_radius_m }
    }

    pub fn numerical_aperture(&self) -> Option<f64> {
        let n_core = self.core.refractive_index()?;
        let n_clad = self.cladding.refractive_index()?;
        Some(sf_dev::fiber_numerical_aperture(n_core, n_clad))
    }

    pub fn acceptance_angle_air_rad(&self) -> Option<f64> {
        let na = self.numerical_aperture()?;
        Some(sf_dev::fiber_acceptance_angle(na, 1.0))
    }

    pub fn v_number(&self, wavelength_m: f64) -> Option<f64> {
        let na = self.numerical_aperture()?;
        Some(sf_dev::fiber_v_number(self.core_radius_m, na, wavelength_m))
    }

    pub fn is_single_mode(&self, wavelength_m: f64) -> Option<bool> {
        self.v_number(wavelength_m).map(|v| v < 2.405)
    }

    pub fn approximate_mode_count(&self, wavelength_m: f64) -> Option<f64> {
        self.v_number(wavelength_m).map(|v| v * v / 2.0)
    }
}
