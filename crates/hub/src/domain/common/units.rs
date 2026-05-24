//! Unit systems and conversion functions.
//!
//! Provides enums for length, mass, time, energy, temperature, angle,
//! and pressure units together with bidirectional SI conversion helpers.

/// Length measurement units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthUnit {
    Meter,
    Kilometer,
    Centimeter,
    Millimeter,
    Micrometer,
    Nanometer,
    Angstrom,
    Mile,
    Yard,
    Foot,
    Inch,
    AstronomicalUnit,
    LightYear,
    Parsec,
}

/// Mass measurement units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MassUnit {
    Kilogram,
    Gram,
    Milligram,
    Microgram,
    Tonne,
    Dalton,
    Pound,
    Ounce,
    SolarMass,
}

/// Time measurement units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
    Minute,
    Hour,
    Day,
    Year,
}

/// Temperature measurement units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureUnit {
    Kelvin,
    Celsius,
    Fahrenheit,
    Rankine,
}

/// Energy measurement units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnergyUnit {
    Joule,
    Kilojoule,
    Calorie,
    Kilocalorie,
    ElectronVolt,
    Erg,
}

/// Pressure measurement units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureUnit {
    Pascal,
    Kilopascal,
    Megapascal,
    Bar,
    Atmosphere,
    Torr,
    Psi,
}

/// Angle measurement units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngleUnit {
    Radian,
    Degree,
    Arcminute,
    Arcsecond,
}

/// Converts a length value to SI (meters).
pub fn length_to_si(value: f64, unit: LengthUnit) -> f64 {
    value
        * match unit {
            LengthUnit::Meter => 1.0,
            LengthUnit::Kilometer => 1e3,
            LengthUnit::Centimeter => 1e-2,
            LengthUnit::Millimeter => 1e-3,
            LengthUnit::Micrometer => 1e-6,
            LengthUnit::Nanometer => 1e-9,
            LengthUnit::Angstrom => 1e-10,
            LengthUnit::Mile => 1609.344,
            LengthUnit::Yard => 0.9144,
            LengthUnit::Foot => 0.3048,
            LengthUnit::Inch => 0.0254,
            LengthUnit::AstronomicalUnit => 1.495_978_707e11,
            LengthUnit::LightYear => 9.460_730_472_58e15,
            LengthUnit::Parsec => 3.085_677_581_28e16,
        }
}

/// Converts a length value from SI (meters) to the target unit.
pub fn length_from_si(value: f64, unit: LengthUnit) -> f64 {
    value / length_to_si(1.0, unit)
}

/// Converts a mass value to SI (kilograms).
pub fn mass_to_si(value: f64, unit: MassUnit) -> f64 {
    value
        * match unit {
            MassUnit::Kilogram => 1.0,
            MassUnit::Gram => 1e-3,
            MassUnit::Milligram => 1e-6,
            MassUnit::Microgram => 1e-9,
            MassUnit::Tonne => 1e3,
            MassUnit::Dalton => 1.660_539_066_6e-27,
            MassUnit::Pound => 0.453_592_37,
            MassUnit::Ounce => 0.028_349_523_125,
            MassUnit::SolarMass => sciforge_lib::constants::SOLAR_MASS,
        }
}

/// Converts a mass value from SI (kilograms) to the target unit.
pub fn mass_from_si(value: f64, unit: MassUnit) -> f64 {
    value / mass_to_si(1.0, unit)
}

/// Converts a time value to SI (seconds).
pub fn time_to_si(value: f64, unit: TimeUnit) -> f64 {
    value
        * match unit {
            TimeUnit::Second => 1.0,
            TimeUnit::Millisecond => 1e-3,
            TimeUnit::Microsecond => 1e-6,
            TimeUnit::Nanosecond => 1e-9,
            TimeUnit::Minute => 60.0,
            TimeUnit::Hour => 3600.0,
            TimeUnit::Day => 86400.0,
            TimeUnit::Year => 365.25 * 86400.0,
        }
}

/// Converts a time value from SI (seconds) to the target unit.
pub fn time_from_si(value: f64, unit: TimeUnit) -> f64 {
    value / time_to_si(1.0, unit)
}

