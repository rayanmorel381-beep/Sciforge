pub fn lennard_jones_potential(r: f64, epsilon: f64, sigma: f64) -> f64 {
    4.0 * epsilon * ((sigma / r).powi(12) - (sigma / r).powi(6))
}

pub fn coulomb_interaction(q1: f64, q2: f64, r: f64, dielectric: f64) -> f64 {
    332.0 * q1 * q2 / (dielectric * r)
}

pub fn desolvation_energy(buried_area: f64, solvation_parameter: f64) -> f64 {
    solvation_parameter * buried_area
}

pub fn shape_complementarity(interface_area: f64, gap_volume: f64) -> f64 {
    1.0 - gap_volume / interface_area.max(1e-30)
}

pub fn binding_free_energy(
    van_der_waals: f64,
    electrostatic: f64,
    desolvation: f64,
    entropy_penalty: f64,
) -> f64 {
    van_der_waals + electrostatic + desolvation + entropy_penalty
}

pub fn kd_from_delta_g(delta_g: f64, temperature: f64) -> f64 {
    let rt = 0.001987 * temperature;
    (delta_g / rt).exp()
}

pub fn buried_surface_area(asa_complex: f64, asa_receptor: f64, asa_ligand: f64) -> f64 {
    asa_receptor + asa_ligand - asa_complex
}

pub fn hydrogen_bond_energy(distance: f64, angle_deg: f64) -> f64 {
    let optimal_distance = 2.8;
    let angle = angle_deg.to_radians();
    let dist_factor = (-(distance - optimal_distance).powi(2) / 0.5).exp();
    let angle_factor = angle.cos().powi(2);
    -2.0 * dist_factor * angle_factor
}

pub fn clash_score(distances: &[f64], vdw_threshold: f64) -> f64 {
    let clashes: usize = distances.iter().filter(|&&d| d < vdw_threshold).count();
    clashes as f64 / distances.len().max(1) as f64 * 1000.0
}

pub fn interface_residue_count(distances_to_partner: &[f64], cutoff: f64) -> usize {
    distances_to_partner.iter().filter(|&&d| d < cutoff).count()
}

pub fn druggability_score(pocket_volume: f64, hydrophobicity: f64, enclosure: f64) -> f64 {
    let vol_score = (pocket_volume / 500.0).min(1.0);
    vol_score * hydrophobicity * enclosure
}
