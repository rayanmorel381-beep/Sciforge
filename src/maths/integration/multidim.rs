pub fn monte_carlo_integrate(
    f: impl Fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    samples: usize,
    seed: u64,
) -> f64 {
    let dim = bounds.len();
    assert!(dim > 0);
    let volume: f64 = bounds.iter().map(|(a, b)| b - a).product();
    let mut rng = LcgRng::new(seed);
    let mut sum = 0.0;

    for _ in 0..samples {
        let point: Vec<f64> = bounds
            .iter()
            .map(|(a, b)| a + rng.next_f64() * (b - a))
            .collect();
        sum += f(&point);
    }
    volume * sum / samples as f64
}

pub fn double_integral(
    f: impl Fn(f64, f64) -> f64,
    x_range: (f64, f64),
    y_range: impl Fn(f64) -> (f64, f64),
    nx: usize,
    ny: usize,
) -> f64 {
    let hx = (x_range.1 - x_range.0) / nx as f64;
    let mut total = 0.0;
    for i in 0..=nx {
        let x = x_range.0 + i as f64 * hx;
        let wx = if i == 0 || i == nx { 0.5 } else { 1.0 };
        let (ya, yb) = y_range(x);
        let hy = (yb - ya) / ny as f64;
        let mut inner = 0.0;
        for j in 0..=ny {
            let y = ya + j as f64 * hy;
            let wy = if j == 0 || j == ny { 0.5 } else { 1.0 };
            inner += wy * f(x, y);
        }
        total += wx * inner * hy;
    }
    total * hx
}

pub fn triple_integral(
    f: impl Fn(f64, f64, f64) -> f64,
    x_range: (f64, f64),
    y_range: impl Fn(f64) -> (f64, f64),
    z_range: impl Fn(f64, f64) -> (f64, f64),
    nx: usize,
    ny: usize,
    nz: usize,
) -> f64 {
    let hx = (x_range.1 - x_range.0) / nx as f64;
    let mut total = 0.0;
    for i in 0..=nx {
        let x = x_range.0 + i as f64 * hx;
        let wx = if i == 0 || i == nx { 0.5 } else { 1.0 };
        let (ya, yb) = y_range(x);
        let hy = (yb - ya) / ny as f64;
        let mut mid = 0.0;
        for j in 0..=ny {
            let y = ya + j as f64 * hy;
            let wy = if j == 0 || j == ny { 0.5 } else { 1.0 };
            let (za, zb) = z_range(x, y);
            let hz = (zb - za) / nz as f64;
            let mut inner = 0.0;
            for k in 0..=nz {
                let z = za + k as f64 * hz;
                let wz = if k == 0 || k == nz { 0.5 } else { 1.0 };
                inner += wz * f(x, y, z);
            }
            mid += wy * inner * hz;
        }
        total += wx * mid * hy;
    }
    total * hx
}

pub fn stratified_monte_carlo(
    f: impl Fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    strata_per_dim: usize,
    samples_per_stratum: usize,
    seed: u64,
) -> f64 {
    let dim = bounds.len();
    let volume: f64 = bounds.iter().map(|(a, b)| b - a).product();
    let total_strata = strata_per_dim.pow(dim as u32);
    let mut rng = LcgRng::new(seed);
    let mut sum = 0.0;
    let mut count = 0usize;

    for stratum in 0..total_strata {
        let mut idx = stratum;
        let point_base: Vec<(f64, f64)> = (0..dim)
            .map(|d| {
                let s = idx % strata_per_dim;
                idx /= strata_per_dim;
                let (a, b) = bounds[d];
                let w = (b - a) / strata_per_dim as f64;
                (a + s as f64 * w, a + (s + 1) as f64 * w)
            })
            .collect();
        for _ in 0..samples_per_stratum {
            let pt: Vec<f64> = point_base
                .iter()
                .map(|(lo, hi)| lo + rng.next_f64() * (hi - lo))
                .collect();
            sum += f(&pt);
            count += 1;
        }
    }
    volume * sum / count as f64
}

pub fn halton_sequence(index: usize, base: usize) -> f64 {
    let mut result = 0.0;
    let mut f = 1.0 / base as f64;
    let mut i = index;
    while i > 0 {
        result += f * (i % base) as f64;
        i /= base;
        f /= base as f64;
    }
    result
}

pub fn quasi_monte_carlo_halton(
    f: impl Fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    samples: usize,
) -> f64 {
    let dim = bounds.len();
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let volume: f64 = bounds.iter().map(|(a, b)| b - a).product();
    let mut sum = 0.0;
    for i in 1..=samples {
        let pt: Vec<f64> = (0..dim)
            .map(|d| {
                let base = if d < primes.len() {
                    primes[d]
                } else {
                    31 + 2 * d
                };
                let h = halton_sequence(i, base);
                bounds[d].0 + h * (bounds[d].1 - bounds[d].0)
            })
            .collect();
        sum += f(&pt);
    }
    volume * sum / samples as f64
}

pub fn polar_integral(
    f: impl Fn(f64, f64) -> f64,
    r_range: (f64, f64),
    theta_range: (f64, f64),
    nr: usize,
    ntheta: usize,
) -> f64 {
    let hr = (r_range.1 - r_range.0) / nr as f64;
    let ht = (theta_range.1 - theta_range.0) / ntheta as f64;
    let mut total = 0.0;
    for i in 0..=nr {
        let r = r_range.0 + i as f64 * hr;
        let wr = if i == 0 || i == nr { 0.5 } else { 1.0 };
        for j in 0..=ntheta {
            let theta = theta_range.0 + j as f64 * ht;
            let wt = if j == 0 || j == ntheta { 0.5 } else { 1.0 };
            total += wr * wt * f(r, theta) * r;
        }
    }
    total * hr * ht
}

