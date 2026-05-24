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
            geared::next_gen(25.0, 4.0, 1.05),
            geared::next_gen(25.0, 4.0, 1.05),
        ],
        fuel: AvFuel::JetA1,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![twin()]
}
