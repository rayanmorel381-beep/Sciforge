pub fn parasite_r0(beta: f64, host_density: f64, recovery_rate: f64, mortality_rate: f64) -> f64 {
    beta * host_density / (recovery_rate + mortality_rate)
}

pub fn parasite_transmission_rate(
    contact_rate: f64,
    infectivity: f64,
    density_susceptible: f64,
    density_infected: f64,
) -> f64 {
    contact_rate * infectivity * density_susceptible * density_infected
}

pub fn sir_parasite_step(
    s: f64,
    i: f64,
    r: f64,
    beta: f64,
    gamma: f64,
    mu: f64,
    dt: f64,
) -> (f64, f64, f64) {
    let n = s + i + r;
    let ds = mu * n - beta * s * i / n - mu * s;
    let di = beta * s * i / n - gamma * i - mu * i;
    let dr = gamma * i - mu * r;
    (
        (s + ds * dt).max(0.0),
        (i + di * dt).max(0.0),
        (r + dr * dt).max(0.0),
    )
}

pub fn parasite_aggregation_k(mean_burden: f64, variance_burden: f64) -> f64 {
    mean_burden * mean_burden / (variance_burden - mean_burden).max(1e-30)
}

pub fn parasite_negative_binomial_prevalence(mean_burden: f64, k: f64) -> f64 {
    1.0 - (1.0 + mean_burden / k).powf(-k)
}

pub fn superinfection_rate(
    current_parasites: usize,
    max_parasites: usize,
    exposure_rate: f64,
) -> f64 {
    if current_parasites >= max_parasites {
        return 0.0;
    }
    exposure_rate * (1.0 - current_parasites as f64 / max_parasites as f64)
}

pub fn vector_borne_r0(
    mosquito_density: f64,
    biting_rate: f64,
    prob_m_to_h: f64,
    prob_h_to_m: f64,
    mosquito_mortality: f64,
    extrinsic_incubation: f64,
    recovery: f64,
) -> f64 {
    let survival = (-mosquito_mortality * extrinsic_incubation).exp();
    mosquito_density * biting_rate * biting_rate * prob_m_to_h * prob_h_to_m * survival
        / (mosquito_mortality * recovery)
}

pub fn definitive_intermediate_host_cycle(
    cercariae_production: f64,
    snail_infection_rate: f64,
    human_exposure: f64,
    worm_establishment: f64,
) -> f64 {
    cercariae_production * snail_infection_rate * human_exposure * worm_establishment
}
