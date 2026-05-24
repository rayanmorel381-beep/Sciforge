use crate::constants::TUMOR_CARRYING_CAPACITY;

pub fn tumor_growth_gompertz(n: f64, n_max: f64, alpha: f64, dt: f64) -> f64 {
    let growth = alpha * n * (n_max / n).ln();
    (n + growth * dt).max(0.0)
}

pub fn tumor_growth_logistic(n: f64, k: f64, r: f64, dt: f64) -> f64 {
    let growth = r * n * (1.0 - n / k);
    (n + growth * dt).max(0.0)
}

pub fn tumor_doubling_time(growth_rate: f64) -> f64 {
    (2.0_f64).ln() / growth_rate
}

pub fn clonal_evolution_fitness(
    clone_sizes: &[f64],
    fitness: &[f64],
    mutation_rate: f64,
) -> Vec<f64> {
    let total: f64 = clone_sizes.iter().sum();
    clone_sizes
        .iter()
        .zip(fitness.iter())
        .map(|(&n, &f)| {
            let growth = f * n * (1.0 - total / TUMOR_CARRYING_CAPACITY);
            let mutation_loss = mutation_rate * n;
            (n + growth - mutation_loss).max(0.0)
        })
        .collect()
}

pub fn metastasis_probability(
    tumor_size: f64,
    shedding_rate: f64,
    survival_fraction: f64,
    colonization_rate: f64,
) -> f64 {
    1.0 - (-(shedding_rate * tumor_size * survival_fraction * colonization_rate)).exp()
}

pub fn tumor_angiogenesis_rate(
    tumor_size: f64,
    vegf_production: f64,
    inhibitor: f64,
    threshold: f64,
) -> f64 {
    let signal = vegf_production * tumor_size - inhibitor;
    if signal > threshold {
        signal - threshold
    } else {
        0.0
    }
}

pub fn norton_simon_regression(n: f64, kill_fraction: f64, gompertz_rate: f64, n_max: f64) -> f64 {
    let effective_kill = kill_fraction * gompertz_rate * (n_max / n.max(1.0)).ln();
    (n * (-effective_kill).exp()).max(0.0)
}

pub fn tumor_growth_exponential(n0: f64, rate: f64, t: f64) -> f64 {
    n0 * (rate * t).exp()
}

pub fn tumor_growth_von_bertalanffy(n: f64, a: f64, b: f64, dt: f64) -> f64 {
    let growth = a * n.powf(2.0 / 3.0) - b * n;
    (n + growth * dt).max(0.0)
}

pub fn tumor_volume_spherical(diameter: f64) -> f64 {
    std::f64::consts::PI / 6.0 * diameter.powi(3)
}

pub fn tumor_volume_ellipsoid(length: f64, width: f64, height: f64) -> f64 {
    std::f64::consts::PI / 6.0 * length * width * height
}

pub fn recist_response(baseline_diameter: f64, current_diameter: f64) -> f64 {
    (current_diameter - baseline_diameter) / baseline_diameter * 100.0
}

pub fn tumor_mutation_burden(somatic_mutations: f64, exome_size_mb: f64) -> f64 {
    somatic_mutations / exome_size_mb
}

pub fn heterogeneity_shannon(clone_fractions: &[f64]) -> f64 {
    clone_fractions
        .iter()
        .filter(|&&p| p > 0.0)
        .map(|&p| -p * p.ln())
        .sum()
}

pub fn circulating_tumor_cells(shedding: f64, tumor_size: f64, half_life: f64) -> f64 {
    shedding * tumor_size * half_life / std::f64::consts::LN_2
}

pub fn warburg_glycolysis_rate(
    glucose: f64,
    vmax: f64,
    km: f64,
    oxygen_inhibition: f64,
    oxygen: f64,
) -> f64 {
    vmax * glucose / (km + glucose) * km / (km + oxygen_inhibition * oxygen)
}

pub fn hypoxia_inducible_factor(po2: f64, km_o2: f64, max_expression: f64) -> f64 {
    max_expression * km_o2 / (km_o2 + po2)
}

pub fn necrotic_core_radius(tumor_radius: f64, diffusion_length: f64) -> f64 {
    (tumor_radius - diffusion_length).max(0.0)
}

pub fn viable_rim_fraction(tumor_radius: f64, necrotic_radius: f64) -> f64 {
    if tumor_radius <= 0.0 {
        return 0.0;
    }
    1.0 - (necrotic_radius / tumor_radius).powi(3)
}

pub fn ctc_cluster_survival(single_ctc_survival: f64, cluster_size: u32) -> f64 {
    1.0 - (1.0 - single_ctc_survival).powi(cluster_size as i32)
}

pub fn invasion_index(invaded_distance: f64, time: f64) -> f64 {
    invaded_distance / time.max(1e-30)
}

pub fn epithelial_mesenchymal_transition(tgf_beta: f64, threshold: f64, hill: f64) -> f64 {
    tgf_beta.powf(hill) / (threshold.powf(hill) + tgf_beta.powf(hill))
}

pub fn microsatellite_instability_score(unstable_markers: u32, total_markers: u32) -> f64 {
    unstable_markers as f64 / total_markers.max(1) as f64
}
