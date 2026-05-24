use super::fan::turbofan::{Turbofan, TurbofanGen};
use super::fuel::AvFuel;
use super::jet::turbojet::{Turbojet, TurbojetType};
use super::propeller::turboprop::{Turboprop, TurbopropGearbox};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurbineKind {
    Turbojet,
    Turbofan { bypass_ratio: f64, fan_pr: f64 },
    Turboprop { shaft_power_kw: f64 },
}

#[derive(Debug, Clone)]
pub enum HomemadeEngine {
    Jet(Turbojet),
    Fan(Turbofan),
    Prop(Turboprop),
}

#[derive(Debug, Clone)]
pub struct HomemadeTurbine {
    pub mass_flow_kg_s: f64,
    pub overall_pressure_ratio: f64,
    pub turbine_entry_temp_k: f64,
    pub fuel: AvFuel,
    pub kind: TurbineKind,
}

impl HomemadeTurbine {
    pub fn new(mass_flow_kg_s: f64, overall_pressure_ratio: f64, turbine_entry_temp_k: f64) -> Self {
        Self {
            mass_flow_kg_s,
            overall_pressure_ratio,
            turbine_entry_temp_k,
            fuel: AvFuel::JetA1,
            kind: TurbineKind::Turbojet,
        }
    }

    pub fn fuel(mut self, fuel: AvFuel) -> Self {
        self.fuel = fuel;
        self
    }

    pub fn as_turbofan(mut self, bypass_ratio: f64, fan_pr: f64) -> Self {
        self.kind = TurbineKind::Turbofan { bypass_ratio, fan_pr };
        self
    }

    pub fn as_turboprop(mut self, shaft_power_kw: f64) -> Self {
        self.kind = TurbineKind::Turboprop { shaft_power_kw };
        self
    }

    pub fn build(self) -> HomemadeEngine {
        let thrust_kn = self.mass_flow_kg_s * 1.1;
        let temp_c = self.turbine_entry_temp_k - 273.15;
        match self.kind {
            TurbineKind::Turbojet => HomemadeEngine::Jet(Turbojet {
                jet_type: TurbojetType::SingleSpool,
                thrust_kn,
                thrust_afterburner_kn: None,
                pressure_ratio: self.overall_pressure_ratio,
                turbine_inlet_temp_c: temp_c,
                mass_flow_kg_s: self.mass_flow_kg_s,
                sfc_kg_kn_h: 0.095,
            }),
            TurbineKind::Turbofan { bypass_ratio, fan_pr: _ } => HomemadeEngine::Fan(Turbofan {
                generation: TurbofanGen::HighBypass,
                thrust_kn,
                bypass_ratio,
                fan_diameter_m: (self.mass_flow_kg_s * (1.0 + bypass_ratio) / 200.0).sqrt(),
                overall_pressure_ratio: self.overall_pressure_ratio,
                turbine_entry_temp_c: temp_c,
                sfc_kg_kn_h: 0.055,
            }),
            TurbineKind::Turboprop { shaft_power_kw } => HomemadeEngine::Prop(Turboprop {
                shaft_power_kw,
                propeller_diameter_m: (shaft_power_kw / 500.0).sqrt(),
                gearbox_type: TurbopropGearbox::ReductionGear,
                gearbox_ratio: 14.5,
                power_turbine_stages: 2,
                sfc_kg_kw_h: 0.27,
                flat_rating_alt_m: 3_000,
            }),
        }
    }
}
