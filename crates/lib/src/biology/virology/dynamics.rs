pub fn target_cell_model(
    v: f64,
    x: f64,
    y: f64,
    beta: f64,
    delta: f64,
    p: f64,
    c: f64,
) -> (f64, f64, f64) {
    let dx = -beta * x * v;
    let dy = beta * x * v - delta * y;
    let dv = p * y - c * v;
    ((x + dx).max(0.0), (y + dy).max(0.0), (v + dv).max(0.0))
}

pub fn basic_reproduction_number_viral(beta: f64, p: f64, x0: f64, delta: f64, c: f64) -> f64 {
    beta * p * x0 / (delta * c)
}

pub fn viral_fitness(replication_rate: f64, clearance_rate: f64) -> f64 {
    replication_rate / clearance_rate
}

pub fn burst_size(p: f64, delta: f64) -> f64 {
    p / delta
}

pub fn viral_load_decay(v0: f64, clearance: f64, t: f64) -> f64 {
    v0 * (-clearance * t).exp()
}

pub fn eclipse_phase_model(
    v: f64,
    x: f64,
    y1: f64,
    y2: f64,
    beta: f64,
    k: f64,
    delta: f64,
    p: f64,
    c: f64,
) -> (f64, f64, f64, f64) {
    let dx = -beta * x * v;
    let dy1 = beta * x * v - k * y1;
    let dy2 = k * y1 - delta * y2;
    let dv = p * y2 - c * v;
    (
        (x + dx).max(0.0),
        (y1 + dy1).max(0.0),
        (y2 + dy2).max(0.0),
        (v + dv).max(0.0),
    )
}

pub fn effective_reproduction_number(r0: f64, susceptible_fraction: f64) -> f64 {
    r0 * susceptible_fraction
}

pub fn viral_load_biphasic_decay(v0: f64, f_fast: f64, delta1: f64, delta2: f64, t: f64) -> f64 {
    v0 * (f_fast * (-delta1 * t).exp() + (1.0 - f_fast) * (-delta2 * t).exp())
}

pub fn within_host_r0(beta: f64, s0: f64, p: f64, delta: f64, c: f64) -> f64 {
    beta * s0 * p / (delta * c)
}

pub fn latently_infected_dynamics(
    x: f64,
    y_l: f64,
    y_a: f64,
    v: f64,
    beta: f64,
    alpha: f64,
    delta_l: f64,
    delta_a: f64,
    p: f64,
    c: f64,
    f_latent: f64,
) -> (f64, f64, f64, f64) {
    let infection = beta * x * v;
    let dx = -infection;
    let dy_l = f_latent * infection - (alpha + delta_l) * y_l;
    let dy_a = (1.0 - f_latent) * infection + alpha * y_l - delta_a * y_a;
    let dv = p * y_a - c * v;
    (dx, dy_l, dy_a, dv)
}

pub fn immune_response_ctl(y: f64, z: f64, k_kill: f64, k_expand: f64, d_z: f64) -> (f64, f64) {
    let killing = k_kill * y * z;
    let dz = k_expand * y * z - d_z * z;
    (-killing, dz)
}

pub fn antibody_neutralization(v: f64, ab: f64, k_neut: f64) -> f64 {
    v / (1.0 + k_neut * ab)
}

pub fn viral_replication_logistic(v: f64, r: f64, k_capacity: f64) -> f64 {
    r * v * (1.0 - v / k_capacity)
}

pub fn cell_to_cell_transmission(y: f64, x: f64, gamma: f64) -> f64 {
    gamma * y * x
}

pub fn defective_interfering_particles(
    v: f64,
    d: f64,
    beta_v: f64,
    beta_d: f64,
    p_v: f64,
    p_d: f64,
    c: f64,
) -> (f64, f64) {
    let dv = p_v * beta_v * v / (1.0 + v + d) - c * v;
    let dd = p_d * beta_d * v / (1.0 + v + d) - c * d;
    (dv, dd)
}

pub fn superinfection_exclusion(
    v1: f64,
    v2: f64,
    beta1: f64,
    beta2: f64,
    exclusion_factor: f64,
) -> (f64, f64) {
    let eff1 = beta1 * v1 / (1.0 + exclusion_factor * v2);
    let eff2 = beta2 * v2 / (1.0 + exclusion_factor * v1);
    (eff1, eff2)
}

pub fn time_to_peak_viremia(beta: f64, x0: f64, p: f64, delta: f64, c: f64) -> f64 {
    let r0 = beta * x0 * p / (delta * c);
    if r0 <= 1.0 {
        return f64::INFINITY;
    }
    (r0.ln()) / (beta * x0 * p / c - delta)
}

pub fn generation_time_viral(eclipse: f64, infectious_lifespan: f64) -> f64 {
    eclipse + infectious_lifespan
}
