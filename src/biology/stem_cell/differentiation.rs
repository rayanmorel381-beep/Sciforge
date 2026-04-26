pub fn waddington_landscape_potential(
    state: f64,
    attractor_a: f64,
    attractor_b: f64,
    barrier: f64,
) -> f64 {
    (state - attractor_a).powi(2) * (state - attractor_b).powi(2) + barrier * state
}

pub fn differentiation_commitment(
    transcription_factor_a: f64,
    transcription_factor_b: f64,
    hill: f64,
) -> f64 {
    transcription_factor_a.powf(hill)
        / (transcription_factor_a.powf(hill) + transcription_factor_b.powf(hill)).max(1e-30)
}

pub fn lineage_progression(
    progenitor: f64,
    differentiation_rate: f64,
    proliferation_rate: f64,
    dt: f64,
) -> (f64, f64) {
    let diff = differentiation_rate * progenitor;
    let prolif = proliferation_rate * progenitor;
    let new_progenitor = (progenitor + (prolif - diff) * dt).max(0.0);
    let differentiated = diff * dt;
    (new_progenitor, differentiated)
}

pub fn multipotency_index(expressed_lineage_genes: &[f64]) -> f64 {
    if expressed_lineage_genes.is_empty() {
        return 0.0;
    }
    let total: f64 = expressed_lineage_genes.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }
    let mut entropy = 0.0;
    for &g in expressed_lineage_genes {
        if g > 0.0 {
            let p = g / total;
            entropy -= p * p.ln();
        }
    }
    entropy / (expressed_lineage_genes.len() as f64).ln()
}

pub fn cell_fate_probability_stochastic(tf_level: f64, noise: f64, threshold: f64) -> f64 {
    1.0 / (1.0 + (-((tf_level + noise) - threshold) * 5.0).exp())
}

pub fn directed_differentiation_efficiency(target_markers: f64, total_cells: f64) -> f64 {
    target_markers / total_cells.max(1e-30)
}

pub fn transdifferentiation_barrier(
    epigenetic_distance: f64,
    reprogramming_factors: f64,
    efficiency_base: f64,
) -> f64 {
    efficiency_base * (-epigenetic_distance / reprogramming_factors.max(1e-30)).exp()
}

pub fn organoid_differentiation_layers(time: f64, layer_rate: f64, max_layers: f64) -> f64 {
    max_layers * (1.0 - (-layer_rate * time).exp())
}

pub fn terminal_differentiation_irreversibility(rb_phosphorylation: f64, cdki_level: f64) -> f64 {
    cdki_level / (cdki_level + rb_phosphorylation).max(1e-30)
}
