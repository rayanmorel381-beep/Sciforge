use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Quaternion {
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Self { w, x, y, z }
    }
    pub fn identity() -> Self {
        Self {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn zero() -> Self {
        Self {
            w: 0.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn pure(x: f64, y: f64, z: f64) -> Self {
        Self { w: 0.0, x, y, z }
    }

    pub fn from_axis_angle(axis: [f64; 3], angle: f64) -> Self {
        let half = angle * 0.5;
        let s = half.sin();
        let norm = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
        if norm < 1e-30 {
            return Self::identity();
        }
        Self {
            w: half.cos(),
            x: axis[0] / norm * s,
            y: axis[1] / norm * s,
            z: axis[2] / norm * s,
        }
    }

    pub fn from_euler(roll: f64, pitch: f64, yaw: f64) -> Self {
        let (sr, cr) = (roll * 0.5).sin_cos();
        let (sp, cp) = (pitch * 0.5).sin_cos();
        let (sy, cy) = (yaw * 0.5).sin_cos();
        Self {
            w: cr * cp * cy + sr * sp * sy,
            x: sr * cp * cy - cr * sp * sy,
            y: cr * sp * cy + sr * cp * sy,
            z: cr * cp * sy - sr * sp * cy,
        }
    }

    pub fn to_axis_angle(&self) -> ([f64; 3], f64) {
        let n = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        let angle = 2.0 * n.atan2(self.w);
        if n < 1e-30 {
            ([0.0, 0.0, 1.0], 0.0)
        } else {
            ([self.x / n, self.y / n, self.z / n], angle)
        }
    }

    pub fn to_rotation_matrix(&self) -> [[f64; 3]; 3] {
        let (w, x, y, z) = (self.w, self.x, self.y, self.z);
        [
            [
                1.0 - 2.0 * (y * y + z * z),
                2.0 * (x * y - w * z),
                2.0 * (x * z + w * y),
            ],
            [
                2.0 * (x * y + w * z),
                1.0 - 2.0 * (x * x + z * z),
                2.0 * (y * z - w * x),
            ],
            [
                2.0 * (x * z - w * y),
                2.0 * (y * z + w * x),
                1.0 - 2.0 * (x * x + y * y),
            ],
        ]
    }

    pub fn conj(&self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
    pub fn norm_sq(&self) -> f64 {
        self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn norm(&self) -> f64 {
        self.norm_sq().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let n = self.norm();
        if n < 1e-30 {
            return Self::identity();
        }
        Self {
            w: self.w / n,
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }

    pub fn inv(&self) -> Self {
        let n = self.norm_sq();
        let c = self.conj();
        Self {
            w: c.w / n,
            x: c.x / n,
            y: c.y / n,
            z: c.z / n,
        }
    }

    pub fn rotate_vector(&self, v: [f64; 3]) -> [f64; 3] {
        let p = Quaternion::pure(v[0], v[1], v[2]);
        let rotated = *self * p * self.conj();
        [rotated.x, rotated.y, rotated.z]
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn slerp(&self, other: &Self, t: f64) -> Self {
        let mut dot = self.dot(other);
        let mut other = *other;
        if dot < 0.0 {
            other = -other;
            dot = -dot;
        }
        if dot > 0.9995 {
            let result = Self {
                w: self.w + t * (other.w - self.w),
                x: self.x + t * (other.x - self.x),
                y: self.y + t * (other.y - self.y),
                z: self.z + t * (other.z - self.z),
            };
            return result.normalize();
        }
        let theta = dot.acos();
        let sin_theta = theta.sin();
        let a = ((1.0 - t) * theta).sin() / sin_theta;
        let b = (t * theta).sin() / sin_theta;
        Self {
            w: a * self.w + b * other.w,
            x: a * self.x + b * other.x,
            y: a * self.y + b * other.y,
            z: a * self.z + b * other.z,
        }
    }

    pub fn scale(&self, s: f64) -> Self {
        Self {
            w: self.w * s,
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    pub fn exp(&self) -> Self {
        let vec_norm = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        let ew = self.w.exp();
        if vec_norm < 1e-30 {
            return Self {
                w: ew,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
        let s = ew * vec_norm.sin() / vec_norm;
        Self {
            w: ew * vec_norm.cos(),
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    pub fn ln(&self) -> Self {
        let n = self.norm();
        let vec_norm = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if vec_norm < 1e-30 {
            return Self {
                w: n.ln(),
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
        let s = (self.w / n).acos() / vec_norm;
        Self {
            w: n.ln(),
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl Add for Quaternion {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            w: self.w + rhs.w,
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Quaternion {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            w: self.w - rhs.w,
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Quaternion {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
        }
    }
}

impl Div for Quaternion {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self.mul(rhs.inv())
    }
}

impl Neg for Quaternion {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            w: -self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl fmt::Display for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i + {}j + {}k", self.w, self.x, self.y, self.z)
    }
}
