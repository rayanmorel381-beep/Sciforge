use sciforge_core::materials::Material;
use sciforge_core::materials::irons::stainless::STAINLESS_316;
use sciforge_core::materials::irons::steels::STEEL_4140;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InjectionType {
    Carbureted,
    PortInjection,
    DirectInjection,
    MultiPointSequential,
    DualInjection,
}

#[derive(Debug, Clone)]
pub struct FuelSystem {
    pub injection_type: InjectionType,
    pub injector_count: u8,
    pub injector_flow_cc_min: f64,
    pub rail_pressure_bar: f64,
    pub pump_flow_l_h: f64,
    pub returnless: bool,
}

impl FuelSystem {
    pub fn rail_material(&self) -> &'static Material {
        if self.rail_pressure_bar >= 100.0 {
            &STEEL_4140
        } else {
            &STAINLESS_316
        }
    }

    pub fn carbureted(cylinders: u8) -> Self {
        Self {
            injection_type: InjectionType::Carbureted,
            injector_count: cylinders / 4,
            injector_flow_cc_min: 0.0,
            rail_pressure_bar: 0.0,
            pump_flow_l_h: 80.0,
            returnless: false,
        }
    }

    pub fn port_injection(cylinders: u8, injector_flow_cc_min: f64) -> Self {
        Self {
            injection_type: InjectionType::MultiPointSequential,
            injector_count: cylinders,
            injector_flow_cc_min,
            rail_pressure_bar: 4.0,
            pump_flow_l_h: 180.0,
            returnless: true,
        }
    }

    pub fn direct_injection(cylinders: u8, injector_flow_cc_min: f64) -> Self {
        Self {
            injection_type: InjectionType::DirectInjection,
            injector_count: cylinders,
            injector_flow_cc_min,
            rail_pressure_bar: 200.0,
            pump_flow_l_h: 250.0,
            returnless: true,
        }
    }

    pub fn dual_injection(cylinders: u8, primary_flow_cc_min: f64, secondary_flow_cc_min: f64) -> Self {
        Self {
            injection_type: InjectionType::DualInjection,
            injector_count: cylinders * 2,
            injector_flow_cc_min: (primary_flow_cc_min + secondary_flow_cc_min) * 0.5,
            rail_pressure_bar: 200.0,
            pump_flow_l_h: 300.0,
            returnless: true,
        }
    }
}
