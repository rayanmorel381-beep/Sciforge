pub fn species_range_overlap(range_a: (f64, f64), range_b: (f64, f64)) -> f64 {
    let lo = range_a.0.max(range_b.0);
    let hi = range_a.1.min(range_b.1);
    (hi - lo).max(0.0)
}

pub fn range_size_latitude(area: f64) -> f64 {
    area
}

pub fn elevational_diversity_gradient(
    elevation: f64,
    peak_elevation: f64,
    max_richness: f64,
) -> f64 {
    let x = (elevation - peak_elevation).abs() / peak_elevation;
    max_richness * (-x * x * 2.0).exp()
}

pub fn biome_niche_model(
    temperature: f64,
    precipitation: f64,
    t_opt: f64,
    p_opt: f64,
    t_width: f64,
    p_width: f64,
) -> f64 {
    let t_term = -((temperature - t_opt) / t_width).powi(2);
    let p_term = -((precipitation - p_opt) / p_width).powi(2);
    (t_term + p_term).exp()
}

pub fn regional_endemism_index(endemic_species: usize, total_species: usize) -> f64 {
    if total_species == 0 {
        return 0.0;
    }
    endemic_species as f64 / total_species as f64
}

pub fn latitudinal_diversity_gradient(latitude: f64, max_richness: f64, steepness: f64) -> f64 {
    max_richness * (-steepness * latitude.abs()).exp()
}

pub fn range_shift_velocity(temp_change_rate: f64, spatial_temp_gradient: f64) -> f64 {
    temp_change_rate / spatial_temp_gradient.abs().max(1e-30)
}

pub fn climate_envelope_suitability(
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
    let t_suit = 1.0 - 2.0 * ((temp - (temp_min + temp_max) / 2.0) / (temp_max - temp_min)).abs();
    let p_suit =
        1.0 - 2.0 * ((precip - (precip_min + precip_max) / 2.0) / (precip_max - precip_min)).abs();
    t_suit * p_suit
}

pub fn refugia_persistence(area: f64, min_viable_area: f64, climate_stability: f64) -> f64 {
    if area < min_viable_area {
        return 0.0;
    }
    1.0 - (-climate_stability * area / min_viable_area).exp()
}

pub fn wallace_line_effect(dispersal_ability: f64, barrier_width: f64) -> f64 {
    (-barrier_width / dispersal_ability.max(1e-30)).exp()
}
