#[derive(Clone, Debug)]
pub struct CubicSpline {
    xs: Vec<f64>,
    ys: Vec<f64>,
    b: Vec<f64>,
    c: Vec<f64>,
    d: Vec<f64>,
}

impl CubicSpline {
    pub fn natural(xs: &[f64], ys: &[f64]) -> Self {
        let n = xs.len() - 1;
        let h: Vec<f64> = (0..n).map(|i| xs[i + 1] - xs[i]).collect();
        let alpha: Vec<f64> = (1..n)
            .map(|i| 3.0 * ((ys[i + 1] - ys[i]) / h[i] - (ys[i] - ys[i - 1]) / h[i - 1]))
            .collect();

        let mut l = vec![1.0; n + 1];
        let mut mu = vec![0.0; n + 1];
        let mut z = vec![0.0; n + 1];

        for i in 1..n {
            l[i] = 2.0 * (xs[i + 1] - xs[i - 1]) - h[i - 1] * mu[i - 1];
            mu[i] = h[i] / l[i];
            z[i] = (alpha[i - 1] - h[i - 1] * z[i - 1]) / l[i];
        }

        let mut c_vec = vec![0.0; n + 1];
        let mut b_vec = vec![0.0; n];
        let mut d_vec = vec![0.0; n];

        for j in (0..n).rev() {
            c_vec[j] = z[j] - mu[j] * c_vec[j + 1];
            b_vec[j] = (ys[j + 1] - ys[j]) / h[j] - h[j] * (c_vec[j + 1] + 2.0 * c_vec[j]) / 3.0;
            d_vec[j] = (c_vec[j + 1] - c_vec[j]) / (3.0 * h[j]);
        }

        Self {
            b: b_vec,
            c: c_vec[..n].to_vec(),
            d: d_vec,
            xs: xs.to_vec(),
            ys: ys.to_vec(),
        }
    }

    pub fn eval(&self, x: f64) -> f64 {
        let n = self.xs.len() - 1;
        let mut i = 0;
        for k in 0..n {
            if x >= self.xs[k] {
                i = k;
            }
        }
        i = i.min(n - 1);
        let dx = x - self.xs[i];
        self.ys[i] + self.b[i] * dx + self.c[i] * dx * dx + self.d[i] * dx * dx * dx
    }

    pub fn integrate(&self, a: f64, b: f64) -> f64 {
        let n = self.xs.len() - 1;
        let clamp_a = a.max(self.xs[0]).min(self.xs[n]);
        let clamp_b = b.max(self.xs[0]).min(self.xs[n]);
        let mut total = 0.0;
        let mut x_lo = clamp_a;
        for k in 0..n {
            if x_lo >= clamp_b {
                break;
            }
            if x_lo >= self.xs[k + 1] {
                continue;
            }
            let x_hi = clamp_b.min(self.xs[k + 1]);
            let lo = x_lo - self.xs[k];
            let hi = x_hi - self.xs[k];
            let ai = self.ys[k];
            let bi = self.b[k];
            let ci = self.c[k];
            let di = self.d[k];
            let prim = |t: f64| {
                ai * t + bi * t * t / 2.0 + ci * t * t * t / 3.0 + di * t * t * t * t / 4.0
            };
            total += prim(hi) - prim(lo);
            x_lo = x_hi;
        }
        total
    }

    pub fn derivative(&self, x: f64) -> f64 {
        let n = self.xs.len() - 1;
        let mut i = 0;
        for k in 0..n {
            if x >= self.xs[k] {
                i = k;
            }
        }
        i = i.min(n - 1);
        let dx = x - self.xs[i];
        self.b[i] + 2.0 * self.c[i] * dx + 3.0 * self.d[i] * dx * dx
    }
}

pub fn linear_interpolate(xs: &[f64], ys: &[f64], x: f64) -> f64 {
    let n = xs.len();
    if x <= xs[0] {
        return ys[0];
    }
    if x >= xs[n - 1] {
        return ys[n - 1];
    }
    let mut i = 0;
    for k in 0..n - 1 {
        if x >= xs[k] && x <= xs[k + 1] {
            i = k;
            break;
        }
    }
    let t = (x - xs[i]) / (xs[i + 1] - xs[i]);
    ys[i] * (1.0 - t) + ys[i + 1] * t
}

pub fn bilinear_interpolate(
    x: f64,
    y: f64,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
    q11: f64,
    q12: f64,
    q21: f64,
    q22: f64,
) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let t = (x - x1) / dx;
    let u = (y - y1) / dy;
    q11 * (1.0 - t) * (1.0 - u) + q21 * t * (1.0 - u) + q12 * (1.0 - t) * u + q22 * t * u
}

pub fn hermite_interpolate(x0: f64, y0: f64, m0: f64, x1: f64, y1: f64, m1: f64, x: f64) -> f64 {
    let t = (x - x0) / (x1 - x0);
    let t2 = t * t;
    let t3 = t2 * t;
    let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
    let h10 = t3 - 2.0 * t2 + t;
    let h01 = -2.0 * t3 + 3.0 * t2;
    let h11 = t3 - t2;
    let dx = x1 - x0;
    h00 * y0 + h10 * dx * m0 + h01 * y1 + h11 * dx * m1
}

