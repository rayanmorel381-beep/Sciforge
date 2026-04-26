pub fn cahn_ingold_prelog_priority(atomic_numbers: &[u32]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..atomic_numbers.len()).collect();
    indices.sort_by(|&a, &b| atomic_numbers[b].cmp(&atomic_numbers[a]));
    indices
}

pub fn topological_index_wiener(distance_matrix: &[Vec<u32>]) -> u64 {
    let mut w = 0u64;
    for (i, row) in distance_matrix.iter().enumerate() {
        for val in row.iter().skip(i + 1) {
            w += *val as u64;
        }
    }
    w
}

pub fn randic_index(adjacency: &[Vec<bool>], degrees: &[u32]) -> f64 {
    let n = adjacency.len();
    let mut chi = 0.0;
    for i in 0..n {
        for j in (i + 1)..n {
            if adjacency[i][j] && degrees[i] > 0 && degrees[j] > 0 {
                chi += 1.0 / (degrees[i] as f64 * degrees[j] as f64).sqrt();
            }
        }
    }
    chi
}

pub fn partition_coefficient_log_p(fragments: &[f64]) -> f64 {
    fragments.iter().sum()
}

pub fn topological_polar_surface_area(contributions: &[f64]) -> f64 {
    contributions.iter().sum()
}

pub fn rotatable_bonds(single_bonds: u32, ring_bonds: u32, terminal_bonds: u32) -> u32 {
    single_bonds
        .saturating_sub(ring_bonds)
        .saturating_sub(terminal_bonds)
}
