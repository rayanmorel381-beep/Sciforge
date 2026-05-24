pub fn rmsd(coords_a: &[[f64; 3]], coords_b: &[[f64; 3]]) -> f64 {
    let n = coords_a.len().min(coords_b.len());
    if n == 0 {
        return 0.0;
    }
    let mut sum = 0.0;
    for i in 0..n {
        let dx = coords_a[i][0] - coords_b[i][0];
        let dy = coords_a[i][1] - coords_b[i][1];
        let dz = coords_a[i][2] - coords_b[i][2];
        sum += dx * dx + dy * dy + dz * dz;
    }
    (sum / n as f64).sqrt()
}

pub fn gdt_ts(coords_a: &[[f64; 3]], coords_b: &[[f64; 3]], cutoffs: &[f64]) -> f64 {
    let n = coords_a.len().min(coords_b.len());
    if n == 0 {
        return 0.0;
    }
    let mut total = 0.0;
    for cutoff in cutoffs {
        let count = (0..n)
            .filter(|&i| {
                let dx = coords_a[i][0] - coords_b[i][0];
                let dy = coords_a[i][1] - coords_b[i][1];
                let dz = coords_a[i][2] - coords_b[i][2];
                (dx * dx + dy * dy + dz * dz).sqrt() < *cutoff
            })
            .count();
        total += count as f64 / n as f64;
    }
    total / cutoffs.len().max(1) as f64 * 100.0
}

pub fn tm_score(coords_a: &[[f64; 3]], coords_b: &[[f64; 3]], l_target: usize) -> f64 {
    let n = coords_a.len().min(coords_b.len());
    if n == 0 || l_target == 0 {
        return 0.0;
    }
    let d0 = 1.24 * ((l_target as f64 - 15.0).max(1.0)).cbrt() - 1.8;
    let d0_sq = d0 * d0;
    let mut score = 0.0;
    for i in 0..n {
        let dx = coords_a[i][0] - coords_b[i][0];
        let dy = coords_a[i][1] - coords_b[i][1];
        let dz = coords_a[i][2] - coords_b[i][2];
        let di_sq = dx * dx + dy * dy + dz * dz;
        score += 1.0 / (1.0 + di_sq / d0_sq);
    }
    score / l_target as f64
}

pub fn contact_map(coords: &[[f64; 3]], cutoff: f64) -> Vec<(usize, usize)> {
    let n = coords.len();
    let mut contacts = Vec::new();
    let cutoff_sq = cutoff * cutoff;
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = coords[i][0] - coords[j][0];
            let dy = coords[i][1] - coords[j][1];
            let dz = coords[i][2] - coords[j][2];
            if dx * dx + dy * dy + dz * dz < cutoff_sq {
                contacts.push((i, j));
            }
        }
    }
    contacts
}

pub fn rg_from_coords(coords: &[[f64; 3]]) -> f64 {
    let n = coords.len();
    if n == 0 {
        return 0.0;
    }
    let mut com = [0.0; 3];
    for c in coords {
        com[0] += c[0];
        com[1] += c[1];
        com[2] += c[2];
    }
    com[0] /= n as f64;
    com[1] /= n as f64;
    com[2] /= n as f64;
    let mut rg2 = 0.0;
    for c in coords {
        let dx = c[0] - com[0];
        let dy = c[1] - com[1];
        let dz = c[2] - com[2];
        rg2 += dx * dx + dy * dy + dz * dz;
    }
    (rg2 / n as f64).sqrt()
}

pub fn solvent_accessible_surface_approx(radii: &[f64], probe: f64) -> f64 {
    let mut sasa = 0.0;
    for &r in radii {
        let effective = r + probe;
        sasa += 4.0 * std::f64::consts::PI * effective * effective;
    }
    sasa
}

pub fn lrmsd(coords_a: &[[f64; 3]], coords_b: &[[f64; 3]], residue_indices: &[usize]) -> f64 {
    let n = residue_indices.len();
    if n == 0 {
        return 0.0;
    }
    let mut sum = 0.0;
    for &i in residue_indices {
        if i < coords_a.len() && i < coords_b.len() {
            let dx = coords_a[i][0] - coords_b[i][0];
            let dy = coords_a[i][1] - coords_b[i][1];
            let dz = coords_a[i][2] - coords_b[i][2];
            sum += dx * dx + dy * dy + dz * dz;
        }
    }
    (sum / n as f64).sqrt()
}

pub fn drmsd(coords_a: &[[f64; 3]], coords_b: &[[f64; 3]]) -> f64 {
    let n = coords_a.len().min(coords_b.len());
    if n < 2 {
        return 0.0;
    }
    let mut sum = 0.0;
    let mut count = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let da = ((coords_a[i][0] - coords_a[j][0]).powi(2)
                + (coords_a[i][1] - coords_a[j][1]).powi(2)
                + (coords_a[i][2] - coords_a[j][2]).powi(2))
            .sqrt();
            let db = ((coords_b[i][0] - coords_b[j][0]).powi(2)
                + (coords_b[i][1] - coords_b[j][1]).powi(2)
                + (coords_b[i][2] - coords_b[j][2]).powi(2))
            .sqrt();
            sum += (da - db).powi(2);
            count += 1;
        }
    }
    (sum / count.max(1) as f64).sqrt()
}

pub fn absolute_contact_order(contacts: &[(usize, usize)], n_residues: usize) -> f64 {
    if contacts.is_empty() || n_residues == 0 {
        return 0.0;
    }
    let sum: f64 = contacts
        .iter()
        .map(|&(i, j)| (j as f64 - i as f64).abs())
        .sum();
    sum / (contacts.len() as f64 * n_residues as f64)
}

pub fn b_factor_normalize(b_factors: &[f64]) -> Vec<f64> {
    let n = b_factors.len() as f64;
    if n == 0.0 {
        return Vec::new();
    }
    let mean = b_factors.iter().sum::<f64>() / n;
    let std = (b_factors.iter().map(|&b| (b - mean).powi(2)).sum::<f64>() / n).sqrt();
    if std < 1e-30 {
        return vec![0.0; b_factors.len()];
    }
    b_factors.iter().map(|&b| (b - mean) / std).collect()
}
