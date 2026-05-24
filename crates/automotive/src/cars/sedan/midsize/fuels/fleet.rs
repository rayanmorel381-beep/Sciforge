use crate::components::fuel::{FuelTank, FuelType};

#[derive(Debug, Clone)]
pub struct FuelKit {
    pub tank: FuelTank,
    pub fuel_type: FuelType,
}

pub fn gasoline() -> FuelKit {
    FuelKit {
        tank: FuelTank::standard(FuelType::Gasoline, 60.0),
        fuel_type: FuelType::Gasoline,
    }
}

pub fn diesel() -> FuelKit {
    FuelKit {
        tank: FuelTank::standard(FuelType::Diesel, 60.0),
        fuel_type: FuelType::Diesel,
    }
}

pub fn all() -> Vec<FuelKit> {
    vec![gasoline(), diesel()]
}
