#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TorsionBushingMaterial {
    Rubber,
    Polyurethane,
    Hydromount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TorsionBeamProfile {
    UProfile,
    CProfile,
    Closed,
}

#[derive(Debug, Clone)]
pub struct TorsionBeam {
    pub beam_profile: TorsionBeamProfile,
    pub bushing_material: TorsionBushingMaterial,
    pub beam_width_mm: f64,
    pub beam_thickness_mm: f64,
    pub trailing_arm_length_mm: f64,
    pub spring_rate_n_mm: f64,
    pub roll_stiffness_nm_deg: f64,
}

impl TorsionBeam {
    pub fn standard(beam_width_mm: f64, beam_thickness_mm: f64, trailing_arm_length_mm: f64) -> Self {
        Self {
            beam_profile: TorsionBeamProfile::UProfile,
            bushing_material: TorsionBushingMaterial::Rubber,
            beam_width_mm,
            beam_thickness_mm,
            trailing_arm_length_mm,
            spring_rate_n_mm: 18.0,
            roll_stiffness_nm_deg: 120.0,
        }
    }

    pub fn reinforced(beam_width_mm: f64, beam_thickness_mm: f64, trailing_arm_length_mm: f64) -> Self {
        Self {
            beam_profile: TorsionBeamProfile::Closed,
            bushing_material: TorsionBushingMaterial::Polyurethane,
            beam_width_mm,
            beam_thickness_mm,
            trailing_arm_length_mm,
            spring_rate_n_mm: 24.0,
            roll_stiffness_nm_deg: 180.0,
        }
    }
}
