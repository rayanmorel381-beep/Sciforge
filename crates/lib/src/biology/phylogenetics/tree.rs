pub fn upgma(dist_matrix: &[Vec<f64>]) -> Vec<(usize, usize, f64)> {
    let n = dist_matrix.len();
    let mut d: Vec<Vec<f64>> = dist_matrix.to_vec();
    let mut sizes = vec![1usize; n];
    let mut active: Vec<bool> = vec![true; n];
    let mut merges = Vec::new();
    let mut next_id = n;
    let mut cluster_map: Vec<usize> = (0..n).collect();

    for _ in 0..n - 1 {
        let mut min_d = f64::INFINITY;
        let mut mi = 0;
        let mut mj = 0;
        for (i, di) in d.iter().enumerate() {
            if !active[i] {
                continue;
            }
            for j in (i + 1)..d.len() {
                if !active[j] {
                    continue;
                }
                if di[j] < min_d {
                    min_d = di[j];
                    mi = i;
                    mj = j;
                }
            }
        }

        merges.push((cluster_map[mi], cluster_map[mj], min_d / 2.0));

        let si = sizes[mi];
        let sj = sizes[mj];
        for k in 0..d.len() {
            if !active[k] || k == mi || k == mj {
                continue;
            }
            d[mi][k] = (d[mi][k] * si as f64 + d[mj][k] * sj as f64) / (si + sj) as f64;
            d[k][mi] = d[mi][k];
        }
        sizes[mi] = si + sj;
        active[mj] = false;
        cluster_map[mi] = next_id;
        next_id += 1;
    }
    merges
}

pub fn neighbor_joining(dist_matrix: &[Vec<f64>]) -> Vec<(usize, usize, f64, f64)> {
    let n = dist_matrix.len();
    if n < 2 {
        return Vec::new();
    }
    let mut d: Vec<Vec<f64>> = dist_matrix.to_vec();
    let mut active: Vec<bool> = vec![true; n];
    let mut labels: Vec<usize> = (0..n).collect();
    let mut joins = Vec::new();
    let mut next_label = n;

    for _ in 0..n - 2 {
        let active_count = active.iter().filter(|&&a| a).count();
        if active_count < 2 {
            break;
        }

        let mut r = vec![0.0; d.len()];
        for (i, ri) in r.iter_mut().enumerate() {
            if !active[i] {
                continue;
            }
            for (j, &aj) in active.iter().enumerate() {
                if !aj || i == j {
                    continue;
                }
                *ri += d[i][j];
            }
            *ri /= (active_count as f64 - 2.0).max(1.0);
        }

        let mut min_q = f64::INFINITY;
        let mut mi = 0;
        let mut mj = 0;
        for (i, di) in d.iter().enumerate() {
            if !active[i] {
                continue;
            }
            for j in (i + 1)..d.len() {
                if !active[j] {
                    continue;
                }
                let q = di[j] - r[i] - r[j];
                if q < min_q {
                    min_q = q;
                    mi = i;
                    mj = j;
                }
            }
        }

        let dist_i = 0.5 * d[mi][mj] + 0.5 * (r[mi] - r[mj]);
        let dist_j = d[mi][mj] - dist_i;
        joins.push((labels[mi], labels[mj], dist_i, dist_j));

        for k in 0..d.len() {
            if !active[k] || k == mi || k == mj {
                continue;
            }
            d[mi][k] = 0.5 * (d[mi][k] + d[mj][k] - d[mi][mj]);
            d[k][mi] = d[mi][k];
        }
        active[mj] = false;
        labels[mi] = next_label;
        next_label += 1;
    }

    let remaining: Vec<usize> = (0..d.len()).filter(|&i| active[i]).collect();
    if remaining.len() == 2 {
        let i = remaining[0];
        let j = remaining[1];
        joins.push((labels[i], labels[j], d[i][j] / 2.0, d[i][j] / 2.0));
    }
    joins
}

