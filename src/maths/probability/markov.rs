pub fn transition_probability(matrix: &[Vec<f64>], state: usize, next_state: usize) -> f64 {
    matrix[state][next_state]
}

pub fn state_after_n_steps(matrix: &[Vec<f64>], initial: &[f64], steps: usize) -> Vec<f64> {
    let n = initial.len();
    let mut state = initial.to_vec();
    for _ in 0..steps {
        let mut next = vec![0.0; n];
        for (&si, mi) in state.iter().zip(matrix.iter()) {
            for (nj, &mij) in next.iter_mut().zip(mi.iter()) {
                *nj += si * mij;
            }
        }
        state = next;
    }
    state
}

pub fn steady_state(matrix: &[Vec<f64>], max_iter: usize, tol: f64) -> Vec<f64> {
    let n = matrix.len();
    let mut state = vec![1.0 / n as f64; n];
    for _ in 0..max_iter {
        let mut next = vec![0.0; n];
        for (&si, mi) in state.iter().zip(matrix.iter()) {
            for (nj, &mij) in next.iter_mut().zip(mi.iter()) {
                *nj += si * mij;
            }
        }
        let diff: f64 = state
            .iter()
            .zip(next.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        state = next;
        if diff < tol {
            break;
        }
    }
    state
}

pub fn is_ergodic(matrix: &[Vec<f64>]) -> bool {
    let n = matrix.len();
    let mut reachable = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            if matrix[i][j] > 0.0 {
                reachable[i][j] = true;
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if reachable[i][k] && reachable[k][j] {
                    reachable[i][j] = true;
                }
            }
        }
    }
    reachable.iter().all(|row| row.iter().all(|&x| x))
}

pub fn absorbing_states(matrix: &[Vec<f64>]) -> Vec<usize> {
    let n = matrix.len();
    (0..n)
        .filter(|&i| (matrix[i][i] - 1.0).abs() < 1e-10)
        .collect()
}

pub fn expected_visits(matrix: &[Vec<f64>], start: usize, steps: usize) -> Vec<f64> {
    let n = matrix.len();
    let mut visits = vec![0.0; n];
    let mut state = vec![0.0; n];
    state[start] = 1.0;
    for _ in 0..steps {
        for (vi, &si) in visits.iter_mut().zip(state.iter()) {
            *vi += si;
        }
        let mut next = vec![0.0; n];
        for (&si, mi) in state.iter().zip(matrix.iter()) {
            for (nj, &mij) in next.iter_mut().zip(mi.iter()) {
                *nj += si * mij;
            }
        }
        state = next;
    }
    visits
}

pub fn mean_first_passage_time(matrix: &[Vec<f64>], target: usize, max_iter: usize) -> Vec<f64> {
    let n = matrix.len();
    let mut mfpt = vec![0.0; n];
    for start in 0..n {
        if start == target {
            continue;
        }
        let mut state = vec![0.0; n];
        state[start] = 1.0;
        let mut expected = 0.0;
        for step in 1..=max_iter {
            let mut next = vec![0.0; n];
            for (i, (&si, mi)) in state.iter().zip(matrix.iter()).enumerate() {
                if i == target {
                    continue;
                }
                for (nj, &mij) in next.iter_mut().zip(mi.iter()) {
                    *nj += si * mij;
                }
            }
            expected += step as f64 * next[target];
            state = next;
        }
        mfpt[start] = expected;
    }
    mfpt
}

pub fn communicate_classes(matrix: &[Vec<f64>]) -> Vec<Vec<usize>> {
    let n = matrix.len();
    let mut reachable = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            if matrix[i][j] > 0.0 {
                reachable[i][j] = true;
            }
        }
        reachable[i][i] = true;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if reachable[i][k] && reachable[k][j] {
                    reachable[i][j] = true;
                }
            }
        }
    }
    let mut visited = vec![false; n];
    let mut classes = Vec::new();
    for i in 0..n {
        if visited[i] {
            continue;
        }
        let mut class = vec![i];
        visited[i] = true;
        for j in (i + 1)..n {
            if !visited[j] && reachable[i][j] && reachable[j][i] {
                class.push(j);
                visited[j] = true;
            }
        }
        classes.push(class);
    }
    classes
}

