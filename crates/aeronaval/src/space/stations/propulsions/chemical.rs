#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Propellant {
    Monopropellant,
    Bipropellant,
}

#[derive(Debug, Clone)]
pub struct OrbitalThruster {
    pub thrust_n: f64,
    pub isp_s: f64,
    pub propellant: Propellant,
    pub count: u8,
}

pub fn reboost() -> OrbitalThruster {
    OrbitalThruster {
        thrust_n: 300.0,
        isp_s: 310.0,
        propellant: Propellant::Bipropellant,
        count: 2,
    }
}

pub fn attitude_control() -> OrbitalThruster {
    OrbitalThruster {
        thrust_n: 10.0,
        isp_s: 220.0,
        propellant: Propellant::Monopropellant,
        count: 24,
    }
}

pub fn all() -> Vec<OrbitalThruster> {
    vec![reboost(), attitude_control()]
}
