use sciforge_core::moleculars::liquids::fuels::{
    BIODIESEL_B100, BIODIESEL_B20, DIESEL, ETHANOL, GASOLINE, GASOLINE_E85, JET_B,
    KEROSENE_JET_A1, METHANOL,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fuel {
    Gasoline,
    GasolineE85,
    Diesel,
    BiodieselB20,
    BiodieselB100,
    Ethanol,
    Methanol,
    Kerosene,
    JetB,
}

impl Fuel {
    pub fn code(self) -> char {
        match self {
            Fuel::Gasoline => 'G',
            Fuel::GasolineE85 => 'E',
            Fuel::Diesel => 'D',
            Fuel::BiodieselB20 => 'b',
            Fuel::BiodieselB100 => 'B',
            Fuel::Ethanol => 'T',
            Fuel::Methanol => 'M',
            Fuel::Kerosene => 'K',
            Fuel::JetB => 'J',
        }
    }

    pub fn lhv_j_per_kg(self) -> f64 {
        match self {
            Fuel::Gasoline => 44.0e6,
            Fuel::GasolineE85 => 29.2e6,
            Fuel::Diesel => 42.5e6,
            Fuel::BiodieselB20 => 42.0e6,
            Fuel::BiodieselB100 => 37.2e6,
            Fuel::Ethanol => 26.8e6,
            Fuel::Methanol => 19.9e6,
            Fuel::Kerosene => 43.2e6,
            Fuel::JetB => 42.8e6,
        }
    }

    pub fn afr_stoich(self) -> f64 {
        match self {
            Fuel::Gasoline => 14.7,
            Fuel::GasolineE85 => 9.8,
            Fuel::Diesel => 14.5,
            Fuel::BiodieselB20 => 14.2,
            Fuel::BiodieselB100 => 12.5,
            Fuel::Ethanol => 9.0,
            Fuel::Methanol => 6.47,
            Fuel::Kerosene => 14.7,
            Fuel::JetB => 14.7,
        }
    }

    pub fn density_g_per_cc(self) -> f64 {
        match self {
            Fuel::Gasoline => GASOLINE.density_kg_m3_ref / 1000.0,
            Fuel::GasolineE85 => GASOLINE_E85.density_kg_m3_ref / 1000.0,
            Fuel::Diesel => DIESEL.density_kg_m3_ref / 1000.0,
            Fuel::BiodieselB20 => BIODIESEL_B20.density_kg_m3_ref / 1000.0,
            Fuel::BiodieselB100 => BIODIESEL_B100.density_kg_m3_ref / 1000.0,
            Fuel::Ethanol => ETHANOL.density_kg_m3_ref / 1000.0,
            Fuel::Methanol => METHANOL.density_kg_m3_ref / 1000.0,
            Fuel::Kerosene => KEROSENE_JET_A1.density_kg_m3_ref / 1000.0,
            Fuel::JetB => JET_B.density_kg_m3_ref / 1000.0,
        }
    }

    pub fn gamma_air_fuel(self) -> f64 {
        match self {
            Fuel::Gasoline => 1.35,
            Fuel::GasolineE85 => 1.34,
            Fuel::Diesel => 1.32,
            Fuel::BiodieselB20 => 1.32,
            Fuel::BiodieselB100 => 1.31,
            Fuel::Ethanol => 1.34,
            Fuel::Methanol => 1.33,
            Fuel::Kerosene => 1.33,
            Fuel::JetB => 1.33,
        }
    }

    pub fn combustion_duration_deg(self) -> f64 {
        match self {
            Fuel::Gasoline => 60.0,
            Fuel::GasolineE85 => 55.0,
            Fuel::Diesel => 75.0,
            Fuel::BiodieselB20 => 74.0,
            Fuel::BiodieselB100 => 70.0,
            Fuel::Ethanol => 55.0,
            Fuel::Methanol => 50.0,
            Fuel::Kerosene => 80.0,
            Fuel::JetB => 78.0,
        }
    }

    pub fn charge_cooling_dt_k(self) -> f64 {
        match self {
            Fuel::Gasoline => 25.0,
            Fuel::GasolineE85 => 85.0,
            Fuel::Diesel => 45.0,
            Fuel::BiodieselB20 => 44.0,
            Fuel::BiodieselB100 => 42.0,
            Fuel::Ethanol => 120.0,
            Fuel::Methanol => 145.0,
            Fuel::Kerosene => 40.0,
            Fuel::JetB => 38.0,
        }
    }

    pub fn cetane_or_octane(self) -> f64 {
        match self {
            Fuel::Gasoline => 98.0,
            Fuel::GasolineE85 => 105.0,
            Fuel::Diesel => 51.0,
            Fuel::BiodieselB20 => 52.0,
            Fuel::BiodieselB100 => 58.0,
            Fuel::Ethanol => 109.0,
            Fuel::Methanol => 115.0,
            Fuel::Kerosene => 45.0,
            Fuel::JetB => 40.0,
        }
    }

    pub fn knock_limited(self) -> bool {
        matches!(
            self,
            Fuel::Gasoline | Fuel::GasolineE85 | Fuel::Ethanol | Fuel::Methanol
        )
    }

    pub fn carbon_atoms(self) -> f64 {
        match self {
            Fuel::Gasoline => 8.0,
            Fuel::GasolineE85 => 2.5,
            Fuel::Diesel => 12.0,
            Fuel::BiodieselB20 => 12.0,
            Fuel::BiodieselB100 => 19.0,
            Fuel::Ethanol => 2.0,
            Fuel::Methanol => 1.0,
            Fuel::Kerosene => 12.0,
            Fuel::JetB => 10.0,
        }
    }

    pub fn hydrogen_atoms(self) -> f64 {
        match self {
            Fuel::Gasoline => 18.0,
            Fuel::GasolineE85 => 7.0,
            Fuel::Diesel => 23.0,
            Fuel::BiodieselB20 => 23.0,
            Fuel::BiodieselB100 => 36.0,
            Fuel::Ethanol => 6.0,
            Fuel::Methanol => 4.0,
            Fuel::Kerosene => 26.0,
            Fuel::JetB => 22.0,
        }
    }

    pub fn molecular_weight_g_per_mol(self) -> f64 {
        12.011 * self.carbon_atoms() + 1.008 * self.hydrogen_atoms()
    }

    pub fn soot_baseline_g_per_kwh_na(self) -> f64 {
        match self {
            Fuel::Gasoline => 0.015,
            Fuel::GasolineE85 => 0.008,
            Fuel::Diesel => 0.080,
            Fuel::BiodieselB20 => 0.072,
            Fuel::BiodieselB100 => 0.050,
            Fuel::Ethanol => 0.005,
            Fuel::Methanol => 0.003,
            Fuel::Kerosene => 0.070,
            Fuel::JetB => 0.065,
        }
    }

    pub fn soot_baseline_g_per_kwh_boosted(self) -> f64 {
        match self {
            Fuel::Gasoline => 0.040,
            Fuel::GasolineE85 => 0.020,
            Fuel::Diesel => 0.250,
            Fuel::BiodieselB20 => 0.220,
            Fuel::BiodieselB100 => 0.150,
            Fuel::Ethanol => 0.012,
            Fuel::Methanol => 0.008,
            Fuel::Kerosene => 0.200,
            Fuel::JetB => 0.180,
        }
    }
}
