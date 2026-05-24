use crate::components::steering::{ColumnAdjust, SteeringColumn, SteeringRack, SteeringWheel, WheelMaterial};

#[derive(Debug, Clone)]
pub struct SteeringKit {
    pub rack: SteeringRack,
    pub column: SteeringColumn,
    pub wheel: SteeringWheel,
}

pub fn entry() -> SteeringKit {
    SteeringKit {
        rack: SteeringRack::electric_power(16.0, false),
        column: SteeringColumn::adjustable(ColumnAdjust::Tilt),
        wheel: SteeringWheel::standard(375.0, WheelMaterial::Urethane),
    }
}

pub fn fleet() -> SteeringKit {
    SteeringKit {
        rack: SteeringRack::electric_power(15.5, false),
        column: SteeringColumn::adjustable(ColumnAdjust::TiltAndTelescope),
        wheel: SteeringWheel::standard(370.0, WheelMaterial::Urethane),
    }
}

pub fn all() -> Vec<SteeringKit> {
    vec![entry(), fleet()]
}
