pub fn poiseuille_flow(delta_p: f64, radius: f64, length: f64, viscosity: f64) -> f64 {
    let pi = core::f64::consts::PI;
    pi * radius.powi(4) * delta_p / (8.0 * viscosity * length)
}

pub fn wall_shear_stress(flow_rate: f64, radius: f64, viscosity: f64) -> f64 {
    let pi = core::f64::consts::PI;
    4.0 * viscosity * flow_rate / (pi * radius.powi(3))
}

pub fn reynolds_number(density: f64, velocity: f64, diameter: f64, viscosity: f64) -> f64 {
    density * velocity * diameter / viscosity
}

pub fn murrays_law_radius(parent_radius: f64, n_children: usize) -> f64 {
    parent_radius / (n_children as f64).powf(1.0 / 3.0)
}

pub fn windkessel_2element(
    p0: f64,
    r: f64,
    c: f64,
    flow: impl Fn(f64) -> f64,
    dt: f64,
    steps: usize,
) -> Vec<f64> {
    let mut pressure = Vec::with_capacity(steps + 1);
    let mut p = p0;
    pressure.push(p);
    for step in 0..steps {
        let t = step as f64 * dt;
        let q = flow(t);
        let dp = (q - p / r) / c;
        p += dp * dt;
        pressure.push(p);
    }
    pressure
}

pub fn pulse_wave_velocity(
    elastic_modulus: f64,
    wall_thickness: f64,
    radius: f64,
    density: f64,
) -> f64 {
    (elastic_modulus * wall_thickness / (2.0 * radius * density)).sqrt()
}

pub fn casson_viscosity(tau_y: f64, eta_inf: f64, shear_rate: f64) -> f64 {
    let sqrt_tau = tau_y.sqrt() + (eta_inf * shear_rate).sqrt();
    let tau = sqrt_tau * sqrt_tau;
    if shear_rate < 1e-30 {
        return f64::INFINITY;
    }
    tau / shear_rate
}

pub fn oxygen_dissociation_hill(po2: f64, p50: f64, n: f64) -> f64 {
    let x = (po2 / p50).powf(n);
    x / (1.0 + x)
}

pub fn cardiac_output(stroke_volume: f64, heart_rate: f64) -> f64 {
    stroke_volume * heart_rate
}

pub fn mean_arterial_pressure(systolic: f64, diastolic: f64) -> f64 {
    diastolic + (systolic - diastolic) / 3.0
}

pub fn total_peripheral_resistance(map: f64, cvp: f64, cardiac_output: f64) -> f64 {
    (map - cvp) / cardiac_output.max(1e-30)
}

pub fn womersley_number(radius: f64, angular_freq: f64, kinematic_viscosity: f64) -> f64 {
    radius * (angular_freq / kinematic_viscosity).sqrt()
}

pub fn fahraeus_lindqvist(viscosity_plasma: f64, hematocrit: f64, diameter_um: f64) -> f64 {
    let d = diameter_um;
    let hd = hematocrit;
    viscosity_plasma * (1.0 + (hd / (1.0 - hd)) * (d / (d + 4.29)))
}

pub fn compliance(delta_v: f64, delta_p: f64) -> f64 {
    delta_v / delta_p.max(1e-30)
}

pub fn laplace_law_sphere(pressure: f64, radius: f64, wall_thickness: f64) -> f64 {
    pressure * radius / (2.0 * wall_thickness)
}

pub fn laplace_law_cylinder(pressure: f64, radius: f64, wall_thickness: f64) -> f64 {
    pressure * radius / wall_thickness
}

pub fn bernoulli_velocity(delta_p: f64, density: f64) -> f64 {
    (2.0 * delta_p / density).sqrt()
}

pub fn systemic_vascular_resistance(map: f64, rap: f64, co: f64) -> f64 {
    (map - rap) / co.max(1e-30) * 80.0
}

pub fn ejection_fraction(edv: f64, esv: f64) -> f64 {
    (edv - esv) / edv.max(1e-30)
}
