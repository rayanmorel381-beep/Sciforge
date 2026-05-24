pub fn heat_flux_w_m2(h_w_m2k: f64, delta_t_k: f64) -> f64 {
    h_w_m2k * delta_t_k
}

pub fn convection_resistance_k_w(h_w_m2k: f64, area_m2: f64) -> f64 {
    1.0 / (h_w_m2k * area_m2)
}

pub fn convection_coefficient_w_m2k(nusselt: f64, conductivity_w_mk: f64, length_m: f64) -> f64 {
    nusselt * conductivity_w_mk / length_m
}

pub fn nusselt_dittus_boelter_heating(reynolds: f64, prandtl: f64) -> f64 {
    0.023 * reynolds.powf(0.8) * prandtl.powf(0.4)
}

pub fn nusselt_dittus_boelter_cooling(reynolds: f64, prandtl: f64) -> f64 {
    0.023 * reynolds.powf(0.8) * prandtl.powf(0.3)
}

pub fn nusselt_sieder_tate(reynolds: f64, prandtl: f64, viscosity_ratio: f64) -> f64 {
    0.027 * reynolds.powf(0.8) * prandtl.powf(1.0 / 3.0) * viscosity_ratio.powf(0.14)
}

pub fn nusselt_gnielinski(reynolds: f64, prandtl: f64, friction_factor: f64) -> f64 {
    let f = friction_factor;
    (f / 8.0) * (reynolds - 1000.0) * prandtl
        / (1.0 + 12.7 * (f / 8.0).sqrt() * (prandtl.powf(2.0 / 3.0) - 1.0))
}

pub fn nusselt_churchill_bernstein(reynolds: f64, prandtl: f64) -> f64 {
    let re_pr = reynolds * prandtl;
    if re_pr < 0.2 {
        return f64::NAN;
    }
    0.3 + (0.62 * reynolds.powf(0.5) * prandtl.powf(1.0 / 3.0))
        / (1.0 + (0.4 / prandtl).powf(2.0 / 3.0)).powf(0.25)
        * (1.0 + (reynolds / 282000.0).powf(5.0 / 8.0)).powf(4.0 / 5.0)
}

pub fn darcy_friction_factor_laminar(reynolds: f64) -> f64 {
    64.0 / reynolds
}

pub fn darcy_friction_factor_colebrook(reynolds: f64, roughness_ratio: f64) -> f64 {
    let mut f = 0.02_f64;
    for _ in 0..50 {
        let rhs = -2.0
            * (roughness_ratio / 3.7 + 2.51 / (reynolds * f.sqrt())).log10();
        f = 1.0 / (rhs * rhs);
    }
    f
}
