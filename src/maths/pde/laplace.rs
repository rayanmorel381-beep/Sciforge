pub fn laplace_jacobi(u: &[Vec<f64>], max_iter: usize, tol: f64) -> Vec<Vec<f64>> {
    let ny = u.len();
    let nx = u[0].len();
    let mut phi = u.to_vec();
    for _ in 0..max_iter {
        let mut max_diff = 0.0_f64;
        let old = phi.clone();
        for j in 1..ny - 1 {
            for i in 1..nx - 1 {
                phi[j][i] = 0.25 * (old[j][i + 1] + old[j][i - 1] + old[j + 1][i] + old[j - 1][i]);
                let diff = (phi[j][i] - old[j][i]).abs();
                if diff > max_diff {
                    max_diff = diff;
                }
            }
        }
        if max_diff < tol {
            break;
        }
    }
    phi
}

pub fn laplace_gauss_seidel(u: &[Vec<f64>], max_iter: usize, tol: f64) -> Vec<Vec<f64>> {
    let ny = u.len();
    let nx = u[0].len();
    let mut phi = u.to_vec();
    for _ in 0..max_iter {
        let mut max_diff = 0.0_f64;
        for j in 1..ny - 1 {
            for i in 1..nx - 1 {
                let old_val = phi[j][i];
                phi[j][i] = 0.25 * (phi[j][i + 1] + phi[j][i - 1] + phi[j + 1][i] + phi[j - 1][i]);
                let diff = (phi[j][i] - old_val).abs();
                if diff > max_diff {
                    max_diff = diff;
                }
            }
        }
        if max_diff < tol {
            break;
        }
    }
    phi
}

pub fn laplace_sor(u: &[Vec<f64>], omega: f64, max_iter: usize, tol: f64) -> Vec<Vec<f64>> {
    let ny = u.len();
    let nx = u[0].len();
    let mut phi = u.to_vec();
    for _ in 0..max_iter {
        let mut max_diff = 0.0_f64;
        for j in 1..ny - 1 {
            for i in 1..nx - 1 {
                let old_val = phi[j][i];
                let gs = 0.25 * (phi[j][i + 1] + phi[j][i - 1] + phi[j + 1][i] + phi[j - 1][i]);
                phi[j][i] = (1.0 - omega) * old_val + omega * gs;
                let diff = (phi[j][i] - old_val).abs();
                if diff > max_diff {
                    max_diff = diff;
                }
            }
        }
        if max_diff < tol {
            break;
        }
    }
    phi
}

pub fn optimal_sor_omega(nx: usize, ny: usize) -> f64 {
    let rho_j =
        ((std::f64::consts::PI / nx as f64).cos() + (std::f64::consts::PI / ny as f64).cos()) / 2.0;
    2.0 / (1.0 + (1.0 - rho_j * rho_j).sqrt())
}

pub fn poisson_jacobi(
    u: &[Vec<f64>],
    f_rhs: &[Vec<f64>],
    dx: f64,
    dy: f64,
    max_iter: usize,
    tol: f64,
) -> Vec<Vec<f64>> {
    let ny = u.len();
    let nx = u[0].len();
    let mut phi = u.to_vec();
    let dx2 = dx * dx;
    let dy2 = dy * dy;
    let denom = 2.0 * (1.0 / dx2 + 1.0 / dy2);
    for _ in 0..max_iter {
        let mut max_diff = 0.0_f64;
        let old = phi.clone();
        for j in 1..ny - 1 {
            for i in 1..nx - 1 {
                phi[j][i] = ((old[j][i + 1] + old[j][i - 1]) / dx2
                    + (old[j + 1][i] + old[j - 1][i]) / dy2
                    - f_rhs[j][i])
                    / denom;
                let diff = (phi[j][i] - old[j][i]).abs();
                if diff > max_diff {
                    max_diff = diff;
                }
            }
        }
        if max_diff < tol {
            break;
        }
    }
    phi
}

