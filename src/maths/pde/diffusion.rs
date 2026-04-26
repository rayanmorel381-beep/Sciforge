pub fn heat_equation_1d_explicit(u: &[f64], dx: f64, dt: f64, alpha: f64) -> Vec<f64> {
    let n = u.len();
    let r = alpha * dt / (dx * dx);
    let mut u_new = u.to_vec();
    for i in 1..n - 1 {
        u_new[i] = u[i] + r * (u[i + 1] - 2.0 * u[i] + u[i - 1]);
    }
    u_new
}

pub fn heat_equation_1d_implicit(u: &[f64], dx: f64, dt: f64, alpha: f64) -> Vec<f64> {
    let n = u.len();
    let r = alpha * dt / (dx * dx);
    let mut a = vec![0.0; n];
    let mut b = vec![0.0; n];
    let mut c = vec![0.0; n];
    let mut d = u.to_vec();
    b[0] = 1.0;
    b[n - 1] = 1.0;
    for i in 1..n - 1 {
        a[i] = -r;
        b[i] = 1.0 + 2.0 * r;
        c[i] = -r;
    }
    thomas_solve(&a, &b, &c, &mut d);
    d
}

pub fn heat_equation_1d_crank_nicolson(u: &[f64], dx: f64, dt: f64, alpha: f64) -> Vec<f64> {
    let n = u.len();
    let r = alpha * dt / (2.0 * dx * dx);
    let mut a = vec![0.0; n];
    let mut b = vec![0.0; n];
    let mut c = vec![0.0; n];
    let mut d = vec![0.0; n];
    b[0] = 1.0;
    d[0] = u[0];
    b[n - 1] = 1.0;
    d[n - 1] = u[n - 1];
    for i in 1..n - 1 {
        a[i] = -r;
        b[i] = 1.0 + 2.0 * r;
        c[i] = -r;
        d[i] = r * u[i - 1] + (1.0 - 2.0 * r) * u[i] + r * u[i + 1];
    }
    thomas_solve(&a, &b, &c, &mut d);
    d
}

pub fn diffusion_2d_explicit(
    u: &[Vec<f64>],
    dx: f64,
    dy: f64,
    dt: f64,
    alpha: f64,
) -> Vec<Vec<f64>> {
    let ny = u.len();
    let nx = u[0].len();
    let rx = alpha * dt / (dx * dx);
    let ry = alpha * dt / (dy * dy);
    let mut u_new = u.to_vec();
    for j in 1..ny - 1 {
        for i in 1..nx - 1 {
            u_new[j][i] = u[j][i]
                + rx * (u[j][i + 1] - 2.0 * u[j][i] + u[j][i - 1])
                + ry * (u[j + 1][i] - 2.0 * u[j][i] + u[j - 1][i]);
        }
    }
    u_new
}

pub fn stability_criterion_explicit(dx: f64, alpha: f64) -> f64 {
    dx * dx / (2.0 * alpha)
}

pub fn diffusion_green_function(x: f64, t: f64, alpha: f64) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    (1.0 / (4.0 * std::f64::consts::PI * alpha * t).sqrt()) * (-x * x / (4.0 * alpha * t)).exp()
}

pub fn advection_diffusion_1d(u: &[f64], dx: f64, dt: f64, v: f64, alpha: f64) -> Vec<f64> {
    let n = u.len();
    let mut u_new = u.to_vec();
    let r = alpha * dt / (dx * dx);
    let c = v * dt / (2.0 * dx);
    for i in 1..n - 1 {
        u_new[i] = u[i] + r * (u[i + 1] - 2.0 * u[i] + u[i - 1]) - c * (u[i + 1] - u[i - 1]);
    }
    u_new
}

pub fn diffusion_reaction_1d(
    u: &[f64],
    dx: f64,
    dt: f64,
    alpha: f64,
    reaction_rate: f64,
) -> Vec<f64> {
    let n = u.len();
    let r = alpha * dt / (dx * dx);
    let mut u_new = u.to_vec();
    for i in 1..n - 1 {
        u_new[i] = u[i]
            + r * (u[i + 1] - 2.0 * u[i] + u[i - 1])
            + dt * reaction_rate * u[i] * (1.0 - u[i]);
    }
    u_new
}

pub fn fisher_kpp_step(u: &[f64], dx: f64, dt: f64, d: f64, r: f64) -> Vec<f64> {
    let n = u.len();
    let ratio = d * dt / (dx * dx);
    let mut u_new = u.to_vec();
    for i in 1..n - 1 {
        let diffusion = ratio * (u[i + 1] - 2.0 * u[i] + u[i - 1]);
        let reaction = r * u[i] * (1.0 - u[i]);
        u_new[i] = u[i] + diffusion + dt * reaction;
    }
    u_new
}

