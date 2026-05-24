use crate::moleculars::Material;
use sciforge_hub::prelude::physics::optics::interference as sf_int;
use sciforge_hub::prelude::physics::optics::polarization as sf_pol;

pub fn malus_intensity(initial_intensity: f64, angle_rad: f64) -> f64 {
    sf_pol::malus_law(initial_intensity, angle_rad)
}

pub fn brewster_reflectance_air_to(refractive_index: f64) -> f64 {
    sf_pol::brewster_reflectance(1.0, refractive_index)
}

pub fn stokes_parameters(ex: f64, ey: f64, delta_rad: f64) -> [f64; 4] {
    sf_pol::stokes_parameters(ex, ey, delta_rad)
}

pub fn degree_of_polarization(stokes: &[f64; 4]) -> f64 {
    sf_pol::degree_of_polarization(stokes)
}

pub fn jones_rotation_matrix(angle_rad: f64) -> [[f64; 2]; 2] {
    sf_pol::jones_rotation(angle_rad)
}

pub fn ellipticity(major: f64, minor: f64) -> f64 {
    sf_pol::ellipticity(major, minor)
}

pub fn circular_dichroism(absorbance_left: f64, absorbance_right: f64) -> f64 {
    sf_pol::circular_dichroism(absorbance_left, absorbance_right)
}

#[derive(Debug, Clone, Copy)]
pub struct WavePlate {
    pub material: Material,
    pub thickness_m: f64,
    pub n_fast: f64,
    pub n_slow: f64,
}

impl WavePlate {
    pub fn new(material: Material, thickness_m: f64, n_fast: f64, n_slow: f64) -> Self {
        Self { material, thickness_m, n_fast, n_slow }
    }

    pub fn birefringence(&self) -> f64 {
        sf_pol::birefringence(self.n_slow, self.n_fast)
    }

    pub fn retardance_rad(&self, wavelength_m: f64) -> f64 {
        sf_pol::retardance(self.birefringence(), self.thickness_m, wavelength_m)
    }

    pub fn quarter_wave_phase(&self, wavelength_m: f64) -> f64 {
        sf_pol::quarter_wave_plate_phase(wavelength_m, self.n_fast, self.n_slow)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ThinFilm {
    pub material: Material,
    pub thickness_m: f64,
}

impl ThinFilm {
    pub fn new(material: Material, thickness_m: f64) -> Self {
        Self { material, thickness_m }
    }

    pub fn phase_shift_rad(&self, wavelength_m: f64, incidence_rad: f64) -> Option<f64> {
        let n = self.material.refractive_index()?;
        Some(sf_int::thin_film_phase_shift(n, self.thickness_m, wavelength_m, incidence_rad))
    }

    pub fn optical_thickness_m(&self) -> Option<f64> {
        self.material.refractive_index().map(|n| n * self.thickness_m)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FabryPerot {
    pub mirror_reflectance: f64,
    pub gap_m: f64,
    pub refractive_index: f64,
}

impl FabryPerot {
    pub fn new(mirror_reflectance: f64, gap_m: f64, refractive_index: f64) -> Self {
        Self { mirror_reflectance, gap_m, refractive_index }
    }

    pub fn transmittance(&self, phase_rad: f64) -> f64 {
        sf_int::fabry_perot_transmittance(self.mirror_reflectance, phase_rad)
    }

    pub fn finesse(&self) -> f64 {
        sf_int::fabry_perot_finesse(self.mirror_reflectance)
    }

    pub fn free_spectral_range_hz(&self) -> f64 {
        sf_int::free_spectral_range(self.gap_m, self.refractive_index)
    }
}
