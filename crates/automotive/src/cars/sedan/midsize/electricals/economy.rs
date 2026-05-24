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

pub fn all() -> Vec<ElectricalKit> {
    vec![entry()]
}
