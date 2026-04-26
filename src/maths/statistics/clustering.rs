pub fn covariance_matrix(data: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = data.len();
    if n == 0 {
        return Vec::new();
    }
    let d = data[0].len();
    let means: Vec<f64> = (0..d)
        .map(|j| data.iter().map(|row| row[j]).sum::<f64>() / n as f64)
        .collect();
    let mut cov = vec![vec![0.0; d]; d];
    for row in data {
        for i in 0..d {
            for j in 0..d {
                cov[i][j] += (row[i] - means[i]) * (row[j] - means[j]);
            }
        }
    }
    for i in 0..d {
        for j in 0..d {
            cov[i][j] /= (n - 1) as f64;
        }
    }
    cov
}

pub fn pca(data: &[Vec<f64>], n_components: usize, max_iter: usize, tol: f64) -> Vec<Vec<f64>> {
    let n = data.len();
    if n == 0 {
        return Vec::new();
    }
    let d = data[0].len();
    let means: Vec<f64> = (0..d)
        .map(|j| data.iter().map(|row| row[j]).sum::<f64>() / n as f64)
        .collect();
    let centered: Vec<Vec<f64>> = data
        .iter()
        .map(|row| row.iter().zip(means.iter()).map(|(x, m)| x - m).collect())
        .collect();
    let cov = covariance_matrix(&centered);
    let components = n_components.min(d);
    let mut result = Vec::with_capacity(components);
    let mut deflated = cov.clone();
    for _ in 0..components {
        let (_, eigvec) = power_iteration_local(&deflated, max_iter, tol);
        let ev_norm = eigvec.iter().map(|x| x * x).sum::<f64>().sqrt();
        let ev: Vec<f64> = eigvec.iter().map(|x| x / ev_norm.max(1e-30)).collect();
        let eigenval: f64 = (0..d)
            .map(|i| {
                ev[i]
                    * deflated[i]
                        .iter()
                        .zip(ev.iter())
                        .map(|(a, b)| a * b)
                        .sum::<f64>()
            })
            .sum();
        for i in 0..d {
            for j in 0..d {
                deflated[i][j] -= eigenval * ev[i] * ev[j];
            }
        }
        result.push(ev);
    }
    result
}

fn power_iteration_local(a: &[Vec<f64>], max_iter: usize, tol: f64) -> (f64, Vec<f64>) {
    let n = a.len();
    let mut v = vec![1.0 / (n as f64).sqrt(); n];
    let mut eigenvalue = 0.0;
    for _ in 0..max_iter {
        let mut av = vec![0.0; n];
        for i in 0..n {
            for (j, &aij) in a[i].iter().enumerate() {
                av[i] += aij * v[j];
            }
        }
        let new_eigenvalue: f64 = av.iter().zip(v.iter()).map(|(a, b)| a * b).sum();
        let norm = av.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm < 1e-30 {
            break;
        }
        for i in 0..n {
            v[i] = av[i] / norm;
        }
        if (new_eigenvalue - eigenvalue).abs() < tol {
            break;
        }
        eigenvalue = new_eigenvalue;
    }
    (eigenvalue, v)
}

pub fn project_pca(data: &[Vec<f64>], components: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let d = if data.is_empty() { 0 } else { data[0].len() };
    let means: Vec<f64> = (0..d)
        .map(|j| data.iter().map(|row| row[j]).sum::<f64>() / data.len().max(1) as f64)
        .collect();
    data.iter()
        .map(|row| {
            let centered: Vec<f64> = row.iter().zip(means.iter()).map(|(x, m)| x - m).collect();
            components
                .iter()
                .map(|comp| centered.iter().zip(comp.iter()).map(|(x, c)| x * c).sum())
                .collect()
        })
        .collect()
}

