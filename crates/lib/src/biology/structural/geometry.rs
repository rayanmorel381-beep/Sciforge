pub fn euclidean_distance_3d(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn bond_angle(a: &[f64; 3], b: &[f64; 3], c: &[f64; 3]) -> f64 {
    let ba = [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let bc = [c[0] - b[0], c[1] - b[1], c[2] - b[2]];
    let dot = ba[0] * bc[0] + ba[1] * bc[1] + ba[2] * bc[2];
    let mag_ba = (ba[0] * ba[0] + ba[1] * ba[1] + ba[2] * ba[2]).sqrt();
    let mag_bc = (bc[0] * bc[0] + bc[1] * bc[1] + bc[2] * bc[2]).sqrt();
    let cos_theta = dot / (mag_ba * mag_bc).max(1e-30);
    cos_theta.clamp(-1.0, 1.0).acos()
}

pub fn dihedral_angle(a: &[f64; 3], b: &[f64; 3], c: &[f64; 3], d: &[f64; 3]) -> f64 {
    let b1 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let b2 = [c[0] - b[0], c[1] - b[1], c[2] - b[2]];
    let b3 = [d[0] - c[0], d[1] - c[1], d[2] - c[2]];
    let n1 = cross(&b1, &b2);
    let n2 = cross(&b2, &b3);
    let m = cross(&n1, &b2);
    let mag_b2 = (b2[0] * b2[0] + b2[1] * b2[1] + b2[2] * b2[2]).sqrt();
    let x = dot3(&n1, &n2);
    let y = dot3(&m, &n2) / mag_b2.max(1e-30);
    y.atan2(x)
}

fn cross(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot3(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn center_of_mass(coords: &[[f64; 3]], masses: &[f64]) -> [f64; 3] {
    let n = coords.len().min(masses.len());
    let mut com = [0.0; 3];
    let mut total_mass = 0.0;
    for i in 0..n {
        com[0] += masses[i] * coords[i][0];
        com[1] += masses[i] * coords[i][1];
        com[2] += masses[i] * coords[i][2];
        total_mass += masses[i];
    }
    if total_mass > 1e-30 {
        com[0] /= total_mass;
        com[1] /= total_mass;
        com[2] /= total_mass;
    }
    com
}

pub fn radius_of_gyration(coords: &[[f64; 3]], masses: &[f64]) -> f64 {
    let com = center_of_mass(coords, masses);
    let n = coords.len().min(masses.len());
    let mut total_mass = 0.0;
    let mut rg2 = 0.0;
    for i in 0..n {
        let dx = coords[i][0] - com[0];
        let dy = coords[i][1] - com[1];
        let dz = coords[i][2] - com[2];
        rg2 += masses[i] * (dx * dx + dy * dy + dz * dz);
        total_mass += masses[i];
    }
    (rg2 / total_mass.max(1e-30)).sqrt()
}

pub fn solvent_accessible_distance(point: &[f64; 3], surface_points: &[[f64; 3]]) -> f64 {
    surface_points
        .iter()
        .map(|s| euclidean_distance_3d(point, s))
        .fold(f64::INFINITY, f64::min)
}

pub fn inertia_tensor(coords: &[[f64; 3]], masses: &[f64]) -> [[f64; 3]; 3] {
    let com = center_of_mass(coords, masses);
    let n = coords.len().min(masses.len());
    let mut tensor = [[0.0; 3]; 3];
    for i in 0..n {
        let dx = coords[i][0] - com[0];
        let dy = coords[i][1] - com[1];
        let dz = coords[i][2] - com[2];
        let r2 = dx * dx + dy * dy + dz * dz;
        tensor[0][0] += masses[i] * (r2 - dx * dx);
        tensor[1][1] += masses[i] * (r2 - dy * dy);
        tensor[2][2] += masses[i] * (r2 - dz * dz);
        tensor[0][1] -= masses[i] * dx * dy;
        tensor[0][2] -= masses[i] * dx * dz;
        tensor[1][2] -= masses[i] * dy * dz;
    }
    tensor[1][0] = tensor[0][1];
    tensor[2][0] = tensor[0][2];
    tensor[2][1] = tensor[1][2];
    tensor
}

pub fn planarity(coords: &[[f64; 3]]) -> f64 {
    if coords.len() < 3 {
        return 0.0;
    }
    let n = coords.len();
    let cx: f64 = coords.iter().map(|c| c[0]).sum::<f64>() / n as f64;
    let cy: f64 = coords.iter().map(|c| c[1]).sum::<f64>() / n as f64;
    let cz: f64 = coords.iter().map(|c| c[2]).sum::<f64>() / n as f64;
    let mut deviation = 0.0;
    for c in coords {
        deviation += (c[2] - cz).powi(2);
    }
    let _ = cx + cy;
    (deviation / n as f64).sqrt()
}
