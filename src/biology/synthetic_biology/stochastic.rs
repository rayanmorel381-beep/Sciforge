pub fn stochastic_gene_expression_mean(
    transcription_rate: f64,
    translation_rate: f64,
    mrna_decay: f64,
    protein_decay: f64,
) -> f64 {
    (transcription_rate * translation_rate) / (mrna_decay * protein_decay)
}

pub fn stochastic_gene_expression_variance(
    transcription_rate: f64,
    translation_rate: f64,
    mrna_decay: f64,
    protein_decay: f64,
) -> f64 {
    let mean = stochastic_gene_expression_mean(
        transcription_rate,
        translation_rate,
        mrna_decay,
        protein_decay,
    );
    let burst_size = translation_rate / (mrna_decay + protein_decay);
    mean * (1.0 + burst_size)
}

pub fn fano_factor(variance: f64, mean: f64) -> f64 {
    variance / mean
}

pub fn noise_intrinsic(burst_size: f64, mean_protein: f64) -> f64 {
    (1.0 + burst_size) / mean_protein
}

pub fn noise_extrinsic(cv_parameter: f64) -> f64 {
    cv_parameter * cv_parameter
}

pub fn total_noise(intrinsic: f64, extrinsic: f64) -> f64 {
    intrinsic + extrinsic
}

pub fn gillespie_propensity_birth(rate: f64) -> f64 {
    rate
}

pub fn gillespie_propensity_death(rate: f64, population: f64) -> f64 {
    rate * population
}

pub fn gillespie_tau(total_propensity: f64, random_uniform: f64) -> f64 {
    -random_uniform.ln() / total_propensity
}

pub fn burst_frequency(transcription_rate: f64, promoter_off_rate: f64) -> f64 {
    transcription_rate / (transcription_rate + promoter_off_rate)
}

pub fn burst_size_mean(translation_rate: f64, mrna_decay: f64) -> f64 {
    translation_rate / mrna_decay
}

pub fn coefficient_of_variation_squared(mean: f64, variance: f64) -> f64 {
    variance / (mean * mean)
}

pub fn two_state_promoter_mean(k_on: f64, k_off: f64, transcription: f64, mrna_decay: f64) -> f64 {
    transcription * k_on / ((k_on + k_off) * mrna_decay)
}

pub fn two_state_promoter_fano(k_on: f64, k_off: f64, transcription: f64, mrna_decay: f64) -> f64 {
    1.0 + transcription * k_off / ((k_on + k_off) * (k_on + k_off + mrna_decay))
}

pub fn langevin_approximation_step(x: f64, drift: f64, diffusion: f64, dt: f64, noise: f64) -> f64 {
    (x + drift * dt + diffusion * dt.sqrt() * noise).max(0.0)
}

pub fn chemical_master_equation_steady_state_poisson(production: f64, degradation: f64) -> f64 {
    production / degradation
}

pub fn binomial_partitioning_noise(n: f64, p: f64) -> f64 {
    p * (1.0 - p) / n
}

pub fn gene_expression_delay_gamma(mean_delay: f64, shape: u32, t: f64) -> f64 {
    let rate = shape as f64 / mean_delay;
    let n = shape as f64;
    rate.powf(n) * t.powf(n - 1.0) * (-rate * t).exp() / gamma_approx(n)
}

fn gamma_approx(n: f64) -> f64 {
    if n <= 1.0 {
        return 1.0;
    }
    let mut result = 1.0;
    let mut x = n - 1.0;
    while x > 1.0 {
        result *= x;
        x -= 1.0;
    }
    result
}
