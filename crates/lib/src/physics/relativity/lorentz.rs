use crate::constants::C;

pub fn gamma_factor(v: f64) -> f64 {
    let beta = v / C;
    1.0 / (1.0 - beta * beta).sqrt()
}

pub fn beta(v: f64) -> f64 {
    v / C
}

pub fn time_dilation(proper_time: f64, v: f64) -> f64 {
    proper_time * gamma_factor(v)
}

pub fn length_contraction(proper_length: f64, v: f64) -> f64 {
    proper_length / gamma_factor(v)
}

pub fn lorentz_transform(t: f64, x: f64, v: f64) -> (f64, f64) {
    let g = gamma_factor(v);
    let b = beta(v);
    let t_prime = g * (t - b * x / C);
    let x_prime = g * (x - v * t);
    (t_prime, x_prime)
}

pub fn inverse_lorentz_transform(t_prime: f64, x_prime: f64, v: f64) -> (f64, f64) {
    let g = gamma_factor(v);
    let b = beta(v);
    let t = g * (t_prime + b * x_prime / C);
    let x = g * (x_prime + v * t_prime);
    (t, x)
}

pub fn lorentz_transform_4(event: [f64; 4], v: [f64; 3]) -> [f64; 4] {
    let v_mag = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if v_mag < 1e-30 {
        return event;
    }
    let g = gamma_factor(v_mag);
    let bx = v[0] / C;
    let by = v[1] / C;
    let bz = v[2] / C;
    let b2 = (v_mag / C).powi(2);
    let (t, x, y, z) = (event[0], event[1], event[2], event[3]);
    let bdotr = bx * x + by * y + bz * z;

    let t_prime = g * (t - bdotr / C);
    let factor = (g - 1.0) * bdotr / b2 - g * t * v_mag / C;
    let x_prime = x + bx * factor / (v_mag / C);
    let y_prime = y + by * factor / (v_mag / C);
    let z_prime = z + bz * factor / (v_mag / C);
    [t_prime, x_prime, y_prime, z_prime]
}

pub fn boost_matrix(v: [f64; 3]) -> [[f64; 4]; 4] {
    let v_mag = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if v_mag < 1e-30 {
        return [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
    }
    let g = gamma_factor(v_mag);
    let nx = v[0] / v_mag;
    let ny = v[1] / v_mag;
    let nz = v[2] / v_mag;
    let b = v_mag / C;

    [
        [g, -g * b * nx, -g * b * ny, -g * b * nz],
        [
            -g * b * nx,
            1.0 + (g - 1.0) * nx * nx,
            (g - 1.0) * nx * ny,
            (g - 1.0) * nx * nz,
        ],
        [
            -g * b * ny,
            (g - 1.0) * ny * nx,
            1.0 + (g - 1.0) * ny * ny,
            (g - 1.0) * ny * nz,
        ],
        [
            -g * b * nz,
            (g - 1.0) * nz * nx,
            (g - 1.0) * nz * ny,
            1.0 + (g - 1.0) * nz * nz,
        ],
    ]
}

pub fn rapidity(v: f64) -> f64 {
    (beta(v)).atanh()
}

pub fn velocity_from_rapidity(phi: f64) -> f64 {
    C * phi.tanh()
}
