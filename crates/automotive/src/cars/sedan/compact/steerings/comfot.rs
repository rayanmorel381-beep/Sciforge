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

pub fn comfort() -> SteeringKit {
    SteeringKit {
        rack: SteeringRack::electric_power(15.5, false),
        column: SteeringColumn::adjustable(ColumnAdjust::TiltAndTelescope),
        wheel: SteeringWheel::heated(370.0, WheelMaterial::Leather),
    }
}

pub fn premium() -> SteeringKit {
    SteeringKit {
        rack: SteeringRack::electric_power(15.0, true),
        column: SteeringColumn::adjustable(ColumnAdjust::FullyElectric),
        wheel: SteeringWheel::heated(365.0, WheelMaterial::AlcantaraLeather),
    }
}

pub fn all() -> Vec<SteeringKit> {
    vec![entry(), comfort(), premium()]
}
