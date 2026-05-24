#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HubKind {
    LiveAxle,
    IndependentDriven,
    UnitBearing,
    QuickReleaseRacing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WheelBoltPattern {
    Stud4x100,
    Stud4x108,
    Stud5x100,
    Stud5x108,
    Stud5x112,
    Stud5x114_3,
    Stud5x120,
    Stud6x114_3,
    Stud6x139_7,
    CenterLockSingleNut,
}

#[derive(Debug, Clone, Copy)]
pub struct WheelHub {
    pub kind: HubKind,
    pub bolt_pattern: WheelBoltPattern,
    pub bearing_outer_diameter_mm: f64,
    pub bearing_inner_diameter_mm: f64,
    pub flange_diameter_mm: f64,
    pub max_axial_load_n: f64,
    pub max_radial_load_n: f64,
    pub max_torque_nm: f64,
    pub mass_kg: f64,
}

fn bolt_pattern_pcd_mm(pat: WheelBoltPattern) -> f64 {
    match pat {
        WheelBoltPattern::Stud4x100 => 100.0,
        WheelBoltPattern::Stud4x108 => 108.0,
        WheelBoltPattern::Stud5x100 => 100.0,
        WheelBoltPattern::Stud5x108 => 108.0,
        WheelBoltPattern::Stud5x112 => 112.0,
        WheelBoltPattern::Stud5x114_3 => 114.3,
        WheelBoltPattern::Stud5x120 => 120.0,
        WheelBoltPattern::Stud6x114_3 => 114.3,
        WheelBoltPattern::Stud6x139_7 => 139.7,
        WheelBoltPattern::CenterLockSingleNut => 0.0,
    }
}

pub fn assemble(
    kind: HubKind,
    bolt_pattern: WheelBoltPattern,
    max_axial_load_n: f64,
    max_radial_load_n: f64,
    max_torque_nm: f64,
) -> WheelHub {
    let pcd_mm = bolt_pattern_pcd_mm(bolt_pattern);
    let load_factor = (max_radial_load_n / 5000.0).max(1.0).sqrt();
    let bearing_outer_mm = (72.0 * load_factor).clamp(60.0, 140.0);
    let bearing_inner_mm = bearing_outer_mm * 0.55;
    let flange_diameter_mm = pcd_mm.max(80.0) + 35.0;
    let mass_kg = match kind {
        HubKind::LiveAxle => 4.0 + load_factor * 1.2,
        HubKind::IndependentDriven => 2.8 + load_factor * 0.9,
        HubKind::UnitBearing => 3.2 + load_factor * 1.0,
        HubKind::QuickReleaseRacing => 2.2 + load_factor * 0.7,
    };
    WheelHub {
        kind,
        bolt_pattern,
        bearing_outer_diameter_mm: (bearing_outer_mm * 10.0).round() / 10.0,
        bearing_inner_diameter_mm: (bearing_inner_mm * 10.0).round() / 10.0,
        flange_diameter_mm: (flange_diameter_mm * 10.0).round() / 10.0,
        max_axial_load_n,
        max_radial_load_n,
        max_torque_nm,
        mass_kg: (mass_kg * 100.0).round() / 100.0,
    }
}
