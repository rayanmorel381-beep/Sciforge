pub fn diffusion_dispersal(density: f64, diffusion_coeff: f64, gradient: f64) -> f64 {
    -diffusion_coeff * gradient + density * 0.0
}

pub fn reaction_diffusion_fisher(n: f64, r: f64, k: f64, d: f64, laplacian: f64) -> f64 {
    r * n * (1.0 - n / k) + d * laplacian
}

pub fn fisher_wave_speed(r: f64, d: f64) -> f64 {
    2.0 * (r * d).sqrt()
}

pub fn range_expansion_rate(diffusion: f64, growth_rate: f64) -> f64 {
    2.0 * (diffusion * growth_rate).sqrt()
}

pub fn stepping_stone_migration(
    source_density: f64,
    target_density: f64,
    migration_rate: f64,
) -> f64 {
    migration_rate * (source_density - target_density)
}

pub fn isolation_by_distance(fst: f64) -> f64 {
    fst / (1.0 - fst).max(1e-30)
}

pub fn landscape_resistance(distance: f64, resistance_cost: f64) -> f64 {
    distance * resistance_cost
}

pub fn gravity_model_migration(pop_source: f64, pop_dest: f64, distance: f64, alpha: f64) -> f64 {
    alpha * pop_source * pop_dest / (distance * distance).max(1e-30)
}

pub fn corridor_effectiveness(
    width: f64,
    length: f64,
    habitat_quality: f64,
    species_mobility: f64,
) -> f64 {
    habitat_quality * (width / length.max(1e-30)) * (1.0 - (-species_mobility * width).exp())
}

pub fn allee_effect_spatial(density: f64, allee_threshold: f64, r: f64, k: f64) -> f64 {
    r * density * (density / allee_threshold - 1.0) * (1.0 - density / k)
}

pub fn kernel_based_dispersal(distance: f64, alpha: f64, shape: f64) -> f64 {
    let norm = shape / (2.0 * std::f64::consts::PI * alpha * alpha * (2.0 / shape));
    norm * (-(distance / alpha).powf(shape)).exp()
}
