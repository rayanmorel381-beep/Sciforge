use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Debug)]
pub struct Tensor {
    pub(crate) data: Vec<f64>,
    pub(crate) shape: Vec<usize>,
    pub(crate) strides: Vec<usize>,
}

impl Tensor {
    pub(crate) fn compute_strides(shape: &[usize]) -> Vec<usize> {
        let mut strides = vec![1; shape.len()];
        for i in (0..shape.len().saturating_sub(1)).rev() {
            strides[i] = strides[i + 1] * shape[i + 1];
        }
        strides
    }

    pub(crate) fn flat_index(&self, indices: &[usize]) -> usize {
        indices.iter().zip(&self.strides).map(|(i, s)| i * s).sum()
    }

    pub fn zeros(shape: &[usize]) -> Self {
        let size: usize = shape.iter().product();
        Self {
            data: vec![0.0; size],
            shape: shape.to_vec(),
            strides: Self::compute_strides(shape),
        }
    }

    pub fn ones(shape: &[usize]) -> Self {
        let size: usize = shape.iter().product();
        Self {
            data: vec![1.0; size],
            shape: shape.to_vec(),
            strides: Self::compute_strides(shape),
        }
    }

    pub fn from_vec(shape: &[usize], data: Vec<f64>) -> Self {
        assert_eq!(shape.iter().product::<usize>(), data.len());
        Self {
            data,
            shape: shape.to_vec(),
            strides: Self::compute_strides(shape),
        }
    }

    pub fn from_fn(shape: &[usize], f: impl Fn(&[usize]) -> f64) -> Self {
        let size: usize = shape.iter().product();
        let mut data = Vec::with_capacity(size);
        let mut indices = vec![0usize; shape.len()];
        for _ in 0..size {
            data.push(f(&indices));
            for k in (0..shape.len()).rev() {
                indices[k] += 1;
                if indices[k] < shape[k] {
                    break;
                }
                indices[k] = 0;
            }
        }
        Self {
            data,
            shape: shape.to_vec(),
            strides: Self::compute_strides(shape),
        }
    }

    pub fn scalar(val: f64) -> Self {
        Self {
            data: vec![val],
            shape: vec![],
            strides: vec![],
        }
    }

    pub fn identity(n: usize) -> Self {
        Self::from_fn(&[n, n], |idx| if idx[0] == idx[1] { 1.0 } else { 0.0 })
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }
    pub fn rank(&self) -> usize {
        self.shape.len()
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn data(&self) -> &[f64] {
        &self.data
    }

    pub fn get(&self, indices: &[usize]) -> f64 {
        self.data[self.flat_index(indices)]
    }

    pub fn set(&mut self, indices: &[usize], value: f64) {
        let idx = self.flat_index(indices);
        self.data[idx] = value;
    }

    pub fn scale(&self, s: f64) -> Self {
        Self {
            data: self.data.iter().map(|x| x * s).collect(),
            shape: self.shape.clone(),
            strides: self.strides.clone(),
        }
    }

    pub fn map(&self, f: impl Fn(f64) -> f64) -> Self {
        Self {
            data: self.data.iter().map(|&x| f(x)).collect(),
            shape: self.shape.clone(),
            strides: self.strides.clone(),
        }
    }

    pub fn elementwise(&self, other: &Tensor, f: impl Fn(f64, f64) -> f64) -> Self {
        assert_eq!(self.shape, other.shape);
        Self::from_vec(
            &self.shape,
            self.data
                .iter()
                .zip(&other.data)
                .map(|(&a, &b)| f(a, b))
                .collect(),
        )
    }

    pub fn sum(&self) -> f64 {
        self.data.iter().sum()
    }
    pub fn max(&self) -> f64 {
        self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }
    pub fn min(&self) -> f64 {
        self.data.iter().cloned().fold(f64::INFINITY, f64::min)
    }
    pub fn frobenius_norm(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum::<f64>().sqrt()
    }
}

impl Add for &Tensor {
    type Output = Tensor;
    fn add(self, rhs: Self) -> Tensor {
        self.elementwise(rhs, |a, b| a + b)
    }
}

impl Sub for &Tensor {
    type Output = Tensor;
    fn sub(self, rhs: Self) -> Tensor {
        self.elementwise(rhs, |a, b| a - b)
    }
}

impl Mul<f64> for &Tensor {
    type Output = Tensor;
    fn mul(self, s: f64) -> Tensor {
        self.scale(s)
    }
}

impl Div<f64> for &Tensor {
    type Output = Tensor;
    fn div(self, s: f64) -> Tensor {
        self.scale(1.0 / s)
    }
}

impl Neg for &Tensor {
    type Output = Tensor;
    fn neg(self) -> Tensor {
        self.scale(-1.0)
    }
}
