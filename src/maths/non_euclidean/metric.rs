use crate::maths::tensor::Tensor;

#[derive(Clone, Debug)]
pub struct MetricSpace {
    pub(crate) dimension: usize,
    pub(crate) metric_fn: MetricType,
}

#[derive(Clone, Debug)]
pub enum MetricType {
    Euclidean,
    Minkowski,
    Schwarzschild {
        rs: f64,
    },
    Hyperbolic {
        curvature: f64,
    },
    Spherical {
        radius: f64,
    },
    Custom {
        components: Vec<f64>,
        dimension: usize,
    },
}

impl MetricSpace {
    pub fn euclidean(dim: usize) -> Self {
        Self {
            dimension: dim,
            metric_fn: MetricType::Euclidean,
        }
    }

    pub fn minkowski() -> Self {
        Self {
            dimension: 4,
            metric_fn: MetricType::Minkowski,
        }
    }

    pub fn schwarzschild(rs: f64) -> Self {
        Self {
            dimension: 4,
            metric_fn: MetricType::Schwarzschild { rs },
        }
    }

    pub fn hyperbolic(curvature: f64) -> Self {
        Self {
            dimension: 2,
            metric_fn: MetricType::Hyperbolic { curvature },
        }
    }

    pub fn spherical(radius: f64) -> Self {
        Self {
            dimension: 2,
            metric_fn: MetricType::Spherical { radius },
        }
    }

