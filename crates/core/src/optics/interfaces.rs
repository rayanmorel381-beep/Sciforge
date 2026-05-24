use crate::moleculars::{Gas, Liquid, Material};
use sciforge_hub::prelude::physics::optics::refraction as sf_refr;

pub trait OpticalMedium {
    fn refractive_index_value(&self) -> Option<f64>;
}

impl OpticalMedium for Material {
    fn refractive_index_value(&self) -> Option<f64> {
        self.refractive_index()
    }
}

impl OpticalMedium for Liquid {
    fn refractive_index_value(&self) -> Option<f64> {
        self.refractive_index()
    }
}

impl OpticalMedium for Gas {
    fn refractive_index_value(&self) -> Option<f64> {
        self.refractive_index()
    }
}

pub fn snell_angle_rad<A: OpticalMedium, B: OpticalMedium>(
    medium_in: &A,
    incidence_rad: f64,
    medium_out: &B,
) -> Option<f64> {
    let n1 = medium_in.refractive_index_value()?;
    let n2 = medium_out.refractive_index_value()?;
    Some(sf_refr::snell(n1, incidence_rad, n2))
}

pub fn critical_angle_rad<A: OpticalMedium, B: OpticalMedium>(
    medium_dense: &A,
    medium_light: &B,
) -> Option<f64> {
    let n1 = medium_dense.refractive_index_value()?;
    let n2 = medium_light.refractive_index_value()?;
    Some(sf_refr::critical_angle(n1, n2))
}

pub fn brewster_angle_rad<A: OpticalMedium, B: OpticalMedium>(
    medium_in: &A,
    medium_out: &B,
) -> Option<f64> {
    let n1 = medium_in.refractive_index_value()?;
    let n2 = medium_out.refractive_index_value()?;
    Some(sf_refr::brewster_angle(n1, n2))
}

pub fn fresnel_reflectance_s<A: OpticalMedium, B: OpticalMedium>(
    medium_in: &A,
    incidence_rad: f64,
    medium_out: &B,
) -> Option<f64> {
    let n1 = medium_in.refractive_index_value()?;
    let n2 = medium_out.refractive_index_value()?;
    let theta_t = sf_refr::snell(n1, incidence_rad, n2);
    Some(sf_refr::fresnel_reflectance_s(n1, incidence_rad, n2, theta_t))
}

pub fn fresnel_reflectance_p<A: OpticalMedium, B: OpticalMedium>(
    medium_in: &A,
    incidence_rad: f64,
    medium_out: &B,
) -> Option<f64> {
    let n1 = medium_in.refractive_index_value()?;
    let n2 = medium_out.refractive_index_value()?;
    let theta_t = sf_refr::snell(n1, incidence_rad, n2);
    Some(sf_refr::fresnel_reflectance_p(n1, incidence_rad, n2, theta_t))
}

pub fn fresnel_reflectance_unpolarized<A: OpticalMedium, B: OpticalMedium>(
    medium_in: &A,
    incidence_rad: f64,
    medium_out: &B,
) -> Option<f64> {
    let rs = fresnel_reflectance_s(medium_in, incidence_rad, medium_out)?;
    let rp = fresnel_reflectance_p(medium_in, incidence_rad, medium_out)?;
    Some(0.5 * (rs + rp))
}

pub fn transmittance<A: OpticalMedium, B: OpticalMedium>(
    medium_in: &A,
    incidence_rad: f64,
    medium_out: &B,
) -> Option<f64> {
    fresnel_reflectance_unpolarized(medium_in, incidence_rad, medium_out).map(|r| 1.0 - r)
}