pub fn kmeans(
    data: &[Vec<f64>],
    k: usize,
    max_iter: usize,
    seed: u64,
) -> (Vec<usize>, Vec<Vec<f64>>) {
    let n = data.len();
    if n == 0 || k == 0 {
        return (Vec::new(), Vec::new());
    }
    let d = data[0].len();
    let mut rng = seed;
    let mut centroids: Vec<Vec<f64>> = (0..k.min(n))
        .map(|_| {
            rng = rng
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            let idx = ((rng >> 33) as usize) % n;
            data[idx].clone()
        })
        .collect();
    let mut assignments = vec![0usize; n];
    for _ in 0..max_iter {
        let mut changed = false;
        for (i, point) in data.iter().enumerate() {
            let best = centroids
                .iter()
                .enumerate()
                .map(|(ci, c)| {
                    let dist: f64 = point
                        .iter()
                        .zip(c.iter())
                        .map(|(x, cx)| (x - cx).powi(2))
                        .sum();
                    (ci, dist)
                })
                .fold(
                    (0, f64::INFINITY),
                    |(bi, bv), (ci, v)| {
                        if v < bv { (ci, v) } else { (bi, bv) }
                    },
                )
                .0;
            if assignments[i] != best {
                assignments[i] = best;
                changed = true;
            }
        }
        if !changed {
            break;
        }
        let mut sums = vec![vec![0.0; d]; k];
        let mut counts = vec![0usize; k];
        for (i, point) in data.iter().enumerate() {
            let c = assignments[i];
            counts[c] += 1;
            for j in 0..d {
                sums[c][j] += point[j];
            }
        }
        for c in 0..k {
            if counts[c] > 0 {
                for j in 0..d {
                    centroids[c][j] = sums[c][j] / counts[c] as f64;
                }
            }
        }
    }
    (assignments, centroids)
}

pub fn silhouette_score(data: &[Vec<f64>], assignments: &[usize], k: usize) -> f64 {
    let n = data.len();
    if n < 2 {
        return 0.0;
    }
    let mut total = 0.0;
    for i in 0..n {
        let ci = assignments[i];
        let same: Vec<usize> = (0..n).filter(|&j| j != i && assignments[j] == ci).collect();
        let a = if same.is_empty() {
            0.0
        } else {
            same.iter()
                .map(|&j| euclidean_dist(&data[i], &data[j]))
                .sum::<f64>()
                / same.len() as f64
        };
        let b = (0..k)
            .filter(|&c| c != ci)
            .map(|c| {
                let other: Vec<usize> = (0..n).filter(|&j| assignments[j] == c).collect();
                if other.is_empty() {
                    f64::INFINITY
                } else {
                    other
                        .iter()
                        .map(|&j| euclidean_dist(&data[i], &data[j]))
                        .sum::<f64>()
                        / other.len() as f64
                }
            })
            .fold(f64::INFINITY, f64::min);
        let s = (b - a) / a.max(b).max(1e-30);
        total += s;
    }
    total / n as f64
}

fn euclidean_dist(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn dbscan(data: &[Vec<f64>], eps: f64, min_pts: usize) -> Vec<Option<usize>> {
    let n = data.len();
    let mut labels: Vec<Option<usize>> = vec![None; n];
    let mut visited = vec![false; n];
    let mut cluster_id = 0usize;
    for i in 0..n {
        if visited[i] {
            continue;
        }
        visited[i] = true;
        let neighbors = region_query(data, i, eps);
        if neighbors.len() < min_pts {
            continue;
        }
        let mut queue = neighbors.clone();
        labels[i] = Some(cluster_id);
        let mut qi = 0;
        while qi < queue.len() {
            let j = queue[qi];
            qi += 1;
            if !visited[j] {
                visited[j] = true;
                let n2 = region_query(data, j, eps);
                if n2.len() >= min_pts {
                    for &p in &n2 {
                        if !queue.contains(&p) {
                            queue.push(p);
                        }
                    }
                }
            }
            if labels[j].is_none() {
                labels[j] = Some(cluster_id);
            }
        }
        cluster_id += 1;
    }
    labels
}

fn region_query(data: &[Vec<f64>], idx: usize, eps: f64) -> Vec<usize> {
    data.iter()
        .enumerate()
        .filter(|(j, point)| *j != idx && euclidean_dist(&data[idx], point) <= eps)
        .map(|(j, _)| j)
        .collect()
}
