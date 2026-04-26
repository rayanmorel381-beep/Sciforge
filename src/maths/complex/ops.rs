use super::types::Complex;

pub fn roots_of_unity(n: usize) -> Vec<Complex> {
    (0..n)
        .map(|k| {
            let theta = 2.0 * std::f64::consts::PI * k as f64 / n as f64;
            Complex::from_polar(1.0, theta)
        })
        .collect()
}

pub fn complex_polynomial_eval(coeffs: &[Complex], z: Complex) -> Complex {
    let mut result = Complex::zero();
    let mut zn = Complex::one();
    for &c in coeffs {
        result = result + c * zn;
        zn = zn * z;
    }
    result
}

pub fn mandelbrot_iterate(c: Complex, max_iter: u32) -> u32 {
    let mut z = Complex::zero();
    for i in 0..max_iter {
        if z.norm_sq() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iter
}

pub fn julia_iterate(z0: Complex, c: Complex, max_iter: u32) -> u32 {
    let mut z = z0;
    for i in 0..max_iter {
        if z.norm_sq() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iter
}

pub fn newton_fractal_step(z: Complex, coeffs: &[Complex], deriv_coeffs: &[Complex]) -> Complex {
    let fz = complex_polynomial_eval(coeffs, z);
    let dfz = complex_polynomial_eval(deriv_coeffs, z);
    if dfz.norm_sq() < 1e-30 {
        return z;
    }
    z - fz / dfz
}

pub fn complex_matrix_mul(a: &[Vec<Complex>], b: &[Vec<Complex>]) -> Vec<Vec<Complex>> {
    let m = a.len();
    let n = b[0].len();
    let mut result = vec![vec![Complex::zero(); n]; m];
    for (i, ri) in result.iter_mut().enumerate() {
        for (j, rij) in ri.iter_mut().enumerate() {
            for (k, &aik) in a[i].iter().enumerate() {
                *rij = *rij + aik * b[k][j];
            }
        }
    }
    result
}

pub fn complex_matrix_det(m: &[Vec<Complex>]) -> Complex {
    let n = m.len();
    if n == 1 {
        return m[0][0];
    }
    if n == 2 {
        return m[0][0] * m[1][1] - m[0][1] * m[1][0];
    }
    let mut det = Complex::zero();
    for j in 0..n {
        let sign = if j % 2 == 0 {
            Complex::one()
        } else {
            -Complex::one()
        };
        let minor: Vec<Vec<Complex>> = (1..n)
            .map(|i| (0..n).filter(|&k| k != j).map(|k| m[i][k]).collect())
            .collect();
        det = det + sign * m[0][j] * complex_matrix_det(&minor);
    }
    det
}

pub fn complex_exp(z: Complex) -> Complex {
    let r = z.re.exp();
    Complex::new(r * z.im.cos(), r * z.im.sin())
}

pub fn complex_log(z: Complex) -> Complex {
    Complex::new(z.norm().ln(), z.im.atan2(z.re))
}

pub fn complex_sin(z: Complex) -> Complex {
    Complex::new(z.re.sin() * z.im.cosh(), z.re.cos() * z.im.sinh())
}

pub fn complex_cos(z: Complex) -> Complex {
    Complex::new(z.re.cos() * z.im.cosh(), -z.re.sin() * z.im.sinh())
}

pub fn complex_tan(z: Complex) -> Complex {
    let s = complex_sin(z);
    let c = complex_cos(z);
    if c.norm_sq() < 1e-30 {
        return Complex::new(f64::INFINITY, 0.0);
    }
    s / c
}

pub fn complex_sinh(z: Complex) -> Complex {
    Complex::new(z.re.sinh() * z.im.cos(), z.re.cosh() * z.im.sin())
}

pub fn complex_cosh(z: Complex) -> Complex {
    Complex::new(z.re.cosh() * z.im.cos(), z.re.sinh() * z.im.sin())
}

pub fn complex_sqrt(z: Complex) -> Complex {
    let r = z.norm();
    let theta = z.im.atan2(z.re);
    Complex::from_polar(r.sqrt(), theta / 2.0)
}

pub fn complex_power(z: Complex, w: Complex) -> Complex {
    if z.norm_sq() < 1e-60 {
        return Complex::zero();
    }
    complex_exp(w * complex_log(z))
}

pub fn complex_power_real(z: Complex, n: f64) -> Complex {
    let r = z.norm();
    let theta = z.im.atan2(z.re);
    Complex::from_polar(r.powf(n), n * theta)
}

pub fn mobius_transform(z: Complex, a: Complex, b: Complex, c: Complex, d: Complex) -> Complex {
    let num = a * z + b;
    let den = c * z + d;
    if den.norm_sq() < 1e-60 {
        return Complex::new(f64::INFINITY, 0.0);
    }
    num / den
}

pub fn bilinear_transform(s: Complex, t_sample: f64) -> Complex {
    let one = Complex::one();
    let half_t = Complex::new(t_sample * 0.5, 0.0);
    let num = one + s * half_t;
    let den = one - s * half_t;
    if den.norm_sq() < 1e-60 {
        return Complex::new(f64::INFINITY, 0.0);
    }
    num / den
}

pub fn complex_contour_integral(f: impl Fn(Complex) -> Complex, path: &[Complex]) -> Complex {
    let mut result = Complex::zero();
    for i in 1..path.len() {
        let dz = path[i] - path[i - 1];
        let mid = Complex::new(
            (path[i].re + path[i - 1].re) * 0.5,
            (path[i].im + path[i - 1].im) * 0.5,
        );
        result = result + f(mid) * dz;
    }
    result
}

pub fn complex_conjugate_transpose(m: &[Vec<Complex>]) -> Vec<Vec<Complex>> {
    let rows = m.len();
    let cols = if rows > 0 { m[0].len() } else { 0 };
    let mut result = vec![vec![Complex::zero(); rows]; cols];
    for (i, mi) in m.iter().enumerate() {
        for (j, &mij) in mi.iter().enumerate() {
            result[j][i] = mij.conj();
        }
    }
    result
}

pub fn complex_matrix_trace(m: &[Vec<Complex>]) -> Complex {
    let mut trace = Complex::zero();
    for (i, mi) in m.iter().enumerate() {
        trace = trace + mi[i];
    }
    trace
}

pub fn complex_matrix_add(a: &[Vec<Complex>], b: &[Vec<Complex>]) -> Vec<Vec<Complex>> {
    a.iter()
        .zip(b)
        .map(|(ra, rb)| ra.iter().zip(rb).map(|(&x, &y)| x + y).collect())
        .collect()
}

pub fn complex_matrix_scale(m: &[Vec<Complex>], s: Complex) -> Vec<Vec<Complex>> {
    m.iter()
        .map(|row| row.iter().map(|&x| x * s).collect())
        .collect()
}

pub fn complex_dft(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();
    (0..n)
        .map(|k| {
            let mut sum = Complex::zero();
            for (j, &x) in input.iter().enumerate() {
                let angle = -2.0 * std::f64::consts::PI * k as f64 * j as f64 / n as f64;
                sum = sum + x * Complex::from_polar(1.0, angle);
            }
            sum
        })
        .collect()
}

pub fn complex_idft(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();
    let scale = 1.0 / n as f64;
    (0..n)
        .map(|k| {
            let mut sum = Complex::zero();
            for (j, &x) in input.iter().enumerate() {
                let angle = 2.0 * std::f64::consts::PI * k as f64 * j as f64 / n as f64;
                sum = sum + x * Complex::from_polar(1.0, angle);
            }
            sum.scale(scale)
        })
        .collect()
}
