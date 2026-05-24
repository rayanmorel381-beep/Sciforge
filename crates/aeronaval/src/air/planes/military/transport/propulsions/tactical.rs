use crate::components::powertrain::engines::turbines::assemblies::{
    fuel::AvFuel,
    propeller::{reduction_gear, turboprop::Turboprop},
};

#[derive(Debug, Clone)]
pub struct PropulsionKit {
    pub engines: Vec<Turboprop>,
    pub fuel: AvFuel,
}

pub fn quad() -> PropulsionKit {
    PropulsionKit {
        engines: vec![
            reduction_gear::standard(3_460.0, 4.11, 13.54),
            reduction_gear::standard(3_460.0, 4.11, 13.54),
            reduction_gear::standard(3_460.0, 4.11, 13.54),
            reduction_gear::standard(3_460.0, 4.11, 13.54),
        ],
        fuel: AvFuel::Jp8,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![quad()]
}
