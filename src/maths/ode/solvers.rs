use crate::constants::{
    ADAMS_BASHFORTH_4, RK45_A2, RK45_A3, RK45_A4, RK45_A5, RK45_A6, RK45_B21, RK45_B31, RK45_B32,
    RK45_B41, RK45_B42, RK45_B43, RK45_B51, RK45_B52, RK45_B53, RK45_B54, RK45_B61, RK45_B62,
    RK45_B63, RK45_B64, RK45_B65, RK45_C1, RK45_C3, RK45_C4, RK45_C5, RK45_C6, RK45_D1, RK45_D3,
    RK45_D4, RK45_D5,
};

pub struct OdeResult {
    pub t: Vec<f64>,
    pub y: Vec<Vec<f64>>,
}

pub fn euler(
    f: impl Fn(f64, &[f64]) -> Vec<f64>,
    t_span: (f64, f64),
    y0: &[f64],
    dt: f64,
) -> OdeResult {
    let mut t = t_span.0;
    let mut y = y0.to_vec();
    let mut ts = vec![t];
    let mut ys = vec![y.clone()];

    while t < t_span.1 - 1e-12 {
        let h = dt.min(t_span.1 - t);
        let dy = f(t, &y);
        for i in 0..y.len() {
            y[i] += h * dy[i];
        }
        t += h;
        ts.push(t);
        ys.push(y.clone());
    }
    OdeResult { t: ts, y: ys }
}

pub fn rk4(
    f: impl Fn(f64, &[f64]) -> Vec<f64>,
    t_span: (f64, f64),
    y0: &[f64],
    dt: f64,
) -> OdeResult {
    let n = y0.len();
    let mut t = t_span.0;
    let mut y = y0.to_vec();
    let mut ts = vec![t];
    let mut ys = vec![y.clone()];

    while t < t_span.1 - 1e-12 {
        let h = dt.min(t_span.1 - t);
        let k1 = f(t, &y);
        let y2: Vec<f64> = (0..n).map(|i| y[i] + 0.5 * h * k1[i]).collect();
        let k2 = f(t + 0.5 * h, &y2);
        let y3: Vec<f64> = (0..n).map(|i| y[i] + 0.5 * h * k2[i]).collect();
        let k3 = f(t + 0.5 * h, &y3);
        let y4: Vec<f64> = (0..n).map(|i| y[i] + h * k3[i]).collect();
        let k4 = f(t + h, &y4);

        for i in 0..n {
            y[i] += h / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }
        t += h;
        ts.push(t);
        ys.push(y.clone());
    }
    OdeResult { t: ts, y: ys }
}

pub fn rk45_adaptive(
    f: impl Fn(f64, &[f64]) -> Vec<f64>,
    t_span: (f64, f64),
    y0: &[f64],
    tol: f64,
    dt_init: f64,
) -> OdeResult {
    let n = y0.len();
    let mut t = t_span.0;
    let mut y = y0.to_vec();
    let mut h = dt_init;
    let mut ts = vec![t];
    let mut ys = vec![y.clone()];

    while t < t_span.1 - 1e-12 {
        h = h.min(t_span.1 - t);
        let k1 = f(t, &y);
        let y2: Vec<f64> = (0..n).map(|i| y[i] + h * RK45_B21 * k1[i]).collect();
        let k2 = f(t + RK45_A2 * h, &y2);
        let y3: Vec<f64> = (0..n)
            .map(|i| y[i] + h * (RK45_B31 * k1[i] + RK45_B32 * k2[i]))
            .collect();
        let k3 = f(t + RK45_A3 * h, &y3);
        let y4: Vec<f64> = (0..n)
            .map(|i| y[i] + h * (RK45_B41 * k1[i] + RK45_B42 * k2[i] + RK45_B43 * k3[i]))
            .collect();
        let k4 = f(t + RK45_A4 * h, &y4);
        let y5: Vec<f64> = (0..n)
            .map(|i| {
                y[i] + h
                    * (RK45_B51 * k1[i] + RK45_B52 * k2[i] + RK45_B53 * k3[i] + RK45_B54 * k4[i])
            })
            .collect();
        let k5 = f(t + RK45_A5 * h, &y5);
        let y6: Vec<f64> = (0..n)
            .map(|i| {
                y[i] + h
                    * (RK45_B61 * k1[i]
                        + RK45_B62 * k2[i]
                        + RK45_B63 * k3[i]
                        + RK45_B64 * k4[i]
                        + RK45_B65 * k5[i])
            })
            .collect();
        let k6 = f(t + RK45_A6 * h, &y6);

        let y5th: Vec<f64> = (0..n)
            .map(|i| {
                y[i] + h
                    * (RK45_C1 * k1[i]
                        + RK45_C3 * k3[i]
                        + RK45_C4 * k4[i]
                        + RK45_C5 * k5[i]
                        + RK45_C6 * k6[i])
            })
            .collect();
        let y4th: Vec<f64> = (0..n)
            .map(|i| {
                y[i] + h * (RK45_D1 * k1[i] + RK45_D3 * k3[i] + RK45_D4 * k4[i] + RK45_D5 * k5[i])
            })
            .collect();

        let err: f64 = (0..n)
            .map(|i| (y5th[i] - y4th[i]).powi(2))
            .sum::<f64>()
            .sqrt();

        if err < tol || h < 1e-15 {
            y = y5th;
            t += h;
            ts.push(t);
            ys.push(y.clone());
            if err > 1e-30 {
                h *= 0.9 * (tol / err).powf(0.2);
            } else {
                h *= 2.0;
            }
        } else {
            h *= 0.9 * (tol / err).powf(0.25);
        }
        h = h.max(1e-15);
    }
    OdeResult { t: ts, y: ys }
}

