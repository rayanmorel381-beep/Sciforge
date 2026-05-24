use crate::components::powertrain::engines::turbines::assemblies::{
    fan::{geared, turbofan::Turbofan},
    fuel::AvFuel,
};

#[derive(Debug, Clone)]
pub struct PropulsionKit {
    pub engines: Vec<Turbofan>,
    pub fuel: AvFuel,
}

pub fn twin() -> PropulsionKit {
    PropulsionKit {
        engines: vec![
            geared::regional(80.0, 10.0, 1.83),
            geared::regional(80.0, 10.0, 1.83),
        ],
        fuel: AvFuel::JetA1,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![twin()]
}
