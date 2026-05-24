#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcuGeneration {
    Obd1,
    Obd2,
    Can,
    Autosar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsilLevel {
    Qm,
    A,
    B,
    C,
    D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EcuRole {
    Ecm,
    Tcm,
    Hcu,
    Bms,
    Vcu,
    Inverter,
    OnBoardCharger,
    DcDcConverter,
    AbsEsc,
    Eps,
    SuspensionControl,
    Tpms,
    BrakeBoost,
    Bcm,
    Hvac,
    DoorModule,
    SeatModule,
    LightingControl,
    WiperControl,
    KeylessAccess,
    SunroofControl,
    PowerTailgate,
    Srs,
    OccupantClassification,
    AdasDomain,
    FrontCamera,
    FrontRadar,
    CornerRadar,
    UltrasonicPark,
    SurroundView,
    DriverMonitoring,
    Lidar,
    Infotainment,
    InstrumentCluster,
    HeadUpDisplay,
    AudioAmplifier,
    Telematics,
    V2x,
    WirelessCharging,
    CentralGateway,
}

#[derive(Debug, Clone)]
pub struct Ecu {
    pub role: EcuRole,
    pub generation: EcuGeneration,
    pub processor_mhz: u32,
    pub core_count: u8,
    pub ram_kb: u32,
    pub flash_mb: u32,
    pub ethernet_mbps: u16,
    pub can_buses: u8,
    pub can_fd: bool,
    pub lin_buses: u8,
    pub asil_level: AsilLevel,
    pub power_draw_w: f64,
    pub mass_kg: f64,
    pub voltage_v: f64,
    pub over_the_air: bool,
}

type RoleSpec = (EcuGeneration, u32, u8, u32, u32, u16, u8, bool, u8, AsilLevel, f64, f64, bool);

fn role_spec(role: EcuRole) -> RoleSpec {
    match role {
        EcuRole::Ecm =>                    (EcuGeneration::Autosar,  300, 2,   4096,   16,    0, 2, true,  0, AsilLevel::D, 9.0,  0.62, true),
        EcuRole::Tcm =>                    (EcuGeneration::Autosar,  200, 2,   2048,    8,    0, 2, true,  0, AsilLevel::D, 5.5,  0.48, true),
        EcuRole::Hcu =>                    (EcuGeneration::Autosar,  400, 2,   8192,   16,  100, 3, true,  0, AsilLevel::D, 7.5,  0.72, true),
        EcuRole::Bms =>                    (EcuGeneration::Autosar,  240, 2,   2048,    4,  100, 3, true,  0, AsilLevel::D, 6.0,  1.40, true),
        EcuRole::Vcu =>                    (EcuGeneration::Autosar,  800, 4,  32768,   64, 1000, 4, true,  0, AsilLevel::D, 14.0, 0.95, true),
        EcuRole::Inverter =>               (EcuGeneration::Autosar,  200, 2,   2048,    4,    0, 2, true,  0, AsilLevel::D, 5.0,  0.45, true),
        EcuRole::OnBoardCharger =>         (EcuGeneration::Autosar,  120, 1,   1024,    2,    0, 1, true,  0, AsilLevel::B, 4.0,  0.40, true),
        EcuRole::DcDcConverter =>          (EcuGeneration::Autosar,   80, 1,    512,    1,    0, 1, true,  0, AsilLevel::B, 2.5,  0.25, false),
        EcuRole::AbsEsc =>                 (EcuGeneration::Autosar,  300, 2,   4096,    8,    0, 2, true,  0, AsilLevel::D, 7.0,  0.85, true),
        EcuRole::Eps =>                    (EcuGeneration::Autosar,  240, 2,   2048,    4,    0, 2, true,  0, AsilLevel::D, 4.5,  0.55, true),
        EcuRole::SuspensionControl =>      (EcuGeneration::Autosar,  150, 1,   1024,    2,    0, 1, true,  0, AsilLevel::B, 3.5,  0.45, true),
        EcuRole::Tpms =>                   (EcuGeneration::Can,       50, 1,    256,    1,    0, 1, false, 0, AsilLevel::B, 1.5,  0.18, false),
        EcuRole::BrakeBoost =>             (EcuGeneration::Autosar,  240, 2,   2048,    4,    0, 2, true,  0, AsilLevel::D, 5.5,  0.65, true),
        EcuRole::Bcm =>                    (EcuGeneration::Autosar,  200, 1,   4096,    8,  100, 3, true,  4, AsilLevel::B, 5.5,  0.72, true),
        EcuRole::Hvac =>                   (EcuGeneration::Can,      100, 1,   1024,    2,    0, 1, true,  1, AsilLevel::Qm, 4.0, 0.42, true),
        EcuRole::DoorModule =>             (EcuGeneration::Can,       50, 1,    512,    1,    0, 1, false, 1, AsilLevel::Qm, 2.0, 0.22, false),
        EcuRole::SeatModule =>             (EcuGeneration::Can,       50, 1,    256,    1,    0, 1, false, 1, AsilLevel::Qm, 2.0, 0.25, false),
        EcuRole::LightingControl =>        (EcuGeneration::Can,       80, 1,    512,    1,    0, 1, false, 2, AsilLevel::A,  2.5, 0.32, false),
        EcuRole::WiperControl =>           (EcuGeneration::Can,       30, 1,    128,    1,    0, 1, false, 1, AsilLevel::A,  1.5, 0.16, false),
        EcuRole::KeylessAccess =>          (EcuGeneration::Can,       80, 1,   1024,    2,    0, 1, false, 0, AsilLevel::B,  2.5, 0.26, true),
        EcuRole::SunroofControl =>         (EcuGeneration::Can,       40, 1,    256,    1,    0, 1, false, 1, AsilLevel::Qm, 1.5, 0.20, false),
        EcuRole::PowerTailgate =>          (EcuGeneration::Can,       50, 1,    256,    1,    0, 1, false, 1, AsilLevel::Qm, 1.8, 0.22, false),
        EcuRole::Srs =>                    (EcuGeneration::Autosar,  200, 2,   2048,    4,    0, 2, true,  0, AsilLevel::D, 4.0,  0.52, true),
        EcuRole::OccupantClassification => (EcuGeneration::Can,       50, 1,    256,    1,    0, 1, false, 1, AsilLevel::B, 1.0,  0.16, false),
        EcuRole::AdasDomain =>             (EcuGeneration::Autosar, 8000, 12, 65536,  256, 1000, 4, true,  0, AsilLevel::D, 60.0, 1.50, true),
        EcuRole::FrontCamera =>            (EcuGeneration::Autosar, 1500, 4,  16384,   32, 1000, 2, true,  0, AsilLevel::B, 12.0, 0.42, true),
        EcuRole::FrontRadar =>             (EcuGeneration::Autosar,  240, 2,   4096,    8,    0, 2, true,  0, AsilLevel::B,  8.0, 0.55, true),
        EcuRole::CornerRadar =>            (EcuGeneration::Autosar,  150, 1,   1024,    2,    0, 1, true,  0, AsilLevel::B,  4.0, 0.30, true),
        EcuRole::UltrasonicPark =>         (EcuGeneration::Can,      120, 1,    512,    1,    0, 1, true,  0, AsilLevel::A,  3.0, 0.34, false),
        EcuRole::SurroundView =>           (EcuGeneration::Autosar, 1000, 2,   8192,   16, 1000, 2, true,  0, AsilLevel::A,  8.0, 0.52, true),
        EcuRole::DriverMonitoring =>       (EcuGeneration::Autosar,  800, 2,   4096,    8,  100, 1, true,  0, AsilLevel::B,  6.0, 0.32, true),
        EcuRole::Lidar =>                  (EcuGeneration::Autosar, 2000, 4,  16384,   32, 1000, 2, true,  0, AsilLevel::B, 25.0, 1.10, true),
        EcuRole::Infotainment =>           (EcuGeneration::Autosar, 2000, 8,  65536,  256, 1000, 2, true,  0, AsilLevel::Qm, 25.0, 2.40, true),
        EcuRole::InstrumentCluster =>      (EcuGeneration::Autosar, 1000, 2,  16384,   32,  100, 2, true,  0, AsilLevel::B, 14.0, 1.35, true),
        EcuRole::HeadUpDisplay =>          (EcuGeneration::Autosar,  600, 2,   4096,    8,  100, 1, true,  0, AsilLevel::A, 12.0, 0.85, true),
        EcuRole::AudioAmplifier =>         (EcuGeneration::Can,      400, 2,   4096,    4,    0, 1, true,  0, AsilLevel::Qm, 50.0, 1.60, false),
        EcuRole::Telematics =>             (EcuGeneration::Autosar,  800, 2,   8192,   16,  100, 2, true,  0, AsilLevel::A,  8.0, 0.62, true),
        EcuRole::V2x =>                    (EcuGeneration::Autosar,  240, 1,   2048,    4,  100, 1, true,  0, AsilLevel::A,  3.5, 0.30, true),
        EcuRole::WirelessCharging =>       (EcuGeneration::Can,       80, 1,    256,    1,    0, 1, false, 0, AsilLevel::Qm, 8.0, 0.40, false),
        EcuRole::CentralGateway =>         (EcuGeneration::Autosar,  800, 2,  16384,   32, 1000, 6, true,  4, AsilLevel::B,  8.0, 0.62, true),
    }
}

impl Ecu {
    pub fn for_role(role: EcuRole) -> Self {
        let (generation, processor_mhz, core_count, ram_kb, flash_mb, ethernet_mbps, can_buses, can_fd, lin_buses, asil_level, power_draw_w, mass_kg, over_the_air) = role_spec(role);
        Self {
            role,
            generation,
            processor_mhz,
            core_count,
            ram_kb,
            flash_mb,
            ethernet_mbps,
            can_buses,
            can_fd,
            lin_buses,
            asil_level,
            power_draw_w,
            mass_kg,
            voltage_v: 12.0,
            over_the_air,
        }
    }

    fn legacy(generation: EcuGeneration, processor_mhz: u32, can_buses: u8, over_the_air: bool) -> Self {
        let mut e = Self::for_role(EcuRole::Ecm);
        e.generation = generation;
        e.processor_mhz = processor_mhz;
        e.can_buses = can_buses;
        e.over_the_air = over_the_air;
        e
    }

    pub fn obd1(processor_mhz: u32) -> Self {
        Self::legacy(EcuGeneration::Obd1, processor_mhz, 0, false)
    }

    pub fn obd2(processor_mhz: u32) -> Self {
        Self::legacy(EcuGeneration::Obd2, processor_mhz, 1, false)
    }

    pub fn can(processor_mhz: u32, can_buses: u8) -> Self {
        Self::legacy(EcuGeneration::Can, processor_mhz, can_buses, false)
    }

    pub fn autosar(processor_mhz: u32, can_buses: u8) -> Self {
        Self::legacy(EcuGeneration::Autosar, processor_mhz, can_buses, true)
    }

    pub fn current_draw_a(&self) -> f64 {
        self.power_draw_w / self.voltage_v
    }
}

#[derive(Debug, Clone)]
pub struct EcuSuite {
    pub units: Vec<Ecu>,
}

fn add(units: &mut Vec<Ecu>, role: EcuRole, count: u8) {
    for _ in 0..count {
        units.push(Ecu::for_role(role));
    }
}

fn baseline_modern() -> Vec<Ecu> {
    let mut u = Vec::new();
    add(&mut u, EcuRole::CentralGateway, 1);
    add(&mut u, EcuRole::Ecm, 1);
    add(&mut u, EcuRole::AbsEsc, 1);
    add(&mut u, EcuRole::Eps, 1);
    add(&mut u, EcuRole::Srs, 1);
    add(&mut u, EcuRole::Bcm, 1);
    add(&mut u, EcuRole::Hvac, 1);
    add(&mut u, EcuRole::Tpms, 1);
    add(&mut u, EcuRole::LightingControl, 1);
    add(&mut u, EcuRole::WiperControl, 1);
    add(&mut u, EcuRole::KeylessAccess, 1);
    add(&mut u, EcuRole::InstrumentCluster, 1);
    add(&mut u, EcuRole::Infotainment, 1);
    u
}

impl EcuSuite {
    pub fn from_units(units: Vec<Ecu>) -> Self {
        Self { units }
    }

    pub fn economy_ice() -> Self {
        let mut u = baseline_modern();
        add(&mut u, EcuRole::Telematics, 1);
        add(&mut u, EcuRole::OccupantClassification, 1);
        Self::from_units(u)
    }

    pub fn fleet_ice() -> Self {
        let mut u = baseline_modern();
        add(&mut u, EcuRole::Tcm, 1);
        add(&mut u, EcuRole::Telematics, 1);
        add(&mut u, EcuRole::UltrasonicPark, 1);
        add(&mut u, EcuRole::FrontCamera, 1);
        add(&mut u, EcuRole::OccupantClassification, 1);
        Self::from_units(u)
    }

    pub fn comfort_ice() -> Self {
        let mut u = baseline_modern();
        add(&mut u, EcuRole::Tcm, 1);
        add(&mut u, EcuRole::BrakeBoost, 1);
        add(&mut u, EcuRole::Telematics, 1);
        add(&mut u, EcuRole::AdasDomain, 1);
        add(&mut u, EcuRole::FrontCamera, 1);
        add(&mut u, EcuRole::FrontRadar, 1);
        add(&mut u, EcuRole::UltrasonicPark, 1);
        add(&mut u, EcuRole::DoorModule, 4);
        add(&mut u, EcuRole::SeatModule, 2);
        add(&mut u, EcuRole::OccupantClassification, 1);
        add(&mut u, EcuRole::AudioAmplifier, 1);
        Self::from_units(u)
    }

    pub fn family_ice() -> Self {
        let mut u = Self::comfort_ice().units;
        add(&mut u, EcuRole::CornerRadar, 4);
        add(&mut u, EcuRole::SurroundView, 1);
        add(&mut u, EcuRole::PowerTailgate, 1);
        Self::from_units(u)
    }

    pub fn premium_sport_ice() -> Self {
        let mut u = baseline_modern();
        add(&mut u, EcuRole::Tcm, 1);
        add(&mut u, EcuRole::BrakeBoost, 1);
        add(&mut u, EcuRole::SuspensionControl, 1);
        add(&mut u, EcuRole::Telematics, 1);
        add(&mut u, EcuRole::V2x, 1);
        add(&mut u, EcuRole::AdasDomain, 1);
        add(&mut u, EcuRole::FrontCamera, 1);
        add(&mut u, EcuRole::FrontRadar, 1);
        add(&mut u, EcuRole::CornerRadar, 4);
        add(&mut u, EcuRole::UltrasonicPark, 1);
        add(&mut u, EcuRole::SurroundView, 1);
        add(&mut u, EcuRole::DriverMonitoring, 1);
        add(&mut u, EcuRole::DoorModule, 4);
        add(&mut u, EcuRole::SeatModule, 2);
        add(&mut u, EcuRole::OccupantClassification, 1);
        add(&mut u, EcuRole::AudioAmplifier, 1);
        add(&mut u, EcuRole::HeadUpDisplay, 1);
        add(&mut u, EcuRole::SunroofControl, 1);
        Self::from_units(u)
    }

    pub fn city_hatchback() -> Self {
        let mut u = baseline_modern();
        add(&mut u, EcuRole::OccupantClassification, 1);
        Self::from_units(u)
    }

    pub fn bev_compact() -> Self {
        let mut u = baseline_modern();
        add(&mut u, EcuRole::Vcu, 1);
        add(&mut u, EcuRole::Bms, 1);
        add(&mut u, EcuRole::Inverter, 1);
        add(&mut u, EcuRole::OnBoardCharger, 1);
        add(&mut u, EcuRole::DcDcConverter, 1);
        add(&mut u, EcuRole::BrakeBoost, 1);
        add(&mut u, EcuRole::Telematics, 1);
        add(&mut u, EcuRole::AdasDomain, 1);
        add(&mut u, EcuRole::FrontCamera, 1);
        add(&mut u, EcuRole::FrontRadar, 1);
        add(&mut u, EcuRole::UltrasonicPark, 1);
        add(&mut u, EcuRole::SurroundView, 1);
        add(&mut u, EcuRole::DoorModule, 4);
        add(&mut u, EcuRole::OccupantClassification, 1);
        add(&mut u, EcuRole::AudioAmplifier, 1);
        add(&mut u, EcuRole::WirelessCharging, 1);
        Self::from_units(u)
    }

    pub fn count(&self) -> usize {
        self.units.len()
    }

    pub fn has(&self, role: EcuRole) -> bool {
        self.units.iter().any(|e| e.role == role)
    }

    pub fn count_role(&self, role: EcuRole) -> usize {
        self.units.iter().filter(|e| e.role == role).count()
    }

    pub fn get(&self, role: EcuRole) -> Option<&Ecu> {
        self.units.iter().find(|e| e.role == role)
    }

    pub fn total_mass_kg(&self) -> f64 {
        self.units.iter().map(|e| e.mass_kg).sum()
    }

    pub fn total_power_draw_w(&self) -> f64 {
        self.units.iter().map(|e| e.power_draw_w).sum()
    }

    pub fn total_current_draw_a(&self) -> f64 {
        self.units.iter().map(|e| e.current_draw_a()).sum()
    }

    pub fn total_can_buses(&self) -> u32 {
        self.units.iter().map(|e| e.can_buses as u32).sum()
    }

    pub fn total_processor_mhz(&self) -> u64 {
        self.units.iter().map(|e| e.processor_mhz as u64 * e.core_count as u64).sum()
    }

    pub fn total_ram_mb(&self) -> u32 {
        self.units.iter().map(|e| e.ram_kb / 1024).sum()
    }

    pub fn total_flash_mb(&self) -> u32 {
        self.units.iter().map(|e| e.flash_mb).sum()
    }

    pub fn highest_asil(&self) -> AsilLevel {
        let mut max = AsilLevel::Qm;
        for e in &self.units {
            let rank = |a: AsilLevel| match a {
                AsilLevel::Qm => 0,
                AsilLevel::A => 1,
                AsilLevel::B => 2,
                AsilLevel::C => 3,
                AsilLevel::D => 4,
            };
            if rank(e.asil_level) > rank(max) {
                max = e.asil_level;
            }
        }
        max
    }
}
