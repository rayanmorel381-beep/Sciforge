use crate::components::powertrain::engines::turbines::assemblies::{
    fan::{geared, high_bypass, turbofan::Turbofan},
    fuel::AvFuel,
};

#[derive(Debug, Clone)]
pub struct PropulsionKit {
    pub engines: Vec<Turbofan>,
    pub fuel: AvFuel,
}

pub fn twin_high_bypass() -> PropulsionKit {
    PropulsionKit {
        engines: vec![
            high_bypass::long_range(140.0, 13.0, 1.90),
            high_bypass::long_range(140.0, 13.0, 1.90),
        ],
        fuel: AvFuel::JetA1,
    }
}

pub fn twin_geared() -> PropulsionKit {
    PropulsionKit {
        engines: vec![
            geared::next_gen(130.0, 12.5, 2.06),
            geared::next_gen(130.0, 12.5, 2.06),
        ],
        fuel: AvFuel::JetA,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![twin_high_bypass(), twin_geared()]
}
