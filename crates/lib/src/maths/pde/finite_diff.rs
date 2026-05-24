pub fn first_derivative(f: &[f64], dx: f64) -> Vec<f64> {
    let n = f.len();
    let mut df = vec![0.0; n];
    if n < 2 {
        return df;
    }
    df[0] = (f[1] - f[0]) / dx;
    df[n - 1] = (f[n - 1] - f[n - 2]) / dx;
    for i in 1..n - 1 {
        df[i] = (f[i + 1] - f[i - 1]) / (2.0 * dx);
    }
    df
}

pub fn second_derivative(f: &[f64], dx: f64) -> Vec<f64> {
    let n = f.len();
    let mut d2f = vec![0.0; n];
    if n < 3 {
        return d2f;
    }
    for i in 1..n - 1 {
        d2f[i] = (f[i + 1] - 2.0 * f[i] + f[i - 1]) / (dx * dx);
    }
    d2f
}

pub fn fourth_order_first_derivative(f: &[f64], dx: f64) -> Vec<f64> {
    let n = f.len();
    let mut df = vec![0.0; n];
    if n < 5 {
        return first_derivative(f, dx);
    }
    for i in 2..n - 2 {
        df[i] = (-f[i + 2] + 8.0 * f[i + 1] - 8.0 * f[i - 1] + f[i - 2]) / (12.0 * dx);
    }
    df[0] = (f[1] - f[0]) / dx;
    df[1] = (f[2] - f[0]) / (2.0 * dx);
    df[n - 2] = (f[n - 1] - f[n - 3]) / (2.0 * dx);
    df[n - 1] = (f[n - 1] - f[n - 2]) / dx;
    df
}

pub fn laplacian_2d(u: &[Vec<f64>], dx: f64, dy: f64) -> Vec<Vec<f64>> {
    let ny = u.len();
    let nx = u[0].len();
    let mut lap = vec![vec![0.0; nx]; ny];
    for j in 1..ny - 1 {
        for i in 1..nx - 1 {
            lap[j][i] = (u[j][i + 1] - 2.0 * u[j][i] + u[j][i - 1]) / (dx * dx)
                + (u[j + 1][i] - 2.0 * u[j][i] + u[j - 1][i]) / (dy * dy);
        }
    }
    lap
}

pub fn gradient_2d(u: &[Vec<f64>], dx: f64, dy: f64) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let ny = u.len();
    let nx = u[0].len();
    let mut gx = vec![vec![0.0; nx]; ny];
    let mut gy = vec![vec![0.0; nx]; ny];
    for j in 1..ny - 1 {
        for i in 1..nx - 1 {
            gx[j][i] = (u[j][i + 1] - u[j][i - 1]) / (2.0 * dx);
            gy[j][i] = (u[j + 1][i] - u[j - 1][i]) / (2.0 * dy);
        }
    }
    (gx, gy)
}

pub fn divergence_2d(fx: &[Vec<f64>], fy: &[Vec<f64>], dx: f64, dy: f64) -> Vec<Vec<f64>> {
    let ny = fx.len();
    let nx = fx[0].len();
    let mut div = vec![vec![0.0; nx]; ny];
    for j in 1..ny - 1 {
        for i in 1..nx - 1 {
            div[j][i] = (fx[j][i + 1] - fx[j][i - 1]) / (2.0 * dx)
                + (fy[j + 1][i] - fy[j - 1][i]) / (2.0 * dy);
        }
    }
    div
}

pub fn curl_2d(fx: &[Vec<f64>], fy: &[Vec<f64>], dx: f64, dy: f64) -> Vec<Vec<f64>> {
    let ny = fx.len();
    let nx = fx[0].len();
    let mut curl = vec![vec![0.0; nx]; ny];
    for j in 1..ny - 1 {
        for i in 1..nx - 1 {
            curl[j][i] = (fy[j][i + 1] - fy[j][i - 1]) / (2.0 * dx)
                - (fx[j + 1][i] - fx[j - 1][i]) / (2.0 * dy);
        }
    }
    curl
}

pub fn upwind_advection(u: &[f64], v: f64, dx: f64, dt: f64) -> Vec<f64> {
    let n = u.len();
    let mut u_new = u.to_vec();
    for i in 1..n - 1 {
        if v > 0.0 {
            u_new[i] = u[i] - v * dt / dx * (u[i] - u[i - 1]);
        } else {
            u_new[i] = u[i] - v * dt / dx * (u[i + 1] - u[i]);
        }
    }
    u_new
}

pub fn lax_wendroff(u: &[f64], c: f64, dx: f64, dt: f64) -> Vec<f64> {
    let n = u.len();
    let sigma = c * dt / dx;
    let mut u_new = u.to_vec();
    for i in 1..n - 1 {
        u_new[i] = u[i] - 0.5 * sigma * (u[i + 1] - u[i - 1])
            + 0.5 * sigma * sigma * (u[i + 1] - 2.0 * u[i] + u[i - 1]);
    }
    u_new
}

pub fn third_derivative(f: &[f64], dx: f64) -> Vec<f64> {
    let n = f.len();
    let mut d3f = vec![0.0; n];
    if n < 5 {
        return d3f;
    }
    for i in 2..n - 2 {
        d3f[i] = (f[i + 2] - 2.0 * f[i + 1] + 2.0 * f[i - 1] - f[i - 2]) / (2.0 * dx.powi(3));
    }
    d3f
}

