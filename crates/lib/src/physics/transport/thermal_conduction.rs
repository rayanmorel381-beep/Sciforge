pub fn fourier_law_heat_flux(
    thermal_conductivity_w_per_m_k: f64,
    temperature_gradient_k_per_m: f64,
) -> f64 {
    -thermal_conductivity_w_per_m_k * temperature_gradient_k_per_m
}

pub fn one_d_steady_conduction(
    thermal_conductivity_w_per_m_k: f64,
    area_m2: f64,
    delta_t_k: f64,
    thickness_m: f64,
) -> f64 {
    thermal_conductivity_w_per_m_k * area_m2 * delta_t_k / thickness_m
}

pub fn thermal_diffusivity(
    thermal_conductivity_w_per_m_k: f64,
    density_kg_per_m3: f64,
    specific_heat_j_per_kg_k: f64,
) -> f64 {
    thermal_conductivity_w_per_m_k / (density_kg_per_m3 * specific_heat_j_per_kg_k)
}

pub fn biot_number(
    convective_coefficient_w_per_m2_k: f64,
    characteristic_length_m: f64,
    thermal_conductivity_w_per_m_k: f64,
) -> f64 {
    convective_coefficient_w_per_m2_k * characteristic_length_m / thermal_conductivity_w_per_m_k
}

pub fn fourier_number(
    thermal_diffusivity_m2_per_s: f64,
    time_s: f64,
    characteristic_length_m: f64,
) -> f64 {
    thermal_diffusivity_m2_per_s * time_s / (characteristic_length_m * characteristic_length_m)
}

pub fn series_thermal_resistance(thicknesses_m: &[f64], conductivities_w_per_m_k: &[f64]) -> f64 {
    thicknesses_m
        .iter()
        .zip(conductivities_w_per_m_k.iter())
        .map(|(l, k)| l / k)
        .sum()
}
