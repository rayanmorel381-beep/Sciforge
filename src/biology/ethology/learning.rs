pub fn habituation(response: f64, stimulus_count: usize, decay_rate: f64) -> f64 {
    response * (-decay_rate * stimulus_count as f64).exp()
}

pub fn sensitization(response: f64, noxious_stimulus: f64, gain: f64, saturation: f64) -> f64 {
    response * (1.0 + gain * noxious_stimulus / (saturation + noxious_stimulus))
}

pub fn operant_conditioning_rate(reinforcement_rate: f64, response_rate: f64, k: f64) -> f64 {
    k * reinforcement_rate / (reinforcement_rate + response_rate).max(1e-30)
}

pub fn classical_conditioning_rescorla_wagner(v: f64, alpha: f64, beta: f64, lambda: f64) -> f64 {
    v + alpha * beta * (lambda - v)
}

pub fn spatial_learning_distance(
    trial: usize,
    asymptote: f64,
    learning_rate: f64,
    initial_distance: f64,
) -> f64 {
    asymptote + (initial_distance - asymptote) * (-learning_rate * trial as f64).exp()
}

pub fn imprinting_critical_period(exposure_time: f64, peak_time: f64, width: f64) -> f64 {
    (-(exposure_time - peak_time).powi(2) / (2.0 * width * width)).exp()
}

pub fn social_learning_transmission(demonstrators: f64, naive: f64, transmission_rate: f64) -> f64 {
    transmission_rate * demonstrators * naive
}

pub fn memory_retention_ebbinghaus(strength: f64, time: f64, stability: f64) -> f64 {
    strength * (-time / stability.max(1e-30)).exp()
}

pub fn working_memory_capacity(items: &[f64], capacity: usize) -> f64 {
    let n = items.len().min(capacity);
    let sum: f64 = items[..n].iter().sum();
    sum / n.max(1) as f64
}

pub fn win_stay_lose_shift(previous_outcome: f64, threshold: f64) -> bool {
    previous_outcome >= threshold
}
