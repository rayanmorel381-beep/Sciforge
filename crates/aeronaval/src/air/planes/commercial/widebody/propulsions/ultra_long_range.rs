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
            geared::next_gen(380.0, 11.0, 3.40),
            geared::next_gen(380.0, 11.0, 3.40),
        ],
        fuel: AvFuel::JetA,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![twin()]
}
