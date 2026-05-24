pub fn stoichiometry_flux(stoichiometry: &[Vec<f64>], fluxes: &[f64]) -> Vec<f64> {
    let n_metabolites = stoichiometry.len();
    let mut rates = vec![0.0; n_metabolites];
    for (ri, stoich_row) in rates.iter_mut().zip(stoichiometry.iter()) {
        for (&fj, &sij) in fluxes.iter().zip(stoich_row.iter()) {
            *ri += sij * fj;
        }
    }
    rates
}

pub fn fba_objective(c: &[f64], fluxes: &[f64]) -> f64 {
    let mut obj = 0.0;
    for (&ci, &fi) in c.iter().zip(fluxes.iter()) {
        obj += ci * fi;
    }
    obj
}

pub fn flux_balance_feasibility(
    stoichiometry: &[Vec<f64>],
    fluxes: &[f64],
    tolerance: f64,
) -> bool {
    let rates = stoichiometry_flux(stoichiometry, fluxes);
    rates.iter().all(|&r| r.abs() < tolerance)
}

pub fn flux_variability(
    flux_nominal: f64,
    objective_fraction: f64,
    objective_optimal: f64,
    c_i: f64,
) -> (f64, f64) {
    let min_obj = objective_fraction * objective_optimal;
    let slack = (objective_optimal - min_obj) / c_i.abs().max(1e-30);
    (flux_nominal - slack, flux_nominal + slack)
}

pub fn metabolic_sensitivity(flux: f64, parameter: f64, delta_flux: f64, delta_param: f64) -> f64 {
    if parameter.abs() < 1e-30 || flux.abs() < 1e-30 {
        return 0.0;
    }
    (delta_flux / flux) / (delta_param / parameter)
}

pub fn dead_end_metabolites(stoichiometry: &[Vec<f64>]) -> Vec<usize> {
    let mut dead_ends = Vec::new();
    for (i, row) in stoichiometry.iter().enumerate() {
        let producers = row.iter().filter(|&&v| v > 0.0).count();
        let consumers = row.iter().filter(|&&v| v < 0.0).count();
        if producers == 0 || consumers == 0 {
            dead_ends.push(i);
        }
    }
    dead_ends
}

pub fn elementary_flux_mode_check(
    stoichiometry: &[Vec<f64>],
    mode: &[f64],
    tolerance: f64,
) -> bool {
    let rates = stoichiometry_flux(stoichiometry, mode);
    rates.iter().all(|&r| r.abs() < tolerance) && mode.iter().all(|&v| v >= -tolerance)
}

pub fn yield_coefficient(product_flux: f64, substrate_flux: f64) -> f64 {
    if substrate_flux.abs() < 1e-30 {
        return 0.0;
    }
    product_flux.abs() / substrate_flux.abs()
}

pub fn atp_yield(atp_production_flux: f64, glucose_uptake_flux: f64) -> f64 {
    if glucose_uptake_flux.abs() < 1e-30 {
        return 0.0;
    }
    atp_production_flux / glucose_uptake_flux.abs()
}

pub fn shadow_price(objective_change: f64, constraint_change: f64) -> f64 {
    if constraint_change.abs() < 1e-30 {
        return 0.0;
    }
    objective_change / constraint_change
}
