use super::types::Vec3;

pub fn lerp(a: &Vec3, b: &Vec3, t: f64) -> Vec3 {
    &a.scale(1.0 - t) + &b.scale(t)
}

pub fn angle_between(a: &Vec3, b: &Vec3) -> f64 {
    let d = a.dot(b) / (a.magnitude() * b.magnitude());
    d.clamp(-1.0, 1.0).acos()
}

pub fn project(v: &Vec3, onto: &Vec3) -> Vec3 {
    onto.scale(v.dot(onto) / onto.dot(onto))
}

pub fn reject(v: &Vec3, from: &Vec3) -> Vec3 {
    v - &project(v, from)
}

pub fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    v - &normal.scale(2.0 * v.dot(normal))
}

pub fn triple_scalar(a: &Vec3, b: &Vec3, c: &Vec3) -> f64 {
    a.dot(&b.cross(c))
}

pub fn triple_vector(a: &Vec3, b: &Vec3, c: &Vec3) -> Vec3 {
    &b.scale(a.dot(c)) - &c.scale(a.dot(b))
}

pub fn slerp(a: &Vec3, b: &Vec3, t: f64) -> Vec3 {
    let dot = a.dot(b) / (a.magnitude() * b.magnitude());
    let dot = dot.clamp(-1.0, 1.0);
    let theta = dot.acos();
    if theta.abs() < 1e-10 {
        return lerp(a, b, t);
    }
    let sin_theta = theta.sin();
    let wa = ((1.0 - t) * theta).sin() / sin_theta;
    let wb = (t * theta).sin() / sin_theta;
    &a.scale(wa) + &b.scale(wb)
}

pub fn distance(a: &Vec3, b: &Vec3) -> f64 {
    (b - a).magnitude()
}

pub fn midpoint(a: &Vec3, b: &Vec3) -> Vec3 {
    lerp(a, b, 0.5)
}

pub fn centroid(points: &[Vec3]) -> Vec3 {
    let n = points.len() as f64;
    let mut sum = Vec3::zero();
    for p in points {
        sum = &sum + p;
    }
    sum.scale(1.0 / n)
}

pub fn area_triangle(a: &Vec3, b: &Vec3, c: &Vec3) -> f64 {
    let ab = b - a;
    let ac = c - a;
    0.5 * ab.cross(&ac).magnitude()
}

pub fn normal_triangle(a: &Vec3, b: &Vec3, c: &Vec3) -> Vec3 {
    let ab = b - a;
    let ac = c - a;
    ab.cross(&ac).normalize()
}

pub fn decompose_parallel_perpendicular(v: &Vec3, direction: &Vec3) -> (Vec3, Vec3) {
    let parallel = project(v, direction);
    let perpendicular = v - &parallel;
    (parallel, perpendicular)
}

pub fn rotate_around_axis(v: &Vec3, axis: &Vec3, angle: f64) -> Vec3 {
    let k = axis.normalize();
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    let term1 = v.scale(cos_a);
    let term2 = k.cross(v).scale(sin_a);
    let term3 = k.scale(k.dot(v) * (1.0 - cos_a));
    &(&term1 + &term2) + &term3
}

pub fn spherical_to_cartesian(r: f64, theta: f64, phi: f64) -> Vec3 {
    Vec3::new(
        r * theta.sin() * phi.cos(),
        r * theta.sin() * phi.sin(),
        r * theta.cos(),
    )
}

pub fn cartesian_to_spherical(v: &Vec3) -> (f64, f64, f64) {
    let r = v.magnitude();
    let theta = if r.abs() < 1e-30 {
        0.0
    } else {
        (v.z / r).acos()
    };
    let phi = v.y.atan2(v.x);
    (r, theta, phi)
}

pub fn cylindrical_to_cartesian(rho: f64, phi: f64, z: f64) -> Vec3 {
    Vec3::new(rho * phi.cos(), rho * phi.sin(), z)
}

pub fn cartesian_to_cylindrical(v: &Vec3) -> (f64, f64, f64) {
    let rho = (v.x * v.x + v.y * v.y).sqrt();
    let phi = v.y.atan2(v.x);
    (rho, phi, v.z)
}
