pub fn perfusion_rate(flow_ml_min: f64, volume_ml: f64) -> f64 {
    flow_ml_min / volume_ml
}

pub fn shear_stress_parallel_plate(viscosity: f64, flow_rate: f64, width: f64, height: f64) -> f64 {
    6.0 * viscosity * flow_rate / (width * height * height)
}

pub fn oxygen_transfer_rate(kla: f64, c_star: f64, c_bulk: f64) -> f64 {
    kla * (c_star - c_bulk)
}

pub fn residence_time(volume_ml: f64, flow_rate_ml_min: f64) -> f64 {
    volume_ml / flow_rate_ml_min
}

pub fn spinner_flask_shear(viscosity: f64, rpm: f64, radius: f64) -> f64 {
    let omega = rpm * 2.0 * std::f64::consts::PI / 60.0;
    viscosity * omega * radius
}

pub fn hollow_fiber_mass_transfer(permeability: f64, surface_area: f64, delta_c: f64) -> f64 {
    permeability * surface_area * delta_c
}

pub fn bioreactor_seeding_efficiency(attached: f64, seeded: f64) -> f64 {
    attached / seeded
}

pub fn scaffold_porosity(void_volume: f64, total_volume: f64) -> f64 {
    void_volume / total_volume
}

pub fn degradation_rate_first_order(mass: f64, k_deg: f64, time: f64) -> f64 {
    mass * (-k_deg * time).exp()
}

pub fn mechanical_modulus_scaffold(stress: f64, strain: f64) -> f64 {
    if strain.abs() < 1e-15 {
        return 0.0;
    }
    stress / strain
}

pub fn cell_proliferation_on_scaffold(n0: f64, doubling_time: f64, elapsed: f64) -> f64 {
    n0 * 2.0_f64.powf(elapsed / doubling_time)
}
