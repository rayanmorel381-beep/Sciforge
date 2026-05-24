use crate::components::electrical::{Alternator, Ecu, EcuRole, FuseBox, Starter, VehicleBattery};

#[derive(Debug, Clone)]
pub struct ElectricalKit {
    pub battery: VehicleBattery,
    pub alternator: Alternator,
    pub starter: Starter,
    pub fuse_box: FuseBox,
    pub ecus: Vec<Ecu>,
}

pub fn entry() -> ElectricalKit {
    ElectricalKit {
        battery: VehicleBattery::lead_acid_12v(62.0, 560),
        alternator: Alternator::standard(120),
        starter: Starter::conventional(1.4),
        fuse_box: FuseBox::standard(40),
        ecus: vec![
            Ecu::for_role(EcuRole::Ecm),
            Ecu::for_role(EcuRole::AbsEsc),
            Ecu::for_role(EcuRole::Eps),
            Ecu::for_role(EcuRole::Srs),
            Ecu::for_role(EcuRole::Bcm),
            Ecu::for_role(EcuRole::Hvac),
            Ecu::for_role(EcuRole::InstrumentCluster),
        ],
    }
}

pub fn comfort() -> ElectricalKit {
    ElectricalKit {
        battery: VehicleBattery::agm_12v(72.0, 700),
        alternator: Alternator::variable(150),
        starter: Starter::stop_start(1.6),
        fuse_box: FuseBox::smart(52, 12),
        ecus: vec![
            Ecu::for_role(EcuRole::Ecm),
            Ecu::for_role(EcuRole::AbsEsc),
            Ecu::for_role(EcuRole::Eps),
            Ecu::for_role(EcuRole::Srs),
            Ecu::for_role(EcuRole::Bcm),
            Ecu::for_role(EcuRole::Tpms),
            Ecu::for_role(EcuRole::KeylessAccess),
            Ecu::for_role(EcuRole::UltrasonicPark),
            Ecu::for_role(EcuRole::InstrumentCluster),
            Ecu::for_role(EcuRole::Hvac),
        ],
    }
}

pub fn premium() -> ElectricalKit {
    ElectricalKit {
        battery: VehicleBattery::agm_12v(85.0, 800),
        alternator: Alternator::variable(190),
        starter: Starter::stop_start(1.8),
        fuse_box: FuseBox::smart(64, 16),
        ecus: vec![
            Ecu::for_role(EcuRole::Ecm),
            Ecu::for_role(EcuRole::AbsEsc),
            Ecu::for_role(EcuRole::Eps),
            Ecu::for_role(EcuRole::Srs),
            Ecu::for_role(EcuRole::Bcm),
            Ecu::for_role(EcuRole::Tpms),
            Ecu::for_role(EcuRole::KeylessAccess),
            Ecu::for_role(EcuRole::UltrasonicPark),
            Ecu::for_role(EcuRole::SurroundView),
            Ecu::for_role(EcuRole::InstrumentCluster),
            Ecu::for_role(EcuRole::HeadUpDisplay),
            Ecu::for_role(EcuRole::Hvac),
            Ecu::for_role(EcuRole::Infotainment),
        ],
    }
}

pub fn all() -> Vec<ElectricalKit> {
    vec![entry(), comfort(), premium()]
}
