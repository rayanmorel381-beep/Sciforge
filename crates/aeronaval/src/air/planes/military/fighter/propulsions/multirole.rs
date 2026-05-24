use crate::components::powertrain::engines::turbines::assemblies::{
    fuel::AvFuel,
    jet::{postcombustion, turbojet::Turbojet},
};

#[derive(Debug, Clone)]
pub struct PropulsionKit {
    pub engines: Vec<Turbojet>,
    pub fuel: AvFuel,
}

pub fn twin() -> PropulsionKit {
    PropulsionKit {
        engines: vec![
            postcombustion::fighter(75.0, 115.0, 28.0),
            postcombustion::fighter(75.0, 115.0, 28.0),
        ],
        fuel: AvFuel::JetA,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![twin()]
}
