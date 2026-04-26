pub fn fermentation_yield(substrate: f64, yield_coefficient: f64) -> f64 {
    substrate * yield_coefficient
}

pub fn anaerobic_atp_yield(glucose_moles: f64, pathway_efficiency: f64) -> f64 {
    glucose_moles * 2.0 * pathway_efficiency
}

pub fn chemolithoautotrophy_energy(delta_g: f64, moles_substrate: f64, efficiency: f64) -> f64 {
    -delta_g * moles_substrate * efficiency
}

pub fn nitrogen_fixation_cost(n2_fixed: f64, atp_per_n2: f64) -> f64 {
    n2_fixed * atp_per_n2
}

pub fn denitrification_rate(
    no3: f64,
    carbon_source: f64,
    max_rate: f64,
    ks_no3: f64,
    ks_c: f64,
) -> f64 {
    max_rate * no3 / (ks_no3 + no3) * carbon_source / (ks_c + carbon_source)
}

pub fn sulfate_reduction_rate(
    sulfate: f64,
    electron_donor: f64,
    max_rate: f64,
    ks_so4: f64,
    ks_donor: f64,
) -> f64 {
    max_rate * sulfate / (ks_so4 + sulfate) * electron_donor / (ks_donor + electron_donor)
}

pub fn methanogenesis_rate(
    co2: f64,
    h2: f64,
    max_rate: f64,
    ks_co2: f64,
    ks_h2: f64,
    temperature: f64,
) -> f64 {
    let temp_factor = ((temperature - 35.0).powi(2) / (-200.0)).exp();
    max_rate * co2 / (ks_co2 + co2) * h2 / (ks_h2 + h2) * temp_factor
}

pub fn anammox_rate(nh4: f64, no2: f64, max_rate: f64, ks_nh4: f64, ks_no2: f64) -> f64 {
    max_rate * nh4 / (ks_nh4 + nh4) * no2 / (ks_no2 + no2)
}

pub fn iron_oxidation_rate(fe2: f64, o2: f64, ph: f64, max_rate: f64) -> f64 {
    let ph_factor = if ph < 3.0 { 1.0 } else { (-(ph - 3.0)).exp() };
    max_rate * fe2 * o2 * ph_factor
}

pub fn bioremediation_degradation(
    contaminant: f64,
    biomass: f64,
    max_rate: f64,
    ks: f64,
    inhibition_conc: f64,
) -> f64 {
    max_rate * biomass * contaminant
        / (ks + contaminant + contaminant * contaminant / inhibition_conc)
}