    pub fn custom(dimension: usize, components: Vec<f64>) -> Self {
        assert_eq!(components.len(), dimension * dimension);
        Self {
            dimension,
            metric_fn: MetricType::Custom {
                components,
                dimension,
            },
        }
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn metric_tensor_at(&self, point: &[f64]) -> Tensor {
        let n = self.dimension;
        match &self.metric_fn {
            MetricType::Euclidean => Tensor::identity(n),
            MetricType::Minkowski => {
                let mut g = Tensor::zeros(&[4, 4]);
                g.set(&[0, 0], -1.0);
                g.set(&[1, 1], 1.0);
                g.set(&[2, 2], 1.0);
                g.set(&[3, 3], 1.0);
                g
            }
            MetricType::Schwarzschild { rs } => {
                let r = point[1];
                let theta = point[2];
                let factor = 1.0 - rs / r;
                let mut g = Tensor::zeros(&[4, 4]);
                g.set(&[0, 0], -factor);
                g.set(&[1, 1], 1.0 / factor);
                g.set(&[2, 2], r * r);
                g.set(&[3, 3], r * r * theta.sin() * theta.sin());
                g
            }
            MetricType::Hyperbolic { curvature } => {
                let y = point[1];
                let k = curvature * curvature;
                let scale = k / (y * y);
                let mut g = Tensor::zeros(&[2, 2]);
                g.set(&[0, 0], scale);
                g.set(&[1, 1], scale);
                g
            }
            MetricType::Spherical { radius } => {
                let theta = point[0];
                let mut g = Tensor::zeros(&[2, 2]);
                g.set(&[0, 0], radius * radius);
                g.set(&[1, 1], radius * radius * theta.sin() * theta.sin());
                g
            }
            MetricType::Custom {
                components,
                dimension,
            } => Tensor::from_vec(&[*dimension, *dimension], components.clone()),
        }
    }

    pub fn line_element(&self, point: &[f64], displacement: &[f64]) -> f64 {
        let g = self.metric_tensor_at(point);
        let n = self.dimension;
        let mut ds2 = 0.0;
        for i in 0..n {
            for j in 0..n {
                ds2 += g.get(&[i, j]) * displacement[i] * displacement[j];
            }
        }
        ds2
    }

    pub fn christoffel_at(&self, point: &[f64], epsilon: f64) -> Vec<Vec<Vec<f64>>> {
        let n = self.dimension;
        let g = self.metric_tensor_at(point);
        let g_inv = crate::maths::tensor::inverse(&g)
            .unwrap_or_else(|| crate::maths::tensor::Tensor::identity(n));

        let mut dg = vec![vec![vec![0.0; n]; n]; n];
        for k in 0..n {
            let mut p_plus = point.to_vec();
            let mut p_minus = point.to_vec();
            p_plus[k] += epsilon;
            p_minus[k] -= epsilon;
            let g_plus = self.metric_tensor_at(&p_plus);
            let g_minus = self.metric_tensor_at(&p_minus);
            for (i, dg_ki) in dg[k].iter_mut().enumerate() {
                for (j, dg_ki_j) in dg_ki.iter_mut().enumerate() {
                    *dg_ki_j = (g_plus.get(&[i, j]) - g_minus.get(&[i, j])) / (2.0 * epsilon);
                }
            }
        }

        let mut gamma = vec![vec![vec![0.0; n]; n]; n];
        for (sigma, g_sigma) in gamma.iter_mut().enumerate() {
            for (mu, g_sigma_mu) in g_sigma.iter_mut().enumerate() {
                for (nu, val_ref) in g_sigma_mu.iter_mut().enumerate() {
                    let mut val = 0.0;
                    let dg_mu = &dg[mu];
                    let dg_nu = &dg[nu];
                    for (lambda, ((dg_lambda, dg_mu_l), dg_nu_l)) in
                        dg.iter().zip(dg_mu).zip(dg_nu).enumerate()
                    {
                        val += 0.5
                            * g_inv.get(&[sigma, lambda])
                            * (dg_mu_l[nu] + dg_nu_l[mu] - dg_lambda[mu][nu]);
                    }
                    *val_ref = val;
                }
            }
        }
        gamma
    }

    pub fn inner_product(&self, point: &[f64], u: &[f64], v: &[f64]) -> f64 {
        let g = self.metric_tensor_at(point);
        let n = self.dimension;
        let mut result = 0.0;
        for (i, &ui) in u.iter().enumerate().take(n) {
            for (j, &vj) in v.iter().enumerate().take(n) {
                result += g.get(&[i, j]) * ui * vj;
            }
        }
        result
    }

    pub fn vector_norm(&self, point: &[f64], v: &[f64]) -> f64 {
        let ip = self.inner_product(point, v, v);
        ip.abs().sqrt()
    }

    pub fn angle_between(&self, point: &[f64], u: &[f64], v: &[f64]) -> f64 {
        let dot = self.inner_product(point, u, v);
        let nu = self.vector_norm(point, u);
        let nv = self.vector_norm(point, v);
        if nu < 1e-30 || nv < 1e-30 {
            return 0.0;
        }
        (dot / (nu * nv)).clamp(-1.0, 1.0).acos()
    }

    pub fn curve_length(
        &self,
        curve: impl Fn(f64) -> Vec<f64>,
        t0: f64,
        t1: f64,
        steps: usize,
    ) -> f64 {
        let dt = (t1 - t0) / steps as f64;
        let eps = dt * 1e-6;
        let mut length = 0.0;
        for i in 0..steps {
            let t = t0 + (i as f64 + 0.5) * dt;
            let pos = curve(t);
            let pos_plus = curve(t + eps);
            let pos_minus = curve(t - eps);
            let n = self.dimension;
            let tangent: Vec<f64> = (0..n)
                .map(|k| (pos_plus[k] - pos_minus[k]) / (2.0 * eps))
                .collect();
            let ds = self.vector_norm(&pos, &tangent);
            length += ds * dt;
        }
        length
    }

    pub fn geodesic_distance(&self, start: &[f64], end: &[f64], epsilon: f64, steps: usize) -> f64 {
        let n = self.dimension;
        let direction: Vec<f64> = (0..n).map(|i| end[i] - start[i]).collect();
        let curve = |t: f64| -> Vec<f64> { (0..n).map(|i| start[i] + t * direction[i]).collect() };
        let dt = 1.0 / steps as f64;
        let mut length = 0.0;
        for i in 0..steps {
            let t = (i as f64 + 0.5) * dt;
            let pos = curve(t);
            let pos_plus = curve(t + epsilon);
            let pos_minus = curve(t - epsilon);
            let tangent: Vec<f64> = (0..n)
                .map(|k| (pos_plus[k] - pos_minus[k]) / (2.0 * epsilon))
                .collect();
            let ds2 = self.line_element(&pos, &tangent);
            if ds2 > 0.0 {
                length += ds2.sqrt() * dt;
            }
        }
        length
    }

    pub fn raise_index(&self, point: &[f64], covector: &[f64]) -> Vec<f64> {
        let n = self.dimension;
        let g = self.metric_tensor_at(point);
        let g_inv = crate::maths::tensor::inverse(&g)
            .unwrap_or_else(|| crate::maths::tensor::Tensor::identity(n));
        let mut result = vec![0.0; n];
        for (i, ri) in result.iter_mut().enumerate() {
            for (j, &cj) in covector.iter().enumerate() {
                *ri += g_inv.get(&[i, j]) * cj;
            }
        }
        result
    }

    pub fn lower_index(&self, point: &[f64], vector: &[f64]) -> Vec<f64> {
        let n = self.dimension;
        let g = self.metric_tensor_at(point);
        let mut result = vec![0.0; n];
        for (i, ri) in result.iter_mut().enumerate() {
            for (j, &vj) in vector.iter().enumerate() {
                *ri += g.get(&[i, j]) * vj;
            }
        }
        result
    }

    pub fn volume_element(&self, point: &[f64]) -> f64 {
        let g = self.metric_tensor_at(point);
        let det = crate::maths::tensor::determinant(&g);
        det.abs().sqrt()
    }
}
