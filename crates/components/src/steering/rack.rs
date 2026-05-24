#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RackType {
    Manual,
    HydraulicPower,
    ElectricPower,
    ElectroHydraulic,
    SteerByWire,
}

#[derive(Debug, Clone)]
pub struct SteeringRack {
    pub rack_type: RackType,
    pub ratio: f64,
    pub lock_to_lock_turns: f64,
    pub variable_ratio: bool,
}

impl SteeringRack {
    pub fn manual(ratio: f64) -> Self {
        Self { rack_type: RackType::Manual, ratio, lock_to_lock_turns: 3.5, variable_ratio: false }
    }

    pub fn electric_power(ratio: f64, variable_ratio: bool) -> Self {
        Self { rack_type: RackType::ElectricPower, ratio, lock_to_lock_turns: 2.8, variable_ratio }
    }

    pub fn hydraulic_power(ratio: f64) -> Self {
        Self { rack_type: RackType::HydraulicPower, ratio, lock_to_lock_turns: 3.0, variable_ratio: false }
    }

    pub fn steer_by_wire(ratio: f64) -> Self {
        Self { rack_type: RackType::SteerByWire, ratio, lock_to_lock_turns: 2.5, variable_ratio: true }
    }
}
