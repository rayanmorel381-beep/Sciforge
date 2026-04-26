use super::metric::MetricSpace;
use crate::maths::tensor::Tensor;

pub fn riemann_component(
    space: &MetricSpace,
    point: &[f64],
    epsilon: f64,
    rho: usize,
    sigma: usize,
    mu: usize,
    nu: usize,
) -> f64 {
    let gamma = space.christoffel_at(point, epsilon);
    let n = space.dimension();

    let mut p_plus = point.to_vec();
    let mut p_minus = point.to_vec();
    p_plus[mu] += epsilon;
    p_minus[mu] -= epsilon;
    let gamma_plus = space.christoffel_at(&p_plus, epsilon);
    let gamma_minus = space.christoffel_at(&p_minus, epsilon);
    let d_mu: Vec<Vec<Vec<f64>>> = (0..n)
        .map(|a| {
            (0..n)
                .map(|b| {
                    (0..n)
                        .map(|c| (gamma_plus[a][b][c] - gamma_minus[a][b][c]) / (2.0 * epsilon))
                        .collect()
                })
                .collect()
        })
        .collect();

    p_plus = point.to_vec();
    p_minus = point.to_vec();
    p_plus[nu] += epsilon;
    p_minus[nu] -= epsilon;
    let gamma_plus2 = space.christoffel_at(&p_plus, epsilon);
    let gamma_minus2 = space.christoffel_at(&p_minus, epsilon);
    let d_nu: Vec<Vec<Vec<f64>>> = (0..n)
        .map(|a| {
            (0..n)
                .map(|b| {
                    (0..n)
                        .map(|c| (gamma_plus2[a][b][c] - gamma_minus2[a][b][c]) / (2.0 * epsilon))
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut r = d_mu[rho][nu][sigma] - d_nu[rho][mu][sigma];
    let grm = &gamma[rho][mu];
    let grn = &gamma[rho][nu];
    for (gamma_lambda, (&grml, &grnl)) in gamma.iter().zip(grm.iter().zip(grn)) {
        r += grml * gamma_lambda[nu][sigma] - grnl * gamma_lambda[mu][sigma];
    }
    r
}

pub fn ricci_scalar(space: &MetricSpace, point: &[f64], epsilon: f64) -> f64 {
    let n = space.dimension();
    let g = space.metric_tensor_at(point);
    let g_inv = crate::maths::tensor::inverse(&g).unwrap_or_else(|| Tensor::identity(n));
    let mut scalar = 0.0;
    for mu in 0..n {
        for nu in 0..n {
            let mut ricci_mn = 0.0;
            for rho in 0..n {
                ricci_mn += riemann_component(space, point, epsilon, rho, mu, rho, nu);
            }
            scalar += g_inv.get(&[mu, nu]) * ricci_mn;
        }
    }
    scalar
}

pub fn gaussian_curvature_sphere(radius: f64) -> f64 {
    1.0 / (radius * radius)
}

pub fn gaussian_curvature_hyperbolic(curvature: f64) -> f64 {
    -1.0 / (curvature * curvature)
}

pub fn poincare_disk_distance(p1: &[f64; 2], p2: &[f64; 2]) -> f64 {
    let dx = p1[0] - p2[0];
    let dy = p1[1] - p2[1];
    let num = dx * dx + dy * dy;
    let d1 = 1.0 - p1[0] * p1[0] - p1[1] * p1[1];
    let d2 = 1.0 - p2[0] * p2[0] - p2[1] * p2[1];
    let arg = 1.0 + 2.0 * num / (d1 * d2);
    (arg + (arg * arg - 1.0).sqrt()).ln()
}

pub fn spherical_distance(r: f64, theta1: f64, phi1: f64, theta2: f64, phi2: f64) -> f64 {
    let cos_d = theta1.sin() * theta2.sin() * (phi1 - phi2).cos() + theta1.cos() * theta2.cos();
    r * cos_d.clamp(-1.0, 1.0).acos()
}

pub fn parallel_transport_sphere(
    vector: &[f64; 2],
    start_theta: f64,
    start_phi: f64,
    end_theta: f64,
    end_phi: f64,
    steps: usize,
) -> [f64; 2] {
    let mut v = *vector;
    let dt = 1.0 / steps as f64;
    for i in 0..steps {
        let t = i as f64 * dt;
        let theta = start_theta + t * (end_theta - start_theta);
        let dtheta = (end_theta - start_theta) * dt;
        let dphi = (end_phi - start_phi) * dt;
        let sin_t = theta.sin();
        let cos_t = theta.cos();
        if sin_t.abs() > 1e-15 {
            let dv0 = sin_t * cos_t * dphi * v[1];
            let dv1 =
                -cos_t / sin_t * (dphi * v[0] + dtheta * v[1]) + cos_t / sin_t * dtheta * v[1];
            v[0] -= dv0;
            v[1] -= dv1;
        }
    }
    v
}

pub fn sectional_curvature(
    space: &MetricSpace,
    point: &[f64],
    epsilon: f64,
    u: &[usize; 2],
    v: &[usize; 2],
) -> f64 {
    let r = riemann_component(space, point, epsilon, u[0], v[0], u[1], v[1]);
    let g = space.metric_tensor_at(point);
    let n = space.dimension();
    let guu = g.get(&[u[0], u[1]]);
    let gvv = g.get(&[v[0], v[1]]);
    let guv = g.get(&[u[0], v[1]]);
    let denom = guu * gvv - guv * guv;
    if denom.abs() < 1e-30 {
        return 0.0;
    }
    let _ = n;
    r / denom
}

pub fn ricci_tensor_component(
    space: &MetricSpace,
    point: &[f64],
    epsilon: f64,
    mu: usize,
    nu: usize,
) -> f64 {
    let n = space.dimension();
    let mut acc = 0.0;
    for rho in 0..n {
        acc += riemann_component(space, point, epsilon, rho, mu, rho, nu);
    }
    acc
}

pub fn einstein_tensor_component(
    space: &MetricSpace,
    point: &[f64],
    epsilon: f64,
    mu: usize,
    nu: usize,
) -> f64 {
    let ricci_mn = ricci_tensor_component(space, point, epsilon, mu, nu);
    let scalar = ricci_scalar(space, point, epsilon);
    let g = space.metric_tensor_at(point);
    ricci_mn - 0.5 * scalar * g.get(&[mu, nu])
}

pub fn geodesic_deviation_magnitude(separation: f64, velocity: f64, curvature: f64) -> f64 {
    -curvature * separation * velocity * velocity
}

pub fn upper_half_plane_distance(p1: &[f64; 2], p2: &[f64; 2]) -> f64 {
    let dx = p1[0] - p2[0];
    let dy = p1[1] - p2[1];
    let arg = 1.0 + (dx * dx + dy * dy) / (2.0 * p1[1] * p2[1]);
    (arg + (arg * arg - 1.0).max(0.0).sqrt()).ln()
}

pub fn hyperboloid_distance(p1: &[f64; 3], p2: &[f64; 3]) -> f64 {
    let dot = -p1[0] * p2[0] + p1[1] * p2[1] + p1[2] * p2[2];
    dot.max(1.0).acosh()
}

pub fn spherical_area(r: f64, solid_angle_sr: f64) -> f64 {
    r * r * solid_angle_sr
}

pub fn spherical_excess(angles: &[f64; 3]) -> f64 {
    angles[0] + angles[1] + angles[2] - std::f64::consts::PI
}

pub fn hyperbolic_area_triangle(angles: &[f64; 3]) -> f64 {
    (std::f64::consts::PI - angles[0] - angles[1] - angles[2]).abs()
}

pub fn weyl_tensor_vanishes_2d() -> bool {
    true
}

pub fn kretschmer_scalar_schwarzschild(rs: f64, r: f64) -> f64 {
    48.0 * rs * rs / (r * r * r * r * r * r)
}
