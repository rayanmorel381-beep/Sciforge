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

pub fn front_comfort() -> FrontKit {
    FrontKit {
        multilink: MultiLink::four_link(),
        spring: Spring::progressive_coil(28.0, 268.0),
        damper: Damper::monotube(38.0, 155.0),
        antiroll_bar: AntiRollBar::hollow(19.0, 1100.0, 222.0),
    }
}

pub fn front_premium() -> FrontKit {
    FrontKit {
        multilink: MultiLink::five_link(true),
        spring: Spring::progressive_coil(30.0, 265.0),
        damper: Damper::monotube(40.0, 158.0),
        antiroll_bar: AntiRollBar::hollow(20.0, 1095.0, 220.0),
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

pub fn rear_comfort() -> RearKit {
    RearKit {
        multilink: MultiLink::four_link(),
        spring: Spring::progressive_coil(24.0, 275.0),
        damper: Damper::monotube(36.0, 152.0),
        antiroll_bar: AntiRollBar::hollow(17.0, 1090.0, 215.0),
    }
}

pub fn rear_premium() -> RearKit {
    RearKit {
        multilink: MultiLink::five_link(true),
        spring: Spring::progressive_coil(26.0, 272.0),
        damper: Damper::monotube(38.0, 155.0),
        antiroll_bar: AntiRollBar::hollow(18.0, 1085.0, 212.0),
    }
}
