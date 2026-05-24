pub fn action_potential_shape(
    t: f64,
    v_rest: f64,
    v_peak: f64,
    tau_rise: f64,
    tau_fall: f64,
) -> f64 {
    let rise = 1.0 - (-t / tau_rise).exp();
    let fall = (-t / tau_fall).exp();
    v_rest + (v_peak - v_rest) * rise * fall
}

pub fn cable_equation_steady(v0: f64, x: f64, lambda: f64) -> f64 {
    v0 * (-x.abs() / lambda).exp()
}

pub fn electrotonic_length(physical_length: f64, space_constant: f64) -> f64 {
    physical_length / space_constant
}

pub fn input_resistance_cylinder(rm: f64, ri: f64, diameter: f64) -> f64 {
    let radius = diameter / 2.0;
    let lambda = (rm * radius / (2.0 * ri)).sqrt();
    rm / (std::f64::consts::PI * diameter * lambda)
}

pub fn strength_duration_weiss(rheobase: f64, chronaxie: f64, duration: f64) -> f64 {
    rheobase * (1.0 + chronaxie / duration)
}

pub fn strength_duration_lapicque(rheobase: f64, chronaxie: f64, duration: f64) -> f64 {
    rheobase / (1.0 - (-duration / chronaxie).exp())
}

pub fn local_field_potential(currents: &[f64], distances: &[f64], sigma: f64) -> f64 {
    let n = currents.len().min(distances.len());
    let mut phi = 0.0;
    for i in 0..n {
        phi += currents[i] / (4.0 * std::f64::consts::PI * sigma * distances[i].max(1e-10));
    }
    phi
}

pub fn extracellular_spike_amplitude(transmembrane_current: f64, distance: f64, sigma: f64) -> f64 {
    transmembrane_current / (4.0 * std::f64::consts::PI * sigma * distance.max(1e-10))
}

pub fn impedance_tissue(resistance: f64, capacitance: f64, frequency: f64) -> f64 {
    let omega = 2.0 * std::f64::consts::PI * frequency;
    let xc = 1.0 / (omega * capacitance);
    (resistance * resistance + xc * xc).sqrt()
}

pub fn defibrillation_threshold(body_mass: f64, transthoracic_impedance: f64) -> f64 {
    let energy_per_kg = 2.0;
    energy_per_kg * body_mass * transthoracic_impedance / 50.0
}

pub fn bioimpedance_body_composition(
    impedance: f64,
    height: f64,
    weight: f64,
    age: f64,
    sex_factor: f64,
) -> f64 {
    sex_factor + 0.734 * height * height / impedance + 0.116 * weight - 0.096 * age
}
