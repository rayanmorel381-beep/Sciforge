pub fn alpha_helix_propensity(residue_propensities: &[f64]) -> f64 {
    if residue_propensities.is_empty() {
        return 0.0;
    }
    residue_propensities.iter().sum::<f64>() / residue_propensities.len() as f64
}

pub fn beta_sheet_propensity(residue_propensities: &[f64]) -> f64 {
    if residue_propensities.is_empty() {
        return 0.0;
    }
    residue_propensities.iter().sum::<f64>() / residue_propensities.len() as f64
}

pub fn chou_fasman_nucleation(propensities: &[f64], window: usize, threshold: f64) -> Vec<bool> {
    let n = propensities.len();
    if n < window {
        return vec![false; n];
    }
    let mut result = vec![false; n];
    for i in 0..=n - window {
        let avg: f64 = propensities[i..i + window].iter().sum::<f64>() / window as f64;
        if avg > threshold {
            for item in &mut result[i..i + window] {
                *item = true;
            }
        }
    }
    result
}

pub fn gor_information_value(residue_freq_in_structure: f64, residue_freq_overall: f64) -> f64 {
    (residue_freq_in_structure / residue_freq_overall.max(1e-30)).ln()
}

pub fn coiled_coil_probability(heptad_score: f64, hydrophobic_moment: f64) -> f64 {
    let combined = heptad_score * hydrophobic_moment;
    combined / (1.0 + combined)
}

pub fn disorder_prediction(hydrophobicity: f64, charge: f64, complexity: f64) -> f64 {
    let disorder_score = charge.abs() / hydrophobicity.max(1e-30) + (1.0 - complexity);
    disorder_score / (1.0 + disorder_score)
}

pub fn solvent_accessibility(residue_asa: f64, max_asa: f64) -> f64 {
    residue_asa / max_asa.max(1e-30)
}

pub fn ramachandran_energy(phi: f64, psi: f64) -> f64 {
    let alpha_phi = -57.0_f64.to_radians();
    let alpha_psi = -47.0_f64.to_radians();
    let beta_phi = -120.0_f64.to_radians();
    let beta_psi = 120.0_f64.to_radians();

    let phi_rad = phi.to_radians();
    let psi_rad = psi.to_radians();

    let d_alpha = (phi_rad - alpha_phi).powi(2) + (psi_rad - alpha_psi).powi(2);
    let d_beta = (phi_rad - beta_phi).powi(2) + (psi_rad - beta_psi).powi(2);

    -(-d_alpha / 0.5).exp() - 0.8 * (-d_beta / 0.5).exp()
}

pub fn relative_contact_order(contacts: &[(usize, usize)], chain_length: usize) -> f64 {
    if contacts.is_empty() || chain_length == 0 {
        return 0.0;
    }
    let sum_sep: f64 = contacts
        .iter()
        .map(|(i, j)| (*j as f64 - *i as f64).abs())
        .sum();
    sum_sep / (contacts.len() as f64 * chain_length as f64)
}

pub fn hydrophobic_moment(hydrophobicities: &[f64], angle_deg: f64) -> f64 {
    let angle = angle_deg.to_radians();
    let mut hx = 0.0;
    let mut hy = 0.0;
    for (i, &h) in hydrophobicities.iter().enumerate() {
        hx += h * (i as f64 * angle).cos();
        hy += h * (i as f64 * angle).sin();
    }
    (hx * hx + hy * hy).sqrt() / hydrophobicities.len().max(1) as f64
}
