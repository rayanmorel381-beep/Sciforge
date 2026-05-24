use crate::cars::blocs::{
    BatteryPack, CellFormat, EMotor, Inverter,
    Bloc, Engine, Fuel,
    DrivelineSpec, EngineOrientation,
    assemble_hev, HevPosition, HevPowerpack, HevSpec,
    IcePowerpack,
    AutomaticGearbox,
    WheelBoltPattern,
    Awd, FullTime,
    DriveLayout,
    engines::{
        gazolines::{
            i6::{turbo as gaz_i6_turbo, supercharged as gaz_i6_sc},
            v6::{turbo as gaz_v6_turbo, supercharged as gaz_v6_sc},
            v8::{turbo as gaz_v8_turbo, supercharged as gaz_v8_sc},
            vr6::{turbo as gaz_vr6_turbo, supercharged as gaz_vr6_sc},
            vr8::{turbo as gaz_vr8_turbo, supercharged as gaz_vr8_sc},
            w6::{turbo as gaz_w6_turbo, supercharged as gaz_w6_sc},
            w8::{turbo as gaz_w8_turbo, supercharged as gaz_w8_sc},
            w10::{turbo as gaz_w10_turbo, supercharged as gaz_w10_sc},
            w12::{turbo as gaz_w12_turbo, supercharged as gaz_w12_sc},
        },
        diesels::{
            v6::{turbo as diesel_v6_turbo, supercharged as diesel_v6_sc},
            v8::{turbo as diesel_v8_turbo, supercharged as diesel_v8_sc},
        },
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
const FRONT_SPLIT_PCT: u8 = 40;

const HEV_SYSTEM_VOLTAGE_V: f64 = 400.0;
const HEV_MOTOR_PEAK_KW: f64 = 80.0;
const HEV_BATTERY_SERIES: u32 = 96;
const HEV_BATTERY_PARALLEL: u32 = 4;
const HEV_CELL_AH: f64 = 4.2;
const HEV_CELL_V: f64 = 3.6;
const HEV_EV_ONLY_SPEED_KMH: f64 = 80.0;

#[derive(Debug, Clone, Copy)]
struct AxleTeeth {
    front_pinion: u16,
    front_ring: u16,
    rear_pinion: u16,
    rear_ring: u16,
    center_pinion: u16,
    center_ring: u16,
}

const TEETH_GASOLINE: AxleTeeth = AxleTeeth {
    front_pinion: 11,
    front_ring: 39,
    rear_pinion: 12,
    rear_ring: 41,
    center_pinion: 13,
    center_ring: 13,
};

const TEETH_DIESEL: AxleTeeth = AxleTeeth {
    front_pinion: 14,
    front_ring: 41,
    rear_pinion: 13,
    rear_ring: 38,
    center_pinion: 12,
    center_ring: 14,
};

const TEETH_HYBRID: AxleTeeth = AxleTeeth {
    front_pinion: 14,
    front_ring: 42,
    rear_pinion: 13,
    rear_ring: 39,
    center_pinion: 13,
    center_ring: 13,
};

#[derive(Debug, Clone)]
pub struct PowertrainKit {
    pub ice: Vec<IcePowerpack>,
    pub hev: Vec<HevPowerpack>,
}

fn awd_driveline_spec(teeth: AxleTeeth, input_torque_nm: f64, max_input_speed_rpm: f64) -> DrivelineSpec {
    DrivelineSpec {
        engine_orientation: EngineOrientation::Longitudinal,
        motorpropulsor_position: 1.20,
        layout: DriveLayout::Awd(Awd::FullTime(FullTime::new(FRONT_SPLIT_PCT))),
        input_torque_nm,
        max_input_speed_rpm,
        front_track_mm: FRONT_TRACK_MM,
        rear_track_mm: REAR_TRACK_MM,
        wheelbase_mm: WHEELBASE_MM,
        gearbox_to_rear_axle_mm: GEARBOX_TO_REAR_AXLE_MM,
        bolt_pattern: WheelBoltPattern::Stud5x114_3,
        max_axial_load_n: MAX_AXIAL_LOAD_N,
        max_radial_load_n: MAX_RADIAL_LOAD_N,
        front_drive_pinion_teeth: teeth.front_pinion,
        front_ring_gear_teeth: teeth.front_ring,
        front_spider_gear_count: SPIDER_COUNT,
        front_spider_gear_teeth: SPIDER_TEETH,
        front_side_gear_teeth: SIDE_TEETH,
        rear_drive_pinion_teeth: teeth.rear_pinion,
        rear_ring_gear_teeth: teeth.rear_ring,
        rear_spider_gear_count: SPIDER_COUNT,
        rear_spider_gear_teeth: SPIDER_TEETH,
        rear_side_gear_teeth: SIDE_TEETH,
        center_drive_pinion_teeth: teeth.center_pinion,
        center_ring_gear_teeth: teeth.center_ring,
        center_spider_gear_count: SPIDER_COUNT,
        center_spider_gear_teeth: SPIDER_TEETH,
        center_side_gear_teeth: SIDE_TEETH,
    }
}

fn comfort_8at_gasoline() -> AutomaticGearbox {
    AutomaticGearbox::torque_converter(
        vec![
            (13, 52),
            (15, 46),
            (17, 40),
            (20, 35),
            (22, 32),
            (24, 29),
            (27, 27),
            (30, 25),
        ],
        (12, 40),
    )
}

fn comfort_8at_diesel() -> AutomaticGearbox {
    AutomaticGearbox::torque_converter(
        vec![
            (12, 56),
            (14, 50),
            (16, 44),
            (19, 38),
            (21, 34),
            (23, 30),
            (26, 28),
            (29, 26),
        ],
        (11, 42),
    )
}

fn build_ice_awd(engine: Engine, gearbox: AutomaticGearbox, teeth: AxleTeeth) -> IcePowerpack {
    let input_torque = gearbox.max_input_torque_nm;
    let max_speed = engine.rpm_max;
    let spec = awd_driveline_spec(teeth, input_torque, max_speed);
    IcePowerpack::new_automatic(engine, gearbox).with_driveline(&spec)
}

fn build_hev_awd(ice: IcePowerpack, motor: EMotor) -> HevPowerpack {
    let input_torque = ice.max_input_torque_nm() + motor.peak_torque_nm;
    let max_speed = ice.engine.rpm_max.max(motor.max_speed_rpm);
    let driveline = awd_driveline_spec(TEETH_HYBRID, input_torque, max_speed);
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

fn gaz_i6_blocs() -> Vec<Bloc> {
    vec![
        gaz_i6_turbo::i6_3_5_ss_g(), gaz_i6_turbo::i6_3_5_ts_g(), gaz_i6_turbo::i6_3_5_vg_g(),
        gaz_i6_sc::i6_3_5_rt_g(),    gaz_i6_sc::i6_3_5_tw_g(),    gaz_i6_sc::i6_3_5_cf_g(),
        gaz_i6_turbo::i6_3_2_ss_g(), gaz_i6_turbo::i6_3_2_ts_g(), gaz_i6_turbo::i6_3_2_vg_g(),
        gaz_i6_sc::i6_3_2_rt_g(),    gaz_i6_sc::i6_3_2_tw_g(),    gaz_i6_sc::i6_3_2_cf_g(),
        gaz_i6_turbo::i6_3_0_ss_g(), gaz_i6_turbo::i6_3_0_ts_g(), gaz_i6_turbo::i6_3_0_vg_g(),
        gaz_i6_sc::i6_3_0_rt_g(),    gaz_i6_sc::i6_3_0_tw_g(),    gaz_i6_sc::i6_3_0_cf_g(),
        gaz_i6_turbo::i6_2_8_ss_g(), gaz_i6_turbo::i6_2_8_ts_g(), gaz_i6_turbo::i6_2_8_vg_g(),
        gaz_i6_sc::i6_2_8_rt_g(),    gaz_i6_sc::i6_2_8_tw_g(),    gaz_i6_sc::i6_2_8_cf_g(),
        gaz_i6_turbo::i6_2_5_ss_g(), gaz_i6_turbo::i6_2_5_ts_g(), gaz_i6_turbo::i6_2_5_vg_g(),
        gaz_i6_sc::i6_2_5_rt_g(),    gaz_i6_sc::i6_2_5_tw_g(),    gaz_i6_sc::i6_2_5_cf_g(),
    ]
}

fn gaz_v6_blocs() -> Vec<Bloc> {
    vec![
        gaz_v6_turbo::v6_4_0_ss_g(), gaz_v6_turbo::v6_4_0_ts_g(), gaz_v6_turbo::v6_4_0_vg_g(),
        gaz_v6_sc::v6_4_0_rt_g(),    gaz_v6_sc::v6_4_0_tw_g(),    gaz_v6_sc::v6_4_0_cf_g(),
        gaz_v6_turbo::v6_3_8_ss_g(), gaz_v6_turbo::v6_3_8_ts_g(), gaz_v6_turbo::v6_3_8_vg_g(),
        gaz_v6_sc::v6_3_8_rt_g(),    gaz_v6_sc::v6_3_8_tw_g(),    gaz_v6_sc::v6_3_8_cf_g(),
        gaz_v6_turbo::v6_3_7_ss_g(), gaz_v6_turbo::v6_3_7_ts_g(), gaz_v6_turbo::v6_3_7_vg_g(),
        gaz_v6_sc::v6_3_7_rt_g(),    gaz_v6_sc::v6_3_7_tw_g(),    gaz_v6_sc::v6_3_7_cf_g(),
        gaz_v6_turbo::v6_3_6_ss_g(), gaz_v6_turbo::v6_3_6_ts_g(), gaz_v6_turbo::v6_3_6_vg_g(),
        gaz_v6_sc::v6_3_6_rt_g(),    gaz_v6_sc::v6_3_6_tw_g(),    gaz_v6_sc::v6_3_6_cf_g(),
        gaz_v6_turbo::v6_3_5_ss_g(), gaz_v6_turbo::v6_3_5_ts_g(), gaz_v6_turbo::v6_3_5_vg_g(),
        gaz_v6_sc::v6_3_5_rt_g(),    gaz_v6_sc::v6_3_5_tw_g(),    gaz_v6_sc::v6_3_5_cf_g(),
    ]
}

fn gaz_v8_blocs() -> Vec<Bloc> {
    vec![
        gaz_v8_turbo::v8_7_0_ss_g(), gaz_v8_turbo::v8_7_0_ts_g(), gaz_v8_turbo::v8_7_0_vg_g(),
        gaz_v8_sc::v8_7_0_rt_g(),    gaz_v8_sc::v8_7_0_tw_g(),    gaz_v8_sc::v8_7_0_cf_g(),
        gaz_v8_turbo::v8_6_4_ss_g(), gaz_v8_turbo::v8_6_4_ts_g(), gaz_v8_turbo::v8_6_4_vg_g(),
        gaz_v8_sc::v8_6_4_rt_g(),    gaz_v8_sc::v8_6_4_tw_g(),    gaz_v8_sc::v8_6_4_cf_g(),
        gaz_v8_turbo::v8_6_2_ss_g(), gaz_v8_turbo::v8_6_2_ts_g(), gaz_v8_turbo::v8_6_2_vg_g(),
        gaz_v8_sc::v8_6_2_rt_g(),    gaz_v8_sc::v8_6_2_tw_g(),    gaz_v8_sc::v8_6_2_cf_g(),
        gaz_v8_turbo::v8_6_0_ss_g(), gaz_v8_turbo::v8_6_0_ts_g(), gaz_v8_turbo::v8_6_0_vg_g(),
        gaz_v8_sc::v8_6_0_rt_g(),    gaz_v8_sc::v8_6_0_tw_g(),    gaz_v8_sc::v8_6_0_cf_g(),
        gaz_v8_turbo::v8_5_7_ss_g(), gaz_v8_turbo::v8_5_7_ts_g(), gaz_v8_turbo::v8_5_7_vg_g(),
        gaz_v8_sc::v8_5_7_rt_g(),    gaz_v8_sc::v8_5_7_tw_g(),    gaz_v8_sc::v8_5_7_cf_g(),
    ]
}

fn gaz_vr6_blocs() -> Vec<Bloc> {
    vec![
        gaz_vr6_turbo::vr6_3_6_ss_g(), gaz_vr6_turbo::vr6_3_6_ts_g(), gaz_vr6_turbo::vr6_3_6_vg_g(),
        gaz_vr6_sc::vr6_3_6_rt_g(),    gaz_vr6_sc::vr6_3_6_tw_g(),    gaz_vr6_sc::vr6_3_6_cf_g(),
        gaz_vr6_turbo::vr6_3_2_ss_g(), gaz_vr6_turbo::vr6_3_2_ts_g(), gaz_vr6_turbo::vr6_3_2_vg_g(),
        gaz_vr6_sc::vr6_3_2_rt_g(),    gaz_vr6_sc::vr6_3_2_tw_g(),    gaz_vr6_sc::vr6_3_2_cf_g(),
        gaz_vr6_turbo::vr6_2_8_ss_g(), gaz_vr6_turbo::vr6_2_8_ts_g(), gaz_vr6_turbo::vr6_2_8_vg_g(),
        gaz_vr6_sc::vr6_2_8_rt_g(),    gaz_vr6_sc::vr6_2_8_tw_g(),    gaz_vr6_sc::vr6_2_8_cf_g(),
    ]
}

fn gaz_vr8_blocs() -> Vec<Bloc> {
    vec![
        gaz_vr8_turbo::vr8_4_4_ss_g(), gaz_vr8_turbo::vr8_4_4_ts_g(), gaz_vr8_turbo::vr8_4_4_vg_g(),
        gaz_vr8_sc::vr8_4_4_rt_g(),    gaz_vr8_sc::vr8_4_4_tw_g(),    gaz_vr8_sc::vr8_4_4_cf_g(),
        gaz_vr8_turbo::vr8_4_0_ss_g(), gaz_vr8_turbo::vr8_4_0_ts_g(), gaz_vr8_turbo::vr8_4_0_vg_g(),
        gaz_vr8_sc::vr8_4_0_rt_g(),    gaz_vr8_sc::vr8_4_0_tw_g(),    gaz_vr8_sc::vr8_4_0_cf_g(),
    ]
}

fn gaz_w6_blocs() -> Vec<Bloc> {
    vec![
        gaz_w6_turbo::w6_3_6_ss_g(), gaz_w6_turbo::w6_3_6_ts_g(), gaz_w6_turbo::w6_3_6_vg_g(),
        gaz_w6_sc::w6_3_6_rt_g(),    gaz_w6_sc::w6_3_6_tw_g(),    gaz_w6_sc::w6_3_6_cf_g(),
        gaz_w6_turbo::w6_3_2_ss_g(), gaz_w6_turbo::w6_3_2_ts_g(), gaz_w6_turbo::w6_3_2_vg_g(),
        gaz_w6_sc::w6_3_2_rt_g(),    gaz_w6_sc::w6_3_2_tw_g(),    gaz_w6_sc::w6_3_2_cf_g(),
        gaz_w6_turbo::w6_2_8_ss_g(), gaz_w6_turbo::w6_2_8_ts_g(), gaz_w6_turbo::w6_2_8_vg_g(),
        gaz_w6_sc::w6_2_8_rt_g(),    gaz_w6_sc::w6_2_8_tw_g(),    gaz_w6_sc::w6_2_8_cf_g(),
    ]
}

fn gaz_w8_blocs() -> Vec<Bloc> {
    vec![
        gaz_w8_turbo::w8_4_2_ss_g(), gaz_w8_turbo::w8_4_2_ts_g(), gaz_w8_turbo::w8_4_2_vg_g(),
        gaz_w8_sc::w8_4_2_rt_g(),    gaz_w8_sc::w8_4_2_tw_g(),    gaz_w8_sc::w8_4_2_cf_g(),
        gaz_w8_turbo::w8_4_0_ss_g(), gaz_w8_turbo::w8_4_0_ts_g(), gaz_w8_turbo::w8_4_0_vg_g(),
        gaz_w8_sc::w8_4_0_rt_g(),    gaz_w8_sc::w8_4_0_tw_g(),    gaz_w8_sc::w8_4_0_cf_g(),
        gaz_w8_turbo::w8_3_6_ss_g(), gaz_w8_turbo::w8_3_6_ts_g(), gaz_w8_turbo::w8_3_6_vg_g(),
        gaz_w8_sc::w8_3_6_rt_g(),    gaz_w8_sc::w8_3_6_tw_g(),    gaz_w8_sc::w8_3_6_cf_g(),
    ]
}

fn gaz_w10_blocs() -> Vec<Bloc> {
    vec![
        gaz_w10_turbo::w10_5_5_ss_g(), gaz_w10_turbo::w10_5_5_ts_g(), gaz_w10_turbo::w10_5_5_vg_g(),
        gaz_w10_sc::w10_5_5_rt_g(),    gaz_w10_sc::w10_5_5_tw_g(),    gaz_w10_sc::w10_5_5_cf_g(),
        gaz_w10_turbo::w10_5_0_ss_g(), gaz_w10_turbo::w10_5_0_ts_g(), gaz_w10_turbo::w10_5_0_vg_g(),
        gaz_w10_sc::w10_5_0_rt_g(),    gaz_w10_sc::w10_5_0_tw_g(),    gaz_w10_sc::w10_5_0_cf_g(),
        gaz_w10_turbo::w10_4_5_ss_g(), gaz_w10_turbo::w10_4_5_ts_g(), gaz_w10_turbo::w10_4_5_vg_g(),
        gaz_w10_sc::w10_4_5_rt_g(),    gaz_w10_sc::w10_4_5_tw_g(),    gaz_w10_sc::w10_4_5_cf_g(),
    ]
}

fn gaz_w12_blocs() -> Vec<Bloc> {
    vec![
        gaz_w12_turbo::w12_6_3_ss_g(), gaz_w12_turbo::w12_6_3_ts_g(), gaz_w12_turbo::w12_6_3_vg_g(),
        gaz_w12_sc::w12_6_3_rt_g(),    gaz_w12_sc::w12_6_3_tw_g(),    gaz_w12_sc::w12_6_3_cf_g(),
        gaz_w12_turbo::w12_6_0_ss_g(), gaz_w12_turbo::w12_6_0_ts_g(), gaz_w12_turbo::w12_6_0_vg_g(),
        gaz_w12_sc::w12_6_0_rt_g(),    gaz_w12_sc::w12_6_0_tw_g(),    gaz_w12_sc::w12_6_0_cf_g(),
        gaz_w12_turbo::w12_5_0_ss_g(), gaz_w12_turbo::w12_5_0_ts_g(), gaz_w12_turbo::w12_5_0_vg_g(),
        gaz_w12_sc::w12_5_0_rt_g(),    gaz_w12_sc::w12_5_0_tw_g(),    gaz_w12_sc::w12_5_0_cf_g(),
    ]
}

fn diesel_v6_blocs() -> Vec<Bloc> {
    vec![
        diesel_v6_turbo::v6_4_0_ss_d(), diesel_v6_turbo::v6_4_0_ts_d(), diesel_v6_turbo::v6_4_0_vg_d(),
        diesel_v6_sc::v6_4_0_rt_d(),    diesel_v6_sc::v6_4_0_tw_d(),    diesel_v6_sc::v6_4_0_cf_d(),
        diesel_v6_turbo::v6_3_8_ss_d(), diesel_v6_turbo::v6_3_8_ts_d(), diesel_v6_turbo::v6_3_8_vg_d(),
        diesel_v6_sc::v6_3_8_rt_d(),    diesel_v6_sc::v6_3_8_tw_d(),    diesel_v6_sc::v6_3_8_cf_d(),
        diesel_v6_turbo::v6_3_7_ss_d(), diesel_v6_turbo::v6_3_7_ts_d(), diesel_v6_turbo::v6_3_7_vg_d(),
        diesel_v6_sc::v6_3_7_rt_d(),    diesel_v6_sc::v6_3_7_tw_d(),    diesel_v6_sc::v6_3_7_cf_d(),
        diesel_v6_turbo::v6_3_6_ss_d(), diesel_v6_turbo::v6_3_6_ts_d(), diesel_v6_turbo::v6_3_6_vg_d(),
        diesel_v6_sc::v6_3_6_rt_d(),    diesel_v6_sc::v6_3_6_tw_d(),    diesel_v6_sc::v6_3_6_cf_d(),
        diesel_v6_turbo::v6_3_5_ss_d(), diesel_v6_turbo::v6_3_5_ts_d(), diesel_v6_turbo::v6_3_5_vg_d(),
        diesel_v6_sc::v6_3_5_rt_d(),    diesel_v6_sc::v6_3_5_tw_d(),    diesel_v6_sc::v6_3_5_cf_d(),
    ]
}

fn diesel_v8_blocs() -> Vec<Bloc> {
    vec![
        diesel_v8_turbo::v8_7_0_ss_d(), diesel_v8_turbo::v8_7_0_ts_d(), diesel_v8_turbo::v8_7_0_vg_d(),
        diesel_v8_sc::v8_7_0_rt_d(),    diesel_v8_sc::v8_7_0_tw_d(),    diesel_v8_sc::v8_7_0_cf_d(),
        diesel_v8_turbo::v8_6_4_ss_d(), diesel_v8_turbo::v8_6_4_ts_d(), diesel_v8_turbo::v8_6_4_vg_d(),
        diesel_v8_sc::v8_6_4_rt_d(),    diesel_v8_sc::v8_6_4_tw_d(),    diesel_v8_sc::v8_6_4_cf_d(),
        diesel_v8_turbo::v8_6_2_ss_d(), diesel_v8_turbo::v8_6_2_ts_d(), diesel_v8_turbo::v8_6_2_vg_d(),
        diesel_v8_sc::v8_6_2_rt_d(),    diesel_v8_sc::v8_6_2_tw_d(),    diesel_v8_sc::v8_6_2_cf_d(),
        diesel_v8_turbo::v8_6_0_ss_d(), diesel_v8_turbo::v8_6_0_ts_d(), diesel_v8_turbo::v8_6_0_vg_d(),
        diesel_v8_sc::v8_6_0_rt_d(),    diesel_v8_sc::v8_6_0_tw_d(),    diesel_v8_sc::v8_6_0_cf_d(),
        diesel_v8_turbo::v8_5_7_ss_d(), diesel_v8_turbo::v8_5_7_ts_d(), diesel_v8_turbo::v8_5_7_vg_d(),
        diesel_v8_sc::v8_5_7_rt_d(),    diesel_v8_sc::v8_5_7_tw_d(),    diesel_v8_sc::v8_5_7_cf_d(),
    ]
}

pub fn gasoline() -> Vec<IcePowerpack> {
    let mut all = Vec::new();
    for engine in expand_engines(gaz_i6_blocs(), Fuel::Gasoline) {
        all.push(build_ice_awd(engine, comfort_8at_gasoline(), TEETH_GASOLINE));
    }
    for engine in expand_engines(gaz_v6_blocs(), Fuel::Gasoline) {
        all.push(build_ice_awd(engine, comfort_8at_gasoline(), TEETH_GASOLINE));
    }
    for engine in expand_engines(gaz_v8_blocs(), Fuel::Gasoline) {
        all.push(build_ice_awd(engine, comfort_8at_gasoline(), TEETH_GASOLINE));
    }
    for engine in expand_engines(gaz_vr6_blocs(), Fuel::Gasoline) {
        all.push(build_ice_awd(engine, comfort_8at_gasoline(), TEETH_GASOLINE));
    }
    for engine in expand_engines(gaz_vr8_blocs(), Fuel::Gasoline) {
        all.push(build_ice_awd(engine, comfort_8at_gasoline(), TEETH_GASOLINE));
    }
    for engine in expand_engines(gaz_w6_blocs(), Fuel::Gasoline) {
        all.push(build_ice_awd(engine, comfort_8at_gasoline(), TEETH_GASOLINE));
    }
    for engine in expand_engines(gaz_w8_blocs(), Fuel::Gasoline) {
        all.push(build_ice_awd(engine, comfort_8at_gasoline(), TEETH_GASOLINE));
    }
    for engine in expand_engines(gaz_w10_blocs(), Fuel::Gasoline) {
        all.push(build_ice_awd(engine, comfort_8at_gasoline(), TEETH_GASOLINE));
    }
    for engine in expand_engines(gaz_w12_blocs(), Fuel::Gasoline) {
        all.push(build_ice_awd(engine, comfort_8at_gasoline(), TEETH_GASOLINE));
    }
    all
}

pub fn diesel() -> Vec<IcePowerpack> {
    let mut all = Vec::new();
    for engine in expand_engines(diesel_v6_blocs(), Fuel::Diesel) {
        all.push(build_ice_awd(engine, comfort_8at_diesel(), TEETH_DIESEL));
    }
    for engine in expand_engines(diesel_v8_blocs(), Fuel::Diesel) {
        all.push(build_ice_awd(engine, comfort_8at_diesel(), TEETH_DIESEL));
    }
    all
}

pub fn hybrid() -> Vec<HevPowerpack> {
    let mut all = Vec::new();
    for engine in expand_engines(gaz_i6_blocs(), Fuel::Gasoline) {
        let ice = build_ice_awd(engine, comfort_8at_gasoline(), TEETH_HYBRID);
        all.push(build_hev_awd(ice, hev_motor()));
    }
    for engine in expand_engines(gaz_v6_blocs(), Fuel::Gasoline) {
        let ice = build_ice_awd(engine, comfort_8at_gasoline(), TEETH_HYBRID);
        all.push(build_hev_awd(ice, hev_motor()));
    }
    for engine in expand_engines(gaz_v8_blocs(), Fuel::Gasoline) {
        let ice = build_ice_awd(engine, comfort_8at_gasoline(), TEETH_HYBRID);
        all.push(build_hev_awd(ice, hev_motor()));
    }
    for engine in expand_engines(gaz_vr6_blocs(), Fuel::Gasoline) {
        let ice = build_ice_awd(engine, comfort_8at_gasoline(), TEETH_HYBRID);
        all.push(build_hev_awd(ice, hev_motor()));
    }
    for engine in expand_engines(gaz_vr8_blocs(), Fuel::Gasoline) {
        let ice = build_ice_awd(engine, comfort_8at_gasoline(), TEETH_HYBRID);
        all.push(build_hev_awd(ice, hev_motor()));
    }
    for engine in expand_engines(gaz_w6_blocs(), Fuel::Gasoline) {
        let ice = build_ice_awd(engine, comfort_8at_gasoline(), TEETH_HYBRID);
        all.push(build_hev_awd(ice, hev_motor()));
    }
    for engine in expand_engines(gaz_w8_blocs(), Fuel::Gasoline) {
        let ice = build_ice_awd(engine, comfort_8at_gasoline(), TEETH_HYBRID);
        all.push(build_hev_awd(ice, hev_motor()));
    }
    for engine in expand_engines(gaz_w10_blocs(), Fuel::Gasoline) {
        let ice = build_ice_awd(engine, comfort_8at_gasoline(), TEETH_HYBRID);
        all.push(build_hev_awd(ice, hev_motor()));
    }
    for engine in expand_engines(gaz_w12_blocs(), Fuel::Gasoline) {
        let ice = build_ice_awd(engine, comfort_8at_gasoline(), TEETH_HYBRID);
        all.push(build_hev_awd(ice, hev_motor()));
    }
    all
}

pub fn all() -> Vec<IcePowerpack> {
    let mut v = gasoline();
    v.extend(diesel());
    v
}
