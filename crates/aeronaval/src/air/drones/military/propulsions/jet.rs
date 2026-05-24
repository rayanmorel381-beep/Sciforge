use crate::components::powertrain::engines::turbines::assemblies::{
    fuel::AvFuel,
    jet::{single_spool, turbojet::Turbojet},
};

#[derive(Debug, Clone)]
pub struct PropulsionKit {
    pub engines: Vec<Turbojet>,
    pub fuel: AvFuel,
}

pub fn loitering() -> PropulsionKit {
    PropulsionKit {
        engines: vec![single_spool::high_altitude(5.0, 14.0)],
        fuel: AvFuel::Jp8,
    }
}

pub fn combat() -> PropulsionKit {
    PropulsionKit {
        engines: vec![single_spool::basic(12.0, 16.0)],
        fuel: AvFuel::Jp8,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![loitering(), combat()]
}
