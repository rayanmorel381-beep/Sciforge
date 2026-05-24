use crate::powertrain::engines::electrics::ElectricEngine;
use crate::powertrain::transmissions::differentials::{
    Differential, OpenDifferential,
};
use crate::powertrain::transmissions::drivelines::halfshafts::{self, CvJointKind, Halfshaft, HalfshaftKind};
use crate::powertrain::transmissions::drivelines::hubs::{self, HubKind, WheelBoltPattern, WheelHub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BevArchitecture {
    SingleMotorFront,
    SingleMotorRear,
    DualMotorAwd,
    TriMotor,
    QuadMotorIndependent,
}

#[derive(Debug, Clone)]
pub struct ReducerGearbox {
    pub ratio: f64,
    pub mass_kg: f64,
    pub length_mm: f64,
}

impl ReducerGearbox {
    pub fn single_speed(input_torque_nm: f64) -> Self {
        let ratio = 9.0;
        let mass_kg = 22.0 + (input_torque_nm / 100.0).sqrt() * 3.5;
        Self {
            ratio,
            mass_kg: (mass_kg * 100.0).round() / 100.0,
            length_mm: 220.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BevDriveUnit {
    pub motor: ElectricEngine,
    pub reducer: ReducerGearbox,
    pub differential: Option<Differential>,
    pub halfshafts: (Halfshaft, Halfshaft),
    pub hubs: (WheelHub, WheelHub),
    pub mass_kg: f64,
}

#[derive(Debug, Clone)]
pub struct BevPowerpack {
    pub architecture: BevArchitecture,
    pub front_unit: Option<BevDriveUnit>,
    pub rear_unit: Option<BevDriveUnit>,
    pub front_idle_hubs: Option<(WheelHub, WheelHub)>,
    pub rear_idle_hubs: Option<(WheelHub, WheelHub)>,
    pub differential_count: u8,
    pub motor_count: u8,
    pub total_mass_kg: f64,
    pub peak_power_kw: f64,
    pub peak_torque_nm: f64,
}

#[derive(Debug, Clone)]
pub struct BevSpec {
    pub architecture: BevArchitecture,
    pub front_track_mm: f64,
    pub rear_track_mm: f64,
    pub bolt_pattern: WheelBoltPattern,
    pub max_axial_load_n: f64,
    pub max_radial_load_n: f64,
    pub front_motor: Option<ElectricEngine>,
    pub rear_motor: Option<ElectricEngine>,
    pub front_drive_pinion_teeth: u16,
    pub front_ring_gear_teeth: u16,
    pub front_spider_gear_count: u8,
    pub front_spider_gear_teeth: u16,
    pub front_side_gear_teeth: u16,
    pub rear_drive_pinion_teeth: u16,
    pub rear_ring_gear_teeth: u16,
    pub rear_spider_gear_count: u8,
    pub rear_spider_gear_teeth: u16,
    pub rear_side_gear_teeth: u16,
}

fn ring_gear_diameter_mm(input_torque_nm: f64) -> f64 {
    (140.0 + (input_torque_nm / 100.0).sqrt() * 12.0).clamp(150.0, 320.0)
}

fn build_open_diff(
    axle_torque_nm: f64,
    drive_pinion_teeth: u16,
    ring_gear_teeth: u16,
    spider_gear_count: u8,
    spider_gear_teeth: u16,
    side_gear_teeth: u16,
) -> Differential {
    Differential::Open(OpenDifferential::new(
        ring_gear_diameter_mm(axle_torque_nm),
        1.0,
        drive_pinion_teeth,
        ring_gear_teeth,
        spider_gear_count,
        spider_gear_teeth,
        side_gear_teeth,
    ))
}

fn diff_mass_kg(d: &Differential) -> f64 {
    let ring_mm = match d {
        Differential::Open(o) => o.ring_gear_diameter_mm,
        Differential::Lsd(l) => l.ring_gear_diameter_mm,
        Differential::Locked(_) => 220.0,
        Differential::TorqueVectoring(_) => 240.0,
    };
    (ring_mm / 1000.0).powi(2) * 250.0
}

fn build_axle_pair(
    track_mm: f64,
    axle_torque_nm: f64,
    bolt_pattern: WheelBoltPattern,
    max_axial_load_n: f64,
    max_radial_load_n: f64,
) -> ((Halfshaft, Halfshaft), (WheelHub, WheelHub)) {
    let half_torque = axle_torque_nm / 2.0;
    let length_mm = (track_mm / 2.0 - 60.0).max(150.0);
    let make_hs = || halfshafts::assemble(
        HalfshaftKind::SolidSteel,
        length_mm,
        half_torque,
        CvJointKind::Tripod,
        CvJointKind::BallRzeppa,
    );
    let make_hub = || hubs::assemble(
        HubKind::IndependentDriven,
        bolt_pattern,
        max_axial_load_n,
        max_radial_load_n,
        half_torque,
    );
    ((make_hs(), make_hs()), (make_hub(), make_hub()))
}

fn build_idle_hubs(
    bolt_pattern: WheelBoltPattern,
    max_axial_load_n: f64,
    max_radial_load_n: f64,
) -> (WheelHub, WheelHub) {
    let make = || hubs::assemble(
        HubKind::UnitBearing,
        bolt_pattern,
        max_axial_load_n,
        max_radial_load_n,
        0.0,
    );
    (make(), make())
}

fn build_drive_unit_with_diff(
    motor: ElectricEngine,
    track_mm: f64,
    bolt_pattern: WheelBoltPattern,
    max_axial_load_n: f64,
    max_radial_load_n: f64,
    drive_pinion_teeth: u16,
    ring_gear_teeth: u16,
    spider_gear_count: u8,
    spider_gear_teeth: u16,
    side_gear_teeth: u16,
) -> BevDriveUnit {
    let motor_torque = motor.motor.peak_torque_nm;
    let reducer = ReducerGearbox::single_speed(motor_torque);
    let axle_torque = motor_torque * reducer.ratio;
    let differential = build_open_diff(
        axle_torque,
        drive_pinion_teeth,
        ring_gear_teeth,
        spider_gear_count,
        spider_gear_teeth,
        side_gear_teeth,
    );
    let (halfshafts_pair, hubs_pair) = build_axle_pair(
        track_mm,
        axle_torque,
        bolt_pattern,
        max_axial_load_n,
        max_radial_load_n,
    );
    let motor_mass = 38.0 + motor.motor.peak_power_kw * 0.18;
    let mass = motor_mass
        + reducer.mass_kg
        + diff_mass_kg(&differential)
        + halfshafts_pair.0.mass_kg
        + halfshafts_pair.1.mass_kg
        + hubs_pair.0.mass_kg
        + hubs_pair.1.mass_kg;
    BevDriveUnit {
        motor,
        reducer,
        differential: Some(differential),
        halfshafts: halfshafts_pair,
        hubs: hubs_pair,
        mass_kg: (mass * 100.0).round() / 100.0,
    }
}

pub fn assemble(spec: &BevSpec) -> BevPowerpack {
    match spec.architecture {
        BevArchitecture::SingleMotorFront => {
            let motor = spec.front_motor.clone().unwrap_or_default();
            let unit = build_drive_unit_with_diff(
                motor,
                spec.front_track_mm,
                spec.bolt_pattern,
                spec.max_axial_load_n,
                spec.max_radial_load_n,
                spec.front_drive_pinion_teeth,
                spec.front_ring_gear_teeth,
                spec.front_spider_gear_count,
                spec.front_spider_gear_teeth,
                spec.front_side_gear_teeth,
            );
            let rear_idle = build_idle_hubs(spec.bolt_pattern, spec.max_axial_load_n, spec.max_radial_load_n);
            let total = unit.mass_kg + rear_idle.0.mass_kg + rear_idle.1.mass_kg;
            BevPowerpack {
                architecture: spec.architecture,
                peak_power_kw: unit.motor.motor.peak_power_kw,
                peak_torque_nm: unit.motor.motor.peak_torque_nm,
                front_unit: Some(unit),
                rear_unit: None,
                front_idle_hubs: None,
                rear_idle_hubs: Some(rear_idle),
                differential_count: 1,
                motor_count: 1,
                total_mass_kg: (total * 100.0).round() / 100.0,
            }
        }
        BevArchitecture::SingleMotorRear => {
            let motor = spec.rear_motor.clone().unwrap_or_default();
            let unit = build_drive_unit_with_diff(
                motor,
                spec.rear_track_mm,
                spec.bolt_pattern,
                spec.max_axial_load_n,
                spec.max_radial_load_n,
                spec.rear_drive_pinion_teeth,
                spec.rear_ring_gear_teeth,
                spec.rear_spider_gear_count,
                spec.rear_spider_gear_teeth,
                spec.rear_side_gear_teeth,
            );
            let front_idle = build_idle_hubs(spec.bolt_pattern, spec.max_axial_load_n, spec.max_radial_load_n);
            let total = unit.mass_kg + front_idle.0.mass_kg + front_idle.1.mass_kg;
            BevPowerpack {
                architecture: spec.architecture,
                peak_power_kw: unit.motor.motor.peak_power_kw,
                peak_torque_nm: unit.motor.motor.peak_torque_nm,
                front_unit: None,
                rear_unit: Some(unit),
                front_idle_hubs: Some(front_idle),
                rear_idle_hubs: None,
                differential_count: 1,
                motor_count: 1,
                total_mass_kg: (total * 100.0).round() / 100.0,
            }
        }
        BevArchitecture::DualMotorAwd | BevArchitecture::TriMotor => {
            let front_motor = spec.front_motor.clone().unwrap_or_default();
            let rear_motor = spec.rear_motor.clone().unwrap_or_default();
            let front = build_drive_unit_with_diff(
                front_motor,
                spec.front_track_mm,
                spec.bolt_pattern,
                spec.max_axial_load_n,
                spec.max_radial_load_n,
                spec.front_drive_pinion_teeth,
                spec.front_ring_gear_teeth,
                spec.front_spider_gear_count,
                spec.front_spider_gear_teeth,
                spec.front_side_gear_teeth,
            );
            let rear = build_drive_unit_with_diff(
                rear_motor,
                spec.rear_track_mm,
                spec.bolt_pattern,
                spec.max_axial_load_n,
                spec.max_radial_load_n,
                spec.rear_drive_pinion_teeth,
                spec.rear_ring_gear_teeth,
                spec.rear_spider_gear_count,
                spec.rear_spider_gear_teeth,
                spec.rear_side_gear_teeth,
            );
            let motor_count = if spec.architecture == BevArchitecture::TriMotor { 3 } else { 2 };
            let peak_power_kw = front.motor.motor.peak_power_kw + rear.motor.motor.peak_power_kw;
            let peak_torque_nm = front.motor.motor.peak_torque_nm + rear.motor.motor.peak_torque_nm;
            let total = front.mass_kg + rear.mass_kg;
            BevPowerpack {
                architecture: spec.architecture,
                peak_power_kw,
                peak_torque_nm,
                front_unit: Some(front),
                rear_unit: Some(rear),
                front_idle_hubs: None,
                rear_idle_hubs: None,
                differential_count: 2,
                motor_count,
                total_mass_kg: (total * 100.0).round() / 100.0,
            }
        }
        BevArchitecture::QuadMotorIndependent => {
            let motor = spec.front_motor.clone().unwrap_or_default();
            let half_track_torque = motor.motor.peak_torque_nm;
            let make_quad_unit = |track_mm: f64| {
                let reducer = ReducerGearbox::single_speed(half_track_torque);
                let axle_torque = half_track_torque * reducer.ratio;
                let length_mm = (track_mm / 2.0 - 60.0).max(150.0);
                let hs = halfshafts::assemble(
                    HalfshaftKind::SolidSteel,
                    length_mm,
                    axle_torque,
                    CvJointKind::Tripod,
                    CvJointKind::BallRzeppa,
                );
                let hub = hubs::assemble(
                    HubKind::IndependentDriven,
                    spec.bolt_pattern,
                    spec.max_axial_load_n,
                    spec.max_radial_load_n,
                    axle_torque,
                );
                let motor_mass = 38.0 + motor.motor.peak_power_kw * 0.18;
                let mass = motor_mass + reducer.mass_kg + hs.mass_kg + hub.mass_kg;
                ((hs, hs), (hub, hub), mass)
            };
            let (front_hs, front_hubs, front_mass) = make_quad_unit(spec.front_track_mm);
            let (rear_hs, rear_hubs, rear_mass) = make_quad_unit(spec.rear_track_mm);
            let front_unit = BevDriveUnit {
                motor: motor.clone(),
                reducer: ReducerGearbox::single_speed(half_track_torque),
                differential: None,
                halfshafts: front_hs,
                hubs: front_hubs,
                mass_kg: (front_mass * 2.0 * 100.0).round() / 100.0,
            };
            let rear_unit = BevDriveUnit {
                motor: motor.clone(),
                reducer: ReducerGearbox::single_speed(half_track_torque),
                differential: None,
                halfshafts: rear_hs,
                hubs: rear_hubs,
                mass_kg: (rear_mass * 2.0 * 100.0).round() / 100.0,
            };
            let total = front_unit.mass_kg + rear_unit.mass_kg;
            BevPowerpack {
                architecture: spec.architecture,
                peak_power_kw: motor.motor.peak_power_kw * 4.0,
                peak_torque_nm: motor.motor.peak_torque_nm * 4.0,
                front_unit: Some(front_unit),
                rear_unit: Some(rear_unit),
                front_idle_hubs: None,
                rear_idle_hubs: None,
                differential_count: 0,
                motor_count: 4,
                total_mass_kg: (total * 100.0).round() / 100.0,
            }
        }
    }
}
