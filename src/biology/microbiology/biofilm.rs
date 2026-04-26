pub fn biofilm_formation_rate(
    planktonic: f64,
    attachment_rate: f64,
    surface_area: f64,
    detachment_rate: f64,
    biofilm: f64,
) -> f64 {
    attachment_rate * planktonic * surface_area - detachment_rate * biofilm
}

pub fn biofilm_thickness(growth_rate: f64, nutrient: f64, ks: f64, detachment: f64, t: f64) -> f64 {
    let net_growth = growth_rate * nutrient / (ks + nutrient) - detachment;
    if net_growth <= 0.0 {
        return 0.0;
    }
    net_growth * t
}

pub fn extracellular_matrix_production(
    cell_density: f64,
    signal: f64,
    max_rate: f64,
    threshold: f64,
) -> f64 {
    max_rate * cell_density * signal / (threshold + signal)
}

pub fn biofilm_diffusion_limitation(
    bulk_conc: f64,
    thickness: f64,
    diffusion_biofilm: f64,
    consumption_rate: f64,
) -> f64 {
    let phi = thickness * (consumption_rate / diffusion_biofilm.max(1e-30)).sqrt();
    if phi.abs() < 1e-10 {
        return bulk_conc;
    }
    bulk_conc * phi.tanh() / phi
}

pub fn persister_fraction(
    antibiotic_conc: f64,
    mic: f64,
    base_fraction: f64,
    max_fraction: f64,
) -> f64 {
    base_fraction + (max_fraction - base_fraction) * antibiotic_conc / (mic + antibiotic_conc)
}

pub fn antibiotic_resistance_mutation_rate(
    population: f64,
    mutation_rate: f64,
    selective_advantage: f64,
) -> f64 {
    population * mutation_rate * selective_advantage
}

pub fn minimum_inhibitory_concentration_ratio(mic_resistant: f64, mic_susceptible: f64) -> f64 {
    mic_resistant / mic_susceptible.max(1e-30)
}

pub fn horizontal_gene_transfer(donor: f64, recipient: f64, conjugation_rate: f64) -> f64 {
    conjugation_rate * donor * recipient
}

pub fn competence_transformation(
    dna_conc: f64,
    competent_cells: f64,
    uptake_rate: f64,
    integration_efficiency: f64,
) -> f64 {
    uptake_rate * dna_conc * competent_cells * integration_efficiency
}

pub fn phage_therapy_lysis(
    phage: f64,
    bacteria: f64,
    adsorption_rate: f64,
    burst_size: f64,
    latent_period: f64,
    dt: f64,
) -> (f64, f64) {
    let infections = adsorption_rate * phage * bacteria * dt;
    let new_phage = infections * burst_size * dt / latent_period;
    let new_bacteria = bacteria - infections;
    ((new_phage + phage).max(0.0), new_bacteria.max(0.0))
}
