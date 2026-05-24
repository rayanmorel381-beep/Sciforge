use sciforge_core::materials::Material;
use sciforge_core::materials::irons::cast_iron::CAST_IRON_NODULAR_SG;
use sciforge_core::materials::irons::nitrided::STEEL_NITRIDED_32CRMOV13;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CamshaftLayout {
    Ohv,
    Sohc,
    Dohc,
}

#[derive(Debug, Clone)]
pub struct Camshaft {
    pub layout: CamshaftLayout,
    pub lobe_count: u16,
    pub lift_mm: f64,
    pub duration_deg: f64,
    pub variable_timing: bool,
    pub variable_lift: bool,
}

impl Camshaft {
    pub fn material(&self) -> &'static Material {
        if self.variable_lift || self.lift_mm >= 11.0 {
            &STEEL_NITRIDED_32CRMOV13
        } else {
            &CAST_IRON_NODULAR_SG
        }
    }

    pub fn ohv(cylinders: u16) -> Self {
        Self {
            layout: CamshaftLayout::Ohv,
            lobe_count: cylinders * 2,
            lift_mm: 12.0,
            duration_deg: 228.0,
            variable_timing: false,
            variable_lift: false,
        }
    }

    pub fn sohc(cylinders: u16, vtec: bool) -> Self {
        Self {
            layout: CamshaftLayout::Sohc,
            lobe_count: cylinders * 2,
            lift_mm: 9.8,
            duration_deg: 244.0,
            variable_timing: vtec,
            variable_lift: vtec,
        }
    }

    pub fn dohc(cylinders: u16, vvti: bool) -> Self {
        Self {
            layout: CamshaftLayout::Dohc,
            lobe_count: cylinders * 4,
            lift_mm: 10.4,
            duration_deg: 252.0,
            variable_timing: vvti,
            variable_lift: false,
        }
    }
}
