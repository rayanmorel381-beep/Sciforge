use crate::cars::blocs::{
    BatteryPack, CellFormat, EMotor, Inverter,
    Bloc, Engine, Fuel,
    DrivelineSpec, EngineOrientation,
    assemble_hev, HevPosition, HevPowerpack, HevSpec,
    IcePowerpack,
    AutomaticGearbox, ManualGearbox, RobotizedManual, with_latency,
    Clutch, DryCutch,
    WheelBoltPattern,
    Fwd, Longitudinal, Transverse,
    DriveLayout,
    engines::{
        gazolines::{
            i4::turbo as gaz_i4_turbo,
            i5::turbo as gaz_i5_turbo,
            i6::{turbo as gaz_i6_turbo, supercharged as gaz_i6_sc},
            v6::turbo as gaz_v6_turbo,
        },
        diesels::v6::turbo as diesel_v6_turbo,
    },
};

const FRONT_TRACK_MM: f64 = 1555.0;
const REAR_TRACK_MM: f64 = 1545.0;
const WHEELBASE_MM: f64 = 2700.0;
const GEARBOX_TO_REAR_AXLE_MM: f64 = 2050.0;
const MAX_AXIAL_LOAD_N: f64 = 8500.0;
const MAX_RADIAL_LOAD_N: f64 = 13000.0;

const SPIDER_COUNT: u8 = 4;
const SPIDER_TEETH: u16 = 14;
const SIDE_TEETH: u16 = 18;

const FRONT_PINION_TRANSVERSE: u16 = 11;
const FRONT_RING_TRANSVERSE: u16 = 38;
const FRONT_PINION_LONGITUDINAL: u16 = 12;
const FRONT_RING_LONGITUDINAL: u16 = 42;

const HEV_SYSTEM_VOLTAGE_V: f64 = 400.0;
const HEV_MOTOR_PEAK_KW: f64 = 50.0;
const HEV_BATTERY_SERIES: u32 = 96;
const HEV_BATTERY_PARALLEL: u32 = 3;
const HEV_CELL_AH: f64 = 4.2;
const HEV_CELL_V: f64 = 3.6;
const HEV_EV_ONLY_SPEED_KMH: f64 = 60.0;

fn fwd_transverse_spec(input_torque_nm: f64, max_input_speed_rpm: f64) -> DrivelineSpec {
    DrivelineSpec {
        engine_orientation: EngineOrientation::Transverse,
        motorpropulsor_position: 1.0,
        layout: DriveLayout::Fwd(Fwd::Transverse(Transverse::new(input_torque_nm))),
        input_torque_nm,
        max_input_speed_rpm,
        front_track_mm: FRONT_TRACK_MM,
        rear_track_mm: REAR_TRACK_MM,
        wheelbase_mm: WHEELBASE_MM,
        gearbox_to_rear_axle_mm: GEARBOX_TO_REAR_AXLE_MM,
        bolt_pattern: WheelBoltPattern::Stud5x114_3,
        max_axial_load_n: MAX_AXIAL_LOAD_N,
        max_radial_load_n: MAX_RADIAL_LOAD_N,
        front_drive_pinion_teeth: FRONT_PINION_TRANSVERSE,
        front_ring_gear_teeth: FRONT_RING_TRANSVERSE,
        front_spider_gear_count: SPIDER_COUNT,
        front_spider_gear_teeth: SPIDER_TEETH,
        front_side_gear_teeth: SIDE_TEETH,
        rear_drive_pinion_teeth: 0,
        rear_ring_gear_teeth: 0,
        rear_spider_gear_count: 0,
        rear_spider_gear_teeth: 0,
        rear_side_gear_teeth: 0,
        center_drive_pinion_teeth: 0,
        center_ring_gear_teeth: 0,
        center_spider_gear_count: 0,
        center_spider_gear_teeth: 0,
        center_side_gear_teeth: 0,
    }
}

