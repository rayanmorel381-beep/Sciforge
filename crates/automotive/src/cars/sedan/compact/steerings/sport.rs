use crate::components::steering::{ColumnAdjust, SteeringColumn, SteeringRack, SteeringWheel, WheelMaterial};

#[derive(Debug, Clone)]
pub struct SteeringKit {
    pub rack: SteeringRack,
    pub column: SteeringColumn,
    pub wheel: SteeringWheel,
}

pub fn sport() -> SteeringKit {
    SteeringKit {
        rack: SteeringRack::electric_power(13.5, true),
        column: SteeringColumn::adjustable(ColumnAdjust::TiltAndTelescope),
        wheel: SteeringWheel::heated(360.0, WheelMaterial::AlcantaraLeather),
    }
}

pub fn all() -> Vec<SteeringKit> {
    vec![sport()]
}
