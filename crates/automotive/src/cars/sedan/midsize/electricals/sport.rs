use crate::components::electrical::{Alternator, Ecu, EcuRole, FuseBox, Starter, VehicleBattery};

#[derive(Debug, Clone)]
pub struct ElectricalKit {
    pub battery: VehicleBattery,
    pub alternator: Alternator,
    pub starter: Starter,
    pub fuse_box: FuseBox,
    pub ecus: Vec<Ecu>,
}

pub fn sport() -> ElectricalKit {
    ElectricalKit {
        battery: VehicleBattery::agm_12v(72.0, 800),
        alternator: Alternator::variable(160),
        starter: Starter::stop_start(1.6),
        fuse_box: FuseBox::smart(52, 12),
        ecus: vec![
            Ecu::for_role(EcuRole::Ecm),
            Ecu::for_role(EcuRole::Tcm),
            Ecu::for_role(EcuRole::AbsEsc),
            Ecu::for_role(EcuRole::Eps),
            Ecu::for_role(EcuRole::Srs),
            Ecu::for_role(EcuRole::Bcm),
            Ecu::for_role(EcuRole::Tpms),
            Ecu::for_role(EcuRole::BrakeBoost),
            Ecu::for_role(EcuRole::SuspensionControl),
            Ecu::for_role(EcuRole::InstrumentCluster),
            Ecu::for_role(EcuRole::Hvac),
            Ecu::for_role(EcuRole::KeylessAccess),
        ],
    }
}

pub fn all() -> Vec<ElectricalKit> {
    vec![sport()]
}
