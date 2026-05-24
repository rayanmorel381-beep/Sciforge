pub fn compliance_matrix_orthotropic(
    e1: f64,
    e2: f64,
    e3: f64,
    nu12: f64,
    nu13: f64,
    nu23: f64,
    g12: f64,
    g13: f64,
    g23: f64,
) -> [[f64; 6]; 6] {
    let mut s = [[0.0_f64; 6]; 6];
    s[0][0] = 1.0 / e1;
    s[1][1] = 1.0 / e2;
    s[2][2] = 1.0 / e3;
    s[0][1] = -nu12 / e1;
    s[1][0] = s[0][1];
    s[0][2] = -nu13 / e1;
    s[2][0] = s[0][2];
    s[1][2] = -nu23 / e2;
    s[2][1] = s[1][2];
    s[3][3] = 1.0 / g23;
    s[4][4] = 1.0 / g13;
    s[5][5] = 1.0 / g12;
    s
}

pub fn transform_stress_2d(sx: f64, sy: f64, txy: f64, theta_rad: f64) -> (f64, f64, f64) {
    let c = theta_rad.cos();
    let s = theta_rad.sin();
    let sx_p = sx * c * c + sy * s * s + 2.0 * txy * c * s;
    let sy_p = sx * s * s + sy * c * c - 2.0 * txy * c * s;
    let txy_p = (sy - sx) * c * s + txy * (c * c - s * s);
    (sx_p, sy_p, txy_p)
}

pub fn transform_stress_3d_axis(
    stress: [[f64; 3]; 3],
    rotation: [[f64; 3]; 3],
) -> [[f64; 3]; 3] {
    let mut result = [[0.0_f64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                for l in 0..3 {
                    result[i][j] += rotation[i][k] * rotation[j][l] * stress[k][l];
                }
            }
        }
    }
    result
}

pub fn mohr_circle_principal(sx: f64, sy: f64, txy: f64) -> (f64, f64) {
    let avg = (sx + sy) / 2.0;
    let r = (((sx - sy) / 2.0).powi(2) + txy * txy).sqrt();
    (avg + r, avg - r)
}

pub fn tresca_3d_principal(s1: f64, s2: f64, s3: f64) -> f64 {
    let max = s1.max(s2).max(s3);
    let min = s1.min(s2).min(s3);
    max - min
}
