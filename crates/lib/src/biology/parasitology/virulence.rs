pub fn parasite_virulence_tradeoff(virulence: f64, beta_max: f64, v_half: f64) -> f64 {
    beta_max * virulence / (v_half + virulence) - virulence
}

pub fn optimal_virulence(beta_max: f64, v_half: f64, mortality_background: f64) -> f64 {
    (v_half * (beta_max + mortality_background)).sqrt() - v_half
}

pub fn immune_evasion_probability(parasite_diversity: f64, immune_memory: f64) -> f64 {
    (-immune_memory / parasite_diversity.max(1.0)).exp()
}

pub fn worm_burden_distribution_mean(epg: f64, fecundity: f64) -> f64 {
    epg / fecundity
}

pub fn force_of_infection(
    contact_rate: f64,
    environmental_contamination: f64,
    susceptibility: f64,
) -> f64 {
    contact_rate * environmental_contamination * susceptibility
}

pub fn age_intensity_profile(age: f64, peak_age: f64, max_intensity: f64, shape: f64) -> f64 {
    max_intensity * (age / peak_age) * (-(age - peak_age).powi(2) / (2.0 * shape * shape)).exp()
}

pub fn superinfection_threshold(r0_resident: f64, r0_challenger: f64) -> bool {
    r0_challenger > r0_resident
}

pub fn aggregation_parameter(mean_burden: f64, variance: f64) -> f64 {
    if variance <= mean_burden {
        return f64::INFINITY;
    }
    mean_burden * mean_burden / (variance - mean_burden)
}

pub fn drug_resistance_spread(
    sensitive_freq: f64,
    resistant_fitness: f64,
    treatment_coverage: f64,
) -> f64 {
    let resistant_advantage = resistant_fitness * treatment_coverage;
    (1.0 - sensitive_freq) * resistant_advantage
        / (sensitive_freq + (1.0 - sensitive_freq) * resistant_advantage)
}

pub fn basic_reproduction_number_macroparasite(
    beta: f64,
    lambda: f64,
    mu_host: f64,
    mu_parasite: f64,
    alpha: f64,
) -> f64 {
    beta * lambda / ((mu_host + alpha) * (mu_parasite + mu_host))
}

pub fn case_fatality_rate(virulence: f64, host_resistance: f64) -> f64 {
    virulence / (virulence + host_resistance)
}

pub fn parasite_clearance_rate(
    immune_activity: f64,
    drug_efficacy: f64,
    natural_death: f64,
) -> f64 {
    immune_activity + drug_efficacy + natural_death
}

pub fn morbidity_intensity(burden: f64, threshold: f64, severity_coeff: f64) -> f64 {
    if burden <= threshold {
        return 0.0;
    }
    severity_coeff * (burden - threshold)
}

pub fn transmission_seasonality(baseline_beta: f64, amplitude: f64, t: f64, period: f64) -> f64 {
    baseline_beta * (1.0 + amplitude * (2.0 * std::f64::consts::PI * t / period).cos())
}

pub fn mass_drug_administration_impact(prevalence: f64, coverage: f64, efficacy: f64) -> f64 {
    prevalence * (1.0 - coverage * efficacy)
}

pub fn reinfection_rate(
    force_of_infection: f64,
    waning_immunity: f64,
    time_since_treatment: f64,
) -> f64 {
    force_of_infection * (1.0 - (-waning_immunity * time_since_treatment).exp())
}

pub fn pathogen_shedding_rate(burden: f64, per_parasite_shed: f64, saturation: f64) -> f64 {
    per_parasite_shed * burden / (1.0 + saturation * burden)
}

pub fn environmental_reservoir_decay(contamination: f64, decay_rate: f64, input_rate: f64) -> f64 {
    input_rate - decay_rate * contamination
}

pub fn host_specificity_index(hosts_used: f64, hosts_available: f64) -> f64 {
    hosts_used / hosts_available
}

pub fn virulence_evolution_si(beta: f64, alpha: f64, gamma: f64, mu: f64) -> f64 {
    beta / (alpha + gamma + mu)
}