fn fwd_longitudinal_spec(input_torque_nm: f64, max_input_speed_rpm: f64) -> DrivelineSpec {
    DrivelineSpec {
        engine_orientation: EngineOrientation::Longitudinal,
        motorpropulsor_position: 1.15,
        layout: DriveLayout::Fwd(Fwd::Longitudinal(Longitudinal::new(input_torque_nm))),
        input_torque_nm,
        max_input_speed_rpm,
        front_track_mm: FRONT_TRACK_MM,
        rear_track_mm: REAR_TRACK_MM,
        wheelbase_mm: WHEELBASE_MM,
        gearbox_to_rear_axle_mm: GEARBOX_TO_REAR_AXLE_MM,
        bolt_pattern: WheelBoltPattern::Stud5x114_3,
        max_axial_load_n: MAX_AXIAL_LOAD_N,
        max_radial_load_n: MAX_RADIAL_LOAD_N,
        front_drive_pinion_teeth: FRONT_PINION_LONGITUDINAL,
        front_ring_gear_teeth: FRONT_RING_LONGITUDINAL,
        front_spider_gear_count: SPIDER_COUNT,
        front_spider_gear_teeth: SPIDER_TEETH,
        front_side_gear_teeth: SIDE_TEETH,
        rear_drive_pinion_teeth: 0,
        rear_ring_gear_teeth: 0,
        rear_spider_gear_count: 0,
        rear_spider_gear_teeth: 0,
        rear_side_gear_teeth: 0,
        center_drive_pinion_teeth: 0,
        center_ring_gear_teeth: 0,
        center_spider_gear_count: 0,
        center_spider_gear_teeth: 0,
        center_side_gear_teeth: 0,
    }
}

fn fleet_5speed_manual() -> ManualGearbox {
    ManualGearbox::five_speed(
        vec![(15, 52), (19, 46), (23, 42), (27, 38), (31, 32)],
        (11, 42),
    )
}

fn fleet_6speed_manual() -> ManualGearbox {
    ManualGearbox::six_speed(
        vec![(14, 50), (17, 44), (20, 40), (24, 36), (28, 32), (32, 28)],
        (11, 40),
    )
}

fn fleet_7dct() -> AutomaticGearbox {
    AutomaticGearbox::dct(
        vec![
            (14, 48), (16, 42), (19, 38), (22, 34),
            (25, 30), (28, 27), (32, 25),
        ],
        (11, 38),
    )
}

fn dry_clutch_for(engine: &Engine) -> DryCutch {
    let disp_l = engine.bloc.displacement_cc as f64 / 1000.0;
    let (torque_base, diam_mm): (f64, f64) = if disp_l < 1.0 {
        (80.0, 180.0)
    } else if disp_l < 1.6 {
        (130.0, 200.0)
    } else if disp_l < 2.2 {
        (190.0, 215.0)
    } else if disp_l < 3.0 {
        (270.0, 230.0)
    } else {
        (380.0, 250.0)
    };
    DryCutch::single(diam_mm, (torque_base * 1.3 * 1.05 * 100.0).round() / 100.0)
}

fn build_manual_transverse(engine: Engine, gearbox: ManualGearbox) -> IcePowerpack {
    let input_torque = gearbox.max_input_torque_nm;
    let max_speed = engine.rpm_max;
    let spec = fwd_transverse_spec(input_torque, max_speed);
    let clutch = Clutch::from(dry_clutch_for(&engine));
    IcePowerpack::new_manual(engine, clutch, gearbox).with_driveline(&spec)
}

fn build_robotized_longitudinal(engine: Engine, gearbox: RobotizedManual) -> IcePowerpack {
    let input_torque = gearbox.gearbox.max_input_torque_nm;
    let max_speed = engine.rpm_max;
    let spec = fwd_longitudinal_spec(input_torque, max_speed);
    let clutch = Clutch::from(dry_clutch_for(&engine));
    IcePowerpack::new_robotized(engine, clutch, gearbox).with_driveline(&spec)
}

fn build_dct_transverse_hev(engine: Engine, gearbox: AutomaticGearbox) -> IcePowerpack {
    let input_torque = gearbox.max_input_torque_nm;
    let max_speed = engine.rpm_max;
    let spec = fwd_transverse_spec(input_torque, max_speed);
    let clutch = Clutch::from(dry_clutch_for(&engine));
    IcePowerpack::new_dct(engine, clutch, gearbox).with_driveline(&spec)
}