pub fn nonlinear_diffusion_1d(u: &[f64], dx: f64, dt: f64, m: f64) -> Vec<f64> {
    let n = u.len();
    let mut u_new = u.to_vec();
    for i in 1..n - 1 {
        let d_right = 0.5 * (u[i].abs().powf(m - 1.0) + u[i + 1].abs().powf(m - 1.0));
        let d_left = 0.5 * (u[i].abs().powf(m - 1.0) + u[i - 1].abs().powf(m - 1.0));
        u_new[i] =
            u[i] + dt / (dx * dx) * (d_right * (u[i + 1] - u[i]) - d_left * (u[i] - u[i - 1]));
    }
    u_new
}

pub fn diffusion_2d_adi(u: &[Vec<f64>], dx: f64, dy: f64, dt: f64, alpha: f64) -> Vec<Vec<f64>> {
    let ny = u.len();
    let nx = u[0].len();
    let rx = alpha * dt / (2.0 * dx * dx);
    let ry = alpha * dt / (2.0 * dy * dy);
    let mut half = u.to_vec();
    for j in 1..ny - 1 {
        let mut a = vec![0.0; nx];
        let mut b = vec![0.0; nx];
        let mut c = vec![0.0; nx];
        let mut d = vec![0.0; nx];
        b[0] = 1.0;
        d[0] = u[j][0];
        b[nx - 1] = 1.0;
        d[nx - 1] = u[j][nx - 1];
        for i in 1..nx - 1 {
            a[i] = -rx;
            b[i] = 1.0 + 2.0 * rx;
            c[i] = -rx;
            d[i] = u[j][i] + ry * (u[j + 1][i] - 2.0 * u[j][i] + u[j - 1][i]);
        }
        thomas_solve(&a, &b, &c, &mut d);
        half[j] = d;
    }
    let mut result = half.clone();
    for i in 1..nx - 1 {
        let mut a = vec![0.0; ny];
        let mut b = vec![0.0; ny];
        let mut c = vec![0.0; ny];
        let mut d = vec![0.0; ny];
        b[0] = 1.0;
        d[0] = half[0][i];
        b[ny - 1] = 1.0;
        d[ny - 1] = half[ny - 1][i];
        for j in 1..ny - 1 {
            a[j] = -ry;
            b[j] = 1.0 + 2.0 * ry;
            c[j] = -ry;
            d[j] = half[j][i] + rx * (half[j][i + 1] - 2.0 * half[j][i] + half[j][i - 1]);
        }
        thomas_solve(&a, &b, &c, &mut d);
        for j in 0..ny {
            result[j][i] = d[j];
        }
    }
    result
}

pub fn stability_criterion_2d(dx: f64, dy: f64, alpha: f64) -> f64 {
    1.0 / (2.0 * alpha * (1.0 / (dx * dx) + 1.0 / (dy * dy)))
}

pub fn peclet_number(velocity: f64, length: f64, diffusivity: f64) -> f64 {
    velocity * length / diffusivity
}

pub fn diffusion_steady_state_1d(n: usize, left_bc: f64, right_bc: f64) -> Vec<f64> {
    (0..n)
        .map(|i| left_bc + (right_bc - left_bc) * i as f64 / (n - 1) as f64)
        .collect()
}

pub fn diffusion_analytical_finite(x: f64, t: f64, length: f64, alpha: f64, n_terms: usize) -> f64 {
    let pi = std::f64::consts::PI;
    let mut sum = 0.0;
    for n in 1..=n_terms {
        let nf = n as f64;
        let coeff = if n % 2 == 0 { 0.0 } else { 4.0 / (nf * pi) };
        sum +=
            coeff * (nf * pi * x / length).sin() * (-(nf * pi / length).powi(2) * alpha * t).exp();
    }
    sum
}

pub fn mass_conservation_check(u: &[f64], dx: f64) -> f64 {
    u.iter().sum::<f64>() * dx
}

pub fn diffusion_coefficient_from_msd(msd: f64, time: f64, dimensions: u32) -> f64 {
    msd / (2.0 * dimensions as f64 * time)
}

fn thomas_solve(a: &[f64], b: &[f64], c: &[f64], d: &mut [f64]) {
    let n = d.len();
    let mut cp = vec![0.0; n];
    let mut dp = d.to_vec();
    cp[0] = c[0] / b[0];
    dp[0] = d[0] / b[0];
    for i in 1..n {
        let m = b[i] - a[i] * cp[i - 1];
        cp[i] = if i < n - 1 { c[i] / m } else { 0.0 };
        dp[i] = (d[i] - a[i] * dp[i - 1]) / m;
    }
    d[n - 1] = dp[n - 1];
    for i in (0..n - 1).rev() {
        d[i] = dp[i] - cp[i] * d[i + 1];
    }
}
