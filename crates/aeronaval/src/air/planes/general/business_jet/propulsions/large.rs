use crate::components::powertrain::engines::turbines::assemblies::{
    fan::{high_bypass, turbofan::Turbofan},
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
            high_bypass::standard(60.0, 8.5, 1.70),
            high_bypass::standard(60.0, 8.5, 1.70),
        ],
        fuel: AvFuel::JetA1,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![twin()]
}
