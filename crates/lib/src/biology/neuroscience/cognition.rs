pub fn drift_diffusion_decision(
    drift_rate: f64,
    noise: f64,
    threshold: f64,
    bias: f64,
    dt: f64,
    steps: usize,
) -> (f64, usize) {
    let mut evidence = bias;
    for step in 0..steps {
        evidence += drift_rate * dt;
        let noise_term = noise * dt.sqrt();
        evidence += noise_term * ((step as f64 * 1.618).sin() * 2.0 - 1.0);
        if evidence >= threshold {
            return (evidence, step);
        }
        if evidence <= -threshold {
            return (evidence, step);
        }
    }
    (evidence, steps)
}

pub fn softmax_choice(values: &[f64], temperature: f64) -> Vec<f64> {
    if values.is_empty() {
        return vec![];
    }
    let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let exp_vals: Vec<f64> = values
        .iter()
        .map(|v| ((v - max_val) / temperature.max(1e-30)).exp())
        .collect();
    let sum: f64 = exp_vals.iter().sum();
    exp_vals.iter().map(|e| e / sum).collect()
}

pub fn rescorla_wagner_update(value: f64, reward: f64, alpha: f64) -> f64 {
    value + alpha * (reward - value)
}

pub fn td_learning_update(
    value_current: f64,
    value_next: f64,
    reward: f64,
    alpha: f64,
    gamma: f64,
) -> f64 {
    value_current + alpha * (reward + gamma * value_next - value_current)
}

pub fn reward_prediction_error(reward: f64, expected: f64) -> f64 {
    reward - expected
}

pub fn weber_fraction(jnd: f64, stimulus_intensity: f64) -> f64 {
    jnd / stimulus_intensity.max(1e-30)
}

pub fn signal_to_noise_neural(signal_mean: f64, noise_std: f64) -> f64 {
    signal_mean / noise_std.max(1e-30)
}

pub fn attentional_gain(stimulus: f64, attention: f64, baseline: f64, gain: f64) -> f64 {
    baseline + gain * attention * stimulus
}

pub fn working_memory_decay(
    strength: f64,
    decay_rate: f64,
    interference_count: usize,
    dt: f64,
) -> f64 {
    strength * (-decay_rate * dt * (1.0 + interference_count as f64 * 0.2)).exp()
}

pub fn neural_efficiency(performance: f64, metabolic_cost: f64) -> f64 {
    performance / metabolic_cost.max(1e-30)
}

pub fn bayesian_integration(
    prior_mean: f64,
    prior_var: f64,
    likelihood_mean: f64,
    likelihood_var: f64,
) -> (f64, f64) {
    let posterior_var = 1.0 / (1.0 / prior_var + 1.0 / likelihood_var);
    let posterior_mean =
        posterior_var * (prior_mean / prior_var + likelihood_mean / likelihood_var);
    (posterior_mean, posterior_var)
}
