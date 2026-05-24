use sciforge_hub::prelude::physics_scalar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropshaftKind {
    SinglePiece,
    TwoPieceWithCenterBearing,
    Carbon,
}

#[derive(Debug, Clone, Copy)]
pub struct Propshaft {
    pub kind: PropshaftKind,
    pub length_mm: f64,
    pub outer_diameter_mm: f64,
    pub wall_thickness_mm: f64,
    pub mass_kg: f64,
    pub design_torque_nm: f64,
    pub max_speed_rpm: f64,
    pub critical_speed_rpm: f64,
    pub shear_stress_pa: f64,
    pub safety_margin: f64,
}

fn material_density_kg_m3(kind: PropshaftKind) -> f64 {
    match kind {
        PropshaftKind::SinglePiece | PropshaftKind::TwoPieceWithCenterBearing => 7850.0,
        PropshaftKind::Carbon => 1600.0,
    }
}

fn material_shear_allow_pa(kind: PropshaftKind) -> f64 {
    match kind {
        PropshaftKind::SinglePiece | PropshaftKind::TwoPieceWithCenterBearing => 250.0e6 / 3.0_f64.sqrt() / 2.0,
        PropshaftKind::Carbon => 350.0e6 / 3.0_f64.sqrt() / 2.0,
    }
}

fn material_young_modulus_pa(kind: PropshaftKind) -> f64 {
    match kind {
        PropshaftKind::SinglePiece | PropshaftKind::TwoPieceWithCenterBearing => 210.0e9,
        PropshaftKind::Carbon => 135.0e9,
    }
}

pub fn assemble(kind: PropshaftKind, length_mm: f64, design_torque_nm: f64, max_speed_rpm: f64) -> Propshaft {
    let length_m = length_mm / 1000.0;
    let tau_allow = material_shear_allow_pa(kind);
    let outer_d_mm = (60.0 + (design_torque_nm / 50.0).sqrt() * 8.0).clamp(50.0, 110.0);
    let outer_d_m = outer_d_mm / 1000.0;
    let r_outer_m = outer_d_m / 2.0;
    let wall_m_required = design_torque_nm / (2.0 * std::f64::consts::PI * r_outer_m.powi(2) * tau_allow);
    let wall_m = wall_m_required.max(0.0018);
    let wall_mm = wall_m * 1000.0;
    let inner_m = outer_d_m - 2.0 * wall_m;
    let r_inner_m = inner_m / 2.0;
    let area_m2 = std::f64::consts::PI * (r_outer_m.powi(2) - r_inner_m.powi(2));
    let i_m4 = std::f64::consts::PI * (r_outer_m.powi(4) - r_inner_m.powi(4)) / 4.0;
    let j_m4 = 2.0 * i_m4;
    let shear_stress_pa = design_torque_nm * r_outer_m / j_m4;
    let safety_margin = if shear_stress_pa > 0.0 { tau_allow / shear_stress_pa } else { f64::INFINITY };
    let mass_kg = area_m2 * length_m * material_density_kg_m3(kind);
    let young = material_young_modulus_pa(kind);
    let rho_lin = area_m2 * material_density_kg_m3(kind);
    let omega_crit = (std::f64::consts::PI / length_m).powi(2) * (young * i_m4 / rho_lin).sqrt();
    let critical_speed_rpm = physics_scalar(
        "rad_s_to_rpm",
        &["omega"],
        &[omega_crit],
    )
        .unwrap_or(omega_crit * 60.0 / (2.0 * std::f64::consts::PI));
    Propshaft {
        kind,
        length_mm: (length_mm * 10.0).round() / 10.0,
        outer_diameter_mm: (outer_d_mm * 10.0).round() / 10.0,
        wall_thickness_mm: (wall_mm * 100.0).round() / 100.0,
        mass_kg: (mass_kg * 100.0).round() / 100.0,
        design_torque_nm: (design_torque_nm * 10.0).round() / 10.0,
        max_speed_rpm: (max_speed_rpm * 10.0).round() / 10.0,
        critical_speed_rpm: (critical_speed_rpm * 10.0).round() / 10.0,
        shear_stress_pa,
        safety_margin: (safety_margin * 100.0).round() / 100.0,
    }
}
