pub fn vegf_diffusion_concentration(
    source_concentration: f64,
    distance: f64,
    diffusion_length: f64,
) -> f64 {
    source_concentration * (-distance / diffusion_length).exp()
}

pub fn sprouting_probability(vegf_concentration: f64, threshold: f64, hill_n: f64) -> f64 {
    let v = vegf_concentration.powf(hill_n);
    let t = threshold.powf(hill_n);
    v / (t + v)
}

pub fn vessel_spacing_optimal(
    oxygen_diffusion_coeff: f64,
    consumption_rate: f64,
    po2_arterial: f64,
) -> f64 {
    2.0 * (2.0 * oxygen_diffusion_coeff * po2_arterial / consumption_rate).sqrt()
}

pub fn krogh_cylinder_po2(
    po2_arterial: f64,
    consumption_rate: f64,
    diff_coeff: f64,
    r: f64,
    r_capillary: f64,
) -> f64 {
    po2_arterial
        - consumption_rate / (4.0 * diff_coeff)
            * (r * r
                - r_capillary * r_capillary
                - 2.0 * r_capillary * r_capillary * (r / r_capillary).ln())
}

pub fn angiogenic_switch_balance(pro_angiogenic: f64, anti_angiogenic: f64) -> f64 {
    pro_angiogenic - anti_angiogenic
}

pub fn capillary_density(num_capillaries: f64, tissue_area_mm2: f64) -> f64 {
    num_capillaries / tissue_area_mm2
}

pub fn endothelial_migration_speed(
    chemotactic_gradient: f64,
    sensitivity: f64,
    max_speed: f64,
) -> f64 {
    max_speed * chemotactic_gradient / (sensitivity + chemotactic_gradient)
}

pub fn prevascularization_survival(distance_to_vessel_um: f64, max_diffusion_distance: f64) -> f64 {
    if distance_to_vessel_um <= max_diffusion_distance {
        1.0
    } else {
        (-(distance_to_vessel_um - max_diffusion_distance) / max_diffusion_distance).exp()
    }
}

pub fn microvessel_wall_shear_stress(viscosity: f64, flow_rate: f64, radius: f64) -> f64 {
    4.0 * viscosity * flow_rate / (std::f64::consts::PI * radius.powi(3))
}
