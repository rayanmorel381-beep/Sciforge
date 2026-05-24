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
            high_bypass::long_range(250.0, 9.0, 2.95),
            high_bypass::long_range(250.0, 9.0, 2.95),
            high_bypass::long_range(250.0, 9.0, 2.95),
            high_bypass::long_range(250.0, 9.0, 2.95),
        ],
        fuel: AvFuel::JetA1,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![quad()]
}
