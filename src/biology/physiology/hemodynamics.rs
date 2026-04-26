pub fn poiseuille_flow(radius: f64, length: f64, pressure_drop: f64, viscosity: f64) -> f64 {
    std::f64::consts::PI * radius.powi(4) * pressure_drop / (8.0 * viscosity * length)
}

pub fn wall_shear_stress(viscosity: f64, flow_rate: f64, radius: f64) -> f64 {
    4.0 * viscosity * flow_rate / (std::f64::consts::PI * radius.powi(3))
}

pub fn mean_arterial_pressure(systolic: f64, diastolic: f64) -> f64 {
    diastolic + (systolic - diastolic) / 3.0
}

pub fn cardiac_output(stroke_volume: f64, heart_rate: f64) -> f64 {
    stroke_volume * heart_rate
}

pub fn total_peripheral_resistance(map: f64, cvp: f64, cardiac_output: f64) -> f64 {
    (map - cvp) / cardiac_output
}

pub fn frank_starling(preload: f64, k: f64, max_force: f64) -> f64 {
    max_force * (1.0 - (-k * preload).exp())
}

pub fn pulse_wave_velocity(elasticity: f64, wall_thickness: f64, radius: f64, density: f64) -> f64 {
    (elasticity * wall_thickness / (2.0 * radius * density)).sqrt()
}

pub fn windkessel_2(flow: f64, pressure: f64, resistance: f64, compliance: f64) -> f64 {
    (flow - pressure / resistance) / compliance
}
