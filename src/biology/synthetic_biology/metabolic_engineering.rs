pub fn flux_balance_objective(fluxes: &[f64], objective_coefficients: &[f64]) -> f64 {
    let n = fluxes.len().min(objective_coefficients.len());
    let mut z = 0.0;
    for i in 0..n {
        z += fluxes[i] * objective_coefficients[i];
    }
    z
}

pub fn theoretical_yield(
    substrate_carbons: usize,
    product_carbons: usize,
    pathway_efficiency: f64,
) -> f64 {
    (substrate_carbons as f64 / product_carbons.max(1) as f64) * pathway_efficiency
}

pub fn heterologous_metabolic_burden(
    heterologous_protein_fraction: f64,
    growth_rate_wt: f64,
) -> f64 {
    growth_rate_wt * (1.0 - heterologous_protein_fraction)
}

pub fn pathway_thermodynamic_driving_force(delta_g_steps: &[f64]) -> f64 {
    delta_g_steps.iter().cloned().fold(f64::INFINITY, f64::min)
}

pub fn enzyme_cost_minimization(fluxes: &[f64], kcats: &[f64]) -> f64 {
    let n = fluxes.len().min(kcats.len());
    let mut cost = 0.0;
    for i in 0..n {
        cost += fluxes[i].abs() / kcats[i].max(1e-30);
    }
    cost
}

pub fn cofactor_balance(nadh_produced: f64, nadh_consumed: f64) -> f64 {
    nadh_produced - nadh_consumed
}

pub fn titer_rate_yield(titer: f64, time: f64, substrate_consumed: f64) -> (f64, f64) {
    let rate = titer / time.max(1e-30);
    let yield_val = titer / substrate_consumed.max(1e-30);
    (rate, yield_val)
}

pub fn gene_expression_burden(promoter_strength: f64, copy_number: f64, protein_size: f64) -> f64 {
    promoter_strength * copy_number * protein_size
}

pub fn heterologous_pathway_flux(enzyme_levels: &[f64], kms: &[f64], substrates: &[f64]) -> f64 {
    let n = enzyme_levels.len().min(kms.len()).min(substrates.len());
    if n == 0 {
        return 0.0;
    }
    let mut min_flux = f64::INFINITY;
    for i in 0..n {
        let flux = enzyme_levels[i] * substrates[i] / (kms[i] + substrates[i]);
        if flux < min_flux {
            min_flux = flux;
        }
    }
    min_flux
}
