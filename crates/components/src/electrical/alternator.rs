#[derive(Debug, Clone)]
pub struct Alternator {
    pub output_amps: u16,
    pub voltage_v: f64,
    pub variable_output: bool,
    pub peak_power_w: f64,
    pub efficiency: f64,
    pub min_rpm: u16,
    pub max_rpm: u16,
    pub pulley_diameter_mm: f64,
    pub mass_kg: f64,
    pub mechanical_drag_kw_at_max: f64,
}

fn build(output_amps: u16, voltage_v: f64, variable_output: bool, heavy_duty: bool) -> Alternator {
    let peak_power_w = output_amps as f64 * voltage_v;
    let efficiency = if variable_output { 0.72 } else { 0.62 };
    let mass_kg = if heavy_duty {
        peak_power_w / 1000.0 * 1.2 + 5.0
    } else {
        peak_power_w / 1000.0 * 0.9 + 3.0
    };
    let pulley_diameter_mm = if heavy_duty { 75.0 } else { 56.0 };
    let min_rpm = if variable_output { 1200 } else { 1500 };
    let max_rpm = if heavy_duty { 8000 } else { 14000 };
    let mechanical_drag_kw_at_max = peak_power_w / 1000.0 / efficiency;
    Alternator {
        output_amps,
        voltage_v,
        variable_output,
        peak_power_w: (peak_power_w * 10.0).round() / 10.0,
        efficiency,
        min_rpm,
        max_rpm,
        pulley_diameter_mm,
        mass_kg: (mass_kg * 100.0).round() / 100.0,
        mechanical_drag_kw_at_max: (mechanical_drag_kw_at_max * 100.0).round() / 100.0,
    }
}

impl Alternator {
    pub fn standard(output_amps: u16) -> Self {
        build(output_amps, 14.4, false, false)
    }

    pub fn variable(output_amps: u16) -> Self {
        build(output_amps, 14.4, true, false)
    }

    pub fn heavy_duty(output_amps: u16) -> Self {
        build(output_amps, 28.0, false, true)
    }
}
