pub fn glycolysis_net_atp(glucose: f64) -> f64 {
    2.0 * glucose
}

pub fn glycolysis_pyruvate_yield(glucose: f64) -> f64 {
    2.0 * glucose
}

pub fn gluconeogenesis_cost(glucose: f64) -> f64 {
    6.0 * glucose
}

pub fn pentose_phosphate_nadph(glucose_6p: f64) -> f64 {
    2.0 * glucose_6p
}

pub fn fatty_acid_synthesis_cost(acetyl_coa_units: f64) -> f64 {
    let nadph = (acetyl_coa_units - 1.0).max(0.0) * 2.0;
    let atp = acetyl_coa_units;
    nadph * 2.5 + atp
}

pub fn urea_cycle_cost(amino_acids: f64) -> f64 {
    4.0 * amino_acids
}

pub fn glycogen_storage_efficiency(glucose_units: f64) -> f64 {
    let atp_stored = glucose_units * 37.0;
    let atp_cost = glucose_units * 2.0;
    (atp_stored - atp_cost) / atp_stored
}

pub fn warburg_effect(aerobic_glycolysis_rate: f64, oxidative_rate: f64) -> f64 {
    aerobic_glycolysis_rate / (aerobic_glycolysis_rate + oxidative_rate).max(1e-30)
}

pub fn ketogenesis_yield(acetyl_coa: f64) -> f64 {
    acetyl_coa / 2.0
}

pub fn amino_acid_catabolism_atp(
    carbon_count: usize,
    is_glucogenic: bool,
    is_ketogenic: bool,
) -> f64 {
    let base = carbon_count as f64 * 3.0;
    match (is_glucogenic, is_ketogenic) {
        (true, false) => base * 0.8,
        (false, true) => base * 0.6,
        (true, true) => base * 0.7,
        (false, false) => base * 0.5,
    }
}

pub fn cori_cycle_cost(lactate: f64) -> f64 {
    6.0 * lactate / 2.0
}

pub fn respiratory_quotient(co2_produced: f64, o2_consumed: f64) -> f64 {
    co2_produced / o2_consumed.max(1e-30)
}

pub fn metabolic_flux_control_coefficient(v_enzyme: f64, v_pathway: f64, elasticity: f64) -> f64 {
    elasticity * v_enzyme / v_pathway.max(1e-30)
}
