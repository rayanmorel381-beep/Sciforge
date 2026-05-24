pub fn toggle_switch(
    u: f64,
    v: f64,
    alpha1: f64,
    alpha2: f64,
    beta: f64,
    gamma: f64,
    n: f64,
) -> (f64, f64) {
    let du = alpha1 / (1.0 + v.powf(n)) - beta * u;
    let dv = alpha2 / (1.0 + u.powf(n)) - gamma * v;
    (du, dv)
}

pub fn repressilator(x: &[f64; 3], alpha: f64, alpha0: f64, beta: f64, n: f64) -> [f64; 3] {
    [
        alpha / (1.0 + x[2].powf(n)) + alpha0 - beta * x[0],
        alpha / (1.0 + x[0].powf(n)) + alpha0 - beta * x[1],
        alpha / (1.0 + x[1].powf(n)) + alpha0 - beta * x[2],
    ]
}

pub fn hill_activation(x: f64, k: f64, n: f64) -> f64 {
    x.powf(n) / (k.powf(n) + x.powf(n))
}

pub fn hill_repression(x: f64, k: f64, n: f64) -> f64 {
    k.powf(n) / (k.powf(n) + x.powf(n))
}

pub fn inducible_promoter(inducer: f64, kd: f64, n: f64, basal: f64, max_rate: f64) -> f64 {
    basal + (max_rate - basal) * inducer.powf(n) / (kd.powf(n) + inducer.powf(n))
}

pub fn ribosome_binding_strength(rbs_score: f64, max_translation: f64) -> f64 {
    max_translation * rbs_score
}

pub fn and_gate(input_a: f64, input_b: f64, k_a: f64, k_b: f64, n: f64, max_output: f64) -> f64 {
    let a_active = input_a.powf(n) / (k_a.powf(n) + input_a.powf(n));
    let b_active = input_b.powf(n) / (k_b.powf(n) + input_b.powf(n));
    max_output * a_active * b_active
}

pub fn or_gate(input_a: f64, input_b: f64, k: f64, n: f64, max_output: f64) -> f64 {
    let combined = input_a + input_b;
    max_output * combined.powf(n) / (k.powf(n) + combined.powf(n))
}

pub fn not_gate(input: f64, k: f64, n: f64, max_output: f64) -> f64 {
    max_output * k.powf(n) / (k.powf(n) + input.powf(n))
}

pub fn oscillator_amplitude(protein_levels: &[f64]) -> f64 {
    let max = protein_levels
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let min = protein_levels.iter().cloned().fold(f64::INFINITY, f64::min);
    max - min
}

pub fn nand_gate(input_a: f64, input_b: f64, k: f64, n: f64, max_output: f64) -> f64 {
    let ab = input_a * input_b;
    max_output * k.powf(n) / (k.powf(n) + ab.powf(n))
}

pub fn xor_gate(input_a: f64, input_b: f64, k: f64, n: f64, max_output: f64) -> f64 {
    let a_on = input_a.powf(n) / (k.powf(n) + input_a.powf(n));
    let b_on = input_b.powf(n) / (k.powf(n) + input_b.powf(n));
    max_output * (a_on * (1.0 - b_on) + b_on * (1.0 - a_on))
}

pub fn feed_forward_loop_c1(
    input: f64,
    x: f64,
    k_input: f64,
    k_x: f64,
    n: f64,
    max_rate: f64,
) -> f64 {
    let direct = input.powf(n) / (k_input.powf(n) + input.powf(n));
    let indirect = x.powf(n) / (k_x.powf(n) + x.powf(n));
    max_rate * direct * indirect
}

pub fn negative_autoregulation(protein: f64, k: f64, n: f64, basal: f64, max_rate: f64) -> f64 {
    basal + (max_rate - basal) * k.powf(n) / (k.powf(n) + protein.powf(n))
}

pub fn positive_autoregulation(protein: f64, k: f64, n: f64, basal: f64, max_rate: f64) -> f64 {
    basal + (max_rate - basal) * protein.powf(n) / (k.powf(n) + protein.powf(n))
}

pub fn bandpass_filter(input: f64, k_low: f64, k_high: f64, n: f64, max_output: f64) -> f64 {
    let activation = input.powf(n) / (k_low.powf(n) + input.powf(n));
    let repression = k_high.powf(n) / (k_high.powf(n) + input.powf(n));
    max_output * activation * repression
}

pub fn quorum_sensing_autoinducer(
    cell_density: f64,
    production_rate: f64,
    degradation_rate: f64,
) -> f64 {
    production_rate * cell_density / degradation_rate
}

pub fn quorum_sensing_response(autoinducer: f64, threshold: f64, n: f64, max_response: f64) -> f64 {
    max_response * autoinducer.powf(n) / (threshold.powf(n) + autoinducer.powf(n))
}

pub fn kill_switch_toxin_antitoxin(inducer: f64, k_toxin: f64, k_antitoxin: f64, n: f64) -> f64 {
    let toxin = inducer.powf(n) / (k_toxin.powf(n) + inducer.powf(n));
    let antitoxin = k_antitoxin.powf(n) / (k_antitoxin.powf(n) + inducer.powf(n));
    toxin * (1.0 - antitoxin)
}

pub fn metabolic_burden(circuit_protein: f64, growth_rate_max: f64, burden_coeff: f64) -> f64 {
    growth_rate_max / (1.0 + burden_coeff * circuit_protein)
}

pub fn biosensor_dose_response(
    analyte: f64,
    k_half: f64,
    n: f64,
    output_min: f64,
    output_max: f64,
) -> f64 {
    output_min + (output_max - output_min) * analyte.powf(n) / (k_half.powf(n) + analyte.powf(n))
}
