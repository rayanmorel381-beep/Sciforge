pub fn turing_reaction_diffusion(
    u: &mut [f64],
    v: &mut [f64],
    du: f64,
    dv: f64,
    a: f64,
    b: f64,
    dx: f64,
    dt: f64,
    steps: usize,
) {
    let n = u.len();
    for _ in 0..steps {
        let u_old = u.to_vec();
        let v_old = v.to_vec();
        for i in 1..n - 1 {
            let lap_u = (u_old[i + 1] - 2.0 * u_old[i] + u_old[i - 1]) / (dx * dx);
            let lap_v = (v_old[i + 1] - 2.0 * v_old[i] + v_old[i - 1]) / (dx * dx);
            let reaction_u = a - (b + 1.0) * u_old[i] + u_old[i] * u_old[i] * v_old[i];
            let reaction_v = b * u_old[i] - u_old[i] * u_old[i] * v_old[i];
            u[i] += (du * lap_u + reaction_u) * dt;
            v[i] += (dv * lap_v + reaction_v) * dt;
            u[i] = u[i].max(0.0);
            v[i] = v[i].max(0.0);
        }
    }
}

pub fn turing_instability_condition(du: f64, dv: f64, fu: f64, gv: f64, fg_det: f64) -> bool {
    let trace = fu + gv;
    if trace >= 0.0 {
        return false;
    }
    if fg_det <= 0.0 {
        return false;
    }
    let d_ratio = dv / du;
    let critical = (fu * d_ratio + gv).powi(2) - 4.0 * d_ratio * fg_det;
    critical > 0.0 && (fu * d_ratio + gv) > 0.0
}

pub fn gierer_meinhardt(
    activator: &mut [f64],
    inhibitor: &mut [f64],
    da: f64,
    di: f64,
    rho_a: f64,
    rho_i: f64,
    mu_a: f64,
    mu_i: f64,
    dx: f64,
    dt: f64,
    steps: usize,
) {
    let n = activator.len();
    for _ in 0..steps {
        let a_old = activator.to_vec();
        let h_old = inhibitor.to_vec();
        for i in 1..n - 1 {
            let lap_a = (a_old[i + 1] - 2.0 * a_old[i] + a_old[i - 1]) / (dx * dx);
            let lap_h = (h_old[i + 1] - 2.0 * h_old[i] + h_old[i - 1]) / (dx * dx);
            let src_a = rho_a * a_old[i] * a_old[i] / (h_old[i].max(1e-30)) - mu_a * a_old[i];
            let src_h = rho_i * a_old[i] * a_old[i] - mu_i * h_old[i];
            activator[i] = (a_old[i] + (da * lap_a + src_a) * dt).max(0.0);
            inhibitor[i] = (h_old[i] + (di * lap_h + src_h) * dt).max(0.0);
        }
    }
}

pub fn french_flag_positional(
    position: f64,
    threshold_high: f64,
    threshold_low: f64,
    morphogen_source: f64,
    decay_length: f64,
) -> u8 {
    let concentration = morphogen_source * (-position / decay_length).exp();
    if concentration > threshold_high {
        2
    } else if concentration > threshold_low {
        1
    } else {
        0
    }
}

pub fn lateral_inhibition(
    cells: &mut [f64],
    notch: &mut [f64],
    delta: &mut [f64],
    beta_n: f64,
    beta_d: f64,
    k: f64,
    n: f64,
    dt: f64,
    steps: usize,
) {
    let len = cells.len();
    for _ in 0..steps {
        let d_old = delta.to_vec();
        let n_old = notch.to_vec();
        for i in 0..len {
            let left = if i > 0 { d_old[i - 1] } else { d_old[len - 1] };
            let right = if i < len - 1 { d_old[i + 1] } else { d_old[0] };
            let avg_delta = (left + right) / 2.0;
            let dn = beta_n * avg_delta.powf(n) / (k.powf(n) + avg_delta.powf(n)) - n_old[i];
            let dd = beta_d / (1.0 + n_old[i].powf(n) / k.powf(n)) - d_old[i];
            notch[i] = (n_old[i] + dn * dt).max(0.0);
            delta[i] = (d_old[i] + dd * dt).max(0.0);
            cells[i] = notch[i];
        }
    }
}

pub fn clock_and_wavefront(
    position: f64,
    wavefront_speed: f64,
    frequency: f64,
    t: f64,
    threshold: f64,
) -> bool {
    let wavefront_position = wavefront_speed * t;
    if position > wavefront_position {
        return false;
    }
    let phase = (2.0 * std::f64::consts::PI * frequency * t).sin();
    phase > threshold
}

pub fn schnakenberg(
    u: &mut [f64],
    v: &mut [f64],
    du: f64,
    dv: f64,
    a: f64,
    b: f64,
    gamma: f64,
    dx: f64,
    dt: f64,
    steps: usize,
) {
    let n = u.len();
    for _ in 0..steps {
        let u_old = u.to_vec();
        let v_old = v.to_vec();
        for i in 1..n - 1 {
            let lap_u = (u_old[i + 1] - 2.0 * u_old[i] + u_old[i - 1]) / (dx * dx);
            let lap_v = (v_old[i + 1] - 2.0 * v_old[i] + v_old[i - 1]) / (dx * dx);
            let ru = gamma * (a - u_old[i] + u_old[i] * u_old[i] * v_old[i]);
            let rv = gamma * (b - u_old[i] * u_old[i] * v_old[i]);
            u[i] = (u_old[i] + (du * lap_u + ru) * dt).max(0.0);
            v[i] = (v_old[i] + (dv * lap_v + rv) * dt).max(0.0);
        }
    }
}

pub fn voronoi_cell_sorting(
    positions: &[(f64, f64)],
    types: &[u8],
    adhesion_same: f64,
    adhesion_diff: f64,
) -> Vec<(f64, f64)> {
    let n = positions.len();
    let mut forces: Vec<(f64, f64)> = vec![(0.0, 0.0); n];
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = positions[j].0 - positions[i].0;
            let dy = positions[j].1 - positions[i].1;
            let dist = (dx * dx + dy * dy).sqrt().max(0.1);
            let adhesion = if types[i] == types[j] {
                adhesion_same
            } else {
                adhesion_diff
            };
            let force = adhesion * (1.0 - 1.0 / dist);
            forces[i].0 += force * dx / dist;
            forces[i].1 += force * dy / dist;
            forces[j].0 -= force * dx / dist;
            forces[j].1 -= force * dy / dist;
        }
    }
    positions
        .iter()
        .zip(forces.iter())
        .map(|(&(x, y), &(fx, fy))| (x + fx * 0.01, y + fy * 0.01))
        .collect()
}

pub fn wave_pinning(
    u: &mut [f64],
    v: &mut [f64],
    d: f64,
    k_on: f64,
    k_off: f64,
    k_fb: f64,
    hill_n: f64,
    dx: f64,
    dt: f64,
    steps: usize,
) {
    let n = u.len();
    for _ in 0..steps {
        let u_old = u.to_vec();
        let v_old = v.to_vec();
        for i in 1..n - 1 {
            let lap_v = d * (v_old[i + 1] - 2.0 * v_old[i] + v_old[i - 1]) / (dx * dx);
            let activation = (k_on + k_fb * u_old[i].powf(hill_n)) * v_old[i];
            let deactivation = k_off * u_old[i];
            u[i] = (u_old[i] + (activation - deactivation) * dt).max(0.0);
            v[i] = (v_old[i] + (lap_v - activation + deactivation) * dt).max(0.0);
        }
    }
}
