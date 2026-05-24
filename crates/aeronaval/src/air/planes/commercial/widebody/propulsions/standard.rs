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
            high_bypass::standard(340.0, 9.0, 3.05),
            high_bypass::standard(340.0, 9.0, 3.05),
        ],
        fuel: AvFuel::JetA1,
    }
}

pub fn quad() -> PropulsionKit {
    PropulsionKit {
        engines: vec![
            high_bypass::standard(280.0, 9.5, 2.84),
            high_bypass::standard(280.0, 9.5, 2.84),
            high_bypass::standard(280.0, 9.5, 2.84),
            high_bypass::standard(280.0, 9.5, 2.84),
        ],
        fuel: AvFuel::JetA1,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![twin(), quad()]
}
