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
            high_bypass::long_range(360.0, 10.0, 3.16),
            high_bypass::long_range(360.0, 10.0, 3.16),
        ],
        fuel: AvFuel::JetA1,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![twin()]
}
