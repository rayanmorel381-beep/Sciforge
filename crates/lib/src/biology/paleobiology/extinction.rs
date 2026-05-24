pub fn background_extinction_rate(species_lost: f64, total_species: f64, time_my: f64) -> f64 {
    species_lost / (total_species * time_my).max(1e-30)
}

pub fn mass_extinction_magnitude(species_before: f64, species_after: f64) -> f64 {
    (species_before - species_after) / species_before.max(1e-30)
}

pub fn recovery_time_exponential(species_lost_fraction: f64, origination_rate: f64) -> f64 {
    -(1.0 - species_lost_fraction).max(1e-30).ln() / origination_rate.max(1e-30)
}

pub fn kill_curve_severity(
    environmental_perturbation: f64,
    vulnerability: f64,
    threshold: f64,
) -> f64 {
    if environmental_perturbation < threshold {
        return 0.0;
    }
    1.0 - (-vulnerability * (environmental_perturbation - threshold)).exp()
}

pub fn selectivity_index(extinction_rate_group: f64, extinction_rate_background: f64) -> f64 {
    extinction_rate_group / extinction_rate_background.max(1e-30)
}

pub fn origination_extinction_balance(origination_rate: f64, extinction_rate: f64) -> f64 {
    origination_rate - extinction_rate
}

pub fn survivorship_curve(initial_cohort: f64, extinction_rate: f64, t: f64) -> f64 {
    initial_cohort * (-extinction_rate * t).exp()
}

pub fn lazarus_taxon_probability(
    true_extinction: f64,
    sampling_probability: f64,
    gap_duration: f64,
) -> f64 {
    (1.0 - true_extinction) * (1.0 - sampling_probability).powf(gap_duration)
}

pub fn signor_lipps_effect(last_appearance: f64, sampling_interval: f64) -> f64 {
    last_appearance + sampling_interval / 2.0
}

pub fn biodiversity_through_time(
    origination_rate: f64,
    extinction_rate: f64,
    initial_diversity: f64,
    t: f64,
) -> f64 {
    let net = origination_rate - extinction_rate;
    initial_diversity * (net * t).exp()
}

pub fn waiting_time_to_extinction(population_size: f64, extinction_rate: f64) -> f64 {
    1.0 / (extinction_rate * population_size).max(1e-30)
}
