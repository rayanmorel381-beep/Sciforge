use sciforge_core::moleculars::liquids::fuels::{JET_B, KEROSENE_JET_A1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvFuel {
    JetA,
    JetA1,
    JetB,
    Jp8,
    Jp4,
    AvGas100Ll,
}

impl AvFuel {
    pub fn density_kg_m3(&self) -> f64 {
        match self {
            Self::JetA | Self::JetA1 | Self::Jp8 => KEROSENE_JET_A1.density_kg_m3_ref,
            Self::JetB | Self::Jp4 => JET_B.density_kg_m3_ref,
            Self::AvGas100Ll => 720.0,
        }
    }

    pub fn flash_point_k(&self) -> f64 {
        match self {
            Self::JetA | Self::JetA1 | Self::Jp8 => KEROSENE_JET_A1.flash_point_k,
            Self::JetB | Self::Jp4 => JET_B.flash_point_k,
            Self::AvGas100Ll => 258.0,
        }
    }

    pub fn pour_point_k(&self) -> f64 {
        match self {
            Self::JetA | Self::JetA1 | Self::Jp8 => KEROSENE_JET_A1.pour_point_k,
            Self::JetB | Self::Jp4 => JET_B.pour_point_k,
            Self::AvGas100Ll => 213.15,
        }
    }

    pub fn lhv_j_per_kg(&self) -> f64 {
        match self {
            Self::JetA | Self::JetA1 | Self::Jp8 => 43_200_000.0,
            Self::JetB | Self::Jp4 => 43_500_000.0,
            Self::AvGas100Ll => 43_700_000.0,
        }
    }

    pub fn stoich_afr(&self) -> f64 {
        match self {
            Self::JetA | Self::JetA1 | Self::Jp8 | Self::JetB | Self::Jp4 => 14.7,
            Self::AvGas100Ll => 15.0,
        }
    }

    pub fn fuel_flow_kg_s(&self, thrust_kn: f64, sfc_kg_kn_h: f64) -> f64 {
        thrust_kn * sfc_kg_kn_h / 3_600.0
    }
}
