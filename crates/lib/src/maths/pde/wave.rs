pub fn wave_equation_1d(u: &[f64], u_prev: &[f64], dx: f64, dt: f64, c: f64) -> Vec<f64> {
    let n = u.len();
    let r = c * c * dt * dt / (dx * dx);
    let mut u_new = vec![0.0; n];
    u_new[0] = u[0];
    u_new[n - 1] = u[n - 1];
    for i in 1..n - 1 {
        u_new[i] = 2.0 * u[i] - u_prev[i] + r * (u[i + 1] - 2.0 * u[i] + u[i - 1]);
    }
    u_new
}

pub fn wave_initial_step(u0: &[f64], v0: &[f64], dx: f64, dt: f64, c: f64) -> Vec<f64> {
    let n = u0.len();
    let r = c * c * dt * dt / (dx * dx);
    let mut u1 = vec![0.0; n];
    u1[0] = u0[0];
    u1[n - 1] = u0[n - 1];
    for i in 1..n - 1 {
        u1[i] = u0[i] + dt * v0[i] + 0.5 * r * (u0[i + 1] - 2.0 * u0[i] + u0[i - 1]);
    }
    u1
}

pub fn wave_equation_2d(
    u: &[Vec<f64>],
    u_prev: &[Vec<f64>],
    dx: f64,
    dy: f64,
    dt: f64,
    c: f64,
) -> Vec<Vec<f64>> {
    let ny = u.len();
    let nx = u[0].len();
    let rx = c * c * dt * dt / (dx * dx);
    let ry = c * c * dt * dt / (dy * dy);
    let mut u_new = vec![vec![0.0; nx]; ny];
    for j in 1..ny - 1 {
        for i in 1..nx - 1 {
            u_new[j][i] = 2.0 * u[j][i] - u_prev[j][i]
                + rx * (u[j][i + 1] - 2.0 * u[j][i] + u[j][i - 1])
                + ry * (u[j + 1][i] - 2.0 * u[j][i] + u[j - 1][i]);
        }
    }
    u_new
}

pub fn courant_number(c: f64, dt: f64, dx: f64) -> f64 {
    c * dt / dx
}

pub fn dalembert_solution(x: f64, t: f64, c: f64, f_init: fn(f64) -> f64) -> f64 {
    0.5 * (f_init(x - c * t) + f_init(x + c * t))
}

pub fn wave_energy_density(
    u: &[f64],
    u_prev: &[f64],
    dx: f64,
    dt: f64,
    c: f64,
    rho: f64,
) -> Vec<f64> {
    let n = u.len();
    let mut energy = vec![0.0; n];
    for i in 1..n - 1 {
        let du_dt = (u[i] - u_prev[i]) / dt;
        let du_dx = (u[i + 1] - u[i - 1]) / (2.0 * dx);
        energy[i] = 0.5 * rho * (du_dt * du_dt + c * c * du_dx * du_dx);
    }
    energy
}

pub fn absorbing_boundary(u: &mut [f64], u_prev: &[f64], dx: f64, dt: f64, c: f64) {
    let n = u.len();
    let alpha = c * dt / dx;
    u[0] = u_prev[1] + (alpha - 1.0) / (alpha + 1.0) * (u[1] - u_prev[0]);
    u[n - 1] = u_prev[n - 2] + (alpha - 1.0) / (alpha + 1.0) * (u[n - 2] - u_prev[n - 1]);
}

pub fn string_vibration_mode(x: f64, length: f64, mode: u32) -> f64 {
    (mode as f64 * std::f64::consts::PI * x / length).sin()
}

pub fn wave_equation_1d_implicit(u: &[f64], u_prev: &[f64], dx: f64, dt: f64, c: f64) -> Vec<f64> {
    let n = u.len();
    let r = c * c * dt * dt / (dx * dx);
    let mut a_diag = vec![0.0; n];
    let mut b_diag = vec![0.0; n];
    let mut c_diag = vec![0.0; n];
    let mut d = vec![0.0; n];
    b_diag[0] = 1.0;
    d[0] = u[0];
    b_diag[n - 1] = 1.0;
    d[n - 1] = u[n - 1];
    for i in 1..n - 1 {
        a_diag[i] = -r;
        b_diag[i] = 1.0 + 2.0 * r;
        c_diag[i] = -r;
        d[i] = 2.0 * u[i] - u_prev[i];
    }
    wave_thomas(&a_diag, &b_diag, &c_diag, &mut d);
    d
}

pub fn cfl_check(c: f64, dt: f64, dx: f64) -> bool {
    c * dt / dx <= 1.0
}

pub fn wave_reflection_coefficient(z1: f64, z2: f64) -> f64 {
    (z2 - z1) / (z2 + z1)
}

pub fn wave_transmission_coefficient(z1: f64, z2: f64) -> f64 {
    2.0 * z2 / (z2 + z1)
}

pub fn standing_wave(x: f64, t: f64, k: f64, omega: f64, amplitude: f64) -> f64 {
    2.0 * amplitude * (k * x).sin() * (omega * t).cos()
}

pub fn wave_phase_velocity(omega: f64, k: f64) -> f64 {
    omega / k
}

pub fn wave_group_velocity(domega: f64, dk: f64) -> f64 {
    domega / dk
}

pub fn wave_total_energy(u: &[f64], u_prev: &[f64], dx: f64, dt: f64, c: f64, rho: f64) -> f64 {
    let n = u.len();
    let mut total = 0.0;
    for i in 1..n - 1 {
        let du_dt = (u[i] - u_prev[i]) / dt;
        let du_dx = (u[i + 1] - u[i - 1]) / (2.0 * dx);
        total += 0.5 * rho * (du_dt * du_dt + c * c * du_dx * du_dx) * dx;
    }
    total
}

pub fn spherical_wave_amplitude(r: f64, amplitude_0: f64, r0: f64) -> f64 {
    if r < 1e-30 {
        return 0.0;
    }
    amplitude_0 * r0 / r
}

pub fn wave_packet_gaussian(x: f64, t: f64, k0: f64, sigma: f64, c: f64) -> f64 {
    let xi = x - c * t;
    (-xi * xi / (4.0 * sigma * sigma)).exp() * (k0 * xi).cos()
}

pub fn wave_superposition(
    x: f64,
    t: f64,
    amplitudes: &[f64],
    frequencies: &[f64],
    wave_numbers: &[f64],
) -> f64 {
    let mut val = 0.0;
    for i in 0..amplitudes
        .len()
        .min(frequencies.len())
        .min(wave_numbers.len())
    {
        val += amplitudes[i] * (wave_numbers[i] * x - frequencies[i] * t).sin();
    }
    val
}

pub fn wave_impedance(density: f64, velocity: f64) -> f64 {
    density * velocity
}

fn wave_thomas(a: &[f64], b: &[f64], c: &[f64], d: &mut [f64]) {
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
