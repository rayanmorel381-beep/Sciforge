pub fn competitive_inhibition(substrate: f64, inhibitor: f64, vmax: f64, km: f64, ki: f64) -> f64 {
    vmax * substrate / (km * (1.0 + inhibitor / ki) + substrate)
}

pub fn uncompetitive_inhibition(
    substrate: f64,
    inhibitor: f64,
    vmax: f64,
    km: f64,
    ki: f64,
) -> f64 {
    let alpha_prime = 1.0 + inhibitor / ki;
    vmax * substrate / (km + alpha_prime * substrate)
}

pub fn mixed_inhibition(
    substrate: f64,
    inhibitor: f64,
    vmax: f64,
    km: f64,
    ki: f64,
    ki_prime: f64,
) -> f64 {
    let alpha = 1.0 + inhibitor / ki;
    let alpha_prime = 1.0 + inhibitor / ki_prime;
    vmax * substrate / (alpha * km + alpha_prime * substrate)
}

pub fn noncompetitive_inhibition(
    substrate: f64,
    inhibitor: f64,
    vmax: f64,
    km: f64,
    ki: f64,
) -> f64 {
    let alpha = 1.0 + inhibitor / ki;
    vmax * substrate / (alpha * (km + substrate))
}

pub fn substrate_inhibition_velocity(substrate: f64, vmax: f64, km: f64, ksi: f64) -> f64 {
    vmax * substrate / (km + substrate + substrate * substrate / ksi)
}

pub fn irreversible_inhibition(active_enzyme: f64, inhibitor: f64, k_inact: f64, t: f64) -> f64 {
    active_enzyme * (-k_inact * inhibitor * t).exp()
}

pub fn tight_binding_inhibition(enzyme_total: f64, inhibitor_total: f64, ki_app: f64) -> f64 {
    let a = 1.0;
    let b = -(enzyme_total + inhibitor_total + ki_app);
    let c = enzyme_total * inhibitor_total;
    let disc = (b * b - 4.0 * a * c).max(0.0).sqrt();
    let ei = (-b - disc) / (2.0 * a);
    enzyme_total - ei
}

pub fn ic50_to_ki(ic50: f64, substrate: f64, km: f64) -> f64 {
    ic50 / (1.0 + substrate / km)
}

pub fn ki_to_ic50(ki: f64, substrate: f64, km: f64) -> f64 {
    ki * (1.0 + substrate / km)
}

pub fn cheng_prusoff_uncompetitive(ic50: f64, substrate: f64, km: f64) -> f64 {
    ic50 * (1.0 + substrate / km)
}

pub fn inhibition_constant_dixon(
    v_no_inhibitor: f64,
    v_with_inhibitor: f64,
    inhibitor: f64,
    substrate: f64,
    km: f64,
) -> f64 {
    let apparent_km = km * v_no_inhibitor / v_with_inhibitor.max(1e-30);
    inhibitor / ((apparent_km / km) - 1.0).max(1e-30) * (1.0 + substrate / km)
}
