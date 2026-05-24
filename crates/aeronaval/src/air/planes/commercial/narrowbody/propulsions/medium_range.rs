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
            high_bypass::standard(130.0, 12.5, 1.82),
            high_bypass::standard(130.0, 12.5, 1.82),
        ],
        fuel: AvFuel::JetA1,
    }
}

pub fn twin_geared() -> PropulsionKit {
    use crate::components::powertrain::engines::turbines::assemblies::fan::geared;
    PropulsionKit {
        engines: vec![
            geared::next_gen(120.0, 12.0, 2.06),
            geared::next_gen(120.0, 12.0, 2.06),
        ],
        fuel: AvFuel::JetA1,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![twin(), twin_geared()]
}
