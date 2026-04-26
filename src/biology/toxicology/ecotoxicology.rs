pub fn ssd_rank(concentrations: &[f64], test_concentration: f64) -> f64 {
    if concentrations.is_empty() {
        return 0.0;
    }
    let below = concentrations
        .iter()
        .filter(|&&c| c <= test_concentration)
        .count();
    below as f64 / concentrations.len() as f64
}

pub fn bcf(concentration_organism: f64, concentration_water: f64) -> f64 {
    if concentration_water.abs() < 1e-15 {
        return 0.0;
    }
    concentration_organism / concentration_water
}

pub fn baf(concentration_organism: f64, concentration_environment: f64) -> f64 {
    if concentration_environment.abs() < 1e-15 {
        return 0.0;
    }
    concentration_organism / concentration_environment
}

pub fn bmf(concentration_predator: f64, concentration_prey: f64) -> f64 {
    if concentration_prey.abs() < 1e-15 {
        return 0.0;
    }
    concentration_predator / concentration_prey
}

pub fn lc50_probit(log_concentration: f64, slope: f64, intercept: f64) -> f64 {
    let probit = intercept + slope * log_concentration;
    0.5 * (1.0 + erf_approx((probit - 5.0) / std::f64::consts::SQRT_2))
}

fn erf_approx(x: f64) -> f64 {
    let a = 0.3480242;
    let b = -0.0958798;
    let c = 0.7478556;
    let t = 1.0 / (1.0 + 0.47047 * x.abs());
    let poly = t * (a + t * (b + t * c));
    let result = 1.0 - poly * (-x * x).exp();
    if x >= 0.0 { result } else { -result }
}

pub fn environmental_half_life(k_deg: f64) -> f64 {
    std::f64::consts::LN_2 / k_deg
}

pub fn fugacity_level_one(total_mass: f64, z_values: &[f64], volumes: &[f64]) -> f64 {
    if z_values.len() != volumes.len() {
        return 0.0;
    }
    let denominator: f64 = z_values
        .iter()
        .zip(volumes.iter())
        .map(|(z, v)| z * v)
        .sum();
    if denominator.abs() < 1e-15 {
        return 0.0;
    }
    total_mass / denominator
}

pub fn predicted_no_effect_concentration(ec50: f64, assessment_factor: f64) -> f64 {
    ec50 / assessment_factor
}

pub fn trophic_transfer_efficiency(assimilated: f64, ingested: f64) -> f64 {
    if ingested.abs() < 1e-15 {
        return 0.0;
    }
    assimilated / ingested
}

pub fn acute_toxic_unit(concentration: f64, lc50: f64) -> f64 {
    concentration / lc50
}
