#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuelType {
    Gasoline,
    Diesel,
    Ethanol,
    Kerosene,
    Avgas,
    LiquidHydrogen,
    LiquidOxygen,
    Hypergolic,
    CompressedNaturalGas,
    LpgAutogas,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TankMaterial {
    Steel,
    Aluminum,
    Hdpe,
    CarbonFibre,
    Titanium,
}

#[derive(Debug, Clone)]
pub struct FuelTank {
    pub fuel_type: FuelType,
    pub material: TankMaterial,
    pub capacity_l: f64,
    pub saddle: bool,
    pub baffled: bool,
    pub crash_resistant: bool,
    pub fuel_level_sensor: bool,
    pub in_tank_pump: bool,
}

impl FuelTank {
    pub fn standard(fuel_type: FuelType, capacity_l: f64) -> Self {
        Self {
            fuel_type,
            material: TankMaterial::Steel,
            capacity_l,
            saddle: false,
            baffled: true,
            crash_resistant: false,
            fuel_level_sensor: true,
            in_tank_pump: true,
        }
    }

    pub fn saddle(fuel_type: FuelType, capacity_l: f64) -> Self {
        Self {
            fuel_type,
            material: TankMaterial::Hdpe,
            capacity_l,
            saddle: true,
            baffled: true,
            crash_resistant: false,
            fuel_level_sensor: true,
            in_tank_pump: true,
        }
    }

    pub fn crash_resistant(fuel_type: FuelType, capacity_l: f64) -> Self {
        Self {
            fuel_type,
            material: TankMaterial::Aluminum,
            capacity_l,
            saddle: false,
            baffled: true,
            crash_resistant: true,
            fuel_level_sensor: true,
            in_tank_pump: false,
        }
    }

    pub fn carbon_racing(fuel_type: FuelType, capacity_l: f64) -> Self {
        Self {
            fuel_type,
            material: TankMaterial::CarbonFibre,
            capacity_l,
            saddle: false,
            baffled: true,
            crash_resistant: true,
            fuel_level_sensor: true,
            in_tank_pump: true,
        }
    }
}
