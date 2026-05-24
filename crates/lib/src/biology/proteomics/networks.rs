pub fn ppi_degree(adjacency_row: &[bool]) -> usize {
    adjacency_row.iter().filter(|&&x| x).count()
}

pub fn clustering_coefficient(neighbors_connected: usize, degree: usize) -> f64 {
    if degree < 2 {
        return 0.0;
    }
    2.0 * neighbors_connected as f64 / (degree as f64 * (degree as f64 - 1.0))
}

pub fn betweenness_centrality_approx(
    shortest_paths_through: f64,
    total_shortest_paths: f64,
) -> f64 {
    shortest_paths_through / total_shortest_paths.max(1e-30)
}

pub fn network_density(edges: usize, nodes: usize) -> f64 {
    if nodes < 2 {
        return 0.0;
    }
    2.0 * edges as f64 / (nodes as f64 * (nodes as f64 - 1.0))
}

pub fn scale_free_exponent(degree_distribution: &[f64]) -> f64 {
    let n = degree_distribution.len();
    if n == 0 {
        return 0.0;
    }
    let mut sum_ln = 0.0;
    let mut count = 0;
    for &d in degree_distribution {
        if d > 0.0 {
            sum_ln += d.ln();
            count += 1;
        }
    }
    if count == 0 {
        return 0.0;
    }
    1.0 + count as f64 / sum_ln
}

pub fn hub_score(degree: usize, max_degree: usize) -> f64 {
    degree as f64 / max_degree.max(1) as f64
}

pub fn edge_betweenness(flow_through_edge: f64, total_flow: f64) -> f64 {
    flow_through_edge / total_flow.max(1e-30)
}

pub fn protein_complex_stoichiometry(abundances: &[f64]) -> Vec<f64> {
    if abundances.is_empty() {
        return vec![];
    }
    let min_val = abundances
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min)
        .max(1e-30);
    abundances.iter().map(|a| (a / min_val).round()).collect()
}

pub fn functional_enrichment_odds_ratio(
    hits_in_set: usize,
    set_size: usize,
    hits_total: usize,
    genome_size: usize,
) -> f64 {
    let a = hits_in_set as f64;
    let b = (set_size - hits_in_set) as f64;
    let c = (hits_total - hits_in_set) as f64;
    let d = (genome_size - set_size - hits_total + hits_in_set) as f64;
    (a * d) / (b * c).max(1e-30)
}

pub fn guilt_by_association_score(neighbor_annotations: &[bool]) -> f64 {
    if neighbor_annotations.is_empty() {
        return 0.0;
    }
    let positive: f64 = neighbor_annotations.iter().filter(|&&x| x).count() as f64;
    positive / neighbor_annotations.len() as f64
}
