type HillClimbOptimizer = fn(fn(&[f64]) -> f64, &[f64], &[(f64, f64)], f64, usize, u64) -> Vec<f64>;

pub fn simulated_annealing(
    objective: fn(&[f64]) -> f64,
    x0: &[f64],
    bounds: &[(f64, f64)],
    t_initial: f64,
    t_min: f64,
    cooling_rate: f64,
    steps_per_temp: usize,
    seed: u64,
) -> Vec<f64> {
    let mut rng = SimpleRng::new(seed);
    let mut x = x0.to_vec();
    let mut best = x.clone();
    let mut e = objective(&x);
    let mut e_best = e;
    let mut temp = t_initial;
    while temp > t_min {
        for _ in 0..steps_per_temp {
            let mut candidate = x.clone();
            let idx = rng.next_usize(x.len());
            let range = bounds[idx].1 - bounds[idx].0;
            candidate[idx] += (rng.next_f64() - 0.5) * range * 0.1;
            candidate[idx] = candidate[idx].clamp(bounds[idx].0, bounds[idx].1);
            let e_new = objective(&candidate);
            let delta = e_new - e;
            if delta < 0.0 || rng.next_f64() < (-delta / temp).exp() {
                x = candidate;
                e = e_new;
                if e < e_best {
                    best = x.clone();
                    e_best = e;
                }
            }
        }
        temp *= cooling_rate;
    }
    best
}

pub fn particle_swarm(
    objective: fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    n_particles: usize,
    max_iter: usize,
    w: f64,
    c1: f64,
    c2: f64,
    seed: u64,
) -> Vec<f64> {
    let n = bounds.len();
    let mut rng = SimpleRng::new(seed);
    let mut positions: Vec<Vec<f64>> = (0..n_particles)
        .map(|_| {
            (0..n)
                .map(|j| bounds[j].0 + rng.next_f64() * (bounds[j].1 - bounds[j].0))
                .collect()
        })
        .collect();
    let mut velocities: Vec<Vec<f64>> = (0..n_particles).map(|_| vec![0.0; n]).collect();
    let mut p_best = positions.clone();
    let mut p_best_val: Vec<f64> = positions.iter().map(|p| objective(p)).collect();
    let g_best_idx = p_best_val
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap()
        .0;
    let mut g_best = positions[g_best_idx].clone();
    let mut g_best_val = p_best_val[g_best_idx];
    for _ in 0..max_iter {
        for i in 0..n_particles {
            for j in 0..n {
                let r1 = rng.next_f64();
                let r2 = rng.next_f64();
                velocities[i][j] = w * velocities[i][j]
                    + c1 * r1 * (p_best[i][j] - positions[i][j])
                    + c2 * r2 * (g_best[j] - positions[i][j]);
                positions[i][j] =
                    (positions[i][j] + velocities[i][j]).clamp(bounds[j].0, bounds[j].1);
            }
            let val = objective(&positions[i]);
            if val < p_best_val[i] {
                p_best_val[i] = val;
                p_best[i] = positions[i].clone();
                if val < g_best_val {
                    g_best_val = val;
                    g_best = positions[i].clone();
                }
            }
        }
    }
    g_best
}

