use crate::moleculars::Material;
use sciforge_hub::prelude::constants::C;
use sciforge_hub::prelude::constants::chemistry::molecular::refractive_index;
use sciforge_hub::prelude::physics::optics::refraction as sf_refr;

impl Material {
    pub fn refractive_index(&self) -> Option<f64> {
        refractive_index::by_formula_phase(self.formula, "solid").map(|r| r.n_d)
    }

    pub fn speed_of_light_in_m_s(&self) -> Option<f64> {
        self.refractive_index().map(|n| C / n)
    }

    pub fn optical_path_length_m(&self, geometric_length_m: f64) -> Option<f64> {
        self.refractive_index()
            .map(|n| sf_refr::optical_path_length(n, geometric_length_m))
    }

    pub fn critical_angle_from_air_rad(&self) -> Option<f64> {
        self.refractive_index().map(|n| sf_refr::critical_angle(n, 1.0))
    }

    pub fn brewster_angle_from_air_rad(&self) -> Option<f64> {
        self.refractive_index().map(|n| sf_refr::brewster_angle(1.0, n))
    }

    pub fn snell_refracted_angle_from_air_rad(&self, incidence_rad: f64) -> Option<f64> {
        self.refractive_index()
            .map(|n| sf_refr::snell(1.0, incidence_rad, n))
    }

    pub fn fresnel_reflectance_s_from_air(&self, incidence_rad: f64) -> Option<f64> {
        let n = self.refractive_index()?;
        let theta_t = sf_refr::snell(1.0, incidence_rad, n);
        Some(sf_refr::fresnel_reflectance_s(1.0, incidence_rad, n, theta_t))
    }

    pub fn fresnel_reflectance_p_from_air(&self, incidence_rad: f64) -> Option<f64> {
        let n = self.refractive_index()?;
        let theta_t = sf_refr::snell(1.0, incidence_rad, n);
        Some(sf_refr::fresnel_reflectance_p(1.0, incidence_rad, n, theta_t))
    }

    pub fn fresnel_reflectance_normal_from_air(&self) -> Option<f64> {
        self.refractive_index().map(|n| {
            let r = (n - 1.0) / (n + 1.0);
            r * r
        })
    }

    pub fn lensmaker_focal_length_m(&self, radius_1_m: f64, radius_2_m: f64) -> Option<f64> {
        let n = self.refractive_index()?;
        let inv_f = sf_refr::lensmaker_equation(n, radius_1_m, radius_2_m);
        Some(1.0 / inv_f)
    }
}
