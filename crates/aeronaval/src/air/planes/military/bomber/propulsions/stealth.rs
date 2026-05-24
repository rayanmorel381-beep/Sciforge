use crate::components::powertrain::engines::turbines::assemblies::{
    fan::{low_bypass, turbofan::Turbofan},
    fuel::AvFuel,
};

#[derive(Debug, Clone)]
pub struct PropulsionKit {
    pub engines: Vec<Turbofan>,
    pub fuel: AvFuel,
}

pub fn quad() -> PropulsionKit {
    PropulsionKit {
        engines: vec![
            low_bypass::military(85.0, 0.3),
            low_bypass::military(85.0, 0.3),
            low_bypass::military(85.0, 0.3),
            low_bypass::military(85.0, 0.3),
        ],
        fuel: AvFuel::Jp8,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![quad()]
}
