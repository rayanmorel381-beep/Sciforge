pub fn schmid_factor(angle_phi_rad: f64, angle_lambda_rad: f64) -> f64 {
    angle_phi_rad.cos() * angle_lambda_rad.cos()
}

pub fn resolved_shear_stress(applied_stress_pa: f64, schmid: f64) -> f64 {
    applied_stress_pa * schmid
}

pub fn taylor_factor_fcc() -> f64 {
    3.06
}

pub fn taylor_factor_bcc() -> f64 {
    2.75
}

pub fn polycrystal_yield_stress(critical_resolved_shear_pa: f64, taylor_factor: f64) -> f64 {
    taylor_factor * critical_resolved_shear_pa
}

pub fn power_law_slip_rate(
    tau_pa: f64,
    tau_critical_pa: f64,
    gamma_dot_0: f64,
    rate_sensitivity: f64,
) -> f64 {
    gamma_dot_0
        * (tau_pa.abs() / tau_critical_pa).powf(1.0 / rate_sensitivity)
        * tau_pa.signum()
}

pub fn voce_hardening(tau_0: f64, tau_sat: f64, h0: f64, gamma_acc: f64) -> f64 {
    tau_0 + (tau_sat - tau_0) * (1.0 - (-h0 * gamma_acc / (tau_sat - tau_0)).exp())
}

pub fn slip_systems_count(crystal: &str) -> u32 {
    match crystal {
        "fcc" => 12,
        "bcc" => 48,
        "hcp_basal" => 3,
        "hcp_prismatic" => 3,
        "hcp_pyramidal_a" => 6,
        "hcp_pyramidal_ca" => 12,
        _ => 0,
    }
}
