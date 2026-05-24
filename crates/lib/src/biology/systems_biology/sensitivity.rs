pub fn local_sensitivity(
    output_perturbed: f64,
    output_base: f64,
    param_perturbed: f64,
    param_base: f64,
) -> f64 {
    let delta_output = output_perturbed - output_base;
    let delta_param = param_perturbed - param_base;
    (delta_output / output_base.max(1e-30)) / (delta_param / param_base.max(1e-30))
}

pub fn morris_elementary_effect(output_high: f64, output_base: f64, delta: f64) -> f64 {
    (output_high - output_base) / delta
}

pub fn sobol_first_order(variance_conditional: f64, variance_total: f64) -> f64 {
    variance_conditional / variance_total.max(1e-30)
}

pub fn sobol_total_order(variance_remaining: f64, variance_total: f64) -> f64 {
    1.0 - variance_remaining / variance_total.max(1e-30)
}

pub fn prcc_partial_rank_correlation(r_xy_given_z: f64) -> f64 {
    r_xy_given_z
}

pub fn parameter_identifiability(fisher_information_diagonal: f64) -> f64 {
    1.0 / fisher_information_diagonal.max(1e-30)
}

pub fn robustness_index(
    outputs_perturbed: &[f64],
    output_nominal: f64,
    perturbation_range: f64,
) -> f64 {
    if outputs_perturbed.is_empty() {
        return 1.0;
    }
    let max_deviation = outputs_perturbed
        .iter()
        .map(|o| (o - output_nominal).abs())
        .fold(0.0, f64::max);
    1.0 - max_deviation / (output_nominal.abs() * perturbation_range).max(1e-30)
}

pub fn bifurcation_distance(parameter: f64, critical_value: f64) -> f64 {
    (parameter - critical_value).abs()
}

pub fn latin_hypercube_sample(n_samples: usize, n_params: usize) -> Vec<Vec<f64>> {
    let mut samples = Vec::with_capacity(n_samples);
    for i in 0..n_samples {
        let mut row = Vec::with_capacity(n_params);
        for j in 0..n_params {
            let base = i as f64 / n_samples as f64;
            let offset = ((i + j) % n_samples) as f64 / n_samples as f64;
            row.push((base + offset) % 1.0);
        }
        samples.push(row);
    }
    samples
}

pub fn metabolic_control_coefficient(
    flux_change: f64,
    flux_base: f64,
    enzyme_change: f64,
    enzyme_base: f64,
) -> f64 {
    (flux_change / flux_base.max(1e-30)) / (enzyme_change / enzyme_base.max(1e-30))
}
