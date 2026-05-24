use super::poly::Polynomial;

pub fn legendre(n: usize) -> Polynomial {
    if n == 0 {
        return Polynomial::one();
    }
    if n == 1 {
        return Polynomial::new(vec![0.0, 1.0]);
    }
    let mut p_prev = Polynomial::one();
    let mut p_curr = Polynomial::new(vec![0.0, 1.0]);
    for k in 2..=n {
        let x = Polynomial::new(vec![0.0, 1.0]);
        let a = (x * p_curr.clone()).scale((2 * k - 1) as f64);
        let b = p_prev.scale((k - 1) as f64);
        let next = (a - b).scale(1.0 / k as f64);
        p_prev = p_curr;
        p_curr = next;
    }
    p_curr
}

pub fn chebyshev_t(n: usize) -> Polynomial {
    if n == 0 {
        return Polynomial::one();
    }
    if n == 1 {
        return Polynomial::new(vec![0.0, 1.0]);
    }
    let mut t_prev = Polynomial::one();
    let mut t_curr = Polynomial::new(vec![0.0, 1.0]);
    let two_x = Polynomial::new(vec![0.0, 2.0]);
    for _ in 2..=n {
        let next = two_x.clone() * t_curr.clone() - t_prev;
        t_prev = t_curr;
        t_curr = next;
    }
    t_curr
}

pub fn hermite(n: usize) -> Polynomial {
    if n == 0 {
        return Polynomial::one();
    }
    if n == 1 {
        return Polynomial::new(vec![0.0, 2.0]);
    }
    let mut h_prev = Polynomial::one();
    let mut h_curr = Polynomial::new(vec![0.0, 2.0]);
    let two_x = Polynomial::new(vec![0.0, 2.0]);
    for k in 2..=n {
        let next = two_x.clone() * h_curr.clone() - h_prev.scale(2.0 * (k - 1) as f64);
        h_prev = h_curr;
        h_curr = next;
    }
    h_curr
}

pub fn laguerre(n: usize) -> Polynomial {
    if n == 0 {
        return Polynomial::one();
    }
    if n == 1 {
        return Polynomial::new(vec![1.0, -1.0]);
    }
    let mut l_prev = Polynomial::one();
    let mut l_curr = Polynomial::new(vec![1.0, -1.0]);
    for k in 2..=n {
        let a = Polynomial::new(vec![(2 * k - 1) as f64, -1.0]) * l_curr.clone();
        let b = l_prev.scale((k - 1) as f64);
        let next = (a - b).scale(1.0 / k as f64);
        l_prev = l_curr;
        l_curr = next;
    }
    l_curr
}

pub fn bernstein_basis(n: usize, k: usize, t: f64) -> f64 {
    let binom = binomial(n, k) as f64;
    binom * t.powi(k as i32) * (1.0 - t).powi((n - k) as i32)
}

fn binomial(n: usize, k: usize) -> u64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut result: u64 = 1;
    for i in 0..k {
        result = result * (n - i) as u64 / (i + 1) as u64;
    }
    result
}

pub fn chebyshev_u(n: usize) -> Polynomial {
    if n == 0 {
        return Polynomial::one();
    }
    if n == 1 {
        return Polynomial::new(vec![0.0, 2.0]);
    }
    let mut u_prev = Polynomial::one();
    let mut u_curr = Polynomial::new(vec![0.0, 2.0]);
    let two_x = Polynomial::new(vec![0.0, 2.0]);
    for _ in 2..=n {
        let next = two_x.clone() * u_curr.clone() - u_prev;
        u_prev = u_curr;
        u_curr = next;
    }
    u_curr
}

pub fn gegenbauer(n: usize, alpha: f64) -> Polynomial {
    if n == 0 {
        return Polynomial::one();
    }
    if n == 1 {
        return Polynomial::new(vec![0.0, 2.0 * alpha]);
    }
    let mut p_prev = Polynomial::one();
    let mut p_curr = Polynomial::new(vec![0.0, 2.0 * alpha]);
    let x = Polynomial::new(vec![0.0, 1.0]);
    for k in 2..=n {
        let kf = k as f64;
        let a = (x.clone() * p_curr.clone()).scale(2.0 * (kf - 1.0 + alpha) / kf);
        let b = p_prev.scale((kf - 2.0 + 2.0 * alpha) / kf);
        let next = a - b;
        p_prev = p_curr;
        p_curr = next;
    }
    p_curr
}

