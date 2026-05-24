pub fn principal_stresses_2d(sx: f64, sy: f64, txy: f64) -> (f64, f64) {
    let avg = (sx + sy) / 2.0;
    let r = ((sx - sy) / 2.0).hypot(txy);
    (avg + r, avg - r)
}

pub fn max_shear_stress_2d(sx: f64, sy: f64, txy: f64) -> f64 {
    ((sx - sy) / 2.0).hypot(txy)
}

pub fn mohr_circle_radius(sx: f64, sy: f64, txy: f64) -> f64 {
    ((sx - sy) / 2.0).hypot(txy)
}

pub fn mohr_circle_center(sx: f64, sy: f64) -> f64 {
    (sx + sy) / 2.0
}

pub fn stress_rotation_2d(sx: f64, sy: f64, txy: f64, theta: f64) -> (f64, f64, f64) {
    let c = theta.cos();
    let s = theta.sin();
    let sx2 = sx * c * c + sy * s * s + 2.0 * txy * s * c;
    let sy2 = sx * s * s + sy * c * c - 2.0 * txy * s * c;
    let txy2 = -(sx - sy) * s * c + txy * (c * c - s * s);
    (sx2, sy2, txy2)
}

pub fn deviatoric_stress(sx: f64, sy: f64, sz: f64) -> (f64, f64, f64) {
    let mean = (sx + sy + sz) / 3.0;
    (sx - mean, sy - mean, sz - mean)
}

pub fn stress_invariant_j2(s1: f64, s2: f64, s3: f64) -> f64 {
    let mean = (s1 + s2 + s3) / 3.0;
    let d1 = s1 - mean;
    let d2 = s2 - mean;
    let d3 = s3 - mean;
    0.5 * (d1 * d1 + d2 * d2 + d3 * d3)
}

pub fn beam_bending_stress(m: f64, y: f64, i: f64) -> f64 {
    m * y / i
}

pub fn beam_deflection_cantilever(p: f64, l: f64, e: f64, i: f64) -> f64 {
    p * l.powi(3) / (3.0 * e * i)
}

pub fn torsion_shear_stress(t: f64, r: f64, j: f64) -> f64 {
    t * r / j
}

pub fn column_euler_buckling(e: f64, i: f64, l: f64) -> f64 {
    std::f64::consts::PI * std::f64::consts::PI * e * i / (l * l)
}

pub fn hertz_contact_pressure(force: f64, r1: f64, r2: f64, e_star: f64) -> f64 {
    let r_eff = r1 * r2 / (r1 + r2);
    let a = (3.0 * force * r_eff / (4.0 * e_star)).powf(1.0 / 3.0);
    force / (std::f64::consts::PI * a * a)
}
