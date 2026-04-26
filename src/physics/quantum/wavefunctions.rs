use crate::maths::complex::Complex;

pub fn plane_wave(x: f64, k: f64, omega: f64, t: f64) -> Complex {
    Complex::from_polar(1.0, k * x - omega * t)
}

pub fn gaussian_packet(x: f64, x0: f64, sigma: f64, k0: f64) -> Complex {
    let norm = 1.0 / (sigma * (2.0 * std::f64::consts::PI).sqrt()).sqrt();
    let gauss = (-(x - x0).powi(2) / (4.0 * sigma * sigma)).exp();
    Complex::from_polar(norm * gauss, k0 * x)
}

pub fn normalize(psi: &mut [Complex], dx: f64) {
    let norm_sq: f64 = psi.iter().map(|c| c.norm() * c.norm()).sum::<f64>() * dx;
    let inv_norm = 1.0 / norm_sq.sqrt();
    for c in psi.iter_mut() {
        *c = *c * Complex::new(inv_norm, 0.0);
    }
}

pub fn probability_density(psi: &[Complex]) -> Vec<f64> {
    psi.iter().map(|c| c.norm() * c.norm()).collect()
}

pub fn expectation_position(psi: &[Complex], x: &[f64], dx: f64) -> f64 {
    psi.iter()
        .zip(x.iter())
        .map(|(p, &xi)| p.norm() * p.norm() * xi)
        .sum::<f64>()
        * dx
}

pub fn expectation_momentum(psi: &[Complex], dx: f64) -> f64 {
    use crate::constants::HBAR;
    let n = psi.len();
    let mut sum = 0.0;
    for i in 1..n - 1 {
        let dpsi = (psi[i + 1] - psi[i - 1]) * Complex::new(1.0 / (2.0 * dx), 0.0);
        let integrand = psi[i].conj() * dpsi * Complex::new(0.0, -HBAR);
        sum += integrand.re;
    }
    sum * dx
}

pub fn time_evolve_split_step(
    psi: &mut [Complex],
    v: &[f64],
    dx: f64,
    dt: f64,
    mass: f64,
    steps: usize,
) {
    use crate::constants::HBAR;
    let n = psi.len();
    let dk = 2.0 * std::f64::consts::PI / (n as f64 * dx);

    for _ in 0..steps {
        for i in 0..n {
            let phase = -v[i] * dt / (2.0 * HBAR);
            psi[i] = psi[i] * Complex::from_polar(1.0, phase);
        }

        let mut ft = vec![Complex::new(0.0, 0.0); n];
        for (k, ftk) in ft.iter_mut().enumerate() {
            for (j, &psij) in psi.iter().enumerate() {
                let angle = -2.0 * std::f64::consts::PI * (k as f64) * (j as f64) / (n as f64);
                *ftk = *ftk + psij * Complex::from_polar(1.0, angle);
            }
        }

        for (k, ftk) in ft.iter_mut().enumerate() {
            let kk = if k <= n / 2 {
                k as f64 * dk
            } else {
                (k as f64 - n as f64) * dk
            };
            let phase = -HBAR * kk * kk * dt / (2.0 * mass);
            *ftk = *ftk * Complex::from_polar(1.0, phase);
        }

        for (j, psij) in psi.iter_mut().enumerate() {
            *psij = Complex::new(0.0, 0.0);
            for (k, &ftk) in ft.iter().enumerate() {
                let angle = 2.0 * std::f64::consts::PI * (k as f64) * (j as f64) / (n as f64);
                *psij = *psij + ftk * Complex::from_polar(1.0, angle);
            }
            *psij = *psij * Complex::new(1.0 / n as f64, 0.0);
        }

        for i in 0..n {
            let phase = -v[i] * dt / (2.0 * HBAR);
            psi[i] = psi[i] * Complex::from_polar(1.0, phase);
        }
    }
}

pub fn inner_product(psi: &[Complex], phi: &[Complex], dx: f64) -> Complex {
    let mut sum = Complex::new(0.0, 0.0);
    for i in 0..psi.len() {
        sum = sum + psi[i].conj() * phi[i];
    }
    sum * Complex::new(dx, 0.0)
}

pub fn transition_probability(psi_initial: &[Complex], psi_final: &[Complex], dx: f64) -> f64 {
    let ip = inner_product(psi_final, psi_initial, dx);
    ip.norm() * ip.norm()
}
