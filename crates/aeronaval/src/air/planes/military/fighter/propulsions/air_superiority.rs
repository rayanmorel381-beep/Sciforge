use crate::components::powertrain::engines::turbines::assemblies::{
    fuel::AvFuel,
    jet::{postcombustion, turbojet::Turbojet},
};

#[derive(Debug, Clone)]
pub struct PropulsionKit {
    pub engines: Vec<Turbojet>,
    pub fuel: AvFuel,
}

pub fn single() -> PropulsionKit {
    PropulsionKit {
        engines: vec![postcombustion::fighter(80.0, 125.0, 25.0)],
        fuel: AvFuel::JetA,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![single()]
}
