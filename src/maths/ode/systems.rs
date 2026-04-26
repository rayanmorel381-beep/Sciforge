pub fn lotka_volterra(
    alpha: f64,
    beta: f64,
    delta: f64,
    gamma: f64,
) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| {
        vec![
            alpha * y[0] - beta * y[0] * y[1],
            delta * y[0] * y[1] - gamma * y[1],
        ]
    }
}

pub fn lorenz(sigma: f64, rho: f64, beta: f64) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| {
        vec![
            sigma * (y[1] - y[0]),
            y[0] * (rho - y[2]) - y[1],
            y[0] * y[1] - beta * y[2],
        ]
    }
}

pub fn van_der_pol(mu: f64) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| vec![y[1], mu * (1.0 - y[0] * y[0]) * y[1] - y[0]]
}

pub fn harmonic_oscillator(omega: f64, damping: f64) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| vec![y[1], -2.0 * damping * omega * y[1] - omega * omega * y[0]]
}

pub fn double_pendulum(
    l1: f64,
    l2: f64,
    m1: f64,
    m2: f64,
    g: f64,
) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| {
        let (th1, w1, th2, w2) = (y[0], y[1], y[2], y[3]);
        let delta = th1 - th2;
        let den1 = (m1 + m2) * l1 - m2 * l1 * delta.cos() * delta.cos();
        let den2 = (l2 / l1) * den1;

        let dw1 = (m2 * l1 * w1 * w1 * delta.sin() * delta.cos()
            + m2 * g * th2.sin() * delta.cos()
            + m2 * l2 * w2 * w2 * delta.sin()
            - (m1 + m2) * g * th1.sin())
            / den1;

        let dw2 = (-m2 * l2 * w2 * w2 * delta.sin() * delta.cos()
            + (m1 + m2) * g * th1.sin() * delta.cos()
            - (m1 + m2) * l1 * w1 * w1 * delta.sin()
            - (m1 + m2) * g * th2.sin())
            / den2;

        vec![w1, dw1, w2, dw2]
    }
}

pub fn sir_model(beta: f64, gamma: f64) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| {
        let (s, i, _) = (y[0], y[1], y[2]);
        vec![-beta * s * i, beta * s * i - gamma * i, gamma * i]
    }
}

pub fn rossler(a: f64, b: f64, c: f64) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| vec![-y[1] - y[2], y[0] + a * y[1], b + y[2] * (y[0] - c)]
}

pub fn three_body_planar(m: [f64; 3], g: f64) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| {
        let mut dy = vec![0.0; 12];
        for i in 0..3 {
            dy[2 * i] = y[6 + 2 * i];
            dy[2 * i + 1] = y[6 + 2 * i + 1];
        }
        for i in 0..3 {
            for j in 0..3 {
                if i == j {
                    continue;
                }
                let dx = y[2 * j] - y[2 * i];
                let dy_val = y[2 * j + 1] - y[2 * i + 1];
                let r = (dx * dx + dy_val * dy_val).sqrt();
                if r < 1e-30 {
                    continue;
                }
                let r3 = r * r * r;
                dy[6 + 2 * i] += g * m[j] * dx / r3;
                dy[6 + 2 * i + 1] += g * m[j] * dy_val / r3;
            }
        }
        dy
    }
}

pub fn brusselator(a: f64, b: f64) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| {
        vec![
            a - (b + 1.0) * y[0] + y[0] * y[0] * y[1],
            b * y[0] - y[0] * y[0] * y[1],
        ]
    }
}

pub fn oregonator(epsilon: f64, q: f64, f_param: f64) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| {
        vec![
            (q * y[1] - y[0] * y[1] + y[0] * (1.0 - y[0])) / epsilon,
            -q * y[1] - y[0] * y[1] + f_param * y[2],
            y[0] - y[2],
        ]
    }
}

pub fn fitzhugh_nagumo(a: f64, b: f64, tau: f64, i_ext: f64) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| {
        vec![
            y[0] - y[0] * y[0] * y[0] / 3.0 - y[1] + i_ext,
            (y[0] + a - b * y[1]) / tau,
        ]
    }
}

pub fn duffing(
    alpha: f64,
    beta: f64,
    gamma: f64,
    delta: f64,
    omega: f64,
) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |t: f64, y: &[f64]| {
        vec![
            y[1],
            -delta * y[1] - alpha * y[0] - beta * y[0].powi(3) + gamma * (omega * t).cos(),
        ]
    }
}

pub fn chen_system(a: f64, b: f64, c: f64) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| {
        vec![
            a * (y[1] - y[0]),
            (c - a) * y[0] - y[0] * y[2] + c * y[1],
            y[0] * y[1] - b * y[2],
        ]
    }
}

pub fn chua_circuit(alpha: f64, beta: f64, m0: f64, m1: f64) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| {
        let h = m1 * y[0] + 0.5 * (m0 - m1) * ((y[0] + 1.0).abs() - (y[0] - 1.0).abs());
        vec![alpha * (y[1] - y[0] - h), y[0] - y[1] + y[2], -beta * y[1]]
    }
}

pub fn predator_prey_holling(
    r: f64,
    k: f64,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| {
        let prey = y[0];
        let pred = y[1];
        let functional_response = a * prey / (1.0 + b * prey);
        vec![
            r * prey * (1.0 - prey / k) - functional_response * pred,
            c * functional_response * pred - d * pred,
        ]
    }
}

pub fn stiff_robertson(k1: f64, k2: f64, k3: f64) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| {
        vec![
            -k1 * y[0] + k3 * y[1] * y[2],
            k1 * y[0] - k2 * y[1] * y[1] - k3 * y[1] * y[2],
            k2 * y[1] * y[1],
        ]
    }
}

pub fn rigid_body(i1: f64, i2: f64, i3: f64) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| {
        vec![
            (i2 - i3) / i1 * y[1] * y[2],
            (i3 - i1) / i2 * y[0] * y[2],
            (i1 - i2) / i3 * y[0] * y[1],
        ]
    }
}

pub fn seir_model(beta: f64, sigma: f64, gamma: f64) -> impl Fn(f64, &[f64]) -> Vec<f64> {
    move |_t: f64, y: &[f64]| {
        let (s, e, i, _) = (y[0], y[1], y[2], y[3]);
        vec![
            -beta * s * i,
            beta * s * i - sigma * e,
            sigma * e - gamma * i,
            gamma * i,
        ]
    }
}
