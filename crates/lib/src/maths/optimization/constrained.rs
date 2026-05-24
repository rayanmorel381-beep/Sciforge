pub fn penalty_method(
    f: fn(&[f64]) -> f64,
    constraints: &[fn(&[f64]) -> f64],
    x: &[f64],
    penalty: f64,
) -> f64 {
    let mut cost = f(x);
    for c in constraints {
        let violation = c(x).max(0.0);
        cost += penalty * violation * violation;
    }
    cost
}

pub fn augmented_lagrangian(
    f: fn(&[f64]) -> f64,
    constraints: &[fn(&[f64]) -> f64],
    x: &[f64],
    lambdas: &[f64],
    mu: f64,
) -> f64 {
    let mut cost = f(x);
    for (i, c) in constraints.iter().enumerate() {
        let ci = c(x);
        let li = if i < lambdas.len() { lambdas[i] } else { 0.0 };
        cost += li * ci + mu / 2.0 * ci * ci;
    }
    cost
}

pub fn barrier_method(
    f: fn(&[f64]) -> f64,
    inequalities: &[fn(&[f64]) -> f64],
    x: &[f64],
    t: f64,
) -> f64 {
    let mut cost = t * f(x);
    for g in inequalities {
        let gi = g(x);
        if gi >= 0.0 {
            return f64::INFINITY;
        }
        cost -= (-gi).ln();
    }
    cost
}

pub fn project_box(x: &[f64], lower: &[f64], upper: &[f64]) -> Vec<f64> {
    x.iter()
        .zip(lower.iter().zip(upper.iter()))
        .map(|(&xi, (&lo, &hi))| xi.clamp(lo, hi))
        .collect()
}

pub fn project_simplex(x: &[f64]) -> Vec<f64> {
    let mut sorted: Vec<f64> = x.to_vec();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let mut cumsum = 0.0;
    let mut rho = 0;
    for (i, &si) in sorted.iter().enumerate() {
        cumsum += si;
        if si - (cumsum - 1.0) / (i + 1) as f64 > 0.0 {
            rho = i + 1;
        }
    }
    let theta = (sorted[..rho].iter().sum::<f64>() - 1.0) / rho as f64;
    x.iter().map(|&xi| (xi - theta).max(0.0)).collect()
}

pub fn kkt_violation(
    grad_f: &[f64],
    constraints: &[f64],
    lambdas: &[f64],
    grad_constraints: &[Vec<f64>],
) -> f64 {
    let n = grad_f.len();
    let mut stationarity = vec![0.0; n];
    for (i, si) in stationarity.iter_mut().enumerate() {
        *si = grad_f[i];
        for (j, gc) in grad_constraints.iter().enumerate() {
            if j < lambdas.len() {
                *si += lambdas[j] * gc[i];
            }
        }
    }
    let stat_norm: f64 = stationarity.iter().map(|x| x * x).sum::<f64>().sqrt();
    let feas: f64 = constraints
        .iter()
        .map(|&c| c.max(0.0).powi(2))
        .sum::<f64>()
        .sqrt();
    let comp: f64 = lambdas
        .iter()
        .zip(constraints.iter())
        .map(|(&l, &c)| (l * c).abs())
        .sum::<f64>();
    stat_norm + feas + comp
}

pub fn lagrangian(f: f64, constraints: &[f64], lambdas: &[f64]) -> f64 {
    let mut l = f;
    for (i, &c) in constraints.iter().enumerate() {
        if i < lambdas.len() {
            l += lambdas[i] * c;
        }
    }
    l
}

pub fn projected_gradient_step(
    grad: &[f64],
    x: &[f64],
    lr: f64,
    lower: &[f64],
    upper: &[f64],
) -> Vec<f64> {
    x.iter()
        .enumerate()
        .map(|(i, &xi)| (xi - lr * grad[i]).clamp(lower[i], upper[i]))
        .collect()
}

pub fn frank_wolfe_step(grad: &[f64], lower: &[f64], upper: &[f64]) -> Vec<f64> {
    grad.iter()
        .enumerate()
        .map(|(i, &gi)| if gi > 0.0 { lower[i] } else { upper[i] })
        .collect()
}

pub fn admm_x_update(a: &[Vec<f64>], b: &[f64], z: &[f64], u: &[f64], rho: f64) -> Vec<f64> {
    let n = b.len();
    let mut x = vec![0.0; n];
    for (i, xi) in x.iter_mut().enumerate() {
        *xi = (b[i] + rho * (z[i] - u[i])) / (a[i][i] + rho);
    }
    x
}

pub fn dual_ascent_step(lambdas: &[f64], constraints: &[f64], step_size: f64) -> Vec<f64> {
    lambdas
        .iter()
        .zip(constraints.iter())
        .map(|(&l, &c)| (l + step_size * c).max(0.0))
        .collect()
}

pub fn feasibility_check(x: &[f64], constraints: &[fn(&[f64]) -> f64], tol: f64) -> bool {
    constraints.iter().all(|c| c(x) <= tol)
}

pub fn quadratic_objective(h: &[Vec<f64>], c: &[f64], x: &[f64]) -> f64 {
    let mut val = 0.0;
    for (i, &xi) in x.iter().enumerate() {
        val += c[i] * xi;
        for (j, &xj) in x.iter().enumerate() {
            val += 0.5 * h[i][j] * xi * xj;
        }
    }
    val
}

pub fn linear_constraint_violation(a: &[Vec<f64>], b: &[f64], x: &[f64]) -> f64 {
    let n = x.len();
    let mut max_viol = 0.0f64;
    for (i, ai) in a.iter().enumerate() {
        let mut dot = 0.0;
        for j in 0..n {
            dot += ai[j] * x[j];
        }
        let viol = (dot - b[i]).max(0.0);
        if viol > max_viol {
            max_viol = viol;
        }
    }
    max_viol
}

pub fn l1_penalty(
    f: fn(&[f64]) -> f64,
    constraints: &[fn(&[f64]) -> f64],
    x: &[f64],
    penalty: f64,
) -> f64 {
    let mut cost = f(x);
    for c in constraints {
        cost += penalty * c(x).max(0.0);
    }
    cost
}

pub fn equality_penalty(
    f: fn(&[f64]) -> f64,
    eq_constraints: &[fn(&[f64]) -> f64],
    x: &[f64],
    penalty: f64,
) -> f64 {
    let mut cost = f(x);
    for c in eq_constraints {
        let v = c(x);
        cost += penalty * v * v;
    }
    cost
}

pub fn merit_function(f: f64, constraints: &[f64], mu: f64) -> f64 {
    let violation: f64 = constraints.iter().map(|&c| c.max(0.0)).sum();
    f + mu * violation
}
