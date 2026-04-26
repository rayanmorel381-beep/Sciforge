pub fn tumor_angiogenesis_vegf(vegf: f64, endothelial_proliferation_rate: f64, kd: f64) -> f64 {
    endothelial_proliferation_rate * vegf / (kd + vegf)
}

pub fn vessel_density(
    new_vessels: f64,
    existing_vessels: f64,
    regression_rate: f64,
    dt: f64,
) -> f64 {
    (existing_vessels + (new_vessels - regression_rate * existing_vessels) * dt).max(0.0)
}

pub fn oxygen_diffusion_krogh(
    p_vessel: f64,
    consumption_rate: f64,
    diffusion_coeff: f64,
    r: f64,
    r_vessel: f64,
) -> f64 {
    p_vessel
        - consumption_rate / (4.0 * diffusion_coeff)
            * (r * r - r_vessel * r_vessel - 2.0 * r_vessel * r_vessel * (r / r_vessel).ln())
}

pub fn hypoxia_fraction(distances: &[f64], diffusion_limit: f64) -> f64 {
    if distances.is_empty() {
        return 0.0;
    }
    let hypoxic = distances.iter().filter(|&&d| d > diffusion_limit).count();
    hypoxic as f64 / distances.len() as f64
}

pub fn microenvironment_tmb(mutations: usize, megabases_sequenced: f64) -> f64 {
    mutations as f64 / megabases_sequenced.max(1e-30)
}

pub fn clonal_fitness_advantage(clone_sizes: &[f64], fitness_values: &[f64]) -> f64 {
    let n = clone_sizes.len().min(fitness_values.len());
    let total: f64 = clone_sizes.iter().take(n).sum();
    if total < 1e-30 {
        return 0.0;
    }
    (0..n)
        .map(|i| clone_sizes[i] / total * fitness_values[i])
        .sum()
}

pub fn tumor_heterogeneity_shannon(clone_fractions: &[f64]) -> f64 {
    let mut h = 0.0;
    for &p in clone_fractions {
        if p > 0.0 {
            h -= p * p.ln();
        }
    }
    h
}

pub fn metastatic_probability(
    invasion_rate: f64,
    survival_fraction: f64,
    colonization_rate: f64,
    time: f64,
) -> f64 {
    1.0 - (-(invasion_rate * survival_fraction * colonization_rate * time)).exp()
}

pub fn emt_score(epithelial_markers: &[f64], mesenchymal_markers: &[f64]) -> f64 {
    let e_sum: f64 = epithelial_markers.iter().sum();
    let m_sum: f64 = mesenchymal_markers.iter().sum();
    m_sum / (e_sum + m_sum).max(1e-30)
}

pub fn immune_escape_probability(mhc_expression: f64, pd_l1: f64, neoantigen_load: f64) -> f64 {
    let immune_visibility = mhc_expression * neoantigen_load;
    let escape = pd_l1 / (pd_l1 + 1.0);
    1.0 - immune_visibility * (1.0 - escape)
}

pub fn csc_fraction(
    symmetric_division_rate: f64,
    asymmetric_rate: f64,
    differentiation_rate: f64,
) -> f64 {
    symmetric_division_rate
        / (symmetric_division_rate + asymmetric_rate + differentiation_rate).max(1e-30)
}

pub fn pharmacokinetic_tumor_exposure(
    dose: f64,
    bioavailability: f64,
    volume_distribution: f64,
    tumor_perfusion_fraction: f64,
) -> f64 {
    dose * bioavailability * tumor_perfusion_fraction / volume_distribution.max(1e-30)
}
