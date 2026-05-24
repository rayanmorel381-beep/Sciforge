pub fn huckel_secular_determinant(n: usize, alpha: f64, beta: f64) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; n]; n];
    for i in 0..n {
        matrix[i][i] = alpha;
        if i + 1 < n {
            matrix[i][i + 1] = beta;
            matrix[i + 1][i] = beta;
        }
    }
    matrix
}

pub fn huckel_cyclic_determinant(n: usize, alpha: f64, beta: f64) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; n]; n];
    for i in 0..n {
        matrix[i][i] = alpha;
        if i + 1 < n {
            matrix[i][i + 1] = beta;
            matrix[i + 1][i] = beta;
        }
    }
    if n > 2 {
        matrix[0][n - 1] = beta;
        matrix[n - 1][0] = beta;
    }
    matrix
}

pub fn huckel_charge_density(coefficients: &[Vec<f64>], occupations: &[f64]) -> Vec<f64> {
    let n_atoms = if coefficients.is_empty() {
        0
    } else {
        coefficients[0].len()
    };
    let mut density = vec![0.0; n_atoms];
    for (mo, occ) in coefficients.iter().zip(occupations.iter()) {
        for (r, c) in density.iter_mut().zip(mo.iter()) {
            *r += occ * c * c;
        }
    }
    density
}

pub fn huckel_bond_order(
    coefficients: &[Vec<f64>],
    occupations: &[f64],
    atom_i: usize,
    atom_j: usize,
) -> f64 {
    let mut order = 0.0;
    for (mo, occ) in coefficients.iter().zip(occupations.iter()) {
        if atom_i < mo.len() && atom_j < mo.len() {
            order += occ * mo[atom_i] * mo[atom_j];
        }
    }
    order
}

pub fn huckel_total_pi_energy(eigenvalues: &[f64], occupations: &[f64]) -> f64 {
    eigenvalues
        .iter()
        .zip(occupations.iter())
        .map(|(e, o)| e * o)
        .sum()
}

pub fn huckel_free_valence(bond_orders_sum: f64) -> f64 {
    3.0_f64.sqrt() - bond_orders_sum
}
