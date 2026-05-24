use sciforge_core::materials::Material;
use sciforge_core::materials::irons::nitrided::STEEL_NITRIDED_32CRMOV13;
use sciforge_core::materials::irons::steels::{STEEL_4340, STEEL_8620};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrankPin {
    Flat,
    Cross,
    SplitPin,
    Plane180,
    Plane90,
    Plane60,
    Irregular,
}

#[derive(Debug, Clone)]
pub struct Crankshaft {
    pub pin_layout: CrankPin,
    pub throw_count: u8,
    pub stroke_mm: f64,
    pub main_bearing_count: u8,
    pub forged: bool,
}

impl Crankshaft {
    pub fn material(&self) -> &'static Material {
        if self.throw_count >= 5 && self.forged {
            &STEEL_NITRIDED_32CRMOV13
        } else if self.forged {
            &STEEL_4340
        } else {
            &STEEL_8620
        }
    }

    pub fn inline(cylinders: u8, stroke_mm: f64) -> Self {
        let pin_layout = match cylinders {
            4 => CrankPin::Flat,
            6 => CrankPin::Plane180,
            _ => CrankPin::Irregular,
        };
        Self {
            pin_layout,
            throw_count: cylinders,
            stroke_mm,
            main_bearing_count: cylinders + 1,
            forged: false,
        }
    }

    pub fn v(cylinders: u8, bank_angle_deg: f64, stroke_mm: f64) -> Self {
        let pin_layout = if (bank_angle_deg - 90.0).abs() < 1.0 {
            CrankPin::Cross
        } else if (bank_angle_deg - 60.0).abs() < 1.0 || (bank_angle_deg - 120.0).abs() < 1.0 {
            CrankPin::Plane60
        } else {
            CrankPin::Irregular
        };
        Self {
            pin_layout,
            throw_count: cylinders / 2,
            stroke_mm,
            main_bearing_count: cylinders / 2 + 1,
            forged: cylinders >= 10,
        }
    }

    pub fn flat(cylinders: u8, stroke_mm: f64) -> Self {
        Self {
            pin_layout: CrankPin::Flat,
            throw_count: cylinders / 2,
            stroke_mm,
            main_bearing_count: cylinders / 2 + 1,
            forged: false,
        }
    }

    pub fn w(total_cylinders: u8, stroke_mm: f64) -> Self {
        Self {
            pin_layout: CrankPin::Irregular,
            throw_count: total_cylinders / 2,
            stroke_mm,
            main_bearing_count: (total_cylinders / 2) + 2,
            forged: true,
        }
    }

    pub fn radial(cylinders: u8, stroke_mm: f64) -> Self {
        Self {
            pin_layout: CrankPin::Plane180,
            throw_count: 1,
            stroke_mm,
            main_bearing_count: 3,
            forged: cylinders >= 9,
        }
    }
}