pub fn residual_norm(phi: &[Vec<f64>], f_rhs: &[Vec<f64>], dx: f64, dy: f64) -> f64 {
    let ny = phi.len();
    let nx = phi[0].len();
    let dx2 = dx * dx;
    let dy2 = dy * dy;
    let mut norm = 0.0;
    for j in 1..ny - 1 {
        for i in 1..nx - 1 {
            let lap = (phi[j][i + 1] - 2.0 * phi[j][i] + phi[j][i - 1]) / dx2
                + (phi[j + 1][i] - 2.0 * phi[j][i] + phi[j - 1][i]) / dy2;
            let r = lap - f_rhs[j][i];
            norm += r * r;
        }
    }
    norm.sqrt()
}

pub fn harmonic_function_disk(r: f64, theta: f64, a: &[f64], b: &[f64], radius: f64) -> f64 {
    let mut val = a[0] / 2.0;
    for n in 1..a.len().min(b.len()) {
        let rn = (r / radius).powi(n as i32);
        val += rn * (a[n] * (n as f64 * theta).cos() + b[n] * (n as f64 * theta).sin());
    }
    val
}

pub fn poisson_gauss_seidel(
    u: &[Vec<f64>],
    f_rhs: &[Vec<f64>],
    dx: f64,
    dy: f64,
    max_iter: usize,
    tol: f64,
) -> Vec<Vec<f64>> {
    let ny = u.len();
    let nx = u[0].len();
    let mut phi = u.to_vec();
    let dx2 = dx * dx;
    let dy2 = dy * dy;
    let denom = 2.0 * (1.0 / dx2 + 1.0 / dy2);
    for _ in 0..max_iter {
        let mut max_diff = 0.0_f64;
        for j in 1..ny - 1 {
            for i in 1..nx - 1 {
                let old_val = phi[j][i];
                phi[j][i] = ((phi[j][i + 1] + phi[j][i - 1]) / dx2
                    + (phi[j + 1][i] + phi[j - 1][i]) / dy2
                    - f_rhs[j][i])
                    / denom;
                let diff = (phi[j][i] - old_val).abs();
                if diff > max_diff {
                    max_diff = diff;
                }
            }
        }
        if max_diff < tol {
            break;
        }
    }
    phi
}

pub fn poisson_sor(
    u: &[Vec<f64>],
    f_rhs: &[Vec<f64>],
    dx: f64,
    dy: f64,
    omega: f64,
    max_iter: usize,
    tol: f64,
) -> Vec<Vec<f64>> {
    let ny = u.len();
    let nx = u[0].len();
    let mut phi = u.to_vec();
    let dx2 = dx * dx;
    let dy2 = dy * dy;
    let denom = 2.0 * (1.0 / dx2 + 1.0 / dy2);
    for _ in 0..max_iter {
        let mut max_diff = 0.0_f64;
        for j in 1..ny - 1 {
            for i in 1..nx - 1 {
                let old_val = phi[j][i];
                let gs = ((phi[j][i + 1] + phi[j][i - 1]) / dx2
                    + (phi[j + 1][i] + phi[j - 1][i]) / dy2
                    - f_rhs[j][i])
                    / denom;
                phi[j][i] = (1.0 - omega) * old_val + omega * gs;
                let diff = (phi[j][i] - old_val).abs();
                if diff > max_diff {
                    max_diff = diff;
                }
            }
        }
        if max_diff < tol {
            break;
        }
    }
    phi
}

pub fn max_principle_check(phi: &[Vec<f64>]) -> bool {
    let ny = phi.len();
    let nx = phi[0].len();
    let mut interior_max = f64::NEG_INFINITY;
    let mut interior_min = f64::INFINITY;
    let mut boundary_max = f64::NEG_INFINITY;
    let mut boundary_min = f64::INFINITY;
    for (j, phi_j) in phi.iter().enumerate() {
        for (i, &v) in phi_j.iter().enumerate() {
            if j == 0 || j == ny - 1 || i == 0 || i == nx - 1 {
                if v > boundary_max {
                    boundary_max = v;
                }
                if v < boundary_min {
                    boundary_min = v;
                }
            } else {
                if v > interior_max {
                    interior_max = v;
                }
                if v < interior_min {
                    interior_min = v;
                }
            }
        }
    }
    interior_max <= boundary_max + 1e-10 && interior_min >= boundary_min - 1e-10
}

