use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Debug)]
pub struct Polynomial {
    pub coeffs: Vec<f64>,
}

impl Polynomial {
    pub fn new(coeffs: Vec<f64>) -> Self {
        let mut p = Self { coeffs };
        p.trim();
        p
    }

    pub fn zero() -> Self {
        Self { coeffs: vec![0.0] }
    }
    pub fn one() -> Self {
        Self { coeffs: vec![1.0] }
    }
    pub fn monomial(degree: usize, coeff: f64) -> Self {
        let mut c = vec![0.0; degree + 1];
        c[degree] = coeff;
        Self { coeffs: c }
    }

    fn trim(&mut self) {
        while self.coeffs.len() > 1 && self.coeffs.last().is_some_and(|&c| c.abs() < 1e-30) {
            self.coeffs.pop();
        }
    }

    pub fn degree(&self) -> usize {
        self.coeffs.len().saturating_sub(1)
    }

    pub fn eval(&self, x: f64) -> f64 {
        let mut result = 0.0;
        for c in self.coeffs.iter().rev() {
            result = result * x + c;
        }
        result
    }

    pub fn derivative(&self) -> Self {
        if self.coeffs.len() <= 1 {
            return Self::zero();
        }
        Self::new(
            self.coeffs
                .iter()
                .enumerate()
                .skip(1)
                .map(|(i, &c)| c * i as f64)
                .collect(),
        )
    }

    pub fn integral(&self, constant: f64) -> Self {
        let mut coeffs = vec![constant];
        for (i, &c) in self.coeffs.iter().enumerate() {
            coeffs.push(c / (i + 1) as f64);
        }
        Self::new(coeffs)
    }

    pub fn definite_integral(&self, a: f64, b: f64) -> f64 {
        let anti = self.integral(0.0);
        anti.eval(b) - anti.eval(a)
    }

    pub fn scale(&self, s: f64) -> Self {
        Self::new(self.coeffs.iter().map(|c| c * s).collect())
    }

    pub fn compose(&self, other: &Self) -> Self {
        let mut result = Self::zero();
        let mut power = Self::one();
        for &c in &self.coeffs {
            result = result + power.scale(c);
            power = power.clone() * other.clone();
        }
        result
    }

    pub fn div_rem(&self, divisor: &Self) -> (Self, Self) {
        if divisor.coeffs.iter().all(|c| c.abs() < 1e-30) {
            panic!("division by zero polynomial");
        }
        let mut remainder = self.coeffs.clone();
        let d_deg = divisor.degree();
        let lead = *divisor.coeffs.last().unwrap();
        let mut quotient = vec![];

        while remainder.len() > d_deg && !remainder.is_empty() {
            let coeff = *remainder.last().unwrap() / lead;
            quotient.push(coeff);
            for (i, &dc) in divisor.coeffs.iter().enumerate() {
                let idx = remainder.len() - d_deg - 1 + i;
                remainder[idx] -= coeff * dc;
            }
            remainder.pop();
        }
        quotient.reverse();
        if quotient.is_empty() {
            quotient.push(0.0);
        }
        (Self::new(quotient), Self::new(remainder))
    }

    pub fn gcd(&self, other: &Self) -> Self {
        let mut a = self.clone();
        let mut b = other.clone();
        while b.coeffs.iter().any(|c| c.abs() > 1e-30) {
            let (_, r) = a.div_rem(&b);
            a = b;
            b = r;
        }
        let lead = *a.coeffs.last().unwrap_or(&1.0);
        if lead.abs() > 1e-30 {
            a.scale(1.0 / lead)
        } else {
            a
        }
    }

    pub fn from_roots(roots: &[f64]) -> Self {
        let mut result = Self::one();
        for &r in roots {
            result = result * Self::new(vec![-r, 1.0]);
        }
        result
    }
}

impl Add for Polynomial {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let n = self.coeffs.len().max(rhs.coeffs.len());
        let mut coeffs = vec![0.0; n];
        for (i, &c) in self.coeffs.iter().enumerate() {
            coeffs[i] += c;
        }
        for (i, &c) in rhs.coeffs.iter().enumerate() {
            coeffs[i] += c;
        }
        Self::new(coeffs)
    }
}

impl Sub for Polynomial {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let n = self.coeffs.len().max(rhs.coeffs.len());
        let mut coeffs = vec![0.0; n];
        for (i, &c) in self.coeffs.iter().enumerate() {
            coeffs[i] += c;
        }
        for (i, &c) in rhs.coeffs.iter().enumerate() {
            coeffs[i] -= c;
        }
        Self::new(coeffs)
    }
}

impl Mul for Polynomial {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut coeffs = vec![0.0; self.coeffs.len() + rhs.coeffs.len() - 1];
        for (i, &a) in self.coeffs.iter().enumerate() {
            for (j, &b) in rhs.coeffs.iter().enumerate() {
                coeffs[i + j] += a * b;
            }
        }
        Self::new(coeffs)
    }
}

impl Div for Polynomial {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let (q, _) = self.div_rem(&rhs);
        q
    }
}

impl Neg for Polynomial {
    type Output = Self;
    fn neg(self) -> Self {
        self.scale(-1.0)
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut terms = vec![];
        for (i, &c) in self.coeffs.iter().enumerate().rev() {
            if c.abs() < 1e-30 {
                continue;
            }
            match i {
                0 => terms.push(format!("{c}")),
                1 => terms.push(format!("{c}x")),
                _ => terms.push(format!("{c}x^{i}")),
            }
        }
        if terms.is_empty() {
            write!(f, "0")
        } else {
            write!(f, "{}", terms.join(" + "))
        }
    }
}
