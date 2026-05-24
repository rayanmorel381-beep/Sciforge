use crate::components::powertrain::engines::turbines::assemblies::{
    fan::{high_bypass, turbofan::Turbofan},
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
            high_bypass::standard(75.0, 8.0, 1.50),
            high_bypass::standard(75.0, 8.0, 1.50),
            high_bypass::standard(75.0, 8.0, 1.50),
            high_bypass::standard(75.0, 8.0, 1.50),
        ],
        fuel: AvFuel::Jp8,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![quad()]
}
