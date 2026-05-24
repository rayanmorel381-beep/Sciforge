use crate::components::powertrain::engines::turbines::assemblies::{
    fuel::AvFuel,
    propeller::{free_turbine, turboprop::Turboprop},
};

#[derive(Debug, Clone)]
pub struct PropulsionKit {
    pub engines: Vec<Turboprop>,
    pub fuel: AvFuel,
}

pub fn surveillance() -> PropulsionKit {
    PropulsionKit {
        engines: vec![free_turbine::single_turbine(110.0, 1.68)],
        fuel: AvFuel::Jp8,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![surveillance()]
}
