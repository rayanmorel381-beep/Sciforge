pub fn oral_bioavailability(
    fraction_absorbed: f64,
    gut_wall_extraction: f64,
    hepatic_extraction: f64,
) -> f64 {
    fraction_absorbed * (1.0 - gut_wall_extraction) * (1.0 - hepatic_extraction)
}

pub fn intestinal_permeability_papp(
    amount_receiver: f64,
    area: f64,
    time: f64,
    donor_conc: f64,
) -> f64 {
    amount_receiver / (area * time * donor_conc).max(1e-30)
}

pub fn dissolution_noyes_whitney(
    diffusion_coeff: f64,
    surface_area: f64,
    cs: f64,
    c: f64,
    thickness: f64,
    volume: f64,
) -> f64 {
    diffusion_coeff * surface_area * (cs - c) / (thickness * volume).max(1e-30)
}

pub fn biopharmaceutics_classification(solubility_high: bool, permeability_high: bool) -> u8 {
    match (solubility_high, permeability_high) {
        (true, true) => 1,
        (false, true) => 2,
        (true, false) => 3,
        (false, false) => 4,
    }
}

pub fn hepatic_clearance_well_stirred(liver_blood_flow: f64, fu: f64, cl_int: f64) -> f64 {
    liver_blood_flow * fu * cl_int / (liver_blood_flow + fu * cl_int)
}

pub fn renal_drug_clearance(gfr: f64, fu: f64, secretion: f64, reabsorption_fraction: f64) -> f64 {
    (gfr * fu + secretion) * (1.0 - reabsorption_fraction)
}

pub fn protein_binding(ka: f64, protein_conc: f64) -> f64 {
    ka * protein_conc / (1.0 + ka * protein_conc)
}

pub fn apparent_volume_of_distribution(dose: f64, plasma_concentration: f64) -> f64 {
    dose / plasma_concentration.max(1e-30)
}

pub fn compartment_distribution(dose: f64, kel: f64, k12: f64, k21: f64, t: f64) -> f64 {
    let alpha = 0.5
        * ((kel + k12 + k21)
            + ((kel + k12 + k21).powi(2) - 4.0 * kel * k21)
                .max(0.0)
                .sqrt());
    let beta = 0.5
        * ((kel + k12 + k21)
            - ((kel + k12 + k21).powi(2) - 4.0 * kel * k21)
                .max(0.0)
                .sqrt());
    let a_coeff = dose * (alpha - k21) / (alpha - beta);
    let b_coeff = dose * (k21 - beta) / (alpha - beta);
    a_coeff * (-alpha * t).exp() + b_coeff * (-beta * t).exp()
}

pub fn p_glycoprotein_efflux(intracellular_conc: f64, pgp_activity: f64, km: f64) -> f64 {
    pgp_activity * intracellular_conc / (km + intracellular_conc)
}
