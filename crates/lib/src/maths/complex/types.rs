use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }
    pub fn i() -> Self {
        Self { re: 0.0, im: 1.0 }
    }
    pub fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }
    pub fn one() -> Self {
        Self { re: 1.0, im: 0.0 }
    }

    pub fn conj(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
    pub fn norm_sq(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
    pub fn norm(&self) -> f64 {
        self.norm_sq().sqrt()
    }
    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn inv(&self) -> Self {
        let n = self.norm_sq();
        Self {
            re: self.re / n,
            im: -self.im / n,
        }
    }

    pub fn exp(&self) -> Self {
        let er = self.re.exp();
        Self {
            re: er * self.im.cos(),
            im: er * self.im.sin(),
        }
    }

    pub fn ln(&self) -> Self {
        Self {
            re: self.norm().ln(),
            im: self.arg(),
        }
    }

    pub fn pow(&self, n: Self) -> Self {
        (n * self.ln()).exp()
    }

    pub fn pow_f64(&self, n: f64) -> Self {
        Self::from_polar(self.norm().powf(n), self.arg() * n)
    }

    pub fn sqrt(&self) -> Self {
        self.pow_f64(0.5)
    }

    pub fn sin(&self) -> Self {
        Self {
            re: self.re.sin() * self.im.cosh(),
            im: self.re.cos() * self.im.sinh(),
        }
    }

    pub fn cos(&self) -> Self {
        Self {
            re: self.re.cos() * self.im.cosh(),
            im: -self.re.sin() * self.im.sinh(),
        }
    }

    pub fn tan(&self) -> Self {
        self.sin() / self.cos()
    }

    pub fn sinh(&self) -> Self {
        Self {
            re: self.re.sinh() * self.im.cos(),
            im: self.re.cosh() * self.im.sin(),
        }
    }

    pub fn cosh(&self) -> Self {
        Self {
            re: self.re.cosh() * self.im.cos(),
            im: self.re.sinh() * self.im.sin(),
        }
    }

    pub fn tanh(&self) -> Self {
        self.sinh() / self.cosh()
    }

    pub fn scale(&self, s: f64) -> Self {
        Self {
            re: self.re * s,
            im: self.im * s,
        }
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let d = rhs.re * rhs.re + rhs.im * rhs.im;
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / d,
            im: (self.im * rhs.re - self.re * rhs.im) / d,
        }
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im >= 0.0 {
            write!(f, "{} + {}i", self.re, self.im)
        } else {
            write!(f, "{} - {}i", self.re, -self.im)
        }
    }
}

impl From<f64> for Complex {
    fn from(re: f64) -> Self {
        Self { re, im: 0.0 }
    }
}
