pub fn dispersal_kernel_exponential(distance: f64, mean_dispersal: f64) -> f64 {
    (1.0 / mean_dispersal) * (-distance / mean_dispersal).exp()
}

pub fn dispersal_kernel_2dt(distance: f64, a: f64, p: f64) -> f64 {
    (p - 1.0) / (std::f64::consts::PI * a * a) * (1.0 + distance * distance / (a * a)).powf(-p)
}

pub fn range_shift_rate(warming_rate: f64, lapse_rate: f64) -> f64 {
    warming_rate / lapse_rate.abs().max(1e-30)
}

pub fn latitudinal_gradient(species_tropical: f64, species_polar: f64, lat_range: f64) -> f64 {
    (species_tropical - species_polar) / lat_range
}

pub fn altitudinal_gradient(species_low: f64, species_high: f64, alt_range: f64) -> f64 {
    (species_low - species_high) / alt_range
}

pub fn bioclimatic_envelope(
    temp: f64,
    precip: f64,
    temp_min: f64,
    temp_max: f64,
    precip_min: f64,
    precip_max: f64,
) -> f64 {
    if temp < temp_min || temp > temp_max || precip < precip_min || precip > precip_max {
        return 0.0;
    }
    let t_suit =
        1.0 - ((temp - (temp_min + temp_max) / 2.0) / ((temp_max - temp_min) / 2.0)).powi(2);
    let p_suit = 1.0
        - ((precip - (precip_min + precip_max) / 2.0) / ((precip_max - precip_min) / 2.0)).powi(2);
    (t_suit * p_suit).max(0.0)
}

pub fn species_area_relationship(c: f64, z: f64, area: f64) -> f64 {
    c * area.powf(z)
}

pub fn endemism_index(endemic_species: usize, total_species: usize) -> f64 {
    if total_species == 0 {
        return 0.0;
    }
    endemic_species as f64 / total_species as f64
}

pub fn climate_velocity(temp_change_rate: f64, spatial_gradient: f64) -> f64 {
    temp_change_rate / spatial_gradient.abs().max(1e-30)
}

pub fn habitat_suitability_index(variables: &[f64], optima: &[f64], tolerances: &[f64]) -> f64 {
    let n = variables.len().min(optima.len()).min(tolerances.len());
    let mut product = 1.0;
    for i in 0..n {
        let deviation = (variables[i] - optima[i]) / tolerances[i].max(1e-30);
        product *= (-0.5 * deviation * deviation).exp();
    }
    product
}

pub fn island_equilibrium_richness(
    immigration_max: f64,
    extinction_max: f64,
    area: f64,
    distance: f64,
) -> f64 {
    let i = immigration_max * (-distance / 1000.0).exp();
    let e = extinction_max / area.max(1e-30);
    i / (i + e) * immigration_max
}

pub fn nestedness_temperature(presence_matrix: &[Vec<bool>]) -> f64 {
    let n_rows = presence_matrix.len();
    if n_rows == 0 {
        return 0.0;
    }
    let n_cols = presence_matrix[0].len();
    let row_fills: Vec<f64> = presence_matrix
        .iter()
        .map(|row| row.iter().filter(|&&x| x).count() as f64 / n_cols as f64)
        .collect();
    let col_fills: Vec<f64> = (0..n_cols)
        .map(|j| {
            presence_matrix
                .iter()
                .filter(|row| j < row.len() && row[j])
                .count() as f64
                / n_rows as f64
        })
        .collect();
    let mut unexpected: f64 = 0.0;
    let mut total: f64 = 0.0;
    for (i, row) in presence_matrix.iter().enumerate() {
        for (j, &present) in row.iter().enumerate() {
            let expected = row_fills[i] * col_fills[j];
            if present && expected < 0.5 {
                unexpected += 1.0;
            }
            total += 1.0;
        }
    }
    1.0 - unexpected / total.max(1.0)
}

pub fn mid_domain_effect(domain_size: f64, range_size: f64) -> f64 {
    let max_richness = domain_size - range_size + 1.0;
    max_richness.max(0.0)
}

pub fn beta_diversity_whittaker(gamma: f64, alpha_mean: f64) -> f64 {
    gamma / alpha_mean.max(1e-30) - 1.0
}

pub fn beta_diversity_sorensen(shared: f64, unique_a: f64, unique_b: f64) -> f64 {
    2.0 * shared / (2.0 * shared + unique_a + unique_b).max(1e-30)
}

pub fn rapoport_rule(range_sizes: &[f64], latitudes: &[f64]) -> f64 {
    let n = range_sizes.len().min(latitudes.len());
    if n < 2 {
        return 0.0;
    }
    let x_mean = latitudes.iter().take(n).sum::<f64>() / n as f64;
    let y_mean = range_sizes.iter().take(n).sum::<f64>() / n as f64;
    let mut num = 0.0;
    let mut den = 0.0;
    for i in 0..n {
        let dx = latitudes[i] - x_mean;
        num += dx * (range_sizes[i] - y_mean);
        den += dx * dx;
    }
    if den < 1e-30 { 0.0 } else { num / den }
}

pub fn occupancy_frequency(presences: &[bool]) -> f64 {
    if presences.is_empty() {
        return 0.0;
    }
    presences.iter().filter(|&&p| p).count() as f64 / presences.len() as f64
}
