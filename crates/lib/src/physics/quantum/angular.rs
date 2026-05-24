use crate::maths::complex::Complex;
use std::f64::consts::PI;

fn factorial(n: u64) -> f64 {
    (1..=n).fold(1.0, |acc, k| acc * k as f64)
}

pub fn associated_legendre(l: u32, m: i32, x: f64) -> f64 {
    let m_abs = m.unsigned_abs();
    if m_abs > l {
        return 0.0;
    }

    let mut pmm = 1.0;
    if m_abs > 0 {
        let somx2 = (1.0 - x * x).sqrt();
        let mut fact = 1.0;
        for _ in 0..m_abs {
            pmm *= -fact * somx2;
            fact += 2.0;
        }
    }

    if l == m_abs {
        return if m < 0 { neg_m_factor(l, m) * pmm } else { pmm };
    }

    let mut pmm1 = x * (2 * m_abs + 1) as f64 * pmm;
    if l == m_abs + 1 {
        return if m < 0 {
            neg_m_factor(l, m) * pmm1
        } else {
            pmm1
        };
    }

    let mut pll = 0.0;
    for ll in (m_abs + 2)..=l {
        pll =
            ((2 * ll - 1) as f64 * x * pmm1 - (ll + m_abs - 1) as f64 * pmm) / (ll - m_abs) as f64;
        pmm = pmm1;
        pmm1 = pll;
    }
    if m < 0 { neg_m_factor(l, m) * pll } else { pll }
}

fn neg_m_factor(l: u32, m: i32) -> f64 {
    let m_abs = m.unsigned_abs();
    let sign = if m_abs.is_multiple_of(2) { 1.0 } else { -1.0 };
    sign * factorial((l - m_abs) as u64) / factorial((l + m_abs) as u64)
}

pub fn spherical_harmonic(l: u32, m: i32, theta: f64, phi: f64) -> Complex {
    if m.unsigned_abs() > l {
        return Complex::zero();
    }
    let m_abs = m.unsigned_abs();
    let norm = ((2 * l + 1) as f64 / (4.0 * PI) * factorial((l - m_abs) as u64)
        / factorial((l + m_abs) as u64))
    .sqrt();
    let plm = associated_legendre(l, m.abs(), theta.cos());
    let sign = if m < 0 && !m_abs.is_multiple_of(2) {
        -1.0
    } else {
        1.0
    };
    let phase = Complex::from_polar(1.0, m as f64 * phi);
    phase * Complex::new(sign * norm * plm, 0.0)
}