pub fn akima_interpolate(xs: &[f64], ys: &[f64], x: f64) -> f64 {
    let n = xs.len();
    if n < 3 {
        return linear_interpolate(xs, ys, x);
    }
    let mut slopes = Vec::with_capacity(n - 1);
    for i in 0..n - 1 {
        slopes.push((ys[i + 1] - ys[i]) / (xs[i + 1] - xs[i]));
    }
    let mut tangents = vec![0.0; n];
    tangents[0] = slopes[0];
    tangents[n - 1] = slopes[n - 2];
    for i in 1..n - 1 {
        if i >= 2 && i < n - 2 {
            let w1 = (slopes[i] - slopes[i - 1]).abs();
            let w2 = (slopes[i - 1] - slopes[i - 2]).abs();
            if (w1 + w2).abs() < 1e-30 {
                tangents[i] = 0.5 * (slopes[i - 1] + slopes[i]);
            } else {
                tangents[i] = (w1 * slopes[i - 1] + w2 * slopes[i]) / (w1 + w2);
            }
        } else {
            tangents[i] = 0.5 * (slopes[i.saturating_sub(1)] + slopes[(i).min(n - 2)]);
        }
    }
    let mut seg = 0;
    for (k, &xk) in xs[..n - 1].iter().enumerate() {
        if x >= xk {
            seg = k;
        }
    }
    seg = seg.min(n - 2);
    hermite_interpolate(
        xs[seg],
        ys[seg],
        tangents[seg],
        xs[seg + 1],
        ys[seg + 1],
        tangents[seg + 1],
        x,
    )
}

pub fn catmull_rom(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    0.5 * ((2.0 * p1)
        + (-p0 + p2) * t
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
}

pub fn monotone_cubic(xs: &[f64], ys: &[f64], x: f64) -> f64 {
    let n = xs.len();
    if n < 2 {
        return ys[0];
    }
    let dk: Vec<f64> = (0..n - 1)
        .map(|i| (ys[i + 1] - ys[i]) / (xs[i + 1] - xs[i]))
        .collect();
    let mut mk = vec![0.0; n];
    mk[0] = dk[0];
    mk[n - 1] = dk[n - 2];
    for i in 1..n - 1 {
        if dk[i - 1] * dk[i] <= 0.0 {
            mk[i] = 0.0;
        } else {
            mk[i] = 0.5 * (dk[i - 1] + dk[i]);
        }
    }
    for i in 0..n - 1 {
        if dk[i].abs() < 1e-30 {
            mk[i] = 0.0;
            mk[i + 1] = 0.0;
        } else {
            let a = mk[i] / dk[i];
            let b = mk[i + 1] / dk[i];
            let s = a * a + b * b;
            if s > 9.0 {
                let tau = 3.0 / s.sqrt();
                mk[i] = tau * a * dk[i];
                mk[i + 1] = tau * b * dk[i];
            }
        }
    }
    let mut seg = 0;
    for (k, &xk) in xs[..n - 1].iter().enumerate() {
        if x >= xk {
            seg = k;
        }
    }
    seg = seg.min(n - 2);
    hermite_interpolate(
        xs[seg],
        ys[seg],
        mk[seg],
        xs[seg + 1],
        ys[seg + 1],
        mk[seg + 1],
        x,
    )
}

pub fn pchip_interpolate(xs: &[f64], ys: &[f64], x: f64) -> f64 {
    monotone_cubic(xs, ys, x)
}

pub fn nearest_neighbor(xs: &[f64], ys: &[f64], x: f64) -> f64 {
    let mut best = 0;
    let mut best_dist = (x - xs[0]).abs();
    for (i, &xi) in xs.iter().enumerate().skip(1) {
        let d = (x - xi).abs();
        if d < best_dist {
            best = i;
            best_dist = d;
        }
    }
    ys[best]
}

pub fn trilinear_interpolate(
    x: f64,
    y: f64,
    z: f64,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    c: &[f64; 8],
) -> f64 {
    let xd = (x - x0) / (x1 - x0);
    let yd = (y - y0) / (y1 - y0);
    let zd = (z - z0) / (z1 - z0);
    let c00 = c[0] * (1.0 - xd) + c[4] * xd;
    let c01 = c[1] * (1.0 - xd) + c[5] * xd;
    let c10 = c[2] * (1.0 - xd) + c[6] * xd;
    let c11 = c[3] * (1.0 - xd) + c[7] * xd;
    let c0 = c00 * (1.0 - yd) + c10 * yd;
    let c1 = c01 * (1.0 - yd) + c11 * yd;
    c0 * (1.0 - zd) + c1 * zd
}

pub fn bezier_cubic(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> f64 {
    let u = 1.0 - t;
    u * u * u * p0 + 3.0 * u * u * t * p1 + 3.0 * u * t * t * p2 + t * t * t * p3
}

pub fn bspline_basis(knots: &[f64], i: usize, degree: usize, t: f64) -> f64 {
    if degree == 0 {
        if t >= knots[i] && t < knots[i + 1] {
            return 1.0;
        }
        return 0.0;
    }
    let mut left = 0.0;
    let denom1 = knots[i + degree] - knots[i];
    if denom1.abs() > 1e-30 {
        left = (t - knots[i]) / denom1 * bspline_basis(knots, i, degree - 1, t);
    }
    let mut right = 0.0;
    let denom2 = knots[i + degree + 1] - knots[i + 1];
    if denom2.abs() > 1e-30 {
        right = (knots[i + degree + 1] - t) / denom2 * bspline_basis(knots, i + 1, degree - 1, t);
    }
    left + right
}
