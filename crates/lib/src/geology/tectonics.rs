pub fn plate_velocity(distance: f64, time: f64) -> f64 {
    distance / time
}

pub fn euler_pole_velocity(omega: f64, radius: f64, colatitude: f64) -> f64 {
    omega * radius * colatitude.sin()
}

pub fn isostatic_equilibrium(rho_crust: f64, thickness: f64, rho_mantle: f64) -> f64 {
    thickness * (1.0 - rho_crust / rho_mantle)
}

pub fn pratt_isostasy(rho_ref: f64, d_ref: f64, elevation: f64) -> f64 {
    rho_ref * d_ref / (d_ref + elevation)
}

pub fn airy_root(elevation: f64, rho_crust: f64, rho_mantle: f64) -> f64 {
    elevation * rho_crust / (rho_mantle - rho_crust)
}

pub fn thermal_subsidence(e0: f64, t: f64, tau: f64) -> f64 {
    e0 * (1.0 - (-t / tau).exp())
}

pub fn mckenzie_stretching(beta: f64, rho_m: f64, rho_c: f64, alpha: f64, tl: f64, tc: f64) -> f64 {
    tc * (rho_m * alpha * tl / (2.0 * (rho_m - rho_c))) * (1.0 - 1.0 / beta)
}

pub fn heat_flow(k: f64, dt_dz: f64) -> f64 {
    -k * dt_dz
}

pub fn geothermal_gradient(surface_temp: f64, depth: f64, gradient: f64) -> f64 {
    surface_temp + gradient * depth
}

pub fn flexural_rigidity(e: f64, te: f64, nu: f64) -> f64 {
    e * te.powi(3) / (12.0 * (1.0 - nu * nu))
}

pub fn elastic_thickness_from_rigidity(d: f64, e: f64, nu: f64) -> f64 {
    (12.0 * d * (1.0 - nu * nu) / e).powf(1.0 / 3.0)
}
