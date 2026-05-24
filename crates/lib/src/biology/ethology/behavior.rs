pub fn territory_size(body_mass: f64, scaling_exponent: f64, constant: f64) -> f64 {
    constant * body_mass.powf(scaling_exponent)
}

pub fn territory_defense_cost(perimeter: f64, intruder_rate: f64, cost_per_encounter: f64) -> f64 {
    perimeter * intruder_rate * cost_per_encounter
}

pub fn boldness_shyness_continuum(stimulus: f64, threshold: f64, steepness: f64) -> f64 {
    1.0 / (1.0 + (-(stimulus - threshold) * steepness).exp())
}

pub fn dilution_effect(group_size: f64) -> f64 {
    1.0 / group_size
}

pub fn many_eyes_detection(individual_detection: f64, group_size: f64) -> f64 {
    1.0 - (1.0 - individual_detection).powf(group_size)
}

pub fn dominance_probability(rating_a: f64, rating_b: f64) -> f64 {
    1.0 / (1.0 + 10.0_f64.powf((rating_b - rating_a) / 400.0))
}

pub fn allee_effect_growth(n: f64, k: f64, r: f64, a: f64) -> f64 {
    r * n * (n / a - 1.0) * (1.0 - n / k)
}

pub fn predator_avoidance_flight_distance(body_mass: f64, scaling: f64, risk_factor: f64) -> f64 {
    scaling * body_mass.powf(0.5) * risk_factor
}

pub fn hamilton_relatedness_benefit(relatedness: f64, benefit: f64, cost: f64) -> bool {
    relatedness * benefit > cost
}

pub fn reciprocal_altruism_threshold(benefit: f64, cost: f64, probability_future: f64) -> bool {
    benefit * probability_future > cost
}

pub fn selfish_herd_risk(distance_to_nearest: f64, predator_speed: f64) -> f64 {
    1.0 / (1.0 + distance_to_nearest * predator_speed)
}

pub fn vigilance_group_tradeoff(group_size: f64, individual_scan_rate: f64) -> f64 {
    individual_scan_rate / group_size.sqrt()
}

pub fn confusion_effect(group_size: f64, predator_success_solo: f64) -> f64 {
    predator_success_solo / group_size.ln().max(1.0)
}

pub fn mobbing_probability(group_size: f64, predator_danger: f64, threshold: f64) -> f64 {
    1.0 / (1.0 + (-threshold * (group_size - predator_danger)).exp())
}

pub fn learning_curve_operant(trials: f64, asymptote: f64, rate: f64) -> f64 {
    asymptote * (1.0 - (-rate * trials).exp())
}

pub fn stimulus_generalization(distance: f64, width: f64) -> f64 {
    (-distance.powi(2) / (2.0 * width * width)).exp()
}

pub fn ideal_despotic_distribution(rank: f64, max_rank: f64, total_resource: f64) -> f64 {
    total_resource * (max_rank - rank + 1.0) / (max_rank * (max_rank + 1.0) / 2.0)
}

pub fn aggression_cost_benefit(
    resource_value: f64,
    fighting_ability: f64,
    injury_cost: f64,
) -> f64 {
    resource_value * fighting_ability - injury_cost * (1.0 - fighting_ability)
}

pub fn migration_threshold(food_current: f64, food_destination: f64, travel_cost: f64) -> bool {
    food_destination - travel_cost > food_current
}

pub fn information_center_benefit(colony_size: f64, discovery_prob: f64) -> f64 {
    1.0 - (1.0 - discovery_prob).powf(colony_size)
}

pub fn social_network_centrality(connections: f64, max_connections: f64) -> f64 {
    connections / max_connections
}

pub fn handicap_signal_cost(quality: f64, signal_intensity: f64, cost_coeff: f64) -> f64 {
    quality * signal_intensity - cost_coeff * signal_intensity * signal_intensity
}

pub fn mate_choice_copying(intrinsic_preference: f64, social_info: f64, weight: f64) -> f64 {
    (1.0 - weight) * intrinsic_preference + weight * social_info
}