fn build_dct_longitudinal_hev(engine: Engine, gearbox: AutomaticGearbox) -> IcePowerpack {
    let input_torque = gearbox.max_input_torque_nm;
    let max_speed = engine.rpm_max;
    let spec = fwd_longitudinal_spec(input_torque, max_speed);
    let clutch = Clutch::from(dry_clutch_for(&engine));
    IcePowerpack::new_dct(engine, clutch, gearbox).with_driveline(&spec)
}

fn build_hev_transverse(ice: IcePowerpack, motor: EMotor) -> HevPowerpack {
    let input_torque = ice.max_input_torque_nm() + motor.peak_torque_nm;
    let max_speed = ice.engine.rpm_max.max(motor.max_speed_rpm);
    let driveline = fwd_transverse_spec(input_torque, max_speed);
    let battery = BatteryPack::nmc(
        CellFormat::Pouch,
        HEV_BATTERY_SERIES,
        HEV_BATTERY_PARALLEL,
        HEV_CELL_AH,
        HEV_CELL_V,
    );
    let inverter = Inverter::sic(HEV_MOTOR_PEAK_KW, HEV_SYSTEM_VOLTAGE_V);
    let spec = HevSpec {
        position: HevPosition::P2BetweenClutchAndGearbox,
        system_voltage_v: HEV_SYSTEM_VOLTAGE_V,
        ice,
        motor,
        battery,
        inverter,
        driveline,
        ev_only_speed_kmh: HEV_EV_ONLY_SPEED_KMH,
    };
    assemble_hev(&spec)
}

fn hev_motor() -> EMotor {
    EMotor::pmsm(HEV_MOTOR_PEAK_KW, HEV_SYSTEM_VOLTAGE_V)
}

fn expand_engines(blocs: Vec<Bloc>, fuel: Fuel) -> Vec<Engine> {
    blocs
        .into_iter()
        .flat_map(|bloc| Engine::assemble_all_variants(&bloc, fuel))
        .collect()
}

fn gaz_i4_blocs() -> Vec<Bloc> {
    vec![
        gaz_i4_turbo::i4_1_0_ss_g(), gaz_i4_turbo::i4_1_0_ts_g(), gaz_i4_turbo::i4_1_0_vg_g(),
        gaz_i4_turbo::i4_1_1_ss_g(), gaz_i4_turbo::i4_1_1_ts_g(), gaz_i4_turbo::i4_1_1_vg_g(),
        gaz_i4_turbo::i4_1_2_ss_g(), gaz_i4_turbo::i4_1_2_ts_g(), gaz_i4_turbo::i4_1_2_vg_g(),
        gaz_i4_turbo::i4_1_3_ss_g(), gaz_i4_turbo::i4_1_3_ts_g(), gaz_i4_turbo::i4_1_3_vg_g(),
        gaz_i4_turbo::i4_1_4_ss_g(), gaz_i4_turbo::i4_1_4_ts_g(), gaz_i4_turbo::i4_1_4_vg_g(),
    ]
}

fn gaz_i5_blocs() -> Vec<Bloc> {
    vec![
        gaz_i5_turbo::i5_2_0_ss_g(), gaz_i5_turbo::i5_2_0_ts_g(), gaz_i5_turbo::i5_2_0_vg_g(),
        gaz_i5_turbo::i5_2_1_ss_g(), gaz_i5_turbo::i5_2_1_ts_g(), gaz_i5_turbo::i5_2_1_vg_g(),
    ]
}

fn gaz_i6_blocs() -> Vec<Bloc> {
    vec![
        gaz_i6_turbo::i6_3_5_ss_g(), gaz_i6_turbo::i6_3_5_ts_g(), gaz_i6_turbo::i6_3_5_vg_g(),
        gaz_i6_sc::i6_3_5_rt_g(),    gaz_i6_sc::i6_3_5_tw_g(),    gaz_i6_sc::i6_3_5_cf_g(),
        gaz_i6_turbo::i6_2_8_ss_g(), gaz_i6_turbo::i6_2_8_ts_g(), gaz_i6_turbo::i6_2_8_vg_g(),
        gaz_i6_sc::i6_2_8_rt_g(),    gaz_i6_sc::i6_2_8_tw_g(),    gaz_i6_sc::i6_2_8_cf_g(),
    ]
}

