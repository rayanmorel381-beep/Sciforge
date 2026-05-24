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
            high_bypass::standard(185.0, 8.9, 2.82),
            high_bypass::standard(185.0, 8.9, 2.82),
            high_bypass::standard(185.0, 8.9, 2.82),
            high_bypass::standard(185.0, 8.9, 2.82),
        ],
        fuel: AvFuel::Jp8,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![quad()]
}
