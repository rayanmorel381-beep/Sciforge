pub fn xylem_flow_hagen_poiseuille(
    radius: f64,
    pressure_gradient: f64,
    viscosity: f64,
    length: f64,
) -> f64 {
    std::f64::consts::PI * radius.powi(4) * pressure_gradient / (8.0 * viscosity * length)
}

pub fn leaf_transpiration_rate(stomatal_conductance: f64, vpd: f64) -> f64 {
    stomatal_conductance * vpd
}

pub fn cohesion_tension_water_potential(
    osmotic: f64,
    pressure: f64,
    gravity: f64,
    matric: f64,
) -> f64 {
    osmotic + pressure + gravity + matric
}

pub fn phloem_munch_flow(turgor_source: f64, turgor_sink: f64, resistance: f64) -> f64 {
    (turgor_source - turgor_sink) / resistance.max(1e-30)
}

pub fn root_water_uptake(
    soil_water_potential: f64,
    root_water_potential: f64,
    root_conductance: f64,
) -> f64 {
    root_conductance * (soil_water_potential - root_water_potential)
}

pub fn xylem_cavitation_vulnerability(water_potential: f64, p50: f64, slope: f64) -> f64 {
    1.0 / (1.0 + (slope * (water_potential - p50)).exp())
}

pub fn stomatal_conductance_ball_berry(
    assimilation: f64,
    humidity: f64,
    co2_surface: f64,
    g0: f64,
    g1: f64,
) -> f64 {
    g0 + g1 * assimilation * humidity / co2_surface.max(1e-30)
}

pub fn sugar_loading_rate(sucrose_conc: f64, vmax: f64, km: f64) -> f64 {
    vmax * sucrose_conc / (km + sucrose_conc)
}

pub fn root_hydraulic_conductivity(
    flow_rate: f64,
    root_surface_area: f64,
    pressure_difference: f64,
) -> f64 {
    flow_rate / (root_surface_area * pressure_difference).max(1e-30)
}

pub fn sap_flow_heat_pulse(
    thermal_diffusivity: f64,
    heat_pulse_distance: f64,
    time_to_max: f64,
) -> f64 {
    heat_pulse_distance * heat_pulse_distance / (4.0 * thermal_diffusivity * time_to_max).max(1e-30)
        - thermal_diffusivity / heat_pulse_distance
}
