use crate::lubrications::Grease;

pub fn hamrock_dowson_central_film_thickness_m(
    effective_radius_m: f64,
    entrainment_velocity_m_s: f64,
    dynamic_viscosity_pa_s: f64,
    pressure_viscosity_coefficient_per_pa: f64,
    effective_modulus_pa: f64,
    normal_load_n: f64,
    ellipticity_k: f64,
) -> f64 {
    let u = dynamic_viscosity_pa_s * entrainment_velocity_m_s
        / (effective_modulus_pa * effective_radius_m);
    let g = pressure_viscosity_coefficient_per_pa * effective_modulus_pa;
    let w = normal_load_n / (effective_modulus_pa * effective_radius_m * effective_radius_m);
    2.69 * effective_radius_m
        * u.powf(0.67)
        * g.powf(0.53)
        * w.powf(-0.067)
        * (1.0 - 0.61 * (-0.73 * ellipticity_k).exp())
}

pub fn hamrock_dowson_minimum_film_thickness_m(
    effective_radius_m: f64,
    entrainment_velocity_m_s: f64,
    dynamic_viscosity_pa_s: f64,
    pressure_viscosity_coefficient_per_pa: f64,
    effective_modulus_pa: f64,
    normal_load_n: f64,
    ellipticity_k: f64,
) -> f64 {
    let u = dynamic_viscosity_pa_s * entrainment_velocity_m_s
        / (effective_modulus_pa * effective_radius_m);
    let g = pressure_viscosity_coefficient_per_pa * effective_modulus_pa;
    let w = normal_load_n / (effective_modulus_pa * effective_radius_m * effective_radius_m);
    3.63 * effective_radius_m
        * u.powf(0.68)
        * g.powf(0.49)
        * w.powf(-0.073)
        * (1.0 - (-0.68 * ellipticity_k).exp())
}

pub fn lambda_film_parameter(
    minimum_film_thickness_m: f64,
    composite_roughness_rms_m: f64,
) -> f64 {
    minimum_film_thickness_m / composite_roughness_rms_m
}

pub fn composite_roughness_m(rq1_m: f64, rq2_m: f64) -> f64 {
    (rq1_m * rq1_m + rq2_m * rq2_m).sqrt()
}

#[derive(Debug, Clone, Copy)]
pub struct EhlContact {
    pub grease: Grease,
    pub effective_radius_m: f64,
    pub entrainment_velocity_m_s: f64,
    pub effective_modulus_pa: f64,
    pub normal_load_n: f64,
    pub pressure_viscosity_coefficient_per_pa: f64,
    pub ellipticity_k: f64,
    pub composite_roughness_rms_m: f64,
}

impl EhlContact {
    pub fn central_film_thickness_m(&self) -> f64 {
        hamrock_dowson_central_film_thickness_m(
            self.effective_radius_m,
            self.entrainment_velocity_m_s,
            self.grease.dynamic_viscosity_pa_s(),
            self.pressure_viscosity_coefficient_per_pa,
            self.effective_modulus_pa,
            self.normal_load_n,
            self.ellipticity_k,
        )
    }

    pub fn minimum_film_thickness_m(&self) -> f64 {
        hamrock_dowson_minimum_film_thickness_m(
            self.effective_radius_m,
            self.entrainment_velocity_m_s,
            self.grease.dynamic_viscosity_pa_s(),
            self.pressure_viscosity_coefficient_per_pa,
            self.effective_modulus_pa,
            self.normal_load_n,
            self.ellipticity_k,
        )
    }

    pub fn lambda_ratio(&self) -> f64 {
        lambda_film_parameter(self.minimum_film_thickness_m(), self.composite_roughness_rms_m)
    }
}
