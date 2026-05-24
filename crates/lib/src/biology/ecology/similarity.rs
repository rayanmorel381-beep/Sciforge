pub fn bray_curtis(a: &[f64], b: &[f64]) -> f64 {
    let mut sum_min = 0.0;
    let mut sum_total = 0.0;
    for i in 0..a.len() {
        sum_min += a[i].min(b[i]);
        sum_total += a[i] + b[i];
    }
    if sum_total <= 0.0 {
        return 0.0;
    }
    1.0 - 2.0 * sum_min / sum_total
}

pub fn jaccard(a: &[bool], b: &[bool]) -> f64 {
    let mut intersection = 0;
    let mut union = 0;
    for i in 0..a.len() {
        if a[i] || b[i] {
            union += 1;
        }
        if a[i] && b[i] {
            intersection += 1;
        }
    }
    if union == 0 {
        return 0.0;
    }
    intersection as f64 / union as f64
}

pub fn sorensen(a: &[bool], b: &[bool]) -> f64 {
    let mut shared = 0;
    let mut sa = 0;
    let mut sb = 0;
    for i in 0..a.len() {
        if a[i] {
            sa += 1;
        }
        if b[i] {
            sb += 1;
        }
        if a[i] && b[i] {
            shared += 1;
        }
    }
    if sa + sb == 0 {
        return 0.0;
    }
    2.0 * shared as f64 / (sa + sb) as f64
}

pub fn morisita_horn(a: &[f64], b: &[f64]) -> f64 {
    let na: f64 = a.iter().sum();
    let nb: f64 = b.iter().sum();
    if na <= 0.0 || nb <= 0.0 {
        return 0.0;
    }
    let sum_ab: f64 = a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum();
    let da: f64 = a.iter().map(|&x| x * x).sum::<f64>() / (na * na);
    let db: f64 = b.iter().map(|&x| x * x).sum::<f64>() / (nb * nb);
    let denom = (da + db) * na * nb;
    if denom < 1e-30 {
        return 0.0;
    }
    2.0 * sum_ab / denom
}

pub fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x - y) * (x - y))
        .sum::<f64>()
        .sqrt()
}

pub fn whittaker_beta(gamma: usize, alpha_mean: f64) -> f64 {
    if alpha_mean < 1e-30 {
        return 0.0;
    }
    gamma as f64 / alpha_mean - 1.0
}

pub fn horn_overlap(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len());
    let sum_a: f64 = a.iter().take(n).sum();
    let sum_b: f64 = b.iter().take(n).sum();
    if sum_a <= 0.0 || sum_b <= 0.0 {
        return 0.0;
    }
    let mut h_ab = 0.0;
    let mut h_a = 0.0;
    let mut h_b = 0.0;
    for i in 0..n {
        let pa = a[i] / sum_a;
        let pb = b[i] / sum_b;
        let pm = (pa + pb) / 2.0;
        if pm > 0.0 {
            h_ab -= pm * pm.ln();
        }
        if pa > 0.0 {
            h_a -= pa * pa.ln();
        }
        if pb > 0.0 {
            h_b -= pb * pb.ln();
        }
    }
    let denom = (h_a + h_b) / 2.0;
    if denom < 1e-30 {
        return 1.0;
    }
    1.0 - (h_ab - denom) / (2.0_f64).ln()
}

pub fn chao_jaccard(shared: usize, a_only: usize, b_only: usize, n_a: usize, n_b: usize) -> f64 {
    let u = shared as f64 / n_a.max(1) as f64;
    let v = shared as f64 / n_b.max(1) as f64;
    let correction_u = u + (a_only as f64 / (2.0 * n_a.max(1) as f64)) * (1.0 - u);
    let correction_v = v + (b_only as f64 / (2.0 * n_b.max(1) as f64)) * (1.0 - v);
    (correction_u * correction_v)
        / (correction_u + correction_v - correction_u * correction_v).max(1e-30)
}

pub fn manhattan_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(&x, &y)| (x - y).abs()).sum()
}

pub fn canberra_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| {
            let denom = x.abs() + y.abs();
            if denom < 1e-30 {
                0.0
            } else {
                (x - y).abs() / denom
            }
        })
        .sum()
}
