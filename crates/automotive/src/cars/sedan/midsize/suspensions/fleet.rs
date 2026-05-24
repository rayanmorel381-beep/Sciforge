use crate::components::suspensions::{AntiRollBar, Damper, MultiLink, Spring};

#[derive(Debug, Clone)]
pub struct FrontKit {
    pub multilink: MultiLink,
    pub spring: Spring,
    pub damper: Damper,
    pub antiroll_bar: AntiRollBar,
}

#[derive(Debug, Clone)]
pub struct RearKit {
    pub multilink: MultiLink,
    pub spring: Spring,
    pub damper: Damper,
    pub antiroll_bar: AntiRollBar,
}

pub fn front_entry() -> FrontKit {
    FrontKit {
        multilink: MultiLink::four_link(),
        spring: Spring::coil(26.0, 270.0),
        damper: Damper::twin_tube(36.0, 152.0),
        antiroll_bar: AntiRollBar::solid(18.0, 1105.0, 225.0),
    }
}

pub fn front_fleet() -> FrontKit {
    FrontKit {
        multilink: MultiLink::four_link(),
        spring: Spring::progressive_coil(27.0, 269.0),
        damper: Damper::twin_tube(37.0, 153.0),
        antiroll_bar: AntiRollBar::solid(18.0, 1100.0, 224.0),
    }
}

pub fn rear_entry() -> RearKit {
    RearKit {
        multilink: MultiLink::four_link(),
        spring: Spring::coil(22.0, 278.0),
        damper: Damper::twin_tube(34.0, 150.0),
        antiroll_bar: AntiRollBar::solid(16.0, 1095.0, 218.0),
    }
}

pub fn rear_fleet() -> RearKit {
    RearKit {
        multilink: MultiLink::four_link(),
        spring: Spring::progressive_coil(23.0, 276.0),
        damper: Damper::twin_tube(35.0, 151.0),
        antiroll_bar: AntiRollBar::solid(16.0, 1090.0, 216.0),
    }
}
