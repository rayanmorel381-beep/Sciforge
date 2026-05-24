use crate::components::powertrain::engines::turbines::assemblies::{
    fuel::AvFuel,
    propeller::{free_turbine, turboprop::Turboprop},
};

#[derive(Debug, Clone)]
pub struct PropulsionKit {
    pub engines: Vec<Turboprop>,
    pub fuel: AvFuel,
}

pub fn twin() -> PropulsionKit {
    PropulsionKit {
        engines: vec![
            free_turbine::twin_turbine(2_000.0, 3.6),
            free_turbine::twin_turbine(2_000.0, 3.6),
        ],
        fuel: AvFuel::JetA1,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![twin()]
}
