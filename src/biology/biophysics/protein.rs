pub fn ramachandran_energy(phi: f64, psi: f64) -> f64 {
    let v1_phi = 1.0 * (1.0 + phi.cos());
    let v2_phi = 0.5 * (1.0 - (2.0 * phi).cos());
    let v3_phi = 0.2 * (1.0 + (3.0 * phi).cos());
    let v1_psi = 1.0 * (1.0 + psi.cos());
    let v2_psi = 0.5 * (1.0 - (2.0 * psi).cos());
    let v3_psi = 0.2 * (1.0 + (3.0 * psi).cos());
    v1_phi + v2_phi + v3_phi + v1_psi + v2_psi + v3_psi
}

pub fn hydrophobic_free_energy(sasa_nonpolar: f64, gamma: f64) -> f64 {
    gamma * sasa_nonpolar
}

pub fn hydrogen_bond_energy(r: f64, theta: f64, epsilon: f64, r0: f64) -> f64 {
    let distance_term = -(((r - r0) / 0.5).powi(2)).exp();
    let angle_term = theta.cos().powi(2);
    epsilon * distance_term * angle_term
}

pub fn electrostatic_solvation(charge: f64, radius: f64, epsilon_solvent: f64) -> f64 {
    let k = 8.9875e9;
    -k * charge * charge / (2.0 * radius) * (1.0 - 1.0 / epsilon_solvent)
}

pub fn fold_stability(delta_h: f64, delta_s: f64, delta_cp: f64, t: f64, t_ref: f64) -> f64 {
    let dh = delta_h + delta_cp * (t - t_ref);
    let ds = delta_s + delta_cp * (t / t_ref).ln();
    dh - t * ds
}

pub fn fraction_folded(delta_g: f64, t: f64) -> f64 {
    use crate::constants::{K_B, N_A};
    let rt = K_B * N_A * t;
    let k_eq = (-delta_g / rt).exp();
    k_eq / (1.0 + k_eq)
}

pub fn two_state_folding_rate(k0: f64, delta_g_dagger: f64, t: f64) -> f64 {
    use crate::constants::{K_B, N_A};
    let rt = K_B * N_A * t;
    k0 * (-delta_g_dagger / rt).exp()
}

pub fn zimm_bragg_helix_coil(s: f64, sigma: f64, n: usize) -> f64 {
    let sn = s.powi(n as i32);
    sigma * sn / (1.0 + sigma * sn)
}

pub fn contact_order(contacts: &[(usize, usize)], chain_length: usize) -> f64 {
    if contacts.is_empty() || chain_length == 0 {
        return 0.0;
    }
    let total_sep: f64 = contacts
        .iter()
        .map(|&(i, j)| (j as f64 - i as f64).abs())
        .sum();
    total_sep / (contacts.len() as f64 * chain_length as f64)
}

pub fn phi_value(
    delta_g_mut_folding: f64,
    delta_g_wt_folding: f64,
    delta_g_mut_ts: f64,
    delta_g_wt_ts: f64,
) -> f64 {
    let ddg_fold = delta_g_mut_folding - delta_g_wt_folding;
    let ddg_ts = delta_g_mut_ts - delta_g_wt_ts;
    if ddg_fold.abs() < 1e-30 {
        return 0.0;
    }
    ddg_ts / ddg_fold
}

pub fn kauzmann_hydrophobic(delta_cp: f64, t: f64, t_s: f64, t_h: f64, delta_h_h: f64) -> f64 {
    let dh = delta_h_h + delta_cp * (t - t_h);
    let ds = delta_cp * (t / t_s).ln();
    dh - t * ds
}

pub fn go_model_energy(
    contacts: &[(usize, usize)],
    distances: &[f64],
    native_distances: &[f64],
    epsilon: f64,
) -> f64 {
    let mut energy = 0.0;
    for (k, &(_, _)) in contacts.iter().enumerate() {
        if k < distances.len() && k < native_distances.len() {
            let sr6 = (native_distances[k] / distances[k].max(1e-30)).powi(6);
            energy += epsilon * (5.0 * sr6 * sr6 - 6.0 * sr6);
        }
    }
    energy
}

pub fn native_contact_fraction(
    current_distances: &[f64],
    native_distances: &[f64],
    cutoff: f64,
) -> f64 {
    if native_distances.is_empty() {
        return 0.0;
    }
    let n = current_distances.len().min(native_distances.len());
    let formed = (0..n)
        .filter(|&i| (current_distances[i] - native_distances[i]).abs() < cutoff)
        .count();
    formed as f64 / native_distances.len() as f64
}

pub fn radius_of_gyration_3d(coords: &[(f64, f64, f64)]) -> f64 {
    let n = coords.len() as f64;
    if n < 1.0 {
        return 0.0;
    }
    let cx = coords.iter().map(|c| c.0).sum::<f64>() / n;
    let cy = coords.iter().map(|c| c.1).sum::<f64>() / n;
    let cz = coords.iter().map(|c| c.2).sum::<f64>() / n;
    let sum_sq: f64 = coords
        .iter()
        .map(|c| (c.0 - cx).powi(2) + (c.1 - cy).powi(2) + (c.2 - cz).powi(2))
        .sum();
    (sum_sq / n).sqrt()
}

pub fn denaturation_midpoint(delta_h: f64, delta_s: f64) -> f64 {
    if delta_s.abs() < 1e-30 {
        return 0.0;
    }
    delta_h / delta_s
}

pub fn chevron_plot_folding(k_f_water: f64, m_f: f64, denaturant: f64) -> f64 {
    k_f_water * (-m_f * denaturant).exp()
}

pub fn chevron_plot_unfolding(k_u_water: f64, m_u: f64, denaturant: f64) -> f64 {
    k_u_water * (m_u * denaturant).exp()
}
