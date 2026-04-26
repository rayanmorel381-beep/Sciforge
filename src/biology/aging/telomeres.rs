pub fn telomere_attrition(initial_length: f64, loss_per_division: f64, divisions: usize) -> f64 {
    (initial_length - loss_per_division * divisions as f64).max(0.0)
}

pub fn telomerase_elongation(current_length: f64, rate: f64, telomerase_activity: f64) -> f64 {
    current_length + rate * telomerase_activity
}

pub fn replicative_limit(initial_length: f64, loss_per_division: f64, critical_length: f64) -> f64 {
    if loss_per_division <= 0.0 {
        return f64::INFINITY;
    }
    ((initial_length - critical_length) / loss_per_division)
        .max(0.0)
        .floor()
}

pub fn telomere_length_distribution(mean: f64, std_dev: f64, n_chromosomes: usize) -> Vec<f64> {
    let mut lengths = Vec::with_capacity(n_chromosomes);
    for i in 0..n_chromosomes {
        let x = (i as f64 / n_chromosomes as f64 - 0.5) * 2.0;
        lengths.push((mean + std_dev * x).max(0.0));
    }
    lengths
}

pub fn critical_shortening_probability(mean_length: f64, std_dev: f64, critical: f64) -> f64 {
    if std_dev <= 0.0 {
        return if mean_length <= critical { 1.0 } else { 0.0 };
    }
    let z = (critical - mean_length) / std_dev;
    0.5 * (1.0 + erf_approx(z / std::f64::consts::SQRT_2))
}

fn erf_approx(x: f64) -> f64 {
    let a = 0.3275911;
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + a * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
    sign * y
}

pub fn shelterin_protection(telomere_length: f64, shelterin_kd: f64) -> f64 {
    telomere_length / (shelterin_kd + telomere_length)
}

pub fn end_replication_problem(lagging_strand_loss: f64, divisions: usize) -> f64 {
    lagging_strand_loss * divisions as f64
}

pub fn alt_pathway_elongation(
    recombination_rate: f64,
    donor_length: f64,
    recipient_length: f64,
) -> f64 {
    recipient_length + recombination_rate * (donor_length - recipient_length).max(0.0)
}

pub fn telomere_age_regression(age: f64, birth_length: f64, annual_loss: f64) -> f64 {
    (birth_length - annual_loss * age).max(0.0)
}