pub fn mixed_partial_xy(u: &[Vec<f64>], dx: f64, dy: f64) -> Vec<Vec<f64>> {
    let ny = u.len();
    let nx = u[0].len();
    let mut d2 = vec![vec![0.0; nx]; ny];
    for j in 1..ny - 1 {
        for i in 1..nx - 1 {
            d2[j][i] = (u[j + 1][i + 1] - u[j + 1][i - 1] - u[j - 1][i + 1] + u[j - 1][i - 1])
                / (4.0 * dx * dy);
        }
    }
    d2
}

pub fn compact_fourth_order(f: &[f64], dx: f64) -> Vec<f64> {
    let n = f.len();
    if n < 3 {
        return vec![0.0; n];
    }
    let mut rhs = vec![0.0; n];
    for i in 1..n - 1 {
        rhs[i] = 3.0 / (2.0 * dx) * (f[i + 1] - f[i - 1]);
    }
    rhs[0] = (f[1] - f[0]) / dx;
    rhs[n - 1] = (f[n - 1] - f[n - 2]) / dx;
    let mut a = vec![0.0; n];
    let mut b = vec![0.0; n];
    let mut c = vec![0.0; n];
    b[0] = 1.0;
    b[n - 1] = 1.0;
    for i in 1..n - 1 {
        a[i] = 1.0 / 4.0;
        b[i] = 1.0;
        c[i] = 1.0 / 4.0;
    }
    fd_thomas(&a, &b, &c, &mut rhs);
    rhs
}

pub fn von_neumann_stability(amplification_factor: f64) -> bool {
    amplification_factor.abs() <= 1.0
}

pub fn boundary_ghost_extrapolate(u: &[f64]) -> Vec<f64> {
    let n = u.len();
    let mut extended = vec![0.0; n + 2];
    extended[1..n + 1].copy_from_slice(u);
    extended[0] = 2.0 * u[0] - u[1];
    extended[n + 1] = 2.0 * u[n - 1] - u[n - 2];
    extended
}

pub fn hessian_2d(u: &[Vec<f64>], dx: f64, dy: f64, i: usize, j: usize) -> [[f64; 2]; 2] {
    let uxx = (u[j][i + 1] - 2.0 * u[j][i] + u[j][i - 1]) / (dx * dx);
    let uyy = (u[j + 1][i] - 2.0 * u[j][i] + u[j - 1][i]) / (dy * dy);
    let uxy =
        (u[j + 1][i + 1] - u[j + 1][i - 1] - u[j - 1][i + 1] + u[j - 1][i - 1]) / (4.0 * dx * dy);
    [[uxx, uxy], [uxy, uyy]]
}

pub fn biharmonic_2d(u: &[Vec<f64>], dx: f64, dy: f64) -> Vec<Vec<f64>> {
    let ny = u.len();
    let nx = u[0].len();
    let mut bh = vec![vec![0.0; nx]; ny];
    let dx4 = dx.powi(4);
    let dy4 = dy.powi(4);
    let dx2dy2 = dx * dx * dy * dy;
    for j in 2..ny - 2 {
        for i in 2..nx - 2 {
            let d4x = (u[j][i + 2] - 4.0 * u[j][i + 1] + 6.0 * u[j][i] - 4.0 * u[j][i - 1]
                + u[j][i - 2])
                / dx4;
            let d4y = (u[j + 2][i] - 4.0 * u[j + 1][i] + 6.0 * u[j][i] - 4.0 * u[j - 1][i]
                + u[j - 2][i])
                / dy4;
            let d2xd2y = (u[j + 1][i + 1] - 2.0 * u[j + 1][i] + u[j + 1][i - 1]
                - 2.0 * u[j][i + 1]
                + 4.0 * u[j][i]
                - 2.0 * u[j][i - 1]
                + u[j - 1][i + 1]
                - 2.0 * u[j - 1][i]
                + u[j - 1][i - 1])
                / dx2dy2;
            bh[j][i] = d4x + 2.0 * d2xd2y + d4y;
        }
    }
    bh
}

pub fn forward_euler_pde(u: &[f64], f_rhs: &dyn Fn(usize, &[f64]) -> f64, dt: f64) -> Vec<f64> {
    let n = u.len();
    let mut u_new = u.to_vec();
    for i in 1..n - 1 {
        u_new[i] = u[i] + dt * f_rhs(i, u);
    }
    u_new
}

pub fn lax_friedrichs(u: &[f64], c: f64, dx: f64, dt: f64) -> Vec<f64> {
    let n = u.len();
    let sigma = c * dt / dx;
    let mut u_new = u.to_vec();
    for i in 1..n - 1 {
        u_new[i] = 0.5 * (u[i + 1] + u[i - 1]) - 0.5 * sigma * (u[i + 1] - u[i - 1]);
    }
    u_new
}

pub fn maccormack(u: &[f64], c: f64, dx: f64, dt: f64) -> Vec<f64> {
    let n = u.len();
    let sigma = c * dt / dx;
    let mut u_star = u.to_vec();
    for i in 0..n - 1 {
        u_star[i] = u[i] - sigma * (u[i + 1] - u[i]);
    }
    let mut u_new = u.to_vec();
    for i in 1..n - 1 {
        u_new[i] = 0.5 * (u[i] + u_star[i] - sigma * (u_star[i] - u_star[i - 1]));
    }
    u_new
}

fn fd_thomas(a: &[f64], b: &[f64], c: &[f64], d: &mut [f64]) {
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
