use super::types::{Particle, Vec3};
use crate::constants::{G, K_COULOMB};

pub fn gravitational_force(p1: &Particle, p2: &Particle) -> Vec3 {
    let r = &p2.position - &p1.position;
    let dist = r.magnitude();
    if dist < 1e-30 {
        return Vec3::zero();
    }
    r.normalize().scale(G * p1.mass * p2.mass / (dist * dist))
}

pub fn coulomb_force(p1: &Particle, p2: &Particle) -> Vec3 {
    let r = &p2.position - &p1.position;
    let dist = r.magnitude();
    if dist < 1e-30 {
        return Vec3::zero();
    }
    r.normalize()
        .scale(-K_COULOMB * p1.charge * p2.charge / (dist * dist))
}

pub fn lorentz_force(charge: f64, velocity: &Vec3, e_field: &Vec3, b_field: &Vec3) -> Vec3 {
    &e_field.scale(charge) + &velocity.cross(b_field).scale(charge)
}

pub fn spring_force(k: f64, rest_length: f64, p1: &Vec3, p2: &Vec3) -> Vec3 {
    let r = p2 - p1;
    let dist = r.magnitude();
    if dist < 1e-30 {
        return Vec3::zero();
    }
    r.normalize().scale(k * (dist - rest_length))
}

pub fn drag_force(cd: f64, rho: f64, area: f64, velocity: &Vec3) -> Vec3 {
    let speed = velocity.magnitude();
    if speed < 1e-30 {
        return Vec3::zero();
    }
    velocity
        .normalize()
        .scale(-0.5 * cd * rho * area * speed * speed)
}

pub fn dipole_field(moment: &Vec3, position: &Vec3) -> Vec3 {
    let r = position.magnitude();
    if r < 1e-30 {
        return Vec3::zero();
    }
    let r2 = r * r;
    let r5 = r2 * r2 * r;
    let mdotr = moment.dot(position);
    let term1 = position.scale(3.0 * mdotr / r5);
    let term2 = moment.scale(1.0 / (r2 * r));
    &term1 - &term2
}

pub fn gradient_scalar_field(
    f: impl Fn(f64, f64, f64) -> f64,
    x: f64,
    y: f64,
    z: f64,
    h: f64,
) -> Vec3 {
    let dx = (f(x + h, y, z) - f(x - h, y, z)) / (2.0 * h);
    let dy = (f(x, y + h, z) - f(x, y - h, z)) / (2.0 * h);
    let dz = (f(x, y, z + h) - f(x, y, z - h)) / (2.0 * h);
    Vec3::new(dx, dy, dz)
}

pub fn divergence(
    fx: impl Fn(f64, f64, f64) -> f64,
    fy: impl Fn(f64, f64, f64) -> f64,
    fz: impl Fn(f64, f64, f64) -> f64,
    x: f64,
    y: f64,
    z: f64,
    h: f64,
) -> f64 {
    let dfdx = (fx(x + h, y, z) - fx(x - h, y, z)) / (2.0 * h);
    let dfdy = (fy(x, y + h, z) - fy(x, y - h, z)) / (2.0 * h);
    let dfdz = (fz(x, y, z + h) - fz(x, y, z - h)) / (2.0 * h);
    dfdx + dfdy + dfdz
}

pub fn curl(
    fx: impl Fn(f64, f64, f64) -> f64,
    fy: impl Fn(f64, f64, f64) -> f64,
    fz: impl Fn(f64, f64, f64) -> f64,
    x: f64,
    y: f64,
    z: f64,
    h: f64,
) -> Vec3 {
    let dfz_dy = (fz(x, y + h, z) - fz(x, y - h, z)) / (2.0 * h);
    let dfy_dz = (fy(x, y, z + h) - fy(x, y, z - h)) / (2.0 * h);
    let dfx_dz = (fx(x, y, z + h) - fx(x, y, z - h)) / (2.0 * h);
    let dfz_dx = (fz(x + h, y, z) - fz(x - h, y, z)) / (2.0 * h);
    let dfy_dx = (fy(x + h, y, z) - fy(x - h, y, z)) / (2.0 * h);
    let dfx_dy = (fx(x, y + h, z) - fx(x, y - h, z)) / (2.0 * h);
    Vec3::new(dfz_dy - dfy_dz, dfx_dz - dfz_dx, dfy_dx - dfx_dy)
}

pub fn laplacian_scalar(f: impl Fn(f64, f64, f64) -> f64, x: f64, y: f64, z: f64, h: f64) -> f64 {
    let fxyz = f(x, y, z);
    let d2x = (f(x + h, y, z) - 2.0 * fxyz + f(x - h, y, z)) / (h * h);
    let d2y = (f(x, y + h, z) - 2.0 * fxyz + f(x, y - h, z)) / (h * h);
    let d2z = (f(x, y, z + h) - 2.0 * fxyz + f(x, y, z - h)) / (h * h);
    d2x + d2y + d2z
}

pub fn potential_energy_field(
    force: impl Fn(&Vec3) -> Vec3,
    path_start: &Vec3,
    path_end: &Vec3,
    steps: usize,
) -> f64 {
    let mut total = 0.0;
    for i in 0..steps {
        let t0 = i as f64 / steps as f64;
        let t1 = (i + 1) as f64 / steps as f64;
        let p0 = lerp_vec3(path_start, path_end, t0);
        let p1 = lerp_vec3(path_start, path_end, t1);
        let f_mid = force(&lerp_vec3(path_start, path_end, 0.5 * (t0 + t1)));
        let dr = &p1 - &p0;
        total += f_mid.dot(&dr);
    }
    -total
}

fn lerp_vec3(a: &Vec3, b: &Vec3, t: f64) -> Vec3 {
    Vec3::new(
        a.x + t * (b.x - a.x),
        a.y + t * (b.y - a.y),
        a.z + t * (b.z - a.z),
    )
}

pub fn streamline(
    vx: impl Fn(f64, f64, f64) -> f64,
    vy: impl Fn(f64, f64, f64) -> f64,
    vz: impl Fn(f64, f64, f64) -> f64,
    start: &Vec3,
    dt: f64,
    steps: usize,
) -> Vec<Vec3> {
    let mut points = Vec::with_capacity(steps + 1);
    let mut p = start.clone();
    points.push(p.clone());
    for _ in 0..steps {
        let v = Vec3::new(vx(p.x, p.y, p.z), vy(p.x, p.y, p.z), vz(p.x, p.y, p.z));
        p = &p + &v.scale(dt);
        points.push(p.clone());
    }
    points
}