pub fn spherical_integral(
    f: impl Fn(f64, f64, f64) -> f64,
    r_range: (f64, f64),
    nr: usize,
    ntheta: usize,
    nphi: usize,
) -> f64 {
    let pi = std::f64::consts::PI;
    let hr = (r_range.1 - r_range.0) / nr as f64;
    let ht = pi / ntheta as f64;
    let hp = 2.0 * pi / nphi as f64;
    let mut total = 0.0;
    for i in 0..=nr {
        let r = r_range.0 + i as f64 * hr;
        let wr = if i == 0 || i == nr { 0.5 } else { 1.0 };
        for j in 0..=ntheta {
            let theta = j as f64 * ht;
            let wt = if j == 0 || j == ntheta { 0.5 } else { 1.0 };
            for k in 0..=nphi {
                let phi = k as f64 * hp;
                let wp = if k == 0 || k == nphi { 0.5 } else { 1.0 };
                total += wr * wt * wp * f(r, theta, phi) * r * r * theta.sin();
            }
        }
    }
    total * hr * ht * hp
}

pub fn cylindrical_integral(
    f: impl Fn(f64, f64, f64) -> f64,
    r_range: (f64, f64),
    theta_range: (f64, f64),
    z_range: (f64, f64),
    nr: usize,
    ntheta: usize,
    nz: usize,
) -> f64 {
    let hr = (r_range.1 - r_range.0) / nr as f64;
    let ht = (theta_range.1 - theta_range.0) / ntheta as f64;
    let hz = (z_range.1 - z_range.0) / nz as f64;
    let mut total = 0.0;
    for i in 0..=nr {
        let r = r_range.0 + i as f64 * hr;
        let wr = if i == 0 || i == nr { 0.5 } else { 1.0 };
        for j in 0..=ntheta {
            let theta = theta_range.0 + j as f64 * ht;
            let wt = if j == 0 || j == ntheta { 0.5 } else { 1.0 };
            for k in 0..=nz {
                let z = z_range.0 + k as f64 * hz;
                let wz = if k == 0 || k == nz { 0.5 } else { 1.0 };
                total += wr * wt * wz * f(r, theta, z) * r;
            }
        }
    }
    total * hr * ht * hz
}

pub fn line_integral(
    f: impl Fn(f64, f64) -> f64,
    x: impl Fn(f64) -> f64,
    y: impl Fn(f64) -> f64,
    dx: impl Fn(f64) -> f64,
    dy: impl Fn(f64) -> f64,
    t_range: (f64, f64),
    n: usize,
) -> f64 {
    let h = (t_range.1 - t_range.0) / n as f64;
    let mut sum = 0.0;
    for i in 0..=n {
        let t = t_range.0 + i as f64 * h;
        let w = if i == 0 || i == n { 0.5 } else { 1.0 };
        let speed = (dx(t) * dx(t) + dy(t) * dy(t)).sqrt();
        sum += w * f(x(t), y(t)) * speed;
    }
    sum * h
}

pub fn surface_integral_parametric(
    f: impl Fn(f64, f64, f64) -> f64,
    x: impl Fn(f64, f64) -> f64,
    y: impl Fn(f64, f64) -> f64,
    z: impl Fn(f64, f64) -> f64,
    u_range: (f64, f64),
    v_range: (f64, f64),
    nu: usize,
    nv: usize,
) -> f64 {
    let hu = (u_range.1 - u_range.0) / nu as f64;
    let hv = (v_range.1 - v_range.0) / nv as f64;
    let eps = 1e-8;
    let mut total = 0.0;
    for i in 0..=nu {
        let u = u_range.0 + i as f64 * hu;
        let wu = if i == 0 || i == nu { 0.5 } else { 1.0 };
        for j in 0..=nv {
            let v = v_range.0 + j as f64 * hv;
            let wv = if j == 0 || j == nv { 0.5 } else { 1.0 };
            let xu = (x(u + eps, v) - x(u - eps, v)) / (2.0 * eps);
            let yu = (y(u + eps, v) - y(u - eps, v)) / (2.0 * eps);
            let zu = (z(u + eps, v) - z(u - eps, v)) / (2.0 * eps);
            let xv = (x(u, v + eps) - x(u, v - eps)) / (2.0 * eps);
            let yv = (y(u, v + eps) - y(u, v - eps)) / (2.0 * eps);
            let zv = (z(u, v + eps) - z(u, v - eps)) / (2.0 * eps);
            let nx = yu * zv - zu * yv;
            let ny = zu * xv - xu * zv;
            let nz = xu * yv - yu * xv;
            let ds = (nx * nx + ny * ny + nz * nz).sqrt();
            total += wu * wv * f(x(u, v), y(u, v), z(u, v)) * ds;
        }
    }
    total * hu * hv
}

pub fn importance_sampling(
    f: impl Fn(f64) -> f64,
    pdf: impl Fn(f64) -> f64,
    sample_gen: impl Fn(&mut LcgRng) -> f64,
    samples: usize,
    seed: u64,
) -> f64 {
    let mut rng = LcgRng::new(seed);
    let mut sum = 0.0;
    for _ in 0..samples {
        let x = sample_gen(&mut rng);
        let p = pdf(x);
        if p.abs() > 1e-30 {
            sum += f(x) / p;
        }
    }
    sum / samples as f64
}

pub struct LcgRng {
    state: u64,
}

impl LcgRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}
