pub fn cell_proliferation_3d(
    cells: f64,
    growth_rate: f64,
    nutrient_factor: f64,
    carrying_capacity: f64,
    dt: f64,
) -> f64 {
    let growth = growth_rate * nutrient_factor * cells * (1.0 - cells / carrying_capacity);
    (cells + growth * dt).max(0.0)
}

pub fn oxygen_diffusion_krogh(
    r_tissue: f64,
    r_capillary: f64,
    consumption_rate: f64,
    diffusivity: f64,
    c_surface: f64,
) -> f64 {
    let r_ratio = r_tissue / r_capillary;
    c_surface
        - consumption_rate / (4.0 * diffusivity)
            * (r_tissue * r_tissue
                - r_capillary * r_capillary
                - 2.0 * r_capillary * r_capillary * r_ratio.ln())
}

pub fn nutrient_penetration_depth(
    diffusivity: f64,
    surface_concentration: f64,
    consumption_rate: f64,
) -> f64 {
    (2.0 * diffusivity * surface_concentration / consumption_rate).sqrt()
}

pub fn perfusion_bioreactor_shear(
    viscosity: f64,
    flow_rate: f64,
    channel_height: f64,
    channel_width: f64,
) -> f64 {
    6.0 * viscosity * flow_rate / (channel_width * channel_height * channel_height)
}

pub fn cell_migration_speed(traction_force: f64, drag_coefficient: f64) -> f64 {
    traction_force / drag_coefficient
}

pub fn tissue_maturation_index(
    collagen_content: f64,
    target_collagen: f64,
    mechanical_strength: f64,
    target_strength: f64,
) -> f64 {
    0.5 * (collagen_content / target_collagen).min(1.0)
        + 0.5 * (mechanical_strength / target_strength).min(1.0)
}

pub fn vascularization_density(vessel_length: f64, tissue_volume: f64) -> f64 {
    vessel_length / tissue_volume
}

pub fn extracellular_matrix_production(
    cell_density: f64,
    production_rate: f64,
    stimulus: f64,
    degradation_rate: f64,
    ecm: f64,
    dt: f64,
) -> f64 {
    let production = production_rate * cell_density * stimulus;
    let degradation = degradation_rate * ecm;
    (ecm + (production - degradation) * dt).max(0.0)
}

pub fn cell_sheet_contraction(
    initial_area: f64,
    contractility: f64,
    cell_density: f64,
    t: f64,
) -> f64 {
    initial_area * (-contractility * cell_density * t).exp()
}

pub fn mechanotransduction_response(strain: f64, threshold: f64, sensitivity: f64) -> f64 {
    if strain > threshold {
        sensitivity * (strain - threshold)
    } else {
        0.0
    }
}

pub fn angiogenic_sprouting_rate(vegf: f64, k_vegf: f64, tip_cell_density: f64) -> f64 {
    tip_cell_density * vegf / (k_vegf + vegf)
}

pub fn tissue_compaction(
    initial_volume: f64,
    cell_traction: f64,
    matrix_stiffness: f64,
    t: f64,
) -> f64 {
    initial_volume * (-cell_traction / matrix_stiffness.max(1e-30) * t).exp()
}

pub fn cell_differentiation_rate(transcription_factor: f64, threshold: f64, hill: f64) -> f64 {
    transcription_factor.powf(hill) / (threshold.powf(hill) + transcription_factor.powf(hill))
}

pub fn wound_healing_closure(
    gap_width: f64,
    migration_speed: f64,
    proliferation_rate: f64,
    t: f64,
) -> f64 {
    (gap_width - 2.0 * (migration_speed + proliferation_rate) * t).max(0.0)
}

pub fn nutrient_consumption_michaelis(concentration: f64, vmax: f64, km: f64) -> f64 {
    vmax * concentration / (km + concentration)
}

pub fn cell_viability_hypoxia(po2: f64, critical_po2: f64, sensitivity: f64) -> f64 {
    1.0 / (1.0 + (sensitivity * (critical_po2 - po2)).exp())
}

pub fn collagen_fiber_alignment(strain_direction: f64, fiber_angle: f64) -> f64 {
    (strain_direction - fiber_angle).cos().powi(2)
}

pub fn gag_content_cartilage(
    cell_density: f64,
    tgf_beta: f64,
    production_rate: f64,
    degradation_rate: f64,
    current: f64,
    dt: f64,
) -> f64 {
    let prod = production_rate * cell_density * tgf_beta / (tgf_beta + 1.0);
    (current + (prod - degradation_rate * current) * dt).max(0.0)
}
