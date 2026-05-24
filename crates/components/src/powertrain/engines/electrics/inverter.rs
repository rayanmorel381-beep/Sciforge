#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchTechnology {
    Igbt,
    SiC,
    GaN,
}

#[derive(Debug, Clone)]
pub struct Inverter {
    pub switch_technology: SwitchTechnology,
    pub dc_voltage_v: f64,
    pub continuous_power_kw: f64,
    pub peak_power_kw: f64,
    pub continuous_current_a: f64,
    pub peak_current_a: f64,
    pub switching_frequency_khz: f64,
    pub efficiency: f64,
    pub mass_kg: f64,
    pub peak_losses_kw: f64,
}

impl Default for Inverter {
    fn default() -> Self {
        Self::sic(150.0, 400.0)
    }
}

fn switch_specs(tech: SwitchTechnology) -> (f64, f64, f64) {
    match tech {
        SwitchTechnology::Igbt => (0.965, 12.0, 0.45),
        SwitchTechnology::SiC => (0.985, 30.0, 0.30),
        SwitchTechnology::GaN => (0.99, 80.0, 0.22),
    }
}

fn build(tech: SwitchTechnology, peak_power_kw: f64, dc_voltage_v: f64) -> Inverter {
    let (efficiency, switching_frequency_khz, mass_per_kw) = switch_specs(tech);
    let continuous_power_kw = peak_power_kw * 0.55;
    let peak_current_a = peak_power_kw * 1000.0 / (dc_voltage_v * efficiency);
    let continuous_current_a = continuous_power_kw * 1000.0 / (dc_voltage_v * efficiency);
    let mass_kg = peak_power_kw * mass_per_kw + 3.5;
    let peak_losses_kw = peak_power_kw * (1.0 - efficiency);
    Inverter {
        switch_technology: tech,
        dc_voltage_v,
        continuous_power_kw: (continuous_power_kw * 10.0).round() / 10.0,
        peak_power_kw,
        continuous_current_a: (continuous_current_a * 10.0).round() / 10.0,
        peak_current_a: (peak_current_a * 10.0).round() / 10.0,
        switching_frequency_khz,
        efficiency,
        mass_kg: (mass_kg * 100.0).round() / 100.0,
        peak_losses_kw: (peak_losses_kw * 100.0).round() / 100.0,
    }
}

impl Inverter {
    pub fn igbt(peak_power_kw: f64, dc_voltage_v: f64) -> Self {
        build(SwitchTechnology::Igbt, peak_power_kw, dc_voltage_v)
    }
    pub fn sic(peak_power_kw: f64, dc_voltage_v: f64) -> Self {
        build(SwitchTechnology::SiC, peak_power_kw, dc_voltage_v)
    }
    pub fn gan(peak_power_kw: f64, dc_voltage_v: f64) -> Self {
        build(SwitchTechnology::GaN, peak_power_kw, dc_voltage_v)
    }
}