pub fn transition_matrix_power(matrix: &[Vec<f64>], power: usize) -> Vec<Vec<f64>> {
    let n = matrix.len();
    let mut result = vec![vec![0.0; n]; n];
    for (i, ri) in result.iter_mut().enumerate() {
        ri[i] = 1.0;
    }
    let mut base = matrix.to_vec();
    let mut p = power;
    while p > 0 {
        if p & 1 == 1 {
            result = mat_mul_markov(&result, &base);
        }
        base = mat_mul_markov(&base, &base);
        p >>= 1;
    }
    result
}

fn mat_mul_markov(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = a.len();
    let mut c = vec![vec![0.0; n]; n];
    for i in 0..n {
        for (k, bk) in b.iter().enumerate() {
            if a[i][k] == 0.0 {
                continue;
            }
            for (j, bkj) in bk.iter().enumerate() {
                c[i][j] += a[i][k] * bkj;
            }
        }
    }
    c
}

pub fn hitting_probability(
    matrix: &[Vec<f64>],
    start: usize,
    target: usize,
    max_iter: usize,
) -> f64 {
    let n = matrix.len();
    let mut state = vec![0.0; n];
    state[start] = 1.0;
    let mut total_prob = 0.0;
    for _ in 0..max_iter {
        let mut next = vec![0.0; n];
        for (i, (&si, mi)) in state.iter().zip(matrix.iter()).enumerate() {
            if i == target {
                continue;
            }
            for (nj, &mij) in next.iter_mut().zip(mi.iter()) {
                *nj += si * mij;
            }
        }
        total_prob += next[target];
        state = next;
    }
    total_prob
}

pub fn birth_death_steady_state(birth_rates: &[f64], death_rates: &[f64]) -> Vec<f64> {
    let n = birth_rates.len() + 1;
    let mut pi = vec![0.0; n];
    pi[0] = 1.0;
    for i in 1..n {
        pi[i] = pi[i - 1] * birth_rates[i - 1] / death_rates[i - 1];
    }
    let total: f64 = pi.iter().sum();
    for p in &mut pi {
        *p /= total;
    }
    pi
}

pub fn markov_chain_entropy_rate(matrix: &[Vec<f64>], stationary: &[f64]) -> f64 {
    let mut h = 0.0;
    for (mi, &si) in matrix.iter().zip(stationary) {
        for &mij in mi {
            if mij > 0.0 && si > 0.0 {
                h -= si * mij * mij.ln();
            }
        }
    }
    h
}

pub fn is_doubly_stochastic(matrix: &[Vec<f64>]) -> bool {
    let n = matrix.len();
    for mi in matrix {
        let row_sum: f64 = mi.iter().sum();
        if (row_sum - 1.0).abs() > 1e-8 {
            return false;
        }
    }
    for j in 0..n {
        let col_sum: f64 = matrix.iter().map(|row| row[j]).sum();
        if (col_sum - 1.0).abs() > 1e-8 {
            return false;
        }
    }
    true
}

pub fn mixing_time_estimate(matrix: &[Vec<f64>], epsilon: f64, max_steps: usize) -> usize {
    let n = matrix.len();
    let uniform = vec![1.0 / n as f64; n];
    let mut power = matrix.to_vec();
    for step in 1..=max_steps {
        let mut max_tv = 0.0_f64;
        for pi in &power {
            let tv: f64 = pi
                .iter()
                .zip(uniform.iter())
                .map(|(a, b)| (a - b).abs())
                .sum::<f64>()
                / 2.0;
            if tv > max_tv {
                max_tv = tv;
            }
        }
        if max_tv < epsilon {
            return step;
        }
        power = mat_mul_markov(&power, matrix);
    }
    max_steps
}
