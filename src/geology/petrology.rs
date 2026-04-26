use crate::constants::R_GAS;

pub fn cipw_quartz_norm(sio2: f64, feldspars: f64, mafics: f64) -> f64 {
    (sio2 - feldspars - mafics).max(0.0)
}

pub fn mg_number(mgo: f64, feo: f64) -> f64 {
    let mg = mgo / 40.3;
    let fe = feo / 71.85;
    mg / (mg + fe) * 100.0
}

pub fn differentiation_index(q: f64, or_val: f64, ab: f64, ne: f64) -> f64 {
    q + or_val + ab + ne
}

pub fn total_alkali_silica(na2o: f64, k2o: f64) -> f64 {
    na2o + k2o
}

pub fn alumina_saturation_index(al2o3: f64, cao: f64, na2o: f64, k2o: f64) -> f64 {
    let al = al2o3 / 102.0;
    let ca = cao / 56.0;
    let na = na2o / 62.0;
    let k = k2o / 94.0;
    al / (ca + na + k)
}

pub fn color_index(mafic_minerals: &[f64]) -> f64 {
    mafic_minerals.iter().sum()
}

pub fn liquidus_temperature(composition: f64, t_melt_a: f64, t_melt_b: f64) -> f64 {
    t_melt_a * composition + t_melt_b * (1.0 - composition)
}

pub fn solidus_depression(water_content: f64, base_solidus: f64, k: f64) -> f64 {
    base_solidus - k * water_content.powf(0.6)
}

pub fn crystal_settling_velocity(delta_rho: f64, g: f64, r: f64, mu: f64) -> f64 {
    2.0 * delta_rho * g * r * r / (9.0 * mu)
}

pub fn viscosity_arrhenius(a: f64, ea: f64, t: f64) -> f64 {
    a * (ea / (R_GAS * t)).exp()
}
