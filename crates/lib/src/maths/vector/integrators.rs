use super::types::Vec3;

pub fn euler_step(pos: &Vec3, vel: &Vec3, accel: &Vec3, dt: f64) -> (Vec3, Vec3) {
    let new_vel = &vel.clone() + &accel.scale(dt);
    let new_pos = &pos.clone() + &new_vel.scale(dt);
    (new_pos, new_vel)
}

pub fn verlet_step(
    pos: &Vec3,
    vel: &Vec3,
    accel_fn: impl Fn(&Vec3) -> Vec3,
    dt: f64,
) -> (Vec3, Vec3) {
    let a = accel_fn(pos);
    let new_pos = &(&pos.clone() + &vel.scale(dt)) + &a.scale(0.5 * dt * dt);
    let a_new = accel_fn(&new_pos);
    let new_vel = &vel.clone() + &(&a + &a_new).scale(0.5 * dt);
    (new_pos, new_vel)
}

pub fn rk4_step(y: &Vec3, dydt: impl Fn(&Vec3) -> Vec3, dt: f64) -> Vec3 {
    let k1 = dydt(y);
    let k2 = dydt(&(&y.clone() + &k1.scale(dt * 0.5)));
    let k3 = dydt(&(&y.clone() + &k2.scale(dt * 0.5)));
    let k4 = dydt(&(&y.clone() + &k3.scale(dt)));
    let sum = &(&(&k1 + &k2.scale(2.0)) + &k3.scale(2.0)) + &k4;
    &y.clone() + &sum.scale(dt / 6.0)
}

pub fn leapfrog_step(
    pos: &Vec3,
    vel_half: &Vec3,
    accel_fn: impl Fn(&Vec3) -> Vec3,
    dt: f64,
) -> (Vec3, Vec3) {
    let new_pos = &pos.clone() + &vel_half.scale(dt);
    let a = accel_fn(&new_pos);
    let new_vel_half = &vel_half.clone() + &a.scale(dt);
    (new_pos, new_vel_half)
}

pub fn symplectic_euler_step(
    pos: &Vec3,
    vel: &Vec3,
    accel_fn: impl Fn(&Vec3) -> Vec3,
    dt: f64,
) -> (Vec3, Vec3) {
    let a = accel_fn(pos);
    let new_vel = &vel.clone() + &a.scale(dt);
    let new_pos = &pos.clone() + &new_vel.scale(dt);
    (new_pos, new_vel)
}

pub fn yoshida4_step(
    pos: &Vec3,
    vel: &Vec3,
    accel_fn: impl Fn(&Vec3) -> Vec3,
    dt: f64,
) -> (Vec3, Vec3) {
    let w1 = 1.0 / (2.0 - 2.0_f64.powf(1.0 / 3.0));
    let w0 = -2.0_f64.powf(1.0 / 3.0) * w1;
    let c = [w1 / 2.0, (w0 + w1) / 2.0, (w0 + w1) / 2.0, w1 / 2.0];
    let d = [w1, w0, w1];
    let mut p = pos.clone();
    let mut v = vel.clone();
    p = &p + &v.scale(c[0] * dt);
    let a0 = accel_fn(&p);
    v = &v + &a0.scale(d[0] * dt);
    p = &p + &v.scale(c[1] * dt);
    let a1 = accel_fn(&p);
    v = &v + &a1.scale(d[1] * dt);
    p = &p + &v.scale(c[2] * dt);
    let a2 = accel_fn(&p);
    v = &v + &a2.scale(d[2] * dt);
    p = &p + &v.scale(c[3] * dt);
    (p, v)
}

pub fn rk4_vec3_step(
    pos: &Vec3,
    vel: &Vec3,
    accel_fn: impl Fn(&Vec3, &Vec3) -> Vec3,
    dt: f64,
) -> (Vec3, Vec3) {
    let k1v = accel_fn(pos, vel);
    let k1x = vel.clone();

    let p2 = &pos.clone() + &k1x.scale(0.5 * dt);
    let v2 = &vel.clone() + &k1v.scale(0.5 * dt);
    let k2v = accel_fn(&p2, &v2);
    let k2x = v2.clone();

    let p3 = &pos.clone() + &k2x.scale(0.5 * dt);
    let v3 = &vel.clone() + &k2v.scale(0.5 * dt);
    let k3v = accel_fn(&p3, &v3);
    let k3x = v3.clone();

    let p4 = &pos.clone() + &k3x.scale(dt);
    let v4 = &vel.clone() + &k3v.scale(dt);
    let k4v = accel_fn(&p4, &v4);
    let k4x = v4;

    let dx = &(&(&k1x + &k2x.scale(2.0)) + &k3x.scale(2.0)) + &k4x;
    let dv = &(&(&k1v + &k2v.scale(2.0)) + &k3v.scale(2.0)) + &k4v;

    (
        &pos.clone() + &dx.scale(dt / 6.0),
        &vel.clone() + &dv.scale(dt / 6.0),
    )
}

pub fn stormer_verlet_step(
    pos: &Vec3,
    pos_prev: &Vec3,
    accel_fn: impl Fn(&Vec3) -> Vec3,
    dt: f64,
) -> Vec3 {
    let a = accel_fn(pos);
    &(&pos.scale(2.0) - pos_prev) + &a.scale(dt * dt)
}

pub fn forest_ruth_step(
    pos: &Vec3,
    vel: &Vec3,
    accel_fn: impl Fn(&Vec3) -> Vec3,
    dt: f64,
) -> (Vec3, Vec3) {
    let theta = 1.0 / (2.0 - 2.0_f64.powf(1.0 / 3.0));
    let mut p = &pos.clone() + &vel.scale(theta * dt / 2.0);
    let a1 = accel_fn(&p);
    let mut v = &vel.clone() + &a1.scale(theta * dt);
    p = &p + &v.scale((1.0 - theta) * dt / 2.0);
    let a2 = accel_fn(&p);
    v = &v + &a2.scale((1.0 - 2.0 * theta) * dt);
    p = &p + &v.scale((1.0 - theta) * dt / 2.0);
    let a3 = accel_fn(&p);
    v = &v + &a3.scale(theta * dt);
    p = &p + &v.scale(theta * dt / 2.0);
    (p, v)
}

pub fn adaptive_verlet(
    pos: &Vec3,
    vel: &Vec3,
    accel_fn: impl Fn(&Vec3) -> Vec3,
    dt: f64,
    tol: f64,
) -> (Vec3, Vec3, f64) {
    let (p1, v1) = verlet_step(pos, vel, &accel_fn, dt);
    let half_dt = dt * 0.5;
    let (p_mid, v_mid) = verlet_step(pos, vel, &accel_fn, half_dt);
    let (p2, v2) = verlet_step(&p_mid, &v_mid, &accel_fn, half_dt);
    let err = (&p2 - &p1).magnitude();
    let new_dt = if err > 1e-30 {
        dt * (tol / err).powf(1.0 / 3.0) * 0.9
    } else {
        dt * 2.0
    };
    if err < tol {
        (p2, v2, new_dt)
    } else {
        (p1, v1, new_dt)
    }
}
