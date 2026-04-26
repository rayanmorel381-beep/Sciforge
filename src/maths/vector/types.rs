use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn magnitude(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        if m < 1e-15 {
            return Self::zero();
        }
        self.scale(1.0 / m)
    }

    pub fn scale(&self, s: f64) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl Add for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, s: f64) -> Vec3 {
        self.scale(s)
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, s: f64) -> Vec3 {
        self.scale(1.0 / s)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.6}, {:.6}, {:.6})", self.x, self.y, self.z)
    }
}

#[derive(Clone, Debug)]
pub struct VecN {
    pub components: Vec<f64>,
}

impl VecN {
    pub fn new(components: Vec<f64>) -> Self {
        Self { components }
    }
    pub fn zeros(n: usize) -> Self {
        Self {
            components: vec![0.0; n],
        }
    }
    pub fn dim(&self) -> usize {
        self.components.len()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        assert_eq!(self.dim(), rhs.dim());
        self.components
            .iter()
            .zip(&rhs.components)
            .map(|(a, b)| a * b)
            .sum()
    }

    pub fn magnitude(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        if m < 1e-15 {
            return Self::zeros(self.dim());
        }
        self.scale(1.0 / m)
    }

    pub fn scale(&self, s: f64) -> Self {
        Self {
            components: self.components.iter().map(|x| x * s).collect(),
        }
    }

    pub fn add_vec(&self, rhs: &Self) -> Self {
        assert_eq!(self.dim(), rhs.dim());
        Self {
            components: self
                .components
                .iter()
                .zip(&rhs.components)
                .map(|(a, b)| a + b)
                .collect(),
        }
    }

    pub fn sub_vec(&self, rhs: &Self) -> Self {
        assert_eq!(self.dim(), rhs.dim());
        Self {
            components: self
                .components
                .iter()
                .zip(&rhs.components)
                .map(|(a, b)| a - b)
                .collect(),
        }
    }
}

impl fmt::Display for VecN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for (i, c) in self.components.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:.6}", c)?;
        }
        write!(f, ")")
    }
}

impl Add for &VecN {
    type Output = VecN;
    fn add(self, rhs: Self) -> VecN {
        self.add_vec(rhs)
    }
}

impl Sub for &VecN {
    type Output = VecN;
    fn sub(self, rhs: Self) -> VecN {
        self.sub_vec(rhs)
    }
}

impl Mul<f64> for &VecN {
    type Output = VecN;
    fn mul(self, s: f64) -> VecN {
        self.scale(s)
    }
}

impl Div<f64> for &VecN {
    type Output = VecN;
    fn div(self, s: f64) -> VecN {
        self.scale(1.0 / s)
    }
}

impl Neg for &VecN {
    type Output = VecN;
    fn neg(self) -> VecN {
        self.scale(-1.0)
    }
}

#[derive(Clone, Debug)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f64,
    pub charge: f64,
}

impl Particle {
    pub fn new(position: Vec3, velocity: Vec3, mass: f64, charge: f64) -> Self {
        Self {
            position,
            velocity,
            mass,
            charge,
        }
    }

    pub fn kinetic_energy(&self) -> f64 {
        0.5 * self.mass * self.velocity.dot(&self.velocity)
    }

    pub fn momentum(&self) -> Vec3 {
        self.velocity.scale(self.mass)
    }
}