pub fn nelder_mead(
    f: fn(&[f64]) -> f64,
    x0: &[f64],
    step: f64,
    max_iter: usize,
    tol: f64,
) -> Vec<f64> {
    let n = x0.len();
    let mut simplex: Vec<Vec<f64>> = Vec::with_capacity(n + 1);
    simplex.push(x0.to_vec());
    for i in 0..n {
        let mut v = x0.to_vec();
        v[i] += step;
        simplex.push(v);
    }
    for _ in 0..max_iter {
        let mut order: Vec<usize> = (0..=n).collect();
        order.sort_by(|&a, &b| {
            f(&simplex[a])
                .partial_cmp(&f(&simplex[b]))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let best_val = f(&simplex[order[0]]);
        let worst_val = f(&simplex[order[n]]);
        if worst_val - best_val < tol {
            break;
        }
        let centroid: Vec<f64> = (0..n)
            .map(|j| order[..n].iter().map(|&i| simplex[i][j]).sum::<f64>() / n as f64)
            .collect();
        let reflected: Vec<f64> = (0..n)
            .map(|j| 2.0 * centroid[j] - simplex[order[n]][j])
            .collect();
        let fr = f(&reflected);
        if fr < f(&simplex[order[n - 1]]) && fr >= best_val {
            simplex[order[n]] = reflected;
        } else if fr < best_val {
            let expanded: Vec<f64> = (0..n)
                .map(|j| 3.0 * centroid[j] - 2.0 * simplex[order[n]][j])
                .collect();
            if f(&expanded) < fr {
                simplex[order[n]] = expanded;
            } else {
                simplex[order[n]] = reflected;
            }
        } else {
            let contracted: Vec<f64> = (0..n)
                .map(|j| 0.5 * (centroid[j] + simplex[order[n]][j]))
                .collect();
            if f(&contracted) < worst_val {
                simplex[order[n]] = contracted;
            } else {
                for i in 1..=n {
                    let best = simplex[order[0]].clone();
                    for (sij, &bj) in simplex[order[i]].iter_mut().zip(&best) {
                        *sij = 0.5 * (*sij + bj);
                    }
                }
            }
        }
    }
    let mut order: Vec<usize> = (0..=n).collect();
    order.sort_by(|&a, &b| {
        f(&simplex[a])
            .partial_cmp(&f(&simplex[b]))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    simplex[order[0]].clone()
}

struct SimpleRng {
    state: u64,
}
impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn next_usize(&mut self, max: usize) -> usize {
        (self.next_f64() * max as f64) as usize % max
    }
}

pub fn tabu_search(
    objective: fn(&[f64]) -> f64,
    x0: &[f64],
    bounds: &[(f64, f64)],
    tabu_size: usize,
    n_neighbors: usize,
    max_iter: usize,
    seed: u64,
) -> Vec<f64> {
    let n = x0.len();
    let mut rng = SimpleRng::new(seed);
    let mut best = x0.to_vec();
    let mut best_val = objective(&best);
    let mut current = best.clone();
    let mut tabu_list: Vec<Vec<f64>> = Vec::new();
    for _ in 0..max_iter {
        let mut best_neighbor = current.clone();
        let mut best_neighbor_val = f64::INFINITY;
        for _ in 0..n_neighbors {
            let mut neighbor = current.clone();
            let idx = rng.next_usize(n);
            let range = bounds[idx].1 - bounds[idx].0;
            neighbor[idx] = (neighbor[idx] + (rng.next_f64() - 0.5) * range * 0.2)
                .clamp(bounds[idx].0, bounds[idx].1);
            let is_tabu = tabu_list.iter().any(|t| {
                t.iter()
                    .zip(neighbor.iter())
                    .all(|(a, b)| (a - b).abs() < 1e-10)
            });
            if is_tabu {
                continue;
            }
            let val = objective(&neighbor);
            if val < best_neighbor_val {
                best_neighbor = neighbor;
                best_neighbor_val = val;
            }
        }
        current = best_neighbor.clone();
        tabu_list.push(best_neighbor.clone());
        if tabu_list.len() > tabu_size {
            tabu_list.remove(0);
        }
        if best_neighbor_val < best_val {
            best = best_neighbor;
            best_val = best_neighbor_val;
        }
    }
    best
}

pub fn firefly_algorithm(
    objective: fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    pop_size: usize,
    max_iter: usize,
    alpha: f64,
    beta0: f64,
    gamma: f64,
    seed: u64,
) -> Vec<f64> {
    let n = bounds.len();
    let mut rng = SimpleRng::new(seed);
    let mut pop: Vec<Vec<f64>> = (0..pop_size)
        .map(|_| {
            (0..n)
                .map(|j| bounds[j].0 + rng.next_f64() * (bounds[j].1 - bounds[j].0))
                .collect()
        })
        .collect();
    let mut fitness: Vec<f64> = pop.iter().map(|p| objective(p)).collect();
    for _ in 0..max_iter {
        for i in 0..pop_size {
            for j in 0..pop_size {
                if fitness[j] < fitness[i] {
                    let dist_sq: f64 = pop[i]
                        .iter()
                        .zip(pop[j].iter())
                        .map(|(a, b)| (a - b) * (a - b))
                        .sum();
                    let beta = beta0 * (-gamma * dist_sq).exp();
                    for (k, bk) in bounds.iter().enumerate() {
                        pop[i][k] +=
                            beta * (pop[j][k] - pop[i][k]) + alpha * (rng.next_f64() - 0.5);
                        pop[i][k] = pop[i][k].clamp(bk.0, bk.1);
                    }
                    fitness[i] = objective(&pop[i]);
                }
            }
        }
    }
    let best = fitness
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap()
        .0;
    pop[best].clone()
}

pub fn grey_wolf_optimizer(
    objective: fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    pop_size: usize,
    max_iter: usize,
    seed: u64,
) -> Vec<f64> {
    let n = bounds.len();
    let mut rng = SimpleRng::new(seed);
    let mut pop: Vec<Vec<f64>> = (0..pop_size)
        .map(|_| {
            (0..n)
                .map(|j| bounds[j].0 + rng.next_f64() * (bounds[j].1 - bounds[j].0))
                .collect()
        })
        .collect();
    let mut fitness: Vec<f64> = pop.iter().map(|p| objective(p)).collect();
    for iter in 0..max_iter {
        let mut sorted: Vec<usize> = (0..pop_size).collect();
        sorted.sort_by(|&a, &b| {
            fitness[a]
                .partial_cmp(&fitness[b])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let alpha_w = pop[sorted[0]].clone();
        let beta_w = pop[sorted[1.min(pop_size - 1)]].clone();
        let delta_w = pop[sorted[2.min(pop_size - 1)]].clone();
        let a_coeff = 2.0 * (1.0 - iter as f64 / max_iter as f64);
        for i in 0..pop_size {
            for j in 0..n {
                let r1 = rng.next_f64();
                let r2 = rng.next_f64();
                let a1 = 2.0 * a_coeff * r1 - a_coeff;
                let c1 = 2.0 * r2;
                let d_alpha = (c1 * alpha_w[j] - pop[i][j]).abs();
                let x1 = alpha_w[j] - a1 * d_alpha;

                let r1 = rng.next_f64();
                let r2 = rng.next_f64();
                let a2 = 2.0 * a_coeff * r1 - a_coeff;
                let c2 = 2.0 * r2;
                let d_beta = (c2 * beta_w[j] - pop[i][j]).abs();
                let x2 = beta_w[j] - a2 * d_beta;

                let r1 = rng.next_f64();
                let r2 = rng.next_f64();
                let a3 = 2.0 * a_coeff * r1 - a_coeff;
                let c3 = 2.0 * r2;
                let d_delta = (c3 * delta_w[j] - pop[i][j]).abs();
                let x3 = delta_w[j] - a3 * d_delta;

                pop[i][j] = ((x1 + x2 + x3) / 3.0).clamp(bounds[j].0, bounds[j].1);
            }
            fitness[i] = objective(&pop[i]);
        }
    }
    let best = fitness
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap()
        .0;
    pop[best].clone()
}

pub fn hill_climbing(
    objective: fn(&[f64]) -> f64,
    x0: &[f64],
    bounds: &[(f64, f64)],
    step_size: f64,
    max_iter: usize,
    seed: u64,
) -> Vec<f64> {
    let n = x0.len();
    let mut rng = SimpleRng::new(seed);
    let mut x = x0.to_vec();
    let mut fx = objective(&x);
    for _ in 0..max_iter {
        let mut candidate = x.clone();
        let idx = rng.next_usize(n);
        let range = bounds[idx].1 - bounds[idx].0;
        candidate[idx] = (candidate[idx] + (rng.next_f64() - 0.5) * range * step_size)
            .clamp(bounds[idx].0, bounds[idx].1);
        let fc = objective(&candidate);
        if fc < fx {
            x = candidate;
            fx = fc;
        }
    }
    x
}

pub fn random_search(
    objective: fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    n_samples: usize,
    seed: u64,
) -> Vec<f64> {
    let n = bounds.len();
    let mut rng = SimpleRng::new(seed);
    let mut best: Vec<f64> = (0..n)
        .map(|j| bounds[j].0 + rng.next_f64() * (bounds[j].1 - bounds[j].0))
        .collect();
    let mut best_val = objective(&best);
    for _ in 1..n_samples {
        let candidate: Vec<f64> = (0..n)
            .map(|j| bounds[j].0 + rng.next_f64() * (bounds[j].1 - bounds[j].0))
            .collect();
        let val = objective(&candidate);
        if val < best_val {
            best = candidate;
            best_val = val;
        }
    }
    best
}

pub fn multistart(
    optimizer: HillClimbOptimizer,
    objective: fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    step_size: f64,
    n_starts: usize,
    iter_per_start: usize,
    seed: u64,
) -> Vec<f64> {
    let n = bounds.len();
    let mut rng = SimpleRng::new(seed);
    let mut global_best: Vec<f64> = Vec::new();
    let mut global_best_val = f64::INFINITY;
    for _ in 0..n_starts {
        let x0: Vec<f64> = (0..n)
            .map(|j| bounds[j].0 + rng.next_f64() * (bounds[j].1 - bounds[j].0))
            .collect();
        let s = rng.next_u64();
        let result = optimizer(objective, &x0, bounds, step_size, iter_per_start, s);
        let val = objective(&result);
        if val < global_best_val {
            global_best = result;
            global_best_val = val;
        }
    }
    global_best
}

pub fn whale_optimization(
    objective: fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    pop_size: usize,
    max_iter: usize,
    seed: u64,
) -> Vec<f64> {
    let dim = bounds.len();
    let mut rng = SimpleRng::new(seed);
    let mut pop: Vec<Vec<f64>> = (0..pop_size)
        .map(|_| {
            (0..dim)
                .map(|j| bounds[j].0 + rng.next_f64() * (bounds[j].1 - bounds[j].0))
                .collect()
        })
        .collect();
    let mut fitness: Vec<f64> = pop.iter().map(|p| objective(p)).collect();
    for t in 0..max_iter {
        let best_idx = fitness
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap()
            .0;
        let leader = pop[best_idx].clone();
        let a = 2.0 * (1.0 - t as f64 / max_iter as f64);
        for i in 0..pop_size {
            let p = rng.next_f64();
            if p < 0.5 {
                let a_vec = 2.0 * a * rng.next_f64() - a;
                if a_vec.abs() < 1.0 {
                    for j in 0..dim {
                        let d = (leader[j] - pop[i][j]).abs();
                        pop[i][j] = (leader[j] - a_vec * d).clamp(bounds[j].0, bounds[j].1);
                    }
                } else {
                    let rand_idx = rng.next_usize(pop_size);
                    for (j, bj) in bounds.iter().enumerate() {
                        let d = (pop[rand_idx][j] - pop[i][j]).abs();
                        pop[i][j] = (pop[rand_idx][j] - a_vec * d).clamp(bj.0, bj.1);
                    }
                }
            } else {
                let l = rng.next_f64() * 2.0 - 1.0;
                let b_spiral = 1.0;
                for j in 0..dim {
                    let d = (leader[j] - pop[i][j]).abs();
                    pop[i][j] =
                        (d * (b_spiral * l * std::f64::consts::PI).cos() * (b_spiral * l).exp()
                            + leader[j])
                            .clamp(bounds[j].0, bounds[j].1);
                }
            }
            fitness[i] = objective(&pop[i]);
        }
    }
    let best = fitness
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap()
        .0;
    pop[best].clone()
}
