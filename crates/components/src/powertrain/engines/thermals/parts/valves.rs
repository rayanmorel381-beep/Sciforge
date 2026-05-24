use sciforge_core::materials::Material;
use sciforge_core::materials::irons::stainless::STAINLESS_410;
use sciforge_core::materials::nickels::inconel::INCONEL_718;
use sciforge_core::materials::titaniums::alloys::TI6AL4V_GR5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VvtSystem {
    None,
    SingleVvt,
    DualVvt,
    ContinuousVvt,
}

#[derive(Debug, Clone)]
pub struct ValveTrain {
    pub intake_valve_count: u8,
    pub exhaust_valve_count: u8,
    pub valve_diameter_mm: f64,
    pub variable_timing: VvtSystem,
    pub lift_mm: f64,
}

impl ValveTrain {
    pub fn intake_valve_material(&self) -> &'static Material {
        if matches!(self.variable_timing, VvtSystem::ContinuousVvt | VvtSystem::DualVvt) {
            &TI6AL4V_GR5
        } else {
            &STAINLESS_410
        }
    }

    pub fn exhaust_valve_material(&self) -> &'static Material {
        &INCONEL_718
    }

    pub fn two_valve_ohv(cylinders: u8) -> Self {
        Self {
            intake_valve_count: cylinders,
            exhaust_valve_count: cylinders,
            valve_diameter_mm: 42.0,
            variable_timing: VvtSystem::None,
            lift_mm: 11.0,
        }
    }

    pub fn four_valve_dohc(cylinders: u8, vvt: VvtSystem) -> Self {
        Self {
            intake_valve_count: cylinders * 2,
            exhaust_valve_count: cylinders * 2,
            valve_diameter_mm: 34.0,
            variable_timing: vvt,
            lift_mm: 10.4,
        }
    }

    pub fn five_valve(cylinders: u8) -> Self {
        Self {
            intake_valve_count: cylinders * 3,
            exhaust_valve_count: cylinders * 2,
            valve_diameter_mm: 31.0,
            variable_timing: VvtSystem::None,
            lift_mm: 9.8,
        }
    }
}
