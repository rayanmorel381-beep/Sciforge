pub fn landscape_resistance(cost_surface: &[Vec<f64>], path: &[(usize, usize)]) -> f64 {
    let mut total = 0.0;
    for &(r, c) in path {
        if r < cost_surface.len() && c < cost_surface[r].len() {
            total += cost_surface[r][c];
        }
    }
    total
}

pub fn isolation_by_distance(genetic_dist: &[f64], geographic_dist: &[f64]) -> f64 {
    let n = genetic_dist.len().min(geographic_dist.len());
    if n < 2 {
        return 0.0;
    }
    let mean_g: f64 = genetic_dist.iter().take(n).sum::<f64>() / n as f64;
    let mean_d: f64 = geographic_dist.iter().take(n).sum::<f64>() / n as f64;
    let mut cov = 0.0;
    let mut var_g = 0.0;
    let mut var_d = 0.0;
    for i in 0..n {
        let dg = genetic_dist[i] - mean_g;
        let dd = geographic_dist[i] - mean_d;
        cov += dg * dd;
        var_g += dg * dg;
        var_d += dd * dd;
    }
    cov / (var_g * var_d).sqrt().max(1e-30)
}

pub fn connectivity_index(patch_areas: &[f64], distances: &[Vec<f64>], alpha: f64) -> Vec<f64> {
    let n = patch_areas.len();
    let mut conn = vec![0.0; n];
    for (i, ci) in conn.iter_mut().enumerate() {
        for (j, &paj) in patch_areas.iter().enumerate() {
            if i != j && j < distances[i].len() {
                *ci += paj * (-alpha * distances[i][j]).exp();
            }
        }
    }
    conn
}

pub fn metapopulation_incidence(connectivity: f64, e: f64, colonization_coeff: f64) -> f64 {
    let c = colonization_coeff * connectivity;
    c / (c + e)
}

pub fn habitat_fragmentation_index(total_area: f64, n_patches: usize, perimeter_sum: f64) -> f64 {
    if total_area < 1e-30 {
        return 0.0;
    }
    let shape =
        perimeter_sum / (4.0 * (total_area / n_patches.max(1) as f64).sqrt() * n_patches as f64);
    let frag = 1.0 - (1.0 / n_patches.max(1) as f64);
    (shape + frag) / 2.0
}

pub fn effective_mesh_size(patch_areas: &[f64], total_area: f64) -> f64 {
    if total_area < 1e-30 {
        return 0.0;
    }
    patch_areas.iter().map(|&a| a * a).sum::<f64>() / total_area
}

pub fn proximity_index(focal_area: f64, neighbor_areas: &[f64], distances: &[f64]) -> f64 {
    let n = neighbor_areas.len().min(distances.len());
    let mut sum = 0.0;
    for i in 0..n {
        if distances[i] > 0.0 {
            sum += neighbor_areas[i] / (distances[i] * distances[i]);
        }
    }
    sum * focal_area
}

pub fn corridor_effectiveness(
    corridor_length: f64,
    corridor_width: f64,
    matrix_resistance: f64,
) -> f64 {
    (corridor_width / corridor_length) * (-matrix_resistance * corridor_length).exp()
}

pub fn graph_connectivity(adjacency: &[Vec<bool>]) -> f64 {
    let n = adjacency.len();
    if n < 2 {
        return 1.0;
    }
    let mut edges = 0;
    for (i, adj_i) in adjacency.iter().enumerate() {
        for j in (i + 1)..n {
            if j < adj_i.len() && adj_i[j] {
                edges += 1;
            }
        }
    }
    let max_edges = n * (n - 1) / 2;
    edges as f64 / max_edges as f64
}

pub fn stepping_stone_migration(populations: &[f64], migration_rate: f64) -> Vec<f64> {
    let n = populations.len();
    let mut result = vec![0.0; n];
    for (i, ri) in result.iter_mut().enumerate() {
        let left = if i > 0 {
            populations[i - 1]
        } else {
            populations[n - 1]
        };
        let right = if i < n - 1 {
            populations[i + 1]
        } else {
            populations[0]
        };
        *ri = populations[i] * (1.0 - 2.0 * migration_rate) + migration_rate * (left + right);
    }
    result
}

pub fn least_cost_distance(
    cost_surface: &[Vec<f64>],
    start: (usize, usize),
    end: (usize, usize),
) -> f64 {
    let rows = cost_surface.len();
    if rows == 0 {
        return f64::INFINITY;
    }
    let cols = cost_surface[0].len();
    let mut dist = vec![vec![f64::INFINITY; cols]; rows];
    dist[start.0][start.1] = cost_surface[start.0][start.1];
    let mut changed = true;
    while changed {
        changed = false;
        for r in 0..rows {
            for c in 0..cols {
                let current = dist[r][c];
                let neighbors: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                for &(dr, dc) in &neighbors {
                    let nr = r as i64 + dr;
                    let nc = c as i64 + dc;
                    if nr >= 0 && nr < rows as i64 && nc >= 0 && nc < cols as i64 {
                        let nr = nr as usize;
                        let nc = nc as usize;
                        let new_dist = current + cost_surface[nr][nc];
                        if new_dist < dist[nr][nc] {
                            dist[nr][nc] = new_dist;
                            changed = true;
                        }
                    }
                }
            }
        }
    }
    dist[end.0][end.1]
}

pub fn resistance_distance(conductances: &[Vec<f64>], node_a: usize, node_b: usize) -> f64 {
    let n = conductances.len();
    if node_a >= n || node_b >= n {
        return f64::INFINITY;
    }
    let mut total_a = 0.0;
    let mut total_b = 0.0;
    for j in 0..n {
        if j < conductances[node_a].len() {
            total_a += conductances[node_a][j];
        }
        if j < conductances[node_b].len() {
            total_b += conductances[node_b][j];
        }
    }
    if total_a < 1e-30 || total_b < 1e-30 {
        return f64::INFINITY;
    }
    1.0 / total_a + 1.0 / total_b
}

pub fn patch_cohesion(patch_perimeters: &[f64], patch_areas: &[f64], total_cells: f64) -> f64 {
    let n = patch_perimeters.len().min(patch_areas.len());
    let sum_p: f64 = (0..n).map(|i| patch_perimeters[i]).sum();
    let sum_pa: f64 = (0..n)
        .map(|i| patch_perimeters[i] * patch_areas[i].sqrt())
        .sum();
    if sum_p < 1e-30 {
        return 0.0;
    }
    let z = total_cells.sqrt();
    (1.0 - sum_p / sum_pa.max(1e-30)) / (1.0 - 1.0 / z) * 100.0
}

pub fn circuitscape_effective_resistance(node_conductances: &[f64]) -> f64 {
    if node_conductances.is_empty() {
        return f64::INFINITY;
    }
    let sum_inv: f64 = node_conductances.iter().map(|&c| 1.0 / c.max(1e-30)).sum();
    sum_inv
}
