use crate::moleculars::Material;
use sciforge_hub::prelude::physics::optics::diffraction as sf_diff;
use sciforge_hub::prelude::physics::optics::refraction as sf_refr;

#[derive(Debug, Clone, Copy)]
pub struct ThinLens {
    pub material: Material,
    pub radius_1_m: f64,
    pub radius_2_m: f64,
}

impl ThinLens {
    pub fn new(material: Material, radius_1_m: f64, radius_2_m: f64) -> Self {
        Self { material, radius_1_m, radius_2_m }
    }

    pub fn focal_length_m(&self) -> Option<f64> {
        self.material.lensmaker_focal_length_m(self.radius_1_m, self.radius_2_m)
    }

    pub fn power_diopter(&self) -> Option<f64> {
        self.focal_length_m().map(|f| 1.0 / f)
    }

    pub fn image_distance_m(&self, object_distance_m: f64) -> Option<f64> {
        let f = self.focal_length_m()?;
        Some(sf_refr::thin_lens_equation(f, object_distance_m))
    }

    pub fn magnification(&self, object_distance_m: f64) -> Option<f64> {
        let image = self.image_distance_m(object_distance_m)?;
        Some(sf_refr::magnification(image, object_distance_m))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CircularAperture {
    pub diameter_m: f64,
    pub focal_length_m: f64,
}

impl CircularAperture {
    pub fn new(diameter_m: f64, focal_length_m: f64) -> Self {
        Self { diameter_m, focal_length_m }
    }

    pub fn f_number(&self) -> f64 {
        self.focal_length_m / self.diameter_m
    }

    pub fn airy_disk_radius_m(&self, wavelength_m: f64) -> f64 {
        sf_diff::airy_disk_radius(wavelength_m, self.f_number())
    }

    pub fn rayleigh_resolution_rad(&self, wavelength_m: f64) -> f64 {
        sf_diff::rayleigh_criterion(wavelength_m, self.diameter_m)
    }

    pub fn first_zero_angle_rad(&self, wavelength_m: f64) -> f64 {
        sf_diff::circular_aperture_first_zero(wavelength_m, self.diameter_m)
    }

    pub fn fraunhofer_distance_m(&self, wavelength_m: f64) -> f64 {
        sf_diff::fraunhofer_distance(self.diameter_m, wavelength_m)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DiffractionGrating {
    pub line_spacing_m: f64,
    pub n_slits: u32,
}

impl DiffractionGrating {
    pub fn new(line_spacing_m: f64, n_slits: u32) -> Self {
        Self { line_spacing_m, n_slits }
    }

    pub fn maxima_angle_rad(&self, wavelength_m: f64, order: i32) -> f64 {
        sf_diff::diffraction_grating_maxima(self.line_spacing_m, wavelength_m, order)
    }

    pub fn resolving_power(&self, order: i32) -> f64 {
        sf_diff::resolving_power_grating(order, self.n_slits)
    }

    pub fn dispersion_per_m(&self, order: i32, theta_rad: f64) -> f64 {
        sf_diff::grating_dispersion(order, self.line_spacing_m, theta_rad)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DoubleSlit {
    pub slit_separation_m: f64,
    pub screen_distance_m: f64,
}

impl DoubleSlit {
    pub fn new(slit_separation_m: f64, screen_distance_m: f64) -> Self {
        Self { slit_separation_m, screen_distance_m }
    }

    pub fn fringe_spacing_m(&self, wavelength_m: f64) -> f64 {
        sciforge_hub::prelude::physics::optics::interference::fringe_spacing(
            wavelength_m,
            self.slit_separation_m,
            self.screen_distance_m,
        )
    }

    pub fn intensity_at_angle(&self, theta_rad: f64, wavelength_m: f64) -> f64 {
        sf_diff::double_slit_intensity(theta_rad, self.slit_separation_m, wavelength_m)
    }
}