pub fn wpgma(dist_matrix: &[Vec<f64>]) -> Vec<(usize, usize, f64)> {
    let n = dist_matrix.len();
    let mut d: Vec<Vec<f64>> = dist_matrix.to_vec();
    let mut active: Vec<bool> = vec![true; n];
    let mut merges = Vec::new();
    let mut next_id = n;
    let mut cluster_map: Vec<usize> = (0..n).collect();

    for _ in 0..n - 1 {
        let mut min_d = f64::INFINITY;
        let mut mi = 0;
        let mut mj = 0;
        for (i, di) in d.iter().enumerate() {
            if !active[i] {
                continue;
            }
            for j in (i + 1)..d.len() {
                if !active[j] {
                    continue;
                }
                if di[j] < min_d {
                    min_d = di[j];
                    mi = i;
                    mj = j;
                }
            }
        }

        merges.push((cluster_map[mi], cluster_map[mj], min_d / 2.0));

        for k in 0..d.len() {
            if !active[k] || k == mi || k == mj {
                continue;
            }
            d[mi][k] = 0.5 * (d[mi][k] + d[mj][k]);
            d[k][mi] = d[mi][k];
        }
        active[mj] = false;
        cluster_map[mi] = next_id;
        next_id += 1;
    }
    merges
}

pub fn molecular_clock_test(branch_lengths: &[f64], expected: &[f64]) -> f64 {
    let mut chi2 = 0.0;
    for i in 0..branch_lengths.len() {
        if expected[i] > 1e-30 {
            let d = branch_lengths[i] - expected[i];
            chi2 += d * d / expected[i];
        }
    }
    chi2
}

pub fn robinson_foulds(splits_a: &[Vec<bool>], splits_b: &[Vec<bool>]) -> usize {
    let mut count = 0;
    for sa in splits_a {
        if !splits_b
            .iter()
            .any(|sb| sb == sa || sb.iter().zip(sa.iter()).all(|(&a, &b)| a != b))
        {
            count += 1;
        }
    }
    for sb in splits_b {
        if !splits_a
            .iter()
            .any(|sa| sa == sb || sa.iter().zip(sb.iter()).all(|(&a, &b)| a != b))
        {
            count += 1;
        }
    }
    count
}

pub fn sackin_index(branch_depths: &[usize]) -> f64 {
    branch_depths.iter().map(|&d| d as f64).sum()
}

pub fn colless_index(left_sizes: &[usize], right_sizes: &[usize]) -> f64 {
    left_sizes
        .iter()
        .zip(right_sizes.iter())
        .map(|(&l, &r)| (l as f64 - r as f64).abs())
        .sum()
}

pub fn branch_length_total(branch_lengths: &[f64]) -> f64 {
    branch_lengths.iter().sum()
}

pub fn patristic_distance(tree_distances: &[Vec<f64>], i: usize, j: usize) -> f64 {
    tree_distances[i][j]
}

pub fn parsimony_score(sequences: &[&[u8]], tree_topology: &[(usize, usize)]) -> usize {
    let n_sites = sequences.iter().map(|s| s.len()).min().unwrap_or(0);
    let mut total_score = 0;
    for site in 0..n_sites {
        let mut states: Vec<Vec<u8>> = sequences.iter().map(|s| vec![s[site]]).collect();
        let mut site_score = 0;
        for &(left, right) in tree_topology {
            let left_states = states[left].clone();
            let right_states = states[right].clone();
            let intersection: Vec<u8> = left_states
                .iter()
                .filter(|s| right_states.contains(s))
                .copied()
                .collect();
            if intersection.is_empty() {
                site_score += 1;
                let mut union = left_states;
                for s in &right_states {
                    if !union.contains(s) {
                        union.push(*s);
                    }
                }
                states.push(union);
            } else {
                states.push(intersection);
            }
        }
        total_score += site_score;
    }
    total_score
}

pub fn gamma_rate_categories(alpha: f64, n_categories: usize) -> Vec<f64> {
    let mut rates = Vec::with_capacity(n_categories);
    for i in 0..n_categories {
        let p = (i as f64 + 0.5) / n_categories as f64;
        let rate = alpha * (1.0 + (p - 0.5) * 2.0 / alpha.sqrt());
        rates.push(rate.max(0.01));
    }
    let mean: f64 = rates.iter().sum::<f64>() / n_categories as f64;
    for r in rates.iter_mut() {
        *r /= mean;
    }
    rates
}