/// Converts a temperature to Kelvin.
pub fn temperature_to_kelvin(value: f64, unit: TemperatureUnit) -> f64 {
    match unit {
        TemperatureUnit::Kelvin => value,
        TemperatureUnit::Celsius => value + 273.15,
        TemperatureUnit::Fahrenheit => (value - 32.0) * 5.0 / 9.0 + 273.15,
        TemperatureUnit::Rankine => value * 5.0 / 9.0,
    }
}

/// Converts Kelvin to the target temperature unit.
pub fn kelvin_to_temperature(value: f64, unit: TemperatureUnit) -> f64 {
    match unit {
        TemperatureUnit::Kelvin => value,
        TemperatureUnit::Celsius => value - 273.15,
        TemperatureUnit::Fahrenheit => (value - 273.15) * 9.0 / 5.0 + 32.0,
        TemperatureUnit::Rankine => value * 9.0 / 5.0,
    }
}

/// Converts an energy value to SI (joules).
pub fn energy_to_si(value: f64, unit: EnergyUnit) -> f64 {
    value
        * match unit {
            EnergyUnit::Joule => 1.0,
            EnergyUnit::Kilojoule => 1e3,
            EnergyUnit::Calorie => 4.184,
            EnergyUnit::Kilocalorie => 4184.0,
            EnergyUnit::ElectronVolt => 1.602_176_634e-19,
            EnergyUnit::Erg => 1e-7,
        }
}

/// Converts an energy value from SI (joules) to the target unit.
pub fn energy_from_si(value: f64, unit: EnergyUnit) -> f64 {
    value / energy_to_si(1.0, unit)
}

/// Converts a pressure value to SI (pascals).
pub fn pressure_to_si(value: f64, unit: PressureUnit) -> f64 {
    value
        * match unit {
            PressureUnit::Pascal => 1.0,
            PressureUnit::Kilopascal => 1e3,
            PressureUnit::Megapascal => 1e6,
            PressureUnit::Bar => 1e5,
            PressureUnit::Atmosphere => 101_325.0,
            PressureUnit::Torr => 133.322_368_42,
            PressureUnit::Psi => 6_894.757_293_168,
        }
}

/// Converts a pressure value from SI (pascals) to the target unit.
pub fn pressure_from_si(value: f64, unit: PressureUnit) -> f64 {
    value / pressure_to_si(1.0, unit)
}

/// Converts an angle to radians.
pub fn angle_to_radian(value: f64, unit: AngleUnit) -> f64 {
    value
        * match unit {
            AngleUnit::Radian => 1.0,
            AngleUnit::Degree => std::f64::consts::PI / 180.0,
            AngleUnit::Arcminute => std::f64::consts::PI / 10800.0,
            AngleUnit::Arcsecond => std::f64::consts::PI / 648_000.0,
        }
}

/// Converts radians to the target angle unit.
pub fn radian_to_angle(value: f64, unit: AngleUnit) -> f64 {
    value / angle_to_radian(1.0, unit)
}

/// Converts a length between two arbitrary units.
pub fn convert_length(value: f64, from: LengthUnit, to: LengthUnit) -> f64 {
    length_from_si(length_to_si(value, from), to)
}

/// Converts a mass between two arbitrary units.
pub fn convert_mass(value: f64, from: MassUnit, to: MassUnit) -> f64 {
    mass_from_si(mass_to_si(value, from), to)
}

/// Converts a time between two arbitrary units.
pub fn convert_time(value: f64, from: TimeUnit, to: TimeUnit) -> f64 {
    time_from_si(time_to_si(value, from), to)
}

/// Converts a temperature between two arbitrary units.
pub fn convert_temperature(value: f64, from: TemperatureUnit, to: TemperatureUnit) -> f64 {
    kelvin_to_temperature(temperature_to_kelvin(value, from), to)
}

/// Converts an energy between two arbitrary units.
pub fn convert_energy(value: f64, from: EnergyUnit, to: EnergyUnit) -> f64 {
    energy_from_si(energy_to_si(value, from), to)
}

/// Converts a pressure between two arbitrary units.
pub fn convert_pressure(value: f64, from: PressureUnit, to: PressureUnit) -> f64 {
    pressure_from_si(pressure_to_si(value, from), to)
}

/// Converts an angle between two arbitrary units.
pub fn convert_angle(value: f64, from: AngleUnit, to: AngleUnit) -> f64 {
    radian_to_angle(angle_to_radian(value, from), to)
}