pub fn clebsch_gordan(j1: f64, m1: f64, j2: f64, m2: f64, j: f64, m: f64) -> f64 {
    if (m1 + m2 - m).abs() > 1e-10 {
        return 0.0;
    }
    if j < (j1 - j2).abs() - 1e-10 || j > j1 + j2 + 1e-10 {
        return 0.0;
    }
    if m1.abs() > j1 + 1e-10 || m2.abs() > j2 + 1e-10 || m.abs() > j + 1e-10 {
        return 0.0;
    }

    let to_int = |x: f64| -> i64 { (x + 0.5_f64.copysign(x)).trunc() as i64 };

    let ij1 = to_int(2.0 * j1);
    let ij2 = to_int(2.0 * j2);
    let ij = to_int(2.0 * j);
    let im1 = to_int(2.0 * m1);
    let im2 = to_int(2.0 * m2);
    let im = to_int(2.0 * m);

    if (ij1 + ij2 + ij) % 2 != 0 {
        return 0.0;
    }
    if im1 + im2 != im {
        return 0.0;
    }

    let f = |x: i64| -> f64 { factorial((x / 2) as u64) };

    let j1pj2mj = ij1 + ij2 - ij;
    let jpj1mj2 = ij + ij1 - ij2;
    let jpj2mj1 = ij + ij2 - ij1;
    let j1pm1 = ij1 + im1;
    let j1mm1 = ij1 - im1;
    let j2pm2 = ij2 + im2;
    let j2mm2 = ij2 - im2;
    let jpm = ij + im;
    let jmm = ij - im;

    if j1pj2mj < 0 || jpj1mj2 < 0 || jpj2mj1 < 0 {
        return 0.0;
    }
    if j1pm1 < 0 || j1mm1 < 0 || j2pm2 < 0 || j2mm2 < 0 || jpm < 0 || jmm < 0 {
        return 0.0;
    }

    let prefactor = ((ij + 1) as f64 * f(j1pj2mj) * f(jpj1mj2) * f(jpj2mj1)
        / f(ij1 + ij2 + ij + 2)
        * f(j1pm1)
        * f(j1mm1)
        * f(j2pm2)
        * f(j2mm2)
        * f(jpm)
        * f(jmm))
    .sqrt();

    let k_min = [0, ij2 - ij - im1, ij1 + im2 - ij]
        .iter()
        .map(|&x| (x / 2).max(0))
        .max()
        .unwrap();
    let k_max = [(ij1 + ij2 - ij) / 2, (ij1 - im1) / 2, (ij2 + im2) / 2]
        .iter()
        .copied()
        .min()
        .unwrap();

    let mut sum = 0.0;
    for k in k_min..=k_max {
        let sign = if k % 2 == 0 { 1.0 } else { -1.0 };
        let denom = factorial(k as u64)
            * factorial(((j1pj2mj) / 2 - k) as u64)
            * factorial(((j1mm1) / 2 - k) as u64)
            * factorial(((j2pm2) / 2 - k) as u64)
            * factorial((k - (ij2 - ij - im1) / 2) as u64)
            * factorial((k - (ij1 + im2 - ij) / 2) as u64);
        sum += sign / denom;
    }

    prefactor * sum
}

pub fn wigner_3j(j1: f64, j2: f64, j3: f64, m1: f64, m2: f64, m3: f64) -> f64 {
    if (m1 + m2 + m3).abs() > 1e-10 {
        return 0.0;
    }
    let sign = if ((j1 - j2 - m3) * 2.0).round() as i64 % 4 == 0 {
        1.0
    } else {
        -1.0
    };
    sign / (2.0 * j3 + 1.0).sqrt() * clebsch_gordan(j1, m1, j2, m2, j3, -m3)
}

pub fn spherical_harmonic_real(l: u32, m: i32, theta: f64, phi: f64) -> f64 {
    if m == 0 {
        spherical_harmonic(l, 0, theta, phi).re
    } else if m > 0 {
        let yp = spherical_harmonic(l, m, theta, phi);
        let ym = spherical_harmonic(l, -m, theta, phi);
        let sign = if m % 2 == 0 { 1.0 } else { -1.0 };
        (yp.re + sign * ym.re) / 2.0_f64.sqrt()
    } else {
        let yp = spherical_harmonic(l, -m, theta, phi);
        let ym = spherical_harmonic(l, m, theta, phi);
        let sign = if (-m) % 2 == 0 { 1.0 } else { -1.0 };
        (yp.im - sign * ym.im) / 2.0_f64.sqrt()
    }
}

pub fn angular_momentum_coupling(j1: f64, j2: f64) -> Vec<(f64, f64, f64)> {
    let j_min = (j1 - j2).abs();
    let j_max = j1 + j2;
    let mut states = Vec::new();
    let mut j = j_min;
    while j <= j_max + 1e-10 {
        let mut mj = -j;
        while mj <= j + 1e-10 {
            let mut expansion = Vec::new();
            let mut m1 = -j1;
            while m1 <= j1 + 1e-10 {
                let m2 = mj - m1;
                if m2.abs() <= j2 + 1e-10 {
                    let cg = clebsch_gordan(j1, m1, j2, m2, j, mj);
                    if cg.abs() > 1e-12 {
                        expansion.push((m1, m2, cg));
                    }
                }
                m1 += 1.0;
            }
            for (m1_val, m2_val, _) in &expansion {
                states.push((j, *m1_val, *m2_val));
            }
            mj += 1.0;
        }
        j += 1.0;
    }
    states
}
