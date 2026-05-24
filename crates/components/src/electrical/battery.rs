#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryChemistry {
    LeadAcid,
    Agm,
    Gel,
    LithiumIon,
    LithiumIronPhosphate,
}

#[derive(Debug, Clone)]
pub struct VehicleBattery {
    pub chemistry: BatteryChemistry,
    pub voltage_v: f64,
    pub capacity_ah: f64,
    pub cca: u16,
    pub energy_kwh: f64,
    pub mass_kg: f64,
    pub internal_resistance_mohm: f64,
    pub max_continuous_current_a: f64,
    pub cycle_life: u32,
}

fn chem_specs(chem: BatteryChemistry) -> (f64, u32, f64) {
    match chem {
        BatteryChemistry::LeadAcid => (35.0, 500, 5.0),
        BatteryChemistry::Agm => (45.0, 900, 10.0),
        BatteryChemistry::Gel => (40.0, 800, 8.0),
        BatteryChemistry::LithiumIon => (160.0, 3000, 3.0),
        BatteryChemistry::LithiumIronPhosphate => (130.0, 5000, 2.5),
    }
}

fn build(chemistry: BatteryChemistry, voltage_v: f64, capacity_ah: f64, cca: u16) -> VehicleBattery {
    let (gravimetric_density_wh_kg, cycle_life, c_rate_continuous) = chem_specs(chemistry);
    let energy_kwh = voltage_v * capacity_ah / 1000.0;
    let mass_kg = energy_kwh * 1000.0 / gravimetric_density_wh_kg;
    let internal_resistance_mohm = if cca > 0 {
        (voltage_v - 7.2) / (cca as f64) * 1000.0
    } else {
        10.0
    };
    let max_continuous_current_a = capacity_ah * c_rate_continuous;
    VehicleBattery {
        chemistry,
        voltage_v,
        capacity_ah,
        cca,
        energy_kwh: (energy_kwh * 1000.0).round() / 1000.0,
        mass_kg: (mass_kg * 100.0).round() / 100.0,
        internal_resistance_mohm: (internal_resistance_mohm.abs() * 100.0).round() / 100.0,
        max_continuous_current_a: (max_continuous_current_a * 10.0).round() / 10.0,
        cycle_life,
    }
}

impl VehicleBattery {
    pub fn lead_acid_12v(capacity_ah: f64, cca: u16) -> Self {
        build(BatteryChemistry::LeadAcid, 12.0, capacity_ah, cca)
    }

    pub fn agm_12v(capacity_ah: f64, cca: u16) -> Self {
        build(BatteryChemistry::Agm, 12.0, capacity_ah, cca)
    }

    pub fn gel_12v(capacity_ah: f64, cca: u16) -> Self {
        build(BatteryChemistry::Gel, 12.0, capacity_ah, cca)
    }

    pub fn lithium_12v(capacity_ah: f64, cca: u16) -> Self {
        build(BatteryChemistry::LithiumIon, 12.8, capacity_ah, cca)
    }

    pub fn lifepo4_12v(capacity_ah: f64, cca: u16) -> Self {
        build(BatteryChemistry::LithiumIronPhosphate, 12.8, capacity_ah, cca)
    }

    pub fn heavy_duty_24v(capacity_ah: f64, cca: u16) -> Self {
        build(BatteryChemistry::LeadAcid, 24.0, capacity_ah, cca)
    }

    pub fn lithium_48v(capacity_ah: f64, cca: u16) -> Self {
        build(BatteryChemistry::LithiumIon, 48.0, capacity_ah, cca)
    }

    pub fn cranking_power_kw(&self) -> f64 {
        self.cca as f64 * self.voltage_v / 1000.0
    }
}
