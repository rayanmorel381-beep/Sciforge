pub fn radiation_shielding_half_value(initial_intensity: f64, hvl: f64, thickness: f64) -> f64 {
    initial_intensity * (0.5_f64).powf(thickness / hvl)
}

pub fn shielding_tenth_value(initial_intensity: f64, tvl: f64, thickness: f64) -> f64 {
    initial_intensity * (0.1_f64).powf(thickness / tvl)
}

pub fn mass_attenuation(intensity: f64, mu_over_rho: f64, density: f64, thickness: f64) -> f64 {
    intensity * (-mu_over_rho * density * thickness).exp()
}

pub fn buildup_factor(beam_layers: f64, mu: f64, thickness: f64) -> f64 {
    1.0 + beam_layers * mu * thickness
}

pub fn concrete_shielding_thickness(dose_rate: f64, dose_limit: f64, hvl: f64) -> f64 {
    if dose_rate <= dose_limit {
        return 0.0;
    }
    hvl * (dose_rate / dose_limit).log2()
}

pub fn lead_equivalent_thickness(mu_material: f64, mu_lead: f64, thickness_material: f64) -> f64 {
    mu_material * thickness_material / mu_lead.max(1e-30)
}

pub fn inverse_square_distance(dose_at_d1: f64, d1: f64, d2: f64) -> f64 {
    dose_at_d1 * (d1 / d2).powi(2)
}

pub fn occupancy_factor_dose(dose_unshielded: f64, occupancy: f64, use_factor: f64) -> f64 {
    dose_unshielded * occupancy * use_factor
}

pub fn neutron_shielding_hydrogen(thickness_cm: f64, cross_section: f64, density_h: f64) -> f64 {
    (-cross_section * density_h * thickness_cm).exp()
}

pub fn annual_dose_limit_check(dose_received: f64, dose_limit: f64) -> f64 {
    (dose_limit - dose_received).max(0.0)
}
