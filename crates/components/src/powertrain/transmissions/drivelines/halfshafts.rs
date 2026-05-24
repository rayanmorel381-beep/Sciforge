use sciforge_hub::prelude::physics_scalar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HalfshaftKind {
    SolidSteel,
    HollowSteel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CvJointKind {
    BallRzeppa,
    Tripod,
    DoubleOffset,
}

#[derive(Debug, Clone, Copy)]
pub struct CvJoint {
    pub kind: CvJointKind,
    pub diameter_mm: f64,
    pub max_angle_deg: f64,
    pub max_torque_nm: f64,
    pub mass_kg: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Halfshaft {
    pub kind: HalfshaftKind,
    pub length_mm: f64,
    pub outer_diameter_mm: f64,
    pub inner_diameter_mm: f64,
    pub mass_kg: f64,
    pub design_torque_nm: f64,
    pub shear_stress_pa: f64,
    pub safety_margin: f64,
    pub inner_joint: CvJoint,
    pub outer_joint: CvJoint,
}

fn cv_max_angle(kind: CvJointKind) -> f64 {
    match kind {
        CvJointKind::BallRzeppa => 47.0,
        CvJointKind::Tripod => 26.0,
        CvJointKind::DoubleOffset => 22.0,
    }
}

pub fn assemble_cv(kind: CvJointKind, design_torque_nm: f64) -> CvJoint {
    let diameter_mm = 70.0 + (design_torque_nm / 50.0).sqrt() * 6.0;
    let mass_kg = (diameter_mm / 1000.0).powi(3) * 7850.0 * 0.55;
    CvJoint {
        kind,
        diameter_mm: (diameter_mm * 10.0).round() / 10.0,
        max_angle_deg: cv_max_angle(kind),
        max_torque_nm: (design_torque_nm * 10.0).round() / 10.0,
        mass_kg: (mass_kg * 100.0).round() / 100.0,
    }
}

pub fn assemble(
    kind: HalfshaftKind,
    length_mm: f64,
    design_torque_nm: f64,
    inner_joint_kind: CvJointKind,
    outer_joint_kind: CvJointKind,
) -> Halfshaft {
    let length_m = length_mm / 1000.0;
    let tau_allow = 250.0e6 / 3.0_f64.sqrt() / 1.8;
    let outer_d_mm = (22.0 + (design_torque_nm / 50.0).sqrt() * 4.5).clamp(22.0, 45.0);
    let outer_d_m = outer_d_mm / 1000.0;
    let r_outer_m = outer_d_m / 2.0;
    let inner_d_mm = match kind {
        HalfshaftKind::SolidSteel => 0.0,
        HalfshaftKind::HollowSteel => outer_d_mm * 0.55,
    };
    let inner_d_m = inner_d_mm / 1000.0;
    let r_inner_m = inner_d_m / 2.0;
    let area_m2 = std::f64::consts::PI * (r_outer_m.powi(2) - r_inner_m.powi(2));
    let j_m4 = std::f64::consts::PI * (r_outer_m.powi(4) - r_inner_m.powi(4)) / 2.0;
    let shear_stress_pa = physics_scalar(
        "shaft_torsional_shear",
        &["t", "r", "j"],
        &[design_torque_nm, r_outer_m, j_m4],
    )
        .unwrap_or(design_torque_nm * r_outer_m / j_m4);
    let safety_margin = if shear_stress_pa > 0.0 { tau_allow / shear_stress_pa } else { f64::INFINITY };
    let mass_kg = area_m2 * length_m * 7850.0;
    Halfshaft {
        kind,
        length_mm: (length_mm * 10.0).round() / 10.0,
        outer_diameter_mm: (outer_d_mm * 10.0).round() / 10.0,
        inner_diameter_mm: (inner_d_mm * 10.0).round() / 10.0,
        mass_kg: (mass_kg * 100.0).round() / 100.0,
        design_torque_nm: (design_torque_nm * 10.0).round() / 10.0,
        shear_stress_pa,
        safety_margin: (safety_margin * 100.0).round() / 100.0,
        inner_joint: assemble_cv(inner_joint_kind, design_torque_nm),
        outer_joint: assemble_cv(outer_joint_kind, design_torque_nm),
    }
}
