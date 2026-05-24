pub fn lcao_bonding_energy(alpha: f64, beta: f64, overlap: f64) -> f64 {
    (alpha + beta) / (1.0 + overlap)
}

pub fn lcao_antibonding_energy(alpha: f64, beta: f64, overlap: f64) -> f64 {
    (alpha - beta) / (1.0 - overlap)
}

pub fn overlap_integral_1s(r: f64, zeta: f64) -> f64 {
    let rho = zeta * r;
    (1.0 + rho + rho * rho / 3.0) * (-rho).exp()
}

pub fn hartree_energy(kinetic: f64, nuclear_attraction: f64, electron_repulsion: f64) -> f64 {
    kinetic + nuclear_attraction + electron_repulsion
}

pub fn roothaan_total_energy(
    one_electron: &[f64],
    fock_eigenvalues: &[f64],
    nuclear_repulsion: f64,
) -> f64 {
    let n = one_electron.len().min(fock_eigenvalues.len());
    let mut e = 0.0;
    for i in 0..n {
        e += 0.5 * (one_electron[i] + fock_eigenvalues[i]);
    }
    e + nuclear_repulsion
}

pub fn mulliken_population(density: &[Vec<f64>], overlap: &[Vec<f64>]) -> Vec<f64> {
    let n = density.len();
    let mut populations = vec![0.0; n];
    for i in 0..n {
        for (j, overlap_row) in overlap.iter().enumerate().take(n) {
            populations[i] += density[i][j] * overlap_row[i];
        }
    }
    populations
}

pub fn nuclear_repulsion_energy(z1: f64, z2: f64, r: f64) -> f64 {
    z1 * z2 * 1.389e-9 / r.max(1e-30)
}
