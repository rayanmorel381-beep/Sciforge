#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarterType {
    Conventional,
    Integrated,
    BeltStarter,
}

#[derive(Debug, Clone)]
pub struct Starter {
    pub starter_type: StarterType,
    pub power_kw: f64,
    pub stop_start: bool,
    pub voltage_v: f64,
    pub peak_current_a: f64,
    pub crank_torque_nm: f64,
    pub crank_speed_rpm: u16,
    pub mass_kg: f64,
    pub max_starts_per_minute: u8,
}

fn build(starter_type: StarterType, power_kw: f64, stop_start: bool) -> Starter {
    let voltage_v = match starter_type {
        StarterType::Conventional => 12.0,
        StarterType::Integrated => 48.0,
        StarterType::BeltStarter => 48.0,
    };
    let efficiency = match starter_type {
        StarterType::Conventional => 0.55,
        StarterType::Integrated => 0.85,
        StarterType::BeltStarter => 0.78,
    };
    let peak_current_a = power_kw * 1000.0 / (voltage_v * efficiency);
    let crank_speed_rpm: u16 = match starter_type {
        StarterType::Conventional => 250,
        StarterType::Integrated => 1200,
        StarterType::BeltStarter => 800,
    };
    let omega = (crank_speed_rpm as f64) * 2.0 * std::f64::consts::PI / 60.0;
    let crank_torque_nm = power_kw * 1000.0 / omega;
    let mass_kg = match starter_type {
        StarterType::Conventional => power_kw * 1.8 + 2.5,
        StarterType::Integrated => power_kw * 1.2 + 6.0,
        StarterType::BeltStarter => power_kw * 1.4 + 4.0,
    };
    let max_starts_per_minute = if stop_start { 60 } else { 5 };
    Starter {
        starter_type,
        power_kw,
        stop_start,
        voltage_v,
        peak_current_a: (peak_current_a * 10.0).round() / 10.0,
        crank_torque_nm: (crank_torque_nm * 10.0).round() / 10.0,
        crank_speed_rpm,
        mass_kg: (mass_kg * 100.0).round() / 100.0,
        max_starts_per_minute,
    }
}

impl Starter {
    pub fn conventional(power_kw: f64) -> Self {
        build(StarterType::Conventional, power_kw, false)
    }

    pub fn stop_start(power_kw: f64) -> Self {
        build(StarterType::Conventional, power_kw, true)
    }

    pub fn integrated(power_kw: f64) -> Self {
        build(StarterType::Integrated, power_kw, true)
    }

    pub fn belt_starter(power_kw: f64) -> Self {
        build(StarterType::BeltStarter, power_kw, true)
    }
}
