pub fn host_parasite_lotka_volterra(
    h: f64,
    p: f64,
    r: f64,
    k: f64,
    a: f64,
    c: f64,
    d: f64,
) -> (f64, f64) {
    let dh = r * h * (1.0 - h / k) - a * h * p;
    let dp = c * a * h * p - d * p;
    (dh, dp)
}

pub fn anderson_may(
    h: f64,
    p: f64,
    alpha: f64,
    beta: f64,
    b: f64,
    d_h: f64,
    d_p: f64,
    k_aggregation: f64,
) -> (f64, f64) {
    let m = p / h.max(1e-10);
    let dh = (b - d_h) * h - alpha * p;
    let dp = beta * h * m
        - (d_p + d_h + alpha) * p
        - alpha * (p * m) * (k_aggregation + 1.0) / k_aggregation;
    (dh, dp)
}

pub fn negative_binomial_prevalence(mean_burden: f64, k: f64) -> f64 {
    1.0 - (1.0 + mean_burden / k).powf(-k)
}

pub fn parasite_aggregation_index(variance: f64, mean: f64) -> f64 {
    variance / mean
}

pub fn superinfection_probability(exposure_rate: f64, current_burden: f64, max_burden: f64) -> f64 {
    exposure_rate * (1.0 - current_burden / max_burden).max(0.0)
}

pub fn basic_reproduction_number_parasite(
    beta: f64,
    lambda: f64,
    h: f64,
    mu_p: f64,
    mu_h: f64,
    alpha: f64,
) -> f64 {
    beta * lambda * h / ((mu_p + mu_h + alpha) * (mu_h + alpha))
}

pub fn coevolution_red_queen(
    host_fitness: f64,
    parasite_fitness: f64,
    arms_race_rate: f64,
) -> (f64, f64) {
    let dh = arms_race_rate * (1.0 - parasite_fitness);
    let dp = arms_race_rate * (1.0 - host_fitness);
    (dh, dp)
}

pub fn nicholson_bailey(h: f64, p: f64, r: f64, a: f64) -> (f64, f64) {
    let escape = (-a * p).exp();
    let h_next = r * h * escape;
    let p_next = h * (1.0 - escape);
    (h_next, p_next)
}

pub fn negative_binomial_zero_class(mean_burden: f64, k: f64) -> f64 {
    (1.0 + mean_burden / k).powf(-k)
}

pub fn parasite_induced_mortality(alpha: f64, burden: f64) -> f64 {
    alpha * burden
}

pub fn acquired_immunity_reduction(exposure: f64, max_immunity: f64, half_exposure: f64) -> f64 {
    max_immunity * exposure / (exposure + half_exposure)
}

pub fn intermediate_host_prevalence(beta: f64, h2: f64, mu_l: f64, mu_h2: f64) -> f64 {
    beta * h2 / (mu_l + mu_h2)
}

pub fn cercarial_force_of_infection(
    cercarial_density: f64,
    contact_rate: f64,
    penetration_prob: f64,
) -> f64 {
    contact_rate * cercarial_density * penetration_prob
}

pub fn predator_prey_parasite_manipulation(
    h: f64,
    p: f64,
    prey: f64,
    r: f64,
    a: f64,
    manipulation_factor: f64,
    conversion: f64,
    death: f64,
) -> (f64, f64) {
    let effective_a = a * (1.0 + manipulation_factor * p / h.max(1e-10));
    let dh = -effective_a * h * prey + conversion * effective_a * h * prey - death * h;
    let dprey = r * prey - effective_a * h * prey;
    (dh, dprey)
}

pub fn density_dependent_fecundity(fecundity_max: f64, burden: f64, k_dens: f64) -> f64 {
    fecundity_max / (1.0 + k_dens * burden)
}

pub fn mate_probability_dioecious(burden: f64, k_agg: f64) -> f64 {
    1.0 - (1.0 + burden / (k_agg + 1.0)).powf(-(k_agg + 1.0))
}

pub fn parasite_free_equilibrium(birth: f64, death: f64, carrying_capacity: f64) -> f64 {
    carrying_capacity * (1.0 - death / birth).max(0.0)
}
