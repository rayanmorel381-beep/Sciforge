use super::descriptive::{mean, sample_variance};
use super::distributions::{chi_squared_pdf, normal_cdf, student_t_pdf};

pub fn z_test(sample_mean: f64, pop_mean: f64, pop_std: f64, n: usize) -> (f64, f64) {
    let z = (sample_mean - pop_mean) / (pop_std / (n as f64).sqrt());
    let p = 2.0 * (1.0 - normal_cdf(z.abs(), 0.0, 1.0));
    (z, p)
}

pub fn t_test_one_sample(data: &[f64], mu0: f64) -> (f64, f64) {
    let n = data.len() as f64;
    let m = mean(data);
    let s = sample_variance(data).sqrt();
    let t = (m - mu0) / (s / n.sqrt());
    let nu = n - 1.0;
    let p = 2.0 * t_cdf_approx(-t.abs(), nu);
    (t, p)
}

pub fn t_test_two_sample(a: &[f64], b: &[f64]) -> (f64, f64) {
    let na = a.len() as f64;
    let nb = b.len() as f64;
    let ma = mean(a);
    let mb = mean(b);
    let va = sample_variance(a);
    let vb = sample_variance(b);
    let se = (va / na + vb / nb).sqrt();
    let t = (ma - mb) / se;
    let nu_num = (va / na + vb / nb).powi(2);
    let nu_den = (va / na).powi(2) / (na - 1.0) + (vb / nb).powi(2) / (nb - 1.0);
    let nu = nu_num / nu_den;
    let p = 2.0 * t_cdf_approx(-t.abs(), nu);
    (t, p)
}

pub fn chi_squared_test(observed: &[f64], expected: &[f64]) -> (f64, f64) {
    let chi2: f64 = observed
        .iter()
        .zip(expected)
        .map(|(o, e)| (o - e).powi(2) / e)
        .sum();
    let df = observed.len() - 1;
    let p = 1.0 - chi2_cdf_approx(chi2, df as f64);
    (chi2, p)
}

pub fn anova_one_way(groups: &[&[f64]]) -> (f64, f64) {
    let k = groups.len() as f64;
    let all: Vec<f64> = groups.iter().flat_map(|g| g.iter().cloned()).collect();
    let n = all.len() as f64;
    let grand_mean = mean(&all);

    let ss_between: f64 = groups
        .iter()
        .map(|g| g.len() as f64 * (mean(g) - grand_mean).powi(2))
        .sum();
    let ss_within: f64 = groups
        .iter()
        .map(|g| {
            let gm = mean(g);
            g.iter().map(|x| (x - gm).powi(2)).sum::<f64>()
        })
        .sum();

    let df_between = k - 1.0;
    let df_within = n - k;
    let ms_between = ss_between / df_between;
    let ms_within = ss_within / df_within;
    let f_stat = ms_between / ms_within;
    (f_stat, df_between)
}

pub fn mann_whitney_u(a: &[f64], b: &[f64]) -> (f64, f64) {
    let na = a.len() as f64;
    let nb = b.len() as f64;
    let mut u = 0.0;
    for &ai in a {
        for &bi in b {
            if ai < bi {
                u += 1.0;
            } else if (ai - bi).abs() < 1e-30 {
                u += 0.5;
            }
        }
    }
    let mu = na * nb / 2.0;
    let sigma = (na * nb * (na + nb + 1.0) / 12.0).sqrt();
    let z = (u - mu) / sigma;
    (u, z)
}

fn t_cdf_approx(t: f64, nu: f64) -> f64 {
    let n_steps = 4000;
    let lower = -40.0_f64;
    let h = (t - lower) / n_steps as f64;
    let mut sum = student_t_pdf(lower, nu) + student_t_pdf(t, nu);
    for i in 1..n_steps {
        let x = lower + i as f64 * h;
        let w = if i % 2 == 0 { 2.0 } else { 4.0 };
        sum += w * student_t_pdf(x, nu);
    }
    sum * h / 3.0
}

