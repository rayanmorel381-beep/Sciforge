pub fn trophic_cascade(
    levels: &[f64],
    growth_rates: &[f64],
    carrying_capacities: &[f64],
    interaction_strengths: &[f64],
    dt: f64,
    steps: usize,
) -> Vec<Vec<f64>> {
    let n = levels.len();
    let mut state = levels.to_vec();
    let mut history = Vec::with_capacity(steps + 1);
    history.push(state.clone());

    for _ in 0..steps {
        let mut d = vec![0.0; n];
        d[0] = growth_rates[0] * state[0] * (1.0 - state[0] / carrying_capacities[0]);
        if n > 1 {
            d[0] -= interaction_strengths[0] * state[0] * state[1];
        }
        for i in 1..n {
            let gain = interaction_strengths[i - 1] * state[i - 1] * state[i];
            let loss = growth_rates[i] * state[i];
            d[i] = gain - loss;
            if i + 1 < n {
                d[i] -= interaction_strengths[i] * state[i] * state[i + 1];
            }
        }
        for i in 0..n {
            state[i] += d[i] * dt;
            state[i] = state[i].max(0.0);
        }
        history.push(state.clone());
    }
    history
}

pub fn reaction_diffusion_1d(
    u: &mut [f64],
    v: &mut [f64],
    du: f64,
    dv: f64,
    f_coeff: f64,
    k: f64,
    dx: f64,
    dt: f64,
    steps: usize,
) {
    let n = u.len();
    for _ in 0..steps {
        let u_old: Vec<f64> = u.to_vec();
        let v_old: Vec<f64> = v.to_vec();
        for i in 1..n - 1 {
            let laplacian_u = (u_old[i + 1] - 2.0 * u_old[i] + u_old[i - 1]) / (dx * dx);
            let laplacian_v = (v_old[i + 1] - 2.0 * v_old[i] + v_old[i - 1]) / (dx * dx);
            let uv2 = u_old[i] * v_old[i] * v_old[i];
            u[i] += (du * laplacian_u - uv2 + f_coeff * (1.0 - u_old[i])) * dt;
            v[i] += (dv * laplacian_v + uv2 - (f_coeff + k) * v_old[i]) * dt;
        }
    }
}

pub fn species_area(c: f64, z: f64, area: f64) -> f64 {
    c * area.powf(z)
}

pub fn island_biogeography_equilibrium(immigration_rate: f64, extinction_rate: f64) -> f64 {
    immigration_rate / (immigration_rate + extinction_rate)
}

pub fn carrying_capacity_from_resources(resource: f64, consumption_per_capita: f64) -> f64 {
    resource / consumption_per_capita
}

pub fn succession_model(
    biomass: &[f64],
    growth_rates: &[f64],
    capacities: &[f64],
    competition: &[Vec<f64>],
    dt: f64,
    steps: usize,
) -> Vec<Vec<f64>> {
    let mut state = biomass.to_vec();
    let mut history = Vec::with_capacity(steps + 1);
    history.push(state.clone());
    for _ in 0..steps {
        let old = state.clone();
        for (i, si) in state.iter_mut().enumerate() {
            let mut comp = 0.0;
            for (j, &oj) in old.iter().enumerate() {
                comp += competition[i][j] * oj;
            }
            let dn = growth_rates[i] * old[i] * (1.0 - comp / capacities[i]);
            *si += dn * dt;
            *si = si.max(0.0);
        }
        history.push(state.clone());
    }
    history
}

pub fn dispersal_kernel_gaussian(distance: f64, sigma: f64) -> f64 {
    let two_pi = 2.0 * core::f64::consts::PI;
    (1.0 / (two_pi * sigma * sigma)) * (-distance * distance / (2.0 * sigma * sigma)).exp()
}