pub fn jacobi(n: usize, alpha: f64, beta: f64) -> Polynomial {
    if n == 0 {
        return Polynomial::one();
    }
    if n == 1 {
        return Polynomial::new(vec![(alpha - beta) / 2.0, (alpha + beta + 2.0) / 2.0]);
    }
    let mut p_prev = Polynomial::one();
    let mut p_curr = Polynomial::new(vec![(alpha - beta) / 2.0, (alpha + beta + 2.0) / 2.0]);
    let x = Polynomial::new(vec![0.0, 1.0]);
    for k in 2..=n {
        let kf = k as f64;
        let ab = alpha + beta;
        let a1 = 2.0 * kf * (kf + ab) * (2.0 * kf + ab - 2.0);
        let a2 = (2.0 * kf + ab - 1.0) * (alpha * alpha - beta * beta);
        let a3 = (2.0 * kf + ab - 2.0) * (2.0 * kf + ab - 1.0) * (2.0 * kf + ab);
        let a4 = 2.0 * (kf - 1.0 + alpha) * (kf - 1.0 + beta) * (2.0 * kf + ab);
        if a1.abs() < 1e-30 {
            break;
        }
        let term1 = Polynomial::new(vec![a2 / a1]) + x.clone().scale(a3 / a1);
        let next = term1 * p_curr.clone() - p_prev.scale(a4 / a1);
        p_prev = p_curr;
        p_curr = next;
    }
    p_curr
}

pub fn associated_laguerre(n: usize, k: usize) -> Polynomial {
    if n == 0 {
        return Polynomial::one();
    }
    if n == 1 {
        return Polynomial::new(vec![1.0 + k as f64, -1.0]);
    }
    let mut l_prev = Polynomial::one();
    let mut l_curr = Polynomial::new(vec![1.0 + k as f64, -1.0]);
    for i in 2..=n {
        let a = Polynomial::new(vec![(2 * i + k - 1) as f64, -1.0]) * l_curr.clone();
        let b = l_prev.scale((i + k - 1) as f64);
        let next = (a - b).scale(1.0 / i as f64);
        l_prev = l_curr;
        l_curr = next;
    }
    l_curr
}

pub fn rising_factorial(x: f64, n: usize) -> f64 {
    let mut result = 1.0;
    for k in 0..n {
        result *= x + k as f64;
    }
    result
}

pub fn falling_factorial(x: f64, n: usize) -> f64 {
    let mut result = 1.0;
    for k in 0..n {
        result *= x - k as f64;
    }
    result
}

pub fn bernoulli_polynomial(n: usize) -> Polynomial {
    let bern = bernoulli_numbers(n);
    let mut coeffs = vec![0.0; n + 1];
    for k in 0..=n {
        coeffs[k] = binomial(n, k) as f64 * bern[n - k];
    }
    Polynomial::new(coeffs)
}

fn bernoulli_numbers(n: usize) -> Vec<f64> {
    let mut b = vec![0.0; n + 1];
    b[0] = 1.0;
    for m in 1..=n {
        let mut sum = 0.0;
        for (k, &bk) in b[..m].iter().enumerate() {
            sum += binomial(m + 1, k) as f64 * bk;
        }
        b[m] = -sum / (m + 1) as f64;
    }
    b
}

pub fn euler_polynomial(n: usize) -> Polynomial {
    let mut coeffs = vec![0.0; n + 1];
    let bern_p1 = bernoulli_polynomial(n + 1);
    let scale = 2.0 / (n + 1) as f64;
    for (k, c) in bern_p1.coeffs.iter().enumerate() {
        if k <= n {
            coeffs[k] = c * scale;
        }
    }
    Polynomial::new(coeffs)
}

pub fn abel_polynomial(n: usize, a: f64) -> Polynomial {
    if n == 0 {
        return Polynomial::one();
    }
    let x = Polynomial::new(vec![0.0, 1.0]);
    let x_minus_na = Polynomial::new(vec![-a * n as f64, 1.0]);
    let mut result = x_minus_na;
    for _ in 1..n - 1 {
        result = result * Polynomial::new(vec![-a * n as f64, 1.0]);
    }
    x * result
}
