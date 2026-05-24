#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Propellant {
    SolidComposite,
}

#[derive(Debug, Clone)]
pub struct Rocket {
    pub thrust_kn: f64,
    pub isp_s: f64,
    pub propellant: Propellant,
    pub stages: u8,
    pub burn_time_s: f64,
}

pub fn sounding_small() -> Rocket {
    Rocket {
        thrust_kn: 50.0,
        isp_s: 250.0,
        propellant: Propellant::SolidComposite,
        stages: 1,
        burn_time_s: 10.0,
    }
}

pub fn sounding_medium() -> Rocket {
    Rocket {
        thrust_kn: 150.0,
        isp_s: 265.0,
        propellant: Propellant::SolidComposite,
        stages: 2,
        burn_time_s: 25.0,
    }
}

pub fn tactical_ballistic() -> Rocket {
    Rocket {
        thrust_kn: 340.0,
        isp_s: 280.0,
        propellant: Propellant::SolidComposite,
        stages: 1,
        burn_time_s: 60.0,
    }
}

pub fn all() -> Vec<Rocket> {
    vec![sounding_small(), sounding_medium(), tactical_ballistic()]
}
