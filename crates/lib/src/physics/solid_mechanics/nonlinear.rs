pub fn neo_hookean_stress(stretch: f64, mu_pa: f64) -> f64 {
    mu_pa * (stretch.powi(2) - 1.0 / stretch)
}

pub fn neo_hookean_strain_energy(i1: f64, j: f64, mu_pa: f64, kappa_pa: f64) -> f64 {
    0.5 * mu_pa * (i1 - 3.0 - 2.0 * j.ln()) + 0.5 * kappa_pa * (j - 1.0).powi(2)
}

pub fn mooney_rivlin_stress(stretch: f64, c1_pa: f64, c2_pa: f64) -> f64 {
    let l = stretch;
    2.0 * (l.powi(2) - 1.0 / l) * (c1_pa + c2_pa / l)
}

pub fn mooney_rivlin_strain_energy(i1: f64, i2: f64, c1_pa: f64, c2_pa: f64) -> f64 {
    c1_pa * (i1 - 3.0) + c2_pa * (i2 - 3.0)
}

pub fn ogden_stress(stretch: f64, mu_pa: &[f64], alpha: &[f64]) -> f64 {
    let l = stretch;
    let mut sigma = 0.0;
    for (mi, ai) in mu_pa.iter().zip(alpha.iter()) {
        sigma += mi * (l.powf(*ai) - l.powf(-0.5 * ai));
    }
    sigma
}

pub fn yeoh_strain_energy(i1: f64, c10_pa: f64, c20_pa: f64, c30_pa: f64) -> f64 {
    let i = i1 - 3.0;
    c10_pa * i + c20_pa * i.powi(2) + c30_pa * i.powi(3)
}

pub fn arruda_boyce_strain_energy(i1: f64, mu_pa: f64, lambda_m: f64) -> f64 {
    let coeffs = [
        0.5,
        1.0 / 20.0,
        11.0 / 1050.0,
        19.0 / 7000.0,
        519.0 / 673750.0,
    ];
    let mut energy = 0.0;
    for (n, c) in coeffs.iter().enumerate() {
        let p = (n as i32) + 1;
        energy += c
            * mu_pa
            * (i1.powi(p) - 3f64.powi(p))
            / lambda_m.powi(2 * (p - 1));
    }
    energy
}

pub fn green_lagrange_strain(deformation_gradient: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let f = deformation_gradient;
    let mut c = [[0.0_f64; 3]; 3];
    for (i, ci) in c.iter_mut().enumerate() {
        for (j, cij) in ci.iter_mut().enumerate() {
            for fk in &f {
                *cij += fk[i] * fk[j];
            }
        }
    }
    let mut e = [[0.0_f64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            e[i][j] = 0.5 * (c[i][j] - if i == j { 1.0 } else { 0.0 });
        }
    }
    e
}
