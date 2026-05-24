#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Propellant {
    Monopropellant,
    Bipropellant,
}

#[derive(Debug, Clone)]
pub struct PropulsionKit {
    pub thrust_n: f64,
    pub isp_s: f64,
    pub propellant: Propellant,
    pub delta_v_ms: f64,
}

pub fn monopropellant() -> PropulsionKit {
    PropulsionKit {
        thrust_n: 22.0,
        isp_s: 220.0,
        propellant: Propellant::Monopropellant,
        delta_v_ms: 600.0,
    }
}

pub fn bipropellant() -> PropulsionKit {
    PropulsionKit {
        thrust_n: 400.0,
        isp_s: 315.0,
        propellant: Propellant::Bipropellant,
        delta_v_ms: 1_200.0,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![monopropellant(), bipropellant()]
}
