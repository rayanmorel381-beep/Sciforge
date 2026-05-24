#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellChemistry {
    Nmc,
    Lfp,
    Nca,
    SolidState,
    Lto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellFormat {
    Cylindrical2170,
    Cylindrical4680,
    Pouch,
    Prismatic,
}

#[derive(Debug, Clone)]
pub struct BatteryPack {
    pub chemistry: CellChemistry,
    pub cell_format: CellFormat,
    pub series_count: u32,
    pub parallel_count: u32,
    pub capacity_kwh: f64,
    pub usable_capacity_kwh: f64,
    pub nominal_voltage_v: f64,
    pub max_discharge_c: f64,
    pub max_charge_c: f64,
    pub peak_discharge_kw: f64,
    pub peak_charge_kw: f64,
    pub thermal_limit_c: f64,
    pub mass_kg: f64,
    pub gravimetric_density_wh_kg: f64,
}

impl Default for BatteryPack {
    fn default() -> Self {
        Self::nmc(CellFormat::Cylindrical2170, 96, 46, 4.2, 3.6)
    }
}

fn chemistry_specs(chem: CellChemistry) -> (f64, f64, f64, f64, f64, f64) {
    match chem {
        CellChemistry::Nmc => (3.7, 250.0, 4.0, 2.0, 55.0, 0.90),
        CellChemistry::Lfp => (3.2, 170.0, 3.0, 1.0, 60.0, 0.95),
        CellChemistry::Nca => (3.6, 260.0, 4.0, 1.5, 50.0, 0.88),
        CellChemistry::SolidState => (3.8, 400.0, 5.0, 3.0, 80.0, 0.92),
        CellChemistry::Lto => (2.4, 80.0, 10.0, 6.0, 65.0, 0.97),
    }
}

fn build(
    chemistry: CellChemistry,
    cell_format: CellFormat,
    series_count: u32,
    parallel_count: u32,
    cell_capacity_ah: f64,
    cell_voltage_nominal_v: f64,
) -> BatteryPack {
    let (chem_voltage, gravimetric_density_wh_kg, max_discharge_c, max_charge_c, thermal_limit_c, usable_ratio) =
        chemistry_specs(chemistry);
    let nominal_voltage_v = chem_voltage * series_count as f64;
    let pack_capacity_ah = cell_capacity_ah * parallel_count as f64;
    let capacity_kwh = nominal_voltage_v * pack_capacity_ah / 1000.0;
    let usable_capacity_kwh = capacity_kwh * usable_ratio;
    let peak_discharge_kw = capacity_kwh * max_discharge_c;
    let peak_charge_kw = capacity_kwh * max_charge_c;
    let mass_kg = capacity_kwh * 1000.0 / gravimetric_density_wh_kg;
    let _ = cell_voltage_nominal_v;
    BatteryPack {
        chemistry,
        cell_format,
        series_count,
        parallel_count,
        capacity_kwh: (capacity_kwh * 100.0).round() / 100.0,
        usable_capacity_kwh: (usable_capacity_kwh * 100.0).round() / 100.0,
        nominal_voltage_v: (nominal_voltage_v * 10.0).round() / 10.0,
        max_discharge_c,
        max_charge_c,
        peak_discharge_kw: (peak_discharge_kw * 10.0).round() / 10.0,
        peak_charge_kw: (peak_charge_kw * 10.0).round() / 10.0,
        thermal_limit_c,
        mass_kg: (mass_kg * 100.0).round() / 100.0,
        gravimetric_density_wh_kg,
    }
}

impl BatteryPack {
    pub fn nmc(format: CellFormat, series: u32, parallel: u32, cell_ah: f64, cell_v: f64) -> Self {
        build(CellChemistry::Nmc, format, series, parallel, cell_ah, cell_v)
    }
    pub fn lfp(format: CellFormat, series: u32, parallel: u32, cell_ah: f64, cell_v: f64) -> Self {
        build(CellChemistry::Lfp, format, series, parallel, cell_ah, cell_v)
    }
    pub fn nca(format: CellFormat, series: u32, parallel: u32, cell_ah: f64, cell_v: f64) -> Self {
        build(CellChemistry::Nca, format, series, parallel, cell_ah, cell_v)
    }
    pub fn solid_state(format: CellFormat, series: u32, parallel: u32, cell_ah: f64, cell_v: f64) -> Self {
        build(CellChemistry::SolidState, format, series, parallel, cell_ah, cell_v)
    }
    pub fn lto(format: CellFormat, series: u32, parallel: u32, cell_ah: f64, cell_v: f64) -> Self {
        build(CellChemistry::Lto, format, series, parallel, cell_ah, cell_v)
    }
}
