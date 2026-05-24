use sciforge_core::materials::Material;
use sciforge_core::materials::coppers::pure::COPPER_C110;
use sciforge_core::materials::irons::stainless::STAINLESS_304;
use sciforge_core::materials::plastics::elastomers::FKM_VITON;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadGasketType {
    Composite,
    Mls,
    Copper,
}

#[derive(Debug, Clone)]
pub struct HeadGasket {
    pub cylinders: u8,
    pub gasket_type: HeadGasketType,
    pub thickness_mm: f64,
    pub bore_diameter_mm: f64,
    pub fire_ring: bool,
    pub torque_yield: bool,
}

impl HeadGasket {
    pub fn facing_material(&self) -> &'static Material {
        match self.gasket_type {
            HeadGasketType::Composite => &FKM_VITON,
            HeadGasketType::Mls => &STAINLESS_304,
            HeadGasketType::Copper => &COPPER_C110,
        }
    }

    pub fn code(&self) -> &'static str {
        match self.gasket_type {
            HeadGasketType::Composite => "CMP",
            HeadGasketType::Mls => "MLS",
            HeadGasketType::Copper => "CPR",
        }
    }

    pub fn seal_factor(&self, boost_bar: f64) -> f64 {
        let base = match self.gasket_type {
            HeadGasketType::Composite => 0.90,
            HeadGasketType::Mls => 0.97,
            HeadGasketType::Copper => 0.95,
        };
        let ring_bonus = if self.fire_ring { 0.02 } else { 0.0 };
        let ty_bonus = if self.torque_yield { 0.01 } else { 0.0 };
        let boost_penalty = (boost_bar * 0.02).min(0.10);
        (base + ring_bonus + ty_bonus - boost_penalty).clamp(0.75, 0.99)
    }

    pub fn composite(cylinders: u8, bore_diameter_mm: f64) -> Self {
        Self {
            cylinders,
            gasket_type: HeadGasketType::Composite,
            thickness_mm: 1.4,
            bore_diameter_mm,
            fire_ring: true,
            torque_yield: false,
        }
    }

    pub fn mls(cylinders: u8, bore_diameter_mm: f64) -> Self {
        Self {
            cylinders,
            gasket_type: HeadGasketType::Mls,
            thickness_mm: 1.0,
            bore_diameter_mm,
            fire_ring: true,
            torque_yield: true,
        }
    }

    pub fn copper(cylinders: u8, bore_diameter_mm: f64) -> Self {
        Self {
            cylinders,
            gasket_type: HeadGasketType::Copper,
            thickness_mm: 0.8,
            bore_diameter_mm,
            fire_ring: true,
            torque_yield: true,
        }
    }
}
