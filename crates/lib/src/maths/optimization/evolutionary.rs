pub fn genetic_algorithm(
    fitness: fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    pop_size: usize,
    generations: usize,
    mutation_rate: f64,
    seed: u64,
) -> Vec<f64> {
    let n = bounds.len();
    let mut rng = LcgRng::new(seed);
    let mut population: Vec<Vec<f64>> = (0..pop_size)
        .map(|_| {
            (0..n)
                .map(|j| bounds[j].0 + rng.next_f64() * (bounds[j].1 - bounds[j].0))
                .collect()
        })
        .collect();
    for _ in 0..generations {
        let mut scored: Vec<(f64, usize)> = population
            .iter()
            .enumerate()
            .map(|(i, ind)| (fitness(ind), i))
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        let elite_count = pop_size / 5;
        let mut new_pop: Vec<Vec<f64>> = scored
            .iter()
            .take(elite_count)
            .map(|&(_, i)| population[i].clone())
            .collect();
        while new_pop.len() < pop_size {
            let p1 = &population[scored[rng.next_usize(elite_count)].1];
            let p2 = &population[scored[rng.next_usize(elite_count)].1];
            let child: Vec<f64> = (0..n)
                .map(|j| {
                    let val = if rng.next_f64() < 0.5 { p1[j] } else { p2[j] };
                    if rng.next_f64() < mutation_rate {
                        let range = bounds[j].1 - bounds[j].0;
                        (val + (rng.next_f64() - 0.5) * range * 0.1).clamp(bounds[j].0, bounds[j].1)
                    } else {
                        val
                    }
                })
                .collect();
            new_pop.push(child);
        }
        population = new_pop;
    }
    population
        .iter()
        .max_by(|a, b| {
            fitness(a)
                .partial_cmp(&fitness(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap()
        .clone()
}

pub fn differential_evolution(
    objective: fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    pop_size: usize,
    generations: usize,
    f_weight: f64,
    cr: f64,
    seed: u64,
) -> Vec<f64> {
    let n = bounds.len();
    let mut rng = LcgRng::new(seed);
    let mut population: Vec<Vec<f64>> = (0..pop_size)
        .map(|_| {
            (0..n)
                .map(|j| bounds[j].0 + rng.next_f64() * (bounds[j].1 - bounds[j].0))
                .collect()
        })
        .collect();
    let mut fitness: Vec<f64> = population.iter().map(|ind| objective(ind)).collect();
    for _ in 0..generations {
        for i in 0..pop_size {
            let mut idxs = [0usize; 3];
            for slot in &mut idxs {
                loop {
                    let v = rng.next_usize(pop_size);
                    if v != i {
                        *slot = v;
                        break;
                    }
                }
            }
            let j_rand = rng.next_usize(n);
            let trial: Vec<f64> = (0..n)
                .map(|j| {
                    if rng.next_f64() < cr || j == j_rand {
                        let v = population[idxs[0]][j]
                            + f_weight * (population[idxs[1]][j] - population[idxs[2]][j]);
                        v.clamp(bounds[j].0, bounds[j].1)
                    } else {
                        population[i][j]
                    }
                })
                .collect();
            let trial_f = objective(&trial);
            if trial_f <= fitness[i] {
                population[i] = trial;
                fitness[i] = trial_f;
            }
        }
    }
    let best = fitness
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap()
        .0;
    population[best].clone()
}

struct LcgRng {
    state: u64,
}
impl LcgRng {
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

pub fn evolution_strategy(
    objective: fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    mu: usize,
    lambda: usize,
    sigma: f64,
    generations: usize,
    seed: u64,
) -> Vec<f64> {
    let n = bounds.len();
    let mut rng = LcgRng::new(seed);
    let mut population: Vec<Vec<f64>> = (0..mu)
        .map(|_| {
            (0..n)
                .map(|j| bounds[j].0 + rng.next_f64() * (bounds[j].1 - bounds[j].0))
                .collect()
        })
        .collect();
    for _ in 0..generations {
        let mut offspring = Vec::with_capacity(lambda);
        for _ in 0..lambda {
            let parent_idx = rng.next_usize(mu);
            let child: Vec<f64> = (0..n)
                .map(|j| {
                    let noise = (rng.next_f64() - 0.5) * 2.0 * sigma;
                    (population[parent_idx][j] + noise).clamp(bounds[j].0, bounds[j].1)
                })
                .collect();
            offspring.push(child);
        }
        offspring.sort_by(|a, b| {
            objective(a)
                .partial_cmp(&objective(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        population = offspring.into_iter().take(mu).collect();
    }
    population
        .into_iter()
        .min_by(|a, b| {
            objective(a)
                .partial_cmp(&objective(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap()
}

pub fn scatter_search(
    objective: fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    pop_size: usize,
    ref_size: usize,
    generations: usize,
    seed: u64,
) -> Vec<f64> {
    let n = bounds.len();
    let mut rng = LcgRng::new(seed);
    let mut pop: Vec<Vec<f64>> = (0..pop_size)
        .map(|_| {
            (0..n)
                .map(|j| bounds[j].0 + rng.next_f64() * (bounds[j].1 - bounds[j].0))
                .collect()
        })
        .collect();
    for _ in 0..generations {
        pop.sort_by(|a, b| {
            objective(a)
                .partial_cmp(&objective(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let ref_set: Vec<Vec<f64>> = pop.iter().take(ref_size).cloned().collect();
        let mut new_pop = ref_set.clone();
        for i in 0..ref_size {
            for j in (i + 1)..ref_size {
                let child: Vec<f64> = (0..n)
                    .map(|k| {
                        let alpha = rng.next_f64();
                        (alpha * ref_set[i][k] + (1.0 - alpha) * ref_set[j][k])
                            .clamp(bounds[k].0, bounds[k].1)
                    })
                    .collect();
                new_pop.push(child);
            }
        }
        new_pop.sort_by(|a, b| {
            objective(a)
                .partial_cmp(&objective(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        pop = new_pop.into_iter().take(pop_size).collect();
    }
    pop.into_iter()
        .min_by(|a, b| {
            objective(a)
                .partial_cmp(&objective(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap()
}

pub fn multi_objective_dominates(a: &[f64], b: &[f64]) -> bool {
    let mut at_least_one_better = false;
    for (ai, bi) in a.iter().zip(b.iter()) {
        if ai > bi {
            return false;
        }
        if ai < bi {
            at_least_one_better = true;
        }
    }
    at_least_one_better
}

pub fn crowding_distance(objectives: &[Vec<f64>]) -> Vec<f64> {
    let n = objectives.len();
    if n == 0 {
        return Vec::new();
    }
    let m = objectives[0].len();
    let mut dist = vec![0.0; n];
    for obj_idx in 0..m {
        let col: Vec<f64> = objectives.iter().map(|o| o[obj_idx]).collect();
        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_by(|&a, &b| {
            col[a]
                .partial_cmp(&col[b])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let range = col[indices[n - 1]] - col[indices[0]];
        if range < 1e-30 {
            continue;
        }
        dist[indices[0]] = f64::INFINITY;
        dist[indices[n - 1]] = f64::INFINITY;
        for i in 1..(n - 1) {
            dist[indices[i]] += (col[indices[i + 1]] - col[indices[i - 1]]) / range;
        }
    }
    dist
}

pub fn nsga2_non_dominated_sort(objectives: &[Vec<f64>]) -> Vec<Vec<usize>> {
    let n = objectives.len();
    let mut dominated_count = vec![0usize; n];
    let mut dominates: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if multi_objective_dominates(&objectives[i], &objectives[j]) {
                dominates[i].push(j);
            } else if multi_objective_dominates(&objectives[j], &objectives[i]) {
                dominated_count[i] += 1;
            }
        }
    }
    let mut fronts = Vec::new();
    let mut current_front: Vec<usize> = (0..n).filter(|&i| dominated_count[i] == 0).collect();
    while !current_front.is_empty() {
        let mut next_front = Vec::new();
        for &i in &current_front {
            for &j in &dominates[i] {
                dominated_count[j] -= 1;
                if dominated_count[j] == 0 {
                    next_front.push(j);
                }
            }
        }
        fronts.push(current_front);
        current_front = next_front;
    }
    fronts
}

pub fn hypervolume_2d(points: &[(f64, f64)], ref_point: (f64, f64)) -> f64 {
    let mut filtered: Vec<(f64, f64)> = points
        .iter()
        .filter(|p| p.0 < ref_point.0 && p.1 < ref_point.1)
        .cloned()
        .collect();
    filtered.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut vol = 0.0;
    let mut last_y = ref_point.1;
    for p in &filtered {
        if p.1 < last_y {
            vol += (ref_point.0 - p.0) * (last_y - p.1);
            last_y = p.1;
        }
    }
    vol
}

pub fn island_model(
    objective: fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    n_islands: usize,
    pop_per_island: usize,
    generations: usize,
    migration_interval: usize,
    seed: u64,
) -> Vec<f64> {
    let n = bounds.len();
    let mut rng = LcgRng::new(seed);
    let mut islands: Vec<Vec<Vec<f64>>> = (0..n_islands)
        .map(|_| {
            (0..pop_per_island)
                .map(|_| {
                    (0..n)
                        .map(|j| bounds[j].0 + rng.next_f64() * (bounds[j].1 - bounds[j].0))
                        .collect()
                })
                .collect()
        })
        .collect();
    for generation in 0..generations {
        for island in islands.iter_mut() {
            let mut scored: Vec<(f64, usize)> = island
                .iter()
                .enumerate()
                .map(|(i, ind)| (objective(ind), i))
                .collect();
            scored.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            let best_idx = scored[0].1;
            let best = island[best_idx].clone();
            let worst_idx = scored.last().unwrap().1;
            let mutant: Vec<f64> = (0..n)
                .map(|j| {
                    (best[j] + (rng.next_f64() - 0.5) * (bounds[j].1 - bounds[j].0) * 0.1)
                        .clamp(bounds[j].0, bounds[j].1)
                })
                .collect();
            island[worst_idx] = mutant;
        }
        if generation > 0 && generation % migration_interval == 0 {
            for i in 0..n_islands {
                let next = (i + 1) % n_islands;
                let best_i = islands[i]
                    .iter()
                    .min_by(|a, b| {
                        objective(a)
                            .partial_cmp(&objective(b))
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .unwrap()
                    .clone();
                let worst_idx = islands[next]
                    .iter()
                    .enumerate()
                    .max_by(|a, b| {
                        objective(a.1)
                            .partial_cmp(&objective(b.1))
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .unwrap()
                    .0;
                islands[next][worst_idx] = best_i;
            }
        }
    }
    islands
        .into_iter()
        .flat_map(|island| island.into_iter())
        .min_by(|a, b| {
            objective(a)
                .partial_cmp(&objective(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap()
}
