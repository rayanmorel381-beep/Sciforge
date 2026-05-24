use crate::constants::EARTH_GRAVITY;

pub fn gait_stride_length(velocity: f64, cadence: f64) -> f64 {
    if cadence <= 0.0 {
        return 0.0;
    }
    velocity * 60.0 / cadence
}

pub fn ground_reaction_force(mass: f64, acceleration: f64) -> f64 {
    mass * (EARTH_GRAVITY + acceleration)
}

pub fn joint_moment(force: f64, moment_arm: f64) -> f64 {
    force * moment_arm
}

pub fn joint_power(moment: f64, angular_velocity: f64) -> f64 {
    moment * angular_velocity
}

pub fn center_of_pressure(forces: &[(f64, f64, f64)], positions: &[(f64, f64)]) -> (f64, f64) {
    let n = forces.len().min(positions.len());
    let mut total_fz = 0.0;
    let mut cop_x = 0.0;
    let mut cop_y = 0.0;
    for i in 0..n {
        total_fz += forces[i].2;
        cop_x += forces[i].2 * positions[i].0;
        cop_y += forces[i].2 * positions[i].1;
    }
    if total_fz.abs() < 1e-30 {
        return (0.0, 0.0);
    }
    (cop_x / total_fz, cop_y / total_fz)
}

pub fn inverse_dynamics_moment(
    i_segment: f64,
    alpha: f64,
    proximal_force: f64,
    proximal_arm: f64,
    distal_force: f64,
    distal_arm: f64,
) -> f64 {
    i_segment * alpha + proximal_force * proximal_arm - distal_force * distal_arm
}

pub fn metabolic_cost_of_transport(metabolic_rate: f64, mass: f64, velocity: f64) -> f64 {
    metabolic_rate / (mass * velocity).max(1e-30)
}

pub fn froude_number(velocity: f64, leg_length: f64) -> f64 {
    velocity * velocity / (EARTH_GRAVITY * leg_length)
}

pub fn dynamic_stability_margin(base_of_support: &[(f64, f64)], com: (f64, f64)) -> f64 {
    let mut min_dist = f64::INFINITY;
    let n = base_of_support.len();
    for i in 0..n {
        let j = (i + 1) % n;
        let (x1, y1) = base_of_support[i];
        let (x2, y2) = base_of_support[j];
        let dx = x2 - x1;
        let dy = y2 - y1;
        let t = (((com.0 - x1) * dx + (com.1 - y1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
        let px = x1 + t * dx;
        let py = y1 + t * dy;
        let dist = ((com.0 - px).powi(2) + (com.1 - py).powi(2)).sqrt();
        if dist < min_dist {
            min_dist = dist;
        }
    }
    min_dist
}

pub fn work_loop_area(force: &[f64], length: &[f64]) -> f64 {
    let n = force.len().min(length.len());
    if n < 3 {
        return 0.0;
    }
    let mut area = 0.0;
    for i in 0..n {
        let j = (i + 1) % n;
        area += force[i] * length[j] - force[j] * length[i];
    }
    area.abs() / 2.0
}

pub fn pendulum_energy_recovery(ek_change: f64, ep_change: f64) -> f64 {
    let total_work = ek_change.abs() + ep_change.abs();
    if total_work < 1e-30 {
        return 0.0;
    }
    let external_work = (ek_change + ep_change).abs();
    1.0 - external_work / total_work
}
