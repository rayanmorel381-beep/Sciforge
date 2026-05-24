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

pub fn front_sport() -> FrontKit {
    FrontKit {
        multilink: MultiLink::five_link(true),
        spring: Spring::progressive_coil(36.0, 258.0),
        damper: Damper::monotube(42.0, 148.0),
        antiroll_bar: AntiRollBar::hollow(23.0, 1085.0, 215.0),
    }
}

pub fn rear_sport() -> RearKit {
    RearKit {
        multilink: MultiLink::five_link(true),
        spring: Spring::progressive_coil(32.0, 265.0),
        damper: Damper::monotube(40.0, 145.0),
        antiroll_bar: AntiRollBar::hollow(21.0, 1080.0, 210.0),
    }
}