fn gaz_v6_blocs() -> Vec<Bloc> {
    vec![
        gaz_v6_turbo::v6_2_5_ss_g(), gaz_v6_turbo::v6_2_5_ts_g(), gaz_v6_turbo::v6_2_5_vg_g(),
    ]
}

fn diesel_v6_blocs() -> Vec<Bloc> {
    vec![
        diesel_v6_turbo::v6_2_5_ss_d(), diesel_v6_turbo::v6_2_5_ts_d(), diesel_v6_turbo::v6_2_5_vg_d(),
    ]
}

pub fn gasoline() -> Vec<IcePowerpack> {
    let mut result = Vec::new();
    for engine in expand_engines(gaz_i4_blocs(), Fuel::Gasoline) {
        result.push(build_manual_transverse(engine.clone(), fleet_5speed_manual()));
        result.push(build_manual_transverse(engine, fleet_6speed_manual()));
    }
    for engine in expand_engines(gaz_i5_blocs(), Fuel::Gasoline) {
        result.push(build_manual_transverse(engine.clone(), fleet_5speed_manual()));
        result.push(build_manual_transverse(engine, fleet_6speed_manual()));
    }
    for engine in expand_engines(gaz_i6_blocs(), Fuel::Gasoline) {
        result.push(build_robotized_longitudinal(engine.clone(), with_latency(ManualGearbox::six_speed(
            vec![(13, 52), (16, 46), (19, 42), (22, 38), (26, 34), (30, 30)],
            (11, 42),
        ), engine.bloc.displacement_cc as f64 / 1000.0)));
        result.push(build_robotized_longitudinal(engine.clone(), with_latency(ManualGearbox::seven_speed(
            vec![(13, 54), (15, 48), (18, 44), (21, 40), (24, 36), (27, 32), (31, 28)],
            (11, 44),
        ), engine.bloc.displacement_cc as f64 / 1000.0)));
    }
    for engine in expand_engines(gaz_v6_blocs(), Fuel::Gasoline) {
        result.push(build_robotized_longitudinal(engine.clone(), with_latency(ManualGearbox::six_speed(
            vec![(13, 52), (16, 46), (19, 42), (22, 38), (26, 34), (30, 30)],
            (11, 42),
        ), engine.bloc.displacement_cc as f64 / 1000.0)));
        result.push(build_robotized_longitudinal(engine.clone(), with_latency(ManualGearbox::seven_speed(
            vec![(13, 54), (15, 48), (18, 44), (21, 40), (24, 36), (27, 32), (31, 28)],
            (11, 44),
        ), engine.bloc.displacement_cc as f64 / 1000.0)));
    }
    result
}

pub fn diesel() -> Vec<IcePowerpack> {
    let mut result = Vec::new();
    for engine in expand_engines(diesel_v6_blocs(), Fuel::Diesel) {
        result.push(build_robotized_longitudinal(engine.clone(), with_latency(ManualGearbox::six_speed(
            vec![(13, 52), (16, 46), (19, 42), (22, 38), (26, 34), (30, 30)],
            (11, 42),
        ), engine.bloc.displacement_cc as f64 / 1000.0)));
        result.push(build_robotized_longitudinal(engine.clone(), with_latency(ManualGearbox::seven_speed(
            vec![(13, 54), (15, 48), (18, 44), (21, 40), (24, 36), (27, 32), (31, 28)],
            (11, 44),
        ), engine.bloc.displacement_cc as f64 / 1000.0)));
    }
    result
}

pub fn hybrid() -> Vec<HevPowerpack> {
    let mut result = Vec::new();
    for engine in expand_engines(gaz_i4_blocs(), Fuel::Gasoline) {
        let ice = build_dct_transverse_hev(engine, fleet_7dct());
        result.push(build_hev_transverse(ice, hev_motor()));
    }
    for engine in expand_engines(gaz_i6_blocs(), Fuel::Gasoline) {
        let ice = build_dct_longitudinal_hev(engine, fleet_7dct());
        result.push(build_hev_transverse(ice, hev_motor()));
    }
    result
}

pub fn all() -> Vec<IcePowerpack> {
    let mut v = gasoline();
    v.extend(diesel());
    v
}
