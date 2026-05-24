pub fn fitness_landscape_nk(genotype: &[u8], k: usize, contributions: &[Vec<f64>]) -> f64 {
    let n = genotype.len();
    let mut total = 0.0;
    for (i, ci) in contributions.iter().enumerate() {
        let mut key = 0usize;
        for j in 0..=k {
            let idx = (i + j) % n;
            key = key * 2 + genotype[idx] as usize;
        }
        let idx = key.min(ci.len() - 1);
        total += ci[idx];
    }
    total / n as f64
}

pub fn fitness_landscape_additive(genotype: &[u8], effects: &[f64]) -> f64 {
    let mut w = 1.0;
    for (&gi, &ei) in genotype.iter().zip(effects.iter()) {
        w *= 1.0 + gi as f64 * ei;
    }
    w
}

pub fn fisher_geometric_model(distance: f64, n_dimensions: usize) -> f64 {
    (-distance * distance / (2.0 * n_dimensions as f64)).exp()
}

pub fn mutation_step_probability(distance: f64, step_size: f64, n_dim: usize) -> f64 {
    let nd = n_dim as f64;
    let new_dist_sq = distance * distance + step_size * step_size;
    let old_fitness = (-distance * distance / (2.0 * nd)).exp();
    let new_fitness = (-new_dist_sq / (2.0 * nd)).exp();
    if new_fitness > old_fitness {
        1.0
    } else {
        new_fitness / old_fitness
    }
}

pub fn adaptive_walk(distance0: f64, step_size: f64, n_dim: usize, max_steps: usize) -> Vec<f64> {
    let mut result = Vec::with_capacity(max_steps + 1);
    let mut dist = distance0;
    result.push(fisher_geometric_model(dist, n_dim));
    for _ in 0..max_steps {
        let new_dist = (dist * dist + step_size * step_size).sqrt();
        let p = mutation_step_probability(dist, step_size, n_dim);
        if p > 0.5 {
            dist = new_dist.min(dist);
        }
        result.push(fisher_geometric_model(dist, n_dim));
    }
    result
}

pub fn epistasis(w_ab: f64, w_a: f64, w_b: f64, w_ref: f64) -> f64 {
    (w_ab * w_ref) / (w_a * w_b).max(1e-30) - 1.0
}

pub fn frequency_dependent_fitness(freq: f64, advantage_rare: f64) -> f64 {
    1.0 + advantage_rare * (1.0 - 2.0 * freq)
}

pub fn density_dependent_fitness(population: f64, carrying_capacity: f64, r_max: f64) -> f64 {
    r_max * (1.0 - population / carrying_capacity)
}

pub fn wrightian_fitness(genotype: &[bool], loci_effects: &[f64], dominance: &[f64]) -> f64 {
    let n = genotype.len().min(loci_effects.len()).min(dominance.len());
    let mut w = 1.0;
    for i in 0..n {
        let effect = if genotype[i] {
            loci_effects[i]
        } else {
            dominance[i] * loci_effects[i]
        };
        w *= 1.0 + effect;
    }
    w
}

pub fn fitness_landscape_rugged(genotype: &[u8], peaks: &[(&[u8], f64)]) -> f64 {
    let mut best = 0.0;
    for &(peak, height) in peaks {
        let dist: usize = genotype
            .iter()
            .zip(peak.iter())
            .filter(|&(a, b)| a != b)
            .count();
        let contribution = height * (-0.5 * dist as f64).exp();
        if contribution > best {
            best = contribution;
        }
    }
    best
}
