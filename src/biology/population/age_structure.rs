pub fn leslie_matrix_multiply(matrix: &[Vec<f64>], population: &[f64]) -> Vec<f64> {
    let n = population.len();
    let mut result = vec![0.0; n];
    for (i, row) in matrix.iter().enumerate().take(n) {
        for (j, &pop_j) in population.iter().enumerate() {
            if j < row.len() {
                result[i] += row[j] * pop_j;
            }
        }
    }
    result
}

pub fn stable_age_distribution(fecundities: &[f64], survivals: &[f64]) -> Vec<f64> {
    let n = fecundities.len().min(survivals.len() + 1);
    if n == 0 {
        return vec![];
    }
    let mut pop = vec![1.0; n];
    for _ in 0..200 {
        let mut new_pop = vec![0.0; n];
        for j in 0..n {
            new_pop[0] += fecundities[j] * pop[j];
        }
        for i in 1..n {
            if i - 1 < survivals.len() {
                new_pop[i] = survivals[i - 1] * pop[i - 1];
            }
        }
        let total: f64 = new_pop.iter().sum();
        if total > 0.0 {
            for p in &mut new_pop {
                *p /= total;
            }
        }
        pop = new_pop;
    }
    pop
}

pub fn cohort_generation_time(age_fecundity: &[(f64, f64)], lambda: f64) -> f64 {
    let mut numerator = 0.0;
    let mut denominator = 0.0;
    for &(age, fecundity) in age_fecundity {
        let weight = fecundity * lambda.powf(-age);
        numerator += age * weight;
        denominator += weight;
    }
    numerator / denominator.max(1e-30)
}

pub fn reproductive_value(lx: &[f64], mx: &[f64], lambda: f64) -> Vec<f64> {
    let n = lx.len().min(mx.len());
    let mut rv = vec![0.0; n];
    for x in 0..n {
        let mut sum = 0.0;
        for t in x..n {
            sum += lambda.powf(-(t as f64 - x as f64)) * lx[t] / lx[x].max(1e-30) * mx[t];
        }
        rv[x] = sum;
    }
    rv
}

pub fn euler_lotka_lambda(lx: &[f64], mx: &[f64]) -> f64 {
    let mut lambda: f64 = 1.0;
    for _ in 0..100 {
        let mut sum = 0.0;
        let mut deriv = 0.0;
        let n = lx.len().min(mx.len());
        for x in 0..n {
            let term = lx[x] * mx[x] * lambda.powf(-(x as f64 + 1.0));
            sum += term;
            deriv -= (x as f64 + 1.0) * term / lambda;
        }
        let correction = (sum - 1.0) / deriv.abs().max(1e-30);
        lambda += correction;
        if correction.abs() < 1e-10 {
            break;
        }
    }
    lambda
}

pub fn sensitivity_element(v_i: f64, w_j: f64, vw_dot: f64) -> f64 {
    v_i * w_j / vw_dot.max(1e-30)
}

pub fn elasticity_element(sensitivity: f64, a_ij: f64, lambda: f64) -> f64 {
    sensitivity * a_ij / lambda.max(1e-30)
}