pub fn greens_function_2d_free(x: f64, y: f64, xs: f64, ys: f64) -> f64 {
    let r2 = (x - xs).powi(2) + (y - ys).powi(2);
    if r2 < 1e-30 {
        return 0.0;
    }
    -r2.ln() / (4.0 * std::f64::consts::PI)
}

pub fn mean_value_property(phi: &[Vec<f64>], ci: usize, cj: usize, radius: usize) -> f64 {
    let ny = phi.len();
    let nx = phi[0].len();
    let mut sum = 0.0;
    let mut count = 0;
    let r2 = (radius * radius) as i64;
    for dj in -(radius as i64)..=(radius as i64) {
        for di in -(radius as i64)..=(radius as i64) {
            let dist2 = di * di + dj * dj;
            if dist2 > r2 - 1 && dist2 <= r2 + 1 {
                let ni = ci as i64 + di;
                let nj = cj as i64 + dj;
                if ni >= 0 && ni < nx as i64 && nj >= 0 && nj < ny as i64 {
                    sum += phi[nj as usize][ni as usize];
                    count += 1;
                }
            }
        }
    }
    if count == 0 {
        phi[cj][ci]
    } else {
        sum / count as f64
    }
}

pub fn laplace_3d_jacobi(u: &[Vec<Vec<f64>>], max_iter: usize, tol: f64) -> Vec<Vec<Vec<f64>>> {
    let nz = u.len();
    let ny = u[0].len();
    let nx = u[0][0].len();
    let mut phi = u.to_vec();
    for _ in 0..max_iter {
        let mut max_diff = 0.0_f64;
        let old = phi.clone();
        for k in 1..nz - 1 {
            for j in 1..ny - 1 {
                for i in 1..nx - 1 {
                    phi[k][j][i] = (old[k][j][i + 1]
                        + old[k][j][i - 1]
                        + old[k][j + 1][i]
                        + old[k][j - 1][i]
                        + old[k + 1][j][i]
                        + old[k - 1][j][i])
                        / 6.0;
                    let diff = (phi[k][j][i] - old[k][j][i]).abs();
                    if diff > max_diff {
                        max_diff = diff;
                    }
                }
            }
        }
        if max_diff < tol {
            break;
        }
    }
    phi
}

pub fn dirichlet_energy(phi: &[Vec<f64>], dx: f64, dy: f64) -> f64 {
    let ny = phi.len();
    let nx = phi[0].len();
    let mut energy = 0.0;
    for j in 1..ny - 1 {
        for i in 1..nx - 1 {
            let gx = (phi[j][i + 1] - phi[j][i - 1]) / (2.0 * dx);
            let gy = (phi[j + 1][i] - phi[j - 1][i]) / (2.0 * dy);
            energy += (gx * gx + gy * gy) * dx * dy;
        }
    }
    0.5 * energy
}

pub fn laplace_iteration_count(u: &[Vec<f64>], max_iter: usize, tol: f64) -> usize {
    let ny = u.len();
    let nx = u[0].len();
    let mut phi = u.to_vec();
    for iter in 0..max_iter {
        let mut max_diff = 0.0_f64;
        let old = phi.clone();
        for j in 1..ny - 1 {
            for i in 1..nx - 1 {
                phi[j][i] = 0.25 * (old[j][i + 1] + old[j][i - 1] + old[j + 1][i] + old[j - 1][i]);
                let diff = (phi[j][i] - old[j][i]).abs();
                if diff > max_diff {
                    max_diff = diff;
                }
            }
        }
        if max_diff < tol {
            return iter + 1;
        }
    }
    max_iter
}
