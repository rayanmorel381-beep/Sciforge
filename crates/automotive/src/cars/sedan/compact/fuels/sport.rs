use crate::components::fuel::{FuelTank, FuelType};

#[derive(Debug, Clone)]
pub struct FuelKit {
    pub tank: FuelTank,
    pub fuel_type: FuelType,
}

pub fn sport() -> FuelKit {
    FuelKit {
        tank: FuelTank::standard(FuelType::Gasoline, 55.0),
        fuel_type: FuelType::Gasoline,
    }
}

pub fn all() -> Vec<FuelKit> {
    vec![sport()]
}
