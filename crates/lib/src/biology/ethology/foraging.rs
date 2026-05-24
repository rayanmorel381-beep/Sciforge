pub fn optimal_diet_value(energy_gain: f64, handling_time: f64, encounter_rate: f64) -> f64 {
    let profitability = energy_gain / handling_time;
    profitability * encounter_rate
}

pub fn marginal_value_theorem(
    travel_time: f64,
    gain_curve: impl Fn(f64) -> f64,
    max_t: f64,
) -> f64 {
    let steps = 1000;
    let dt = max_t / steps as f64;
    let mut best_rate = f64::NEG_INFINITY;
    let mut best_t = 0.0;
    for i in 1..=steps {
        let t = i as f64 * dt;
        let rate = gain_curve(t) / (travel_time + t);
        if rate > best_rate {
            best_rate = rate;
            best_t = t;
        }
    }
    best_t
}

pub fn ideal_free_distribution(resource: &[f64], total_individuals: f64) -> Vec<f64> {
    let total_resource: f64 = resource.iter().sum();
    if total_resource <= 0.0 {
        return vec![0.0; resource.len()];
    }
    resource
        .iter()
        .map(|&r| total_individuals * r / total_resource)
        .collect()
}

pub fn hawk_dove_payoff(v: f64, c: f64, hawk_freq: f64) -> (f64, f64) {
    let hawk_payoff = hawk_freq * (v - c) / 2.0 + (1.0 - hawk_freq) * v;
    let dove_payoff = (1.0 - hawk_freq) * v / 2.0;
    (hawk_payoff, dove_payoff)
}

pub fn ess_hawk_frequency(v: f64, c: f64) -> f64 {
    if c <= 0.0 {
        return 1.0;
    }
    (v / c).min(1.0)
}

pub fn tit_for_tat_payoff(r: f64, s: f64, t: f64, p: f64, opponent_cooperates: bool) -> f64 {
    if opponent_cooperates {
        r
    } else {
        s + (t - s) * (1.0 - p / p.max(1e-30))
    }
}

pub fn prey_choice_ranking(prey_types: &[(f64, f64)]) -> Vec<(usize, f64)> {
    let mut ranked: Vec<(usize, f64)> = prey_types
        .iter()
        .enumerate()
        .map(|(i, &(energy, handling))| (i, energy / handling))
        .collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    ranked
}

pub fn risk_sensitive_foraging(mean_gain: f64, variance: f64, risk_aversion: f64) -> f64 {
    mean_gain - risk_aversion * variance
}

pub fn central_place_foraging(
    distance: f64,
    load: f64,
    travel_cost_per_unit: f64,
    gain_per_load: f64,
) -> f64 {
    let travel_cost = 2.0 * distance * travel_cost_per_unit;
    let net = gain_per_load * load - travel_cost;
    net / (2.0 * distance + load)
}

pub fn producer_scrounger_game(p_freq: f64, finder_advantage: f64, group_size: f64) -> (f64, f64) {
    let producer_payoff = finder_advantage / (1.0 + (1.0 - p_freq) * (group_size - 1.0));
    let scrounger_payoff = (1.0 - finder_advantage) * p_freq * group_size / group_size;
    (producer_payoff, scrounger_payoff)
}

pub fn giving_up_density(metabolic_cost: f64, predation_cost: f64, missed_opportunity: f64) -> f64 {
    metabolic_cost + predation_cost + missed_opportunity
}

pub fn patch_residence_time(gain_rate: f64, travel_time: f64, depletion_rate: f64) -> f64 {
    ((gain_rate * travel_time) / depletion_rate).sqrt()
}

pub fn functional_response_type_ii(prey_density: f64, attack_rate: f64, handling_time: f64) -> f64 {
    attack_rate * prey_density / (1.0 + attack_rate * handling_time * prey_density)
}

pub fn functional_response_type_iii(
    prey_density: f64,
    attack_max: f64,
    half_sat: f64,
    handling_time: f64,
) -> f64 {
    let a = attack_max * prey_density / (half_sat + prey_density);
    a * prey_density / (1.0 + a * handling_time * prey_density)
}

pub fn starvation_risk(reserves: f64, daily_cost: f64, variance: f64) -> f64 {
    (-2.0 * reserves * daily_cost / variance.max(1e-30)).exp()
}

pub fn cache_pilferage_rate(competitors: f64, detection_prob: f64, cache_density: f64) -> f64 {
    1.0 - (1.0 - detection_prob * cache_density).powf(competitors)
}

pub fn optimal_load_size(
    distance: f64,
    max_load: f64,
    loading_rate: f64,
    travel_speed: f64,
) -> f64 {
    let travel_time = distance / travel_speed;
    (loading_rate * travel_time).min(max_load)
}

pub fn diet_breadth_threshold(energy: &[f64], handling: &[f64], encounter: &[f64]) -> usize {
    let mut indices: Vec<usize> = (0..energy.len()).collect();
    indices.sort_by(|&a, &b| {
        let pa = energy[a] / handling[a];
        let pb = energy[b] / handling[b];
        pb.partial_cmp(&pa).unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut cum_rate_num = 0.0;
    let mut cum_rate_den = 1.0;
    for (k, &idx) in indices.iter().enumerate() {
        cum_rate_num += encounter[idx] * energy[idx];
        cum_rate_den += encounter[idx] * handling[idx];
        let avg_rate = cum_rate_num / cum_rate_den;
        let next_profit = if k + 1 < indices.len() {
            let ni = indices[k + 1];
            energy[ni] / handling[ni]
        } else {
            0.0
        };
        if avg_rate >= next_profit {
            return k + 1;
        }
    }
    indices.len()
}
