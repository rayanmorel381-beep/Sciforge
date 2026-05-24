pub fn signal_detection_d_prime(hit_rate: f64, false_alarm_rate: f64) -> f64 {
    fn inv_normal(p: f64) -> f64 {
        let p_clamped = p.clamp(1e-10, 1.0 - 1e-10);
        let a = [
            -3.969683028665376e1,
            2.209460984245205e2,
            -2.759285104469687e2,
            1.38357751867269e2,
            -3.066479806614716e1,
            2.506628277459239e0,
        ];
        let b = [
            -5.447609879822406e1,
            1.615858368580409e2,
            -1.556989798598866e2,
            6.680131188771972e1,
            -1.328068155288572e1,
        ];
        let c = [
            -7.784894002430293e-3,
            -3.223964580411365e-1,
            -2.400758277161838e0,
            -2.549732539343734e0,
            4.374664141464968e0,
            2.938163982698783e0,
        ];
        let d = [
            7.784695709041462e-3,
            3.224671290700398e-1,
            2.445134137142996e0,
            3.754408661907416e0,
        ];
        let p_low = 0.02425;
        let p_high = 1.0 - p_low;
        if p_clamped < p_low {
            let q = (-2.0 * p_clamped.ln()).sqrt();
            (((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
                / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
        } else if p_clamped <= p_high {
            let q = p_clamped - 0.5;
            let r = q * q;
            (((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) * r + a[5]) * q
                / (((((b[0] * r + b[1]) * r + b[2]) * r + b[3]) * r + b[4]) * r + 1.0)
        } else {
            let q = (-2.0 * (1.0 - p_clamped).ln()).sqrt();
            -(((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
                / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
        }
    }
    inv_normal(hit_rate) - inv_normal(false_alarm_rate)
}

pub fn honest_signal_handicap(quality: f64, cost_per_signal: f64, benefit_per_signal: f64) -> f64 {
    if cost_per_signal <= 0.0 {
        return 0.0;
    }
    quality * benefit_per_signal / cost_per_signal
}

pub fn alarm_call_kin_selection(
    relatedness: f64,
    benefit_to_kin: f64,
    cost_to_caller: f64,
) -> bool {
    relatedness * benefit_to_kin > cost_to_caller
}

pub fn mate_choice_threshold(
    quality_assessed: f64,
    search_cost: f64,
    encounters: usize,
    threshold: f64,
) -> bool {
    quality_assessed >= threshold - search_cost * encounters as f64
}

pub fn ritualized_contest(size_a: f64, size_b: f64, motivation_a: f64, motivation_b: f64) -> f64 {
    let resource_holding_a = size_a * motivation_a;
    let resource_holding_b = size_b * motivation_b;
    resource_holding_a / (resource_holding_a + resource_holding_b).max(1e-30)
}

pub fn hawk_dove_contest(v: f64, c: f64, p_hawk: f64) -> (f64, f64) {
    let hawk_payoff = p_hawk * (v - c) / 2.0 + (1.0 - p_hawk) * v;
    let dove_payoff = p_hawk * 0.0 + (1.0 - p_hawk) * v / 2.0;
    (hawk_payoff, dove_payoff)
}

pub fn producer_scrounger_frequency(
    producer_payoff: f64,
    scrounger_payoff: f64,
    p_producer: f64,
    selection_strength: f64,
) -> f64 {
    let fitness_p = (selection_strength * producer_payoff).exp();
    let fitness_s = (selection_strength * scrounger_payoff).exp();
    p_producer * fitness_p / (p_producer * fitness_p + (1.0 - p_producer) * fitness_s)
}

pub fn territory_size_optimal(energy_gain_rate: f64, defense_cost_per_area: f64) -> f64 {
    (energy_gain_rate / defense_cost_per_area.max(1e-30)).sqrt()
}

pub fn dominance_index(wins: f64, total_interactions: f64) -> f64 {
    wins / total_interactions.max(1e-30)
}
