pub fn trophic_transfer_efficiency(production_upper: f64, production_lower: f64) -> f64 {
    production_upper / production_lower
}

pub fn fish_growth_von_bertalanffy(l_inf: f64, k: f64, t: f64, t0: f64) -> f64 {
    l_inf * (1.0 - (-k * (t - t0)).exp())
}

pub fn fish_mortality_total(natural: f64, fishing: f64) -> f64 {
    natural + fishing
}

pub fn maximum_sustainable_yield(r: f64, k: f64) -> f64 {
    r * k / 4.0
}

pub fn stock_recruitment_beverton_holt(alpha: f64, beta: f64, spawners: f64) -> f64 {
    alpha * spawners / (1.0 + beta * spawners)
}

pub fn stock_recruitment_ricker(alpha: f64, beta: f64, spawners: f64) -> f64 {
    alpha * spawners * (-beta * spawners).exp()
}

pub fn catch_per_unit_effort(catch: f64, effort: f64) -> f64 {
    catch / effort
}

pub fn decompression_no_stop_time(depth_m: f64, surface_rate: f64) -> f64 {
    let ambient_pressure = 1.0 + depth_m / 10.0;
    surface_rate / (ambient_pressure - 1.0).max(0.01)
}

pub fn schaefer_biomass(biomass: f64, r: f64, k: f64, catch: f64, dt: f64) -> f64 {
    let growth = r * biomass * (1.0 - biomass / k) - catch;
    (biomass + growth * dt).max(0.0)
}

pub fn fishing_mortality_from_effort(catchability: f64, effort: f64) -> f64 {
    catchability * effort
}

pub fn yield_per_recruit(f: f64, m: f64, w_inf: f64, k: f64, t_c: f64, t_r: f64, t0: f64) -> f64 {
    let z = f + m;
    let dt = t_c - t_r;
    let u = (-m * dt).exp();
    let summands: f64 = (0..4)
        .map(|n| {
            let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
            let coeff = [1.0, -3.0, 3.0, -1.0][n];
            let exp_factor = (-k * (t_c - t0) * n as f64).exp();
            sign * coeff * exp_factor / (z + n as f64 * k)
        })
        .sum();
    f * u * w_inf * summands
}

pub fn spawning_stock_biomass(numbers: &[f64], weights: &[f64], maturity: &[f64]) -> f64 {
    numbers
        .iter()
        .zip(weights.iter())
        .zip(maturity.iter())
        .map(|((&n, &w), &m)| n * w * m)
        .sum()
}

pub fn exploitation_rate(f: f64, z: f64) -> f64 {
    f / z * (1.0 - (-z).exp())
}

pub fn virtual_population_analysis(catch: f64, m: f64, f_rate: f64) -> f64 {
    let z = m + f_rate;
    catch * z / (f_rate * (1.0 - (-z).exp()))
}

pub fn length_weight_relation(length: f64, a: f64, b: f64) -> f64 {
    a * length.powf(b)
}

pub fn condition_factor_fulton(weight: f64, length: f64) -> f64 {
    100.0 * weight / length.powi(3)
}

pub fn surplus_production(biomass_t: f64, biomass_t1: f64, catch: f64) -> f64 {
    biomass_t1 - biomass_t + catch
}

pub fn fox_model_equilibrium_yield(f: f64, msy: f64, f_msy: f64) -> f64 {
    msy * (f / f_msy) * (1.0 - (f / f_msy - 1.0)).exp()
}

pub fn selectivity_logistic(length: f64, l50: f64, slope: f64) -> f64 {
    1.0 / (1.0 + (-slope * (length - l50)).exp())
}

pub fn discard_mortality(catch: f64, discard_fraction: f64, discard_survival: f64) -> f64 {
    catch * discard_fraction * (1.0 - discard_survival)
}
