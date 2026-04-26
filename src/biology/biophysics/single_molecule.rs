pub fn optical_trap_force(laser_power: f64, n_medium: f64, trap_efficiency: f64) -> f64 {
    trap_efficiency * n_medium * laser_power / 3e8
}

pub fn fret_efficiency(r: f64, r0: f64) -> f64 {
    let r0_6 = r0.powi(6);
    r0_6 / (r0_6 + r.powi(6))
}

pub fn fret_distance(efficiency: f64, r0: f64) -> f64 {
    if efficiency <= 0.0 || efficiency >= 1.0 {
        return 0.0;
    }
    r0 * ((1.0 / efficiency - 1.0).powf(1.0 / 6.0))
}

pub fn fluorescence_lifetime(quantum_yield: f64, radiative_rate: f64) -> f64 {
    quantum_yield / radiative_rate.max(1e-30)
}

pub fn photobleaching_rate(intensity: f64, cross_section: f64, quantum_yield_bleach: f64) -> f64 {
    intensity * cross_section * quantum_yield_bleach
}

pub fn fluorescence_recovery_half_time(beam_radius: f64, diffusion_coeff: f64) -> f64 {
    beam_radius * beam_radius / (4.0 * diffusion_coeff.max(1e-30))
}

pub fn single_molecule_diffusion_msd(d: f64, t: f64, localization_error: f64) -> f64 {
    4.0 * d * t + 4.0 * localization_error * localization_error
}

pub fn afm_cantilever_force(spring_constant: f64, deflection: f64) -> f64 {
    spring_constant * deflection
}

pub fn hertz_contact_indentation(
    force: f64,
    radius: f64,
    youngs_modulus: f64,
    poisson: f64,
) -> f64 {
    let e_star = youngs_modulus / (1.0 - poisson * poisson);
    (9.0 * force * force / (16.0 * radius * e_star * e_star)).cbrt()
}

pub fn micropipette_aspiration_tension(pressure: f64, pipette_radius: f64) -> f64 {
    pressure * pipette_radius / 2.0
}

pub fn youngs_modulus_from_hertz(
    force: f64,
    indentation: f64,
    tip_radius: f64,
    poisson: f64,
) -> f64 {
    3.0 * (1.0 - poisson * poisson) * force
        / (4.0 * tip_radius.sqrt() * indentation.powf(1.5)).max(1e-30)
}

pub fn traction_force(displacement: f64, substrate_stiffness: f64) -> f64 {
    displacement * substrate_stiffness
}