pub fn implicit_euler(
    f: impl Fn(f64, &[f64]) -> Vec<f64>,
    t_span: (f64, f64),
    y0: &[f64],
    dt: f64,
    newton_iters: usize,
) -> OdeResult {
    let n = y0.len();
    let mut t = t_span.0;
    let mut y = y0.to_vec();
    let mut ts = vec![t];
    let mut ys = vec![y.clone()];

    while t < t_span.1 - 1e-12 {
        let h = dt.min(t_span.1 - t);
        let mut y_new = y.clone();
        let dy = f(t, &y);
        for i in 0..n {
            y_new[i] = y[i] + h * dy[i];
        }

        for _ in 0..newton_iters {
            let f_new = f(t + h, &y_new);
            for i in 0..n {
                y_new[i] = y[i] + h * f_new[i];
            }
        }

        y = y_new;
        t += h;
        ts.push(t);
        ys.push(y.clone());
    }
    OdeResult { t: ts, y: ys }
}

pub fn velocity_verlet(
    accel: impl Fn(f64, &[f64]) -> Vec<f64>,
    t_span: (f64, f64),
    x0: &[f64],
    v0: &[f64],
    dt: f64,
) -> (Vec<f64>, Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let n = x0.len();
    let mut t = t_span.0;
    let mut x = x0.to_vec();
    let mut v = v0.to_vec();
    let mut ts = vec![t];
    let mut xs = vec![x.clone()];
    let mut vs = vec![v.clone()];
    let mut a = accel(t, &x);

    while t < t_span.1 - 1e-12 {
        let h = dt.min(t_span.1 - t);
        for i in 0..n {
            x[i] += v[i] * h + 0.5 * a[i] * h * h;
        }
        t += h;
        let a_new = accel(t, &x);
        for i in 0..n {
            v[i] += 0.5 * (a[i] + a_new[i]) * h;
        }
        a = a_new;
        ts.push(t);
        xs.push(x.clone());
        vs.push(v.clone());
    }
    (ts, xs, vs)
}

pub fn midpoint_method(
    f: impl Fn(f64, &[f64]) -> Vec<f64>,
    t_span: (f64, f64),
    y0: &[f64],
    dt: f64,
) -> OdeResult {
    let n = y0.len();
    let mut t = t_span.0;
    let mut y = y0.to_vec();
    let mut ts = vec![t];
    let mut ys = vec![y.clone()];

    while t < t_span.1 - 1e-12 {
        let h = dt.min(t_span.1 - t);
        let k1 = f(t, &y);
        let ymid: Vec<f64> = (0..n).map(|i| y[i] + 0.5 * h * k1[i]).collect();
        let k2 = f(t + 0.5 * h, &ymid);
        for i in 0..n {
            y[i] += h * k2[i];
        }
        t += h;
        ts.push(t);
        ys.push(y.clone());
    }
    OdeResult { t: ts, y: ys }
}

pub fn heun(
    f: impl Fn(f64, &[f64]) -> Vec<f64>,
    t_span: (f64, f64),
    y0: &[f64],
    dt: f64,
) -> OdeResult {
    let n = y0.len();
    let mut t = t_span.0;
    let mut y = y0.to_vec();
    let mut ts = vec![t];
    let mut ys = vec![y.clone()];

    while t < t_span.1 - 1e-12 {
        let h = dt.min(t_span.1 - t);
        let k1 = f(t, &y);
        let y_pred: Vec<f64> = (0..n).map(|i| y[i] + h * k1[i]).collect();
        let k2 = f(t + h, &y_pred);
        for i in 0..n {
            y[i] += 0.5 * h * (k1[i] + k2[i]);
        }
        t += h;
        ts.push(t);
        ys.push(y.clone());
    }
    OdeResult { t: ts, y: ys }
}