fn chi2_cdf_approx(x: f64, k: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    let ku = k as u32;
    let n_steps = 4000;
    let eps = 1e-10;
    let h = (x - eps) / n_steps as f64;
    let mut sum = chi_squared_pdf(eps, ku) + chi_squared_pdf(x, ku);
    for i in 1..n_steps {
        let xi = eps + i as f64 * h;
        let w = if i % 2 == 0 { 2.0 } else { 4.0 };
        sum += w * chi_squared_pdf(xi, ku);
    }
    sum * h / 3.0
}

pub fn regularized_incomplete_beta(a: f64, b: f64, x: f64) -> f64 {
    if !(0.0..=1.0).contains(&x) {
        return 0.0;
    }
    if x == 0.0 {
        return 0.0;
    }
    if x == 1.0 {
        return 1.0;
    }
    let bt = (super::distributions::ln_gamma(a + b)
        - super::distributions::ln_gamma(a)
        - super::distributions::ln_gamma(b)
        + a * x.ln()
        + b * (1.0 - x).ln())
    .exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        bt * cf_beta(a, b, x) / a
    } else {
        1.0 - bt * cf_beta(b, a, 1.0 - x) / b
    }
}

pub fn cf_beta(a: f64, b: f64, x: f64) -> f64 {
    let max_iter = 200;
    let eps = 1e-14;
    let mut c = 1.0;
    let mut d = 1.0 - (a + b) * x / (a + 1.0);
    if d.abs() < 1e-30 {
        d = 1e-30;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..=max_iter {
        let m = m as f64;
        let num1 = m * (b - m) * x / ((a + 2.0 * m - 1.0) * (a + 2.0 * m));
        d = 1.0 + num1 * d;
        if d.abs() < 1e-30 {
            d = 1e-30;
        }
        c = 1.0 + num1 / c;
        if c.abs() < 1e-30 {
            c = 1e-30;
        }
        d = 1.0 / d;
        h *= d * c;
        let num2 = -(a + m) * (a + b + m) * x / ((a + 2.0 * m) * (a + 2.0 * m + 1.0));
        d = 1.0 + num2 * d;
        if d.abs() < 1e-30 {
            d = 1e-30;
        }
        c = 1.0 + num2 / c;
        if c.abs() < 1e-30 {
            c = 1e-30;
        }
        d = 1.0 / d;
        let delta = d * c;
        h *= delta;
        if (delta - 1.0).abs() < eps {
            break;
        }
    }
    h
}

pub fn regularized_gamma_lower(a: f64, x: f64) -> f64 {
    if x < 0.0 {
        return 0.0;
    }
    let mut sum = 0.0;
    let mut term = 1.0 / a;
    sum += term;
    for n in 1..200 {
        term *= x / (a + n as f64);
        sum += term;
        if term.abs() < 1e-15 {
            break;
        }
    }
    sum * x.powf(a) * (-x).exp() / super::distributions::gamma(a)
}

pub fn paired_t_test(a: &[f64], b: &[f64]) -> (f64, f64) {
    let diffs: Vec<f64> = a.iter().zip(b).map(|(x, y)| x - y).collect();
    t_test_one_sample(&diffs, 0.0)
}

pub fn welch_t_test(a: &[f64], b: &[f64]) -> (f64, f64) {
    t_test_two_sample(a, b)
}

pub fn kruskal_wallis(groups: &[&[f64]]) -> f64 {
    let mut all: Vec<(f64, usize)> = Vec::new();
    for (g, group) in groups.iter().enumerate() {
        for &val in *group {
            all.push((val, g));
        }
    }
    all.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let n = all.len() as f64;
    let mut ranks = vec![0.0; all.len()];
    let mut i = 0;
    while i < all.len() {
        let mut j = i;
        while j < all.len() && (all[j].0 - all[i].0).abs() < 1e-30 {
            j += 1;
        }
        let avg_rank = (i + 1 + j) as f64 / 2.0;
        ranks[i..j].fill(avg_rank);
        i = j;
    }
    let k = groups.len() as f64;
    if k < 2.0 {
        return 0.0;
    }
    let mut h = 0.0;
    let mut idx = 0;
    for group in groups {
        let ni = group.len() as f64;
        let rank_sum: f64 = (0..group.len())
            .map(|_| {
                let r = ranks[idx];
                idx += 1;
                r
            })
            .sum();
        h += rank_sum * rank_sum / ni;
    }
    (12.0 / (n * (n + 1.0))) * h - 3.0 * (n + 1.0)
}

pub fn kolmogorov_smirnov_statistic(data: &[f64], cdf: impl Fn(f64) -> f64) -> f64 {
    let n = data.len() as f64;
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut d = 0.0_f64;
    for (i, &x) in sorted.iter().enumerate() {
        let fn_val = (i + 1) as f64 / n;
        let fn_prev = i as f64 / n;
        let fx = cdf(x);
        d = d.max((fn_val - fx).abs());
        d = d.max((fn_prev - fx).abs());
    }
    d
}

pub fn levene_test(groups: &[&[f64]]) -> f64 {
    let k = groups.len();
    let medians: Vec<f64> = groups
        .iter()
        .map(|g| {
            let mut s = g.to_vec();
            s.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mid = s.len() / 2;
            if s.len() % 2 == 0 {
                (s[mid - 1] + s[mid]) / 2.0
            } else {
                s[mid]
            }
        })
        .collect();

    let z_groups: Vec<Vec<f64>> = groups
        .iter()
        .zip(&medians)
        .map(|(g, med)| g.iter().map(|x| (x - med).abs()).collect())
        .collect();

    let all_z: Vec<f64> = z_groups.iter().flat_map(|g| g.iter().cloned()).collect();
    let n = all_z.len() as f64;
    let grand_mean = mean(&all_z);

    let ss_between: f64 = z_groups
        .iter()
        .map(|g| g.len() as f64 * (mean(g) - grand_mean).powi(2))
        .sum();
    let ss_within: f64 = z_groups
        .iter()
        .map(|g| {
            let gm = mean(g);
            g.iter().map(|x| (x - gm).powi(2)).sum::<f64>()
        })
        .sum();

    let df1 = (k - 1) as f64;
    let df2 = n - k as f64;
    (ss_between / df1) / (ss_within / df2)
}

pub fn fisher_exact_2x2(a: u64, b: u64, c: u64, d: u64) -> f64 {
    let n = a + b + c + d;
    let r1 = a + b;
    let r2 = c + d;
    let c1 = a + c;
    let c2 = b + d;
    let log_p = ln_binom(r1, a) + ln_binom(r2, c) - ln_binom(n, c1);
    let _ = c2;
    log_p.exp()
}

fn ln_binom(n: u64, k: u64) -> f64 {
    if k > n {
        return f64::NEG_INFINITY;
    }
    ln_factorial(n) - ln_factorial(k) - ln_factorial(n - k)
}

fn ln_factorial(n: u64) -> f64 {
    if n <= 1 {
        return 0.0;
    }
    (2..=n).map(|i| (i as f64).ln()).sum()
}

pub fn spearman_rank_correlation(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len() as f64;
    let rx = ranks(x);
    let ry = ranks(y);
    let d_sq: f64 = rx.iter().zip(&ry).map(|(a, b)| (a - b).powi(2)).sum();
    1.0 - 6.0 * d_sq / (n * (n * n - 1.0))
}

fn ranks(data: &[f64]) -> Vec<f64> {
    let n = data.len();
    let mut indexed: Vec<(usize, f64)> = data.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut result = vec![0.0; n];
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j < n && (indexed[j].1 - indexed[i].1).abs() < 1e-30 {
            j += 1;
        }
        let avg = (i + 1 + j) as f64 / 2.0;
        for k in i..j {
            result[indexed[k].0] = avg;
        }
        i = j;
    }
    result
}

pub fn kendall_tau(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len();
    let mut concordant = 0i64;
    let mut discordant = 0i64;
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = x[j] - x[i];
            let dy = y[j] - y[i];
            let product = dx * dy;
            if product > 0.0 {
                concordant += 1;
            } else if product < 0.0 {
                discordant += 1;
            }
        }
    }
    let denom = (n * (n - 1) / 2) as f64;
    (concordant - discordant) as f64 / denom
}
