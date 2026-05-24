pub mod acoustics;
pub mod runtime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutKind {
    Inline,
    Flat,
    V,
    Vr,
    W,
    Radial,
    H,
    X,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceLoudness {
    Muffled,
    Sporty,
    Aggressive,
    Race,
    BoxerRumble,
    CrossPlane,
    FlatPlane,
    VrBurble,
    W8Smooth,
    LuxuryHum,
    RadialDrone,
}

impl VoiceLoudness {
    pub fn base_db(self) -> f64 {
        match self {
            VoiceLoudness::Muffled => 79.0,
            VoiceLoudness::LuxuryHum => 81.0,
            VoiceLoudness::Sporty => 85.0,
            VoiceLoudness::W8Smooth => 86.0,
            VoiceLoudness::VrBurble => 87.0,
            VoiceLoudness::BoxerRumble => 88.0,
            VoiceLoudness::CrossPlane => 90.0,
            VoiceLoudness::Aggressive => 93.0,
            VoiceLoudness::FlatPlane => 96.0,
            VoiceLoudness::RadialDrone => 98.0,
            VoiceLoudness::Race => 103.0,
        }
    }

    pub fn liter_gain_db_per_l(self) -> f64 {
        match self {
            VoiceLoudness::Muffled | VoiceLoudness::LuxuryHum => 2.0,
            VoiceLoudness::Sporty | VoiceLoudness::W8Smooth | VoiceLoudness::VrBurble => 2.5,
            VoiceLoudness::BoxerRumble => 2.8,
            VoiceLoudness::CrossPlane | VoiceLoudness::Aggressive => 3.0,
            VoiceLoudness::FlatPlane | VoiceLoudness::RadialDrone => 3.2,
            VoiceLoudness::Race => 3.5,
        }
    }
}
