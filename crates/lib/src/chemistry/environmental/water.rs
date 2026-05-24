pub fn biochemical_oxygen_demand(bod_ultimate: f64, k: f64, t: f64) -> f64 {
    bod_ultimate * (1.0 - (-k * t).exp())
}

pub fn chemical_oxygen_demand(sample_oxygen: f64, blank_oxygen: f64, volume: f64) -> f64 {
    (blank_oxygen - sample_oxygen) / volume.max(1e-30)
}

pub fn dissolved_oxygen_saturation(t: f64) -> f64 {
    14.62 - 0.3898 * t + 0.006969 * t * t - 5.897e-5 * t * t * t
}

pub fn streeter_phelps(d0: f64, l0: f64, kd: f64, kr: f64, t: f64) -> f64 {
    kd * l0 / (kr - kd).max(1e-30) * ((-kd * t).exp() - (-kr * t).exp()) + d0 * (-kr * t).exp()
}

pub fn critical_point_time(kd: f64, kr: f64, d0: f64, l0: f64) -> f64 {
    if (kr - kd).abs() < 1e-30 {
        return 0.0;
    }
    1.0 / (kr - kd)
        * (kr / kd * (1.0 - d0 * (kr - kd) / (kd * l0).max(1e-30)))
            .max(1e-30)
            .ln()
}

pub fn chlorine_decay(c0: f64, k: f64, t: f64) -> f64 {
    c0 * (-k * t).exp()
}

pub fn ct_disinfection(c: f64, t: f64) -> f64 {
    c * t
}

pub fn hardness_total(ca_mg_l: f64, mg_mg_l: f64) -> f64 {
    ca_mg_l * 2.497 + mg_mg_l * 4.118
}

pub fn langelier_saturation_index(ph: f64, ph_s: f64) -> f64 {
    ph - ph_s
}

pub fn total_dissolved_solids_from_conductivity(conductivity_us: f64) -> f64 {
    conductivity_us * 0.65
}
