pub fn linear_elastic_stress(modulus: f64, strain: f64) -> f64 {
    modulus * strain
}

pub fn kelvin_voigt(modulus: f64, viscosity: f64, strain: f64, strain_rate: f64) -> f64 {
    modulus * strain + viscosity * strain_rate
}

pub fn maxwell_stress_relaxation(sigma0: f64, modulus: f64, viscosity: f64, t: f64) -> f64 {
    sigma0 * (-modulus * t / viscosity).exp()
}

pub fn standard_linear_solid(
    e1: f64,
    e2: f64,
    eta: f64,
    strain: f64,
    strain_rate: f64,
    stress: f64,
) -> f64 {
    let tau = eta / e2;
    (e1 * strain + (e1 + e2) * eta * strain_rate / e2 - stress) / tau
}

pub fn hyperelastic_neo_hookean(c1: f64, lambda: f64) -> f64 {
    2.0 * c1 * (lambda - 1.0 / (lambda * lambda))
}

pub fn mooney_rivlin(c1: f64, c2: f64, lambda: f64) -> f64 {
    2.0 * (lambda - 1.0 / (lambda * lambda)) * (c1 + c2 / lambda)
}

pub fn poroelastic_consolidation(
    stress: f64,
    modulus: f64,
    permeability: f64,
    viscosity: f64,
    thickness: f64,
    t: f64,
) -> f64 {
    let cv = permeability * modulus / viscosity;
    let tau = thickness * thickness / cv;
    stress / modulus * (1.0 - (-t / tau).exp())
}

pub fn strain_energy_density_linear(modulus: f64, strain: f64) -> f64 {
    0.5 * modulus * strain * strain
}

pub fn creep_power_law(a: f64, sigma: f64, n: f64, t: f64) -> f64 {
    a * sigma.powf(n) * t
}

pub fn bone_density_wolff(
    rho0: f64,
    stimulus: f64,
    reference_stimulus: f64,
    rate: f64,
    dt: f64,
) -> f64 {
    let drho = rate * (stimulus - reference_stimulus);
    (rho0 + drho * dt).max(0.0)
}

pub fn ogden_model(mu: f64, alpha: f64, lambda: f64) -> f64 {
    mu * (lambda.powf(alpha - 1.0) - lambda.powf(-0.5 * alpha - 1.0))
}

pub fn fracture_toughness(force: f64, crack_length: f64, width: f64, thickness: f64) -> f64 {
    let y = 1.12;
    force * y / (width * thickness) * (std::f64::consts::PI * crack_length).sqrt()
}

pub fn viscoelastic_prony(moduli: &[f64], taus: &[f64], t: f64) -> f64 {
    moduli
        .iter()
        .zip(taus.iter())
        .map(|(&g, &tau)| g * (-t / tau).exp())
        .sum()
}

pub fn tissue_hydration_swelling(phi_0: f64, pi_ext: f64, bulk_modulus: f64) -> f64 {
    phi_0 * (1.0 + pi_ext / bulk_modulus)
}

pub fn biphasic_permeability(k0: f64, strain: f64, m: f64) -> f64 {
    k0 * (m * strain).exp()
}

pub fn stress_fiber_remodeling(sigma_old: f64, reference: f64, rate: f64, dt: f64) -> f64 {
    sigma_old + rate * (reference - sigma_old) * dt
}

pub fn damage_accumulation(d: f64, stress: f64, threshold: f64, rate: f64, dt: f64) -> f64 {
    if stress <= threshold {
        return d;
    }
    (d + rate * (stress - threshold) * dt).min(1.0)
}

pub fn elastic_modulus_density(rho: f64, c: f64, exponent: f64) -> f64 {
    c * rho.powf(exponent)
}
