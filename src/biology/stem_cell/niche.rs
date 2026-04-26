pub fn stem_cell_niche_occupancy(
    stem_cells: f64,
    niche_capacity: f64,
    adhesion_strength: f64,
) -> f64 {
    stem_cells * adhesion_strength / (niche_capacity + stem_cells * adhesion_strength)
}

pub fn niche_signal_gradient(source_strength: f64, distance: f64, decay_length: f64) -> f64 {
    source_strength * (-distance / decay_length).exp()
}

pub fn quiescence_probability(niche_signal: f64, threshold: f64) -> f64 {
    1.0 / (1.0 + (-(niche_signal - threshold) * 5.0).exp())
}

pub fn niche_asymmetric_division(niche_polarization: f64, cell_polarity: f64) -> f64 {
    (niche_polarization * cell_polarity).min(1.0)
}

pub fn hematopoietic_niche_osteoblast(
    osteoblast_count: f64,
    hsc_supported: f64,
    max_ratio: f64,
) -> f64 {
    (osteoblast_count * max_ratio).min(hsc_supported)
}

pub fn perivascular_niche_oxygen(
    distance_from_vessel: f64,
    vessel_po2: f64,
    consumption_rate: f64,
    diffusion: f64,
) -> f64 {
    (vessel_po2
        - consumption_rate * distance_from_vessel * distance_from_vessel / (4.0 * diffusion))
        .max(0.0)
}

pub fn intestinal_crypt_dynamics(
    stem_cells: f64,
    division_rate: f64,
    loss_rate: f64,
    niche_capacity: f64,
    dt: f64,
) -> f64 {
    let growth = division_rate * stem_cells * (1.0 - stem_cells / niche_capacity);
    let loss = loss_rate * stem_cells;
    (stem_cells + (growth - loss) * dt).max(0.0)
}

pub fn wnt_gradient_crypt(position: f64, crypt_depth: f64, wnt_max: f64) -> f64 {
    wnt_max * (1.0 - position / crypt_depth).max(0.0)
}

pub fn notch_lateral_inhibition_niche(notch_signal: f64, delta_neighbors: f64, gain: f64) -> f64 {
    gain * delta_neighbors / (1.0 + gain * delta_neighbors) * notch_signal
}

pub fn mesenchymal_niche_paracrine(
    mscs: f64,
    growth_factor_per_cell: f64,
    distance: f64,
    decay: f64,
) -> f64 {
    mscs * growth_factor_per_cell * (-distance * decay).exp()
}
