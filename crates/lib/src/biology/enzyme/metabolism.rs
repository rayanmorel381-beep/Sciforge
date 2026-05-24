pub fn metabolic_network_steady_state(
    stoich: &[Vec<f64>],
    rates: impl Fn(&[f64]) -> Vec<f64>,
    initial: &[f64],
    dt: f64,
    steps: usize,
    tolerance: f64,
) -> Vec<f64> {
    let mut conc = initial.to_vec();
    for _ in 0..steps {
        let v = rates(&conc);
        let mut max_change = 0.0_f64;
        for (i, stoich_row) in stoich.iter().enumerate() {
            let mut dc = 0.0;
            for (j, &vj) in v.iter().enumerate() {
                dc += stoich_row[j] * vj;
            }
            conc[i] += dc * dt;
            conc[i] = conc[i].max(0.0);
            max_change = max_change.max(dc.abs() * dt);
        }
        if max_change < tolerance {
            break;
        }
    }
    conc
}

pub fn arrhenius(a: f64, ea: f64, t: f64) -> f64 {
    use crate::constants::K_B;
    a * (-ea / (K_B * t)).exp()
}

pub fn q10_factor(rate1: f64, rate2: f64, t1: f64, t2: f64) -> f64 {
    (rate2 / rate1).powf(10.0 / (t2 - t1))
}

pub fn metabolic_control_coefficient(
    flux_perturbed: f64,
    flux_original: f64,
    enzyme_perturbed: f64,
    enzyme_original: f64,
) -> f64 {
    let df = (flux_perturbed - flux_original) / flux_original;
    let de = (enzyme_perturbed - enzyme_original) / enzyme_original;
    if de.abs() < 1e-30 {
        return 0.0;
    }
    df / de
}

pub fn gibbs_free_energy(delta_g0: f64, t: f64, q: f64) -> f64 {
    use crate::constants::{K_B, N_A};
    let r = K_B * N_A;
    delta_g0 + r * t * q.ln()
}

pub fn mass_action_ratio(
    products: &[f64],
    reactants: &[f64],
    stoich_p: &[f64],
    stoich_r: &[f64],
) -> f64 {
    let num: f64 = products
        .iter()
        .zip(stoich_p.iter())
        .map(|(&c, &s)| c.powf(s))
        .product();
    let den: f64 = reactants
        .iter()
        .zip(stoich_r.iter())
        .map(|(&c, &s)| c.powf(s))
        .product();
    if den < 1e-30 {
        return f64::INFINITY;
    }
    num / den
}

pub fn reaction_quotient_vs_keq(q: f64, keq: f64) -> f64 {
    (keq / q.max(1e-30)).ln()
}

pub fn flux_control_summation(coefficients: &[f64]) -> f64 {
    coefficients.iter().sum()
}

pub fn elasticity_coefficient(
    rate: f64,
    metabolite: f64,
    delta_rate: f64,
    delta_metabolite: f64,
) -> f64 {
    if metabolite.abs() < 1e-30 || rate.abs() < 1e-30 {
        return 0.0;
    }
    (delta_rate / rate) / (delta_metabolite / metabolite)
}

pub fn supply_demand_modular(
    supply_flux: f64,
    demand_flux: f64,
    linking_metabolite: f64,
    elasticity_supply: f64,
    elasticity_demand: f64,
) -> f64 {
    let deviation = supply_flux - demand_flux;
    linking_metabolite + deviation / (elasticity_demand - elasticity_supply).abs().max(1e-30)
}