pub fn rk38(
    f: impl Fn(f64, &[f64]) -> Vec<f64>,
    t_span: (f64, f64),
    y0: &[f64],
    dt: f64,
) -> OdeResult {
    let n = y0.len();
    let mut t = t_span.0;
    let mut y = y0.to_vec();
    let mut ts = vec![t];
    let mut ys = vec![y.clone()];

    while t < t_span.1 - 1e-12 {
        let h = dt.min(t_span.1 - t);
        let k1 = f(t, &y);
        let y2: Vec<f64> = (0..n).map(|i| y[i] + h / 3.0 * k1[i]).collect();
        let k2 = f(t + h / 3.0, &y2);
        let y3: Vec<f64> = (0..n).map(|i| y[i] + h * (-k1[i] / 3.0 + k2[i])).collect();
        let k3 = f(t + 2.0 * h / 3.0, &y3);
        let y4: Vec<f64> = (0..n).map(|i| y[i] + h * (k1[i] - k2[i] + k3[i])).collect();
        let k4 = f(t + h, &y4);
        for i in 0..n {
            y[i] += h / 8.0 * (k1[i] + 3.0 * k2[i] + 3.0 * k3[i] + k4[i]);
        }
        t += h;
        ts.push(t);
        ys.push(y.clone());
    }
    OdeResult { t: ts, y: ys }
}

pub fn adams_bashforth_4(
    f: impl Fn(f64, &[f64]) -> Vec<f64>,
    t_span: (f64, f64),
    y0: &[f64],
    dt: f64,
) -> OdeResult {
    let n = y0.len();
    let bootstrap = rk4(&f, (t_span.0, t_span.0 + 3.0 * dt), y0, dt);
    let mut ts = bootstrap.t.clone();
    let mut ys = bootstrap.y.clone();
    let mut fs: Vec<Vec<f64>> = ts.iter().zip(&ys).map(|(&ti, yi)| f(ti, yi)).collect();
    let mut t = *ts.last().unwrap();
    let mut y = ys.last().unwrap().clone();

    while t < t_span.1 - 1e-12 {
        let h = dt.min(t_span.1 - t);
        let m = fs.len();
        let mut y_new = vec![0.0; n];
        for i in 0..n {
            y_new[i] = y[i]
                + h * (ADAMS_BASHFORTH_4[0] * fs[m - 1][i]
                    + ADAMS_BASHFORTH_4[1] * fs[m - 2][i]
                    + ADAMS_BASHFORTH_4[2] * fs[m - 3][i]
                    + ADAMS_BASHFORTH_4[3] * fs[m - 4][i]);
        }
        t += h;
        y = y_new;
        ts.push(t);
        ys.push(y.clone());
        fs.push(f(t, &y));
    }
    OdeResult { t: ts, y: ys }
}

pub fn symplectic_euler(
    dqdt: impl Fn(&[f64], &[f64]) -> Vec<f64>,
    dpdt: impl Fn(&[f64], &[f64]) -> Vec<f64>,
    t_span: (f64, f64),
    q0: &[f64],
    p0: &[f64],
    dt: f64,
) -> (Vec<f64>, Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let n = q0.len();
    let mut t = t_span.0;
    let mut q = q0.to_vec();
    let mut p = p0.to_vec();
    let mut ts = vec![t];
    let mut qs = vec![q.clone()];
    let mut ps = vec![p.clone()];

    while t < t_span.1 - 1e-12 {
        let h = dt.min(t_span.1 - t);
        let dp = dpdt(&q, &p);
        for i in 0..n {
            p[i] += h * dp[i];
        }
        let dq = dqdt(&q, &p);
        for i in 0..n {
            q[i] += h * dq[i];
        }
        t += h;
        ts.push(t);
        qs.push(q.clone());
        ps.push(p.clone());
    }
    (ts, qs, ps)
}

pub fn stiff_bdf2(
    f: impl Fn(f64, &[f64]) -> Vec<f64>,
    t_span: (f64, f64),
    y0: &[f64],
    dt: f64,
    newton_iters: usize,
) -> OdeResult {
    let n = y0.len();
    let first = implicit_euler(&f, (t_span.0, t_span.0 + dt), y0, dt, newton_iters);
    let mut ts = first.t.clone();
    let mut ys = first.y.clone();
    let mut t = *ts.last().unwrap();

    while t < t_span.1 - 1e-12 {
        let h = dt.min(t_span.1 - t);
        let m = ys.len();
        let y_n = &ys[m - 1];
        let y_nm1 = &ys[m - 2];
        let mut y_new: Vec<f64> = (0..n)
            .map(|i| 4.0 / 3.0 * y_n[i] - 1.0 / 3.0 * y_nm1[i])
            .collect();
        for _ in 0..newton_iters {
            let fv = f(t + h, &y_new);
            for i in 0..n {
                y_new[i] = 4.0 / 3.0 * y_n[i] - 1.0 / 3.0 * y_nm1[i] + 2.0 / 3.0 * h * fv[i];
            }
        }
        t += h;
        ts.push(t);
        ys.push(y_new);
    }
    OdeResult { t: ts, y: ys }
}
