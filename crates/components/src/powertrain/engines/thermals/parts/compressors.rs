use sciforge_core::materials::Material;
use sciforge_core::materials::alus::forged::AL_2618;
use sciforge_core::materials::irons::nitrided::STEEL_NITRIDED_32CRMOV13;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuperchargerType {
    Roots,
    TwinScrew,
    Centrifugal,
}

#[derive(Debug, Clone)]
pub struct Supercharger {
    pub supercharger_type: SuperchargerType,
    pub displacement_cc: u32,
    pub max_boost_bar: f64,
    pub drive_ratio: f64,
    pub intercooled: bool,
}

impl Supercharger {
    pub fn rotor_material(&self) -> &'static Material {
        match self.supercharger_type {
            SuperchargerType::Centrifugal => &AL_2618,
            _ => &STEEL_NITRIDED_32CRMOV13,
        }
    }

    pub fn roots(displacement_cc: u32) -> Self {
        Self {
            supercharger_type: SuperchargerType::Roots,
            displacement_cc,
            max_boost_bar: 0.7,
            drive_ratio: 1.5,
            intercooled: false,
        }
    }

    pub fn twin_screw(displacement_cc: u32, intercooled: bool) -> Self {
        Self {
            supercharger_type: SuperchargerType::TwinScrew,
            displacement_cc,
            max_boost_bar: 1.0,
            drive_ratio: 2.1,
            intercooled,
        }
    }

    pub fn centrifugal(displacement_cc: u32) -> Self {
        Self {
            supercharger_type: SuperchargerType::Centrifugal,
            displacement_cc,
            max_boost_bar: 0.9,
            drive_ratio: 3.5,
            intercooled: true,
        }
    }
}