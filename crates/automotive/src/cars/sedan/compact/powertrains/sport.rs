use crate::components::powertrain::{
    engines::thermals::assemblies::{bloc::Bloc, engine::Engine, fuel::Fuel},
    powerpacks::{
        driveline::{DrivelineSpec, EngineOrientation},
        IcePowerpack,
    },
    transmissions::{
        assemblies::{automatics::AutomaticGearbox, manuals::ManualGearbox, sequentials::SequentialGearbox},
        clutches::{Clutch, DryCutch},
        drivelines::hubs::WheelBoltPattern,
        drivingwheels::rear::{Rwd, Independent},
    },
};
use crate::cars::blocs::engines::gazolines::{
    v8::{turbo as gaz_v8_bt, supercharged as gaz_v8_bsc},
    v10::{turbo as gaz_v10_bt, supercharged as gaz_v10_bsc},
    v12::{turbo as gaz_v12_bt, supercharged as gaz_v12_bsc},
    v16::{turbo as gaz_v16_bt, supercharged as gaz_v16_bsc},
    w6::{turbo as gaz_w6_bt, supercharged as gaz_w6_bsc},
    w8::{turbo as gaz_w8_bt, supercharged as gaz_w8_bsc},
    w10::{turbo as gaz_w10_bt, supercharged as gaz_w10_bsc},
    w12::{turbo as gaz_w12_bt, supercharged as gaz_w12_bsc},
    w16::{turbo as gaz_w16_bt, supercharged as gaz_w16_bsc},
};

const FRONT_TRACK_MM: f64 = 1560.0;
const REAR_TRACK_MM: f64 = 1550.0;
const WHEELBASE_MM: f64 = 2700.0;
const GEARBOX_TO_REAR_AXLE_MM: f64 = 2050.0;
const MAX_AXIAL_LOAD_N: f64 = 9500.0;
const MAX_RADIAL_LOAD_N: f64 = 14000.0;

const SPIDER_COUNT: u8 = 4;
const SPIDER_TEETH: u16 = 16;
const SIDE_TEETH: u16 = 20;

const REAR_PINION: u16 = 12;
const REAR_RING: u16 = 38;

fn rwd_independent_lsd_spec(input_torque_nm: f64, max_input_speed_rpm: f64) -> DrivelineSpec {
    DrivelineSpec {
        engine_orientation: EngineOrientation::Longitudinal,
        motorpropulsor_position: 1.15,
        layout: crate::components::powertrain::transmissions::drivingwheels::DriveLayout::Rwd(
            Rwd::Independent(Independent::with_lsd(input_torque_nm))
        ),
        input_torque_nm,
        max_input_speed_rpm,
        front_track_mm: FRONT_TRACK_MM,
        rear_track_mm: REAR_TRACK_MM,
        wheelbase_mm: WHEELBASE_MM,
        gearbox_to_rear_axle_mm: GEARBOX_TO_REAR_AXLE_MM,
        bolt_pattern: WheelBoltPattern::Stud5x114_3,
        max_axial_load_n: MAX_AXIAL_LOAD_N,
        max_radial_load_n: MAX_RADIAL_LOAD_N,
        front_drive_pinion_teeth: 0,
        front_ring_gear_teeth: 0,
        front_spider_gear_count: 0,
        front_spider_gear_teeth: 0,
        front_side_gear_teeth: 0,
        rear_drive_pinion_teeth: REAR_PINION,
        rear_ring_gear_teeth: REAR_RING,
        rear_spider_gear_count: SPIDER_COUNT,
        rear_spider_gear_teeth: SPIDER_TEETH,
        rear_side_gear_teeth: SIDE_TEETH,
        center_drive_pinion_teeth: 0,
        center_ring_gear_teeth: 0,
        center_spider_gear_count: 0,
        center_spider_gear_teeth: 0,
        center_side_gear_teeth: 0,
    }
}

fn sport_6speed_manual() -> ManualGearbox {
    ManualGearbox::six_speed(
        vec![(12, 48), (15, 42), (18, 38), (21, 34), (25, 30), (28, 28)],
        (12, 36),
    )
}

fn sport_7speed_at() -> AutomaticGearbox {
    AutomaticGearbox::torque_converter(
        vec![
            (11, 46), (13, 40), (16, 36), (19, 32),
            (22, 28), (25, 25), (28, 23),
        ],
        (12, 36),
    )
}

fn sport_8speed_dct() -> AutomaticGearbox {
    AutomaticGearbox::dct(
        vec![
            (12, 46), (14, 40), (17, 36), (20, 32),
            (23, 28), (26, 25), (29, 22), (32, 20),
        ],
        (12, 36),
    )
}

fn sport_7speed_sequential() -> SequentialGearbox {
    SequentialGearbox::seven_speed(
        vec![(13, 50), (16, 44), (19, 40), (22, 36), (25, 32), (28, 28), (31, 25)],
        (12, 38),
    )
}

fn sport_8speed_sequential() -> SequentialGearbox {
    SequentialGearbox::eight_speed(
        vec![
            (12, 50), (15, 44), (18, 40), (21, 36),
            (24, 32), (27, 28), (30, 25), (33, 22),
        ],
        (12, 38),
    )
}

fn dry_clutch_for(engine: &Engine) -> DryCutch {
    let disp_l = engine.bloc.displacement_cc as f64 / 1000.0;
    let (torque_base, diam_mm): (f64, f64) = if disp_l < 4.0 {
        (500.0, 280.0)
    } else if disp_l < 5.5 {
        (700.0, 300.0)
    } else if disp_l < 6.5 {
        (950.0, 320.0)
    } else if disp_l < 8.0 {
        (1400.0, 340.0)
    } else {
        (1800.0, 360.0)
    };
    DryCutch::single(diam_mm, (torque_base * 1.2 * 1.05 * 100.0).round() / 100.0)
}

fn build_manual(engine: Engine, gearbox: ManualGearbox) -> IcePowerpack {
    let input_torque = gearbox.max_input_torque_nm;
    let max_speed = engine.rpm_max;
    let spec = rwd_independent_lsd_spec(input_torque, max_speed);
    let clutch = Clutch::from(dry_clutch_for(&engine));
    IcePowerpack::new_manual(engine, clutch, gearbox).with_driveline(&spec)
}

fn build_7at(engine: Engine, gearbox: AutomaticGearbox) -> IcePowerpack {
    let input_torque = gearbox.max_input_torque_nm;
    let max_speed = engine.rpm_max;
    let spec = rwd_independent_lsd_spec(input_torque, max_speed);
    IcePowerpack::new_automatic(engine, gearbox).with_driveline(&spec)
}

fn build_8dct(engine: Engine, gearbox: AutomaticGearbox) -> IcePowerpack {
    let input_torque = gearbox.max_input_torque_nm;
    let max_speed = engine.rpm_max;
    let spec = rwd_independent_lsd_spec(input_torque, max_speed);
    let clutch = Clutch::from(dry_clutch_for(&engine));
    IcePowerpack::new_dct(engine, clutch, gearbox).with_driveline(&spec)
}

fn build_sequential(engine: Engine, gearbox: SequentialGearbox) -> IcePowerpack {
    let input_torque = gearbox.max_input_torque_nm;
    let max_speed = engine.rpm_max;
    let spec = rwd_independent_lsd_spec(input_torque, max_speed);
    let clutch = Clutch::from(dry_clutch_for(&engine));
    IcePowerpack::new_sequential(engine, clutch, gearbox).with_driveline(&spec)
}

fn expand_engines(blocs: Vec<Bloc>) -> Vec<Engine> {
    blocs
        .into_iter()
        .flat_map(|bloc| Engine::assemble_all_variants(&bloc, Fuel::Gasoline))
        .collect()
}

fn v8_blocs() -> Vec<Bloc> {
    vec![
        gaz_v8_bt::v8_5_0_ss_g(),  gaz_v8_bt::v8_5_0_ts_g(),  gaz_v8_bt::v8_5_0_vg_g(),
        gaz_v8_bsc::v8_5_0_rt_g(), gaz_v8_bsc::v8_5_0_tw_g(), gaz_v8_bsc::v8_5_0_cf_g(),
        gaz_v8_bt::v8_4_5_ss_g(),  gaz_v8_bt::v8_4_5_ts_g(),  gaz_v8_bt::v8_4_5_vg_g(),
        gaz_v8_bsc::v8_4_5_rt_g(), gaz_v8_bsc::v8_4_5_tw_g(), gaz_v8_bsc::v8_4_5_cf_g(),
    ]
}

fn v10_blocs() -> Vec<Bloc> {
    vec![
        gaz_v10_bt::v10_6_0_ss_g(),  gaz_v10_bt::v10_6_0_ts_g(),  gaz_v10_bt::v10_6_0_vg_g(),
        gaz_v10_bsc::v10_6_0_rt_g(), gaz_v10_bsc::v10_6_0_tw_g(), gaz_v10_bsc::v10_6_0_cf_g(),
        gaz_v10_bt::v10_5_5_ss_g(),  gaz_v10_bt::v10_5_5_ts_g(),  gaz_v10_bt::v10_5_5_vg_g(),
        gaz_v10_bsc::v10_5_5_rt_g(), gaz_v10_bsc::v10_5_5_tw_g(), gaz_v10_bsc::v10_5_5_cf_g(),
    ]
}

fn v12_blocs() -> Vec<Bloc> {
    vec![
        gaz_v12_bt::v12_6_5_ss_g(),  gaz_v12_bt::v12_6_5_ts_g(),  gaz_v12_bt::v12_6_5_vg_g(),
        gaz_v12_bsc::v12_6_5_rt_g(), gaz_v12_bsc::v12_6_5_tw_g(), gaz_v12_bsc::v12_6_5_cf_g(),
        gaz_v12_bt::v12_6_0_ss_g(),  gaz_v12_bt::v12_6_0_ts_g(),  gaz_v12_bt::v12_6_0_vg_g(),
        gaz_v12_bsc::v12_6_0_rt_g(), gaz_v12_bsc::v12_6_0_tw_g(), gaz_v12_bsc::v12_6_0_cf_g(),
    ]
}

fn v16_blocs() -> Vec<Bloc> {
    vec![
        gaz_v16_bt::v16_8_0_ss_g(),  gaz_v16_bt::v16_8_0_ts_g(),  gaz_v16_bt::v16_8_0_vg_g(),
        gaz_v16_bsc::v16_8_0_rt_g(), gaz_v16_bsc::v16_8_0_tw_g(), gaz_v16_bsc::v16_8_0_cf_g(),
    ]
}

fn w6_blocs() -> Vec<Bloc> {
    vec![
        gaz_w6_bt::w6_3_5_ss_g(),  gaz_w6_bt::w6_3_5_ts_g(),  gaz_w6_bt::w6_3_5_vg_g(),
        gaz_w6_bsc::w6_3_5_rt_g(), gaz_w6_bsc::w6_3_5_tw_g(), gaz_w6_bsc::w6_3_5_cf_g(),
    ]
}

fn w8_blocs() -> Vec<Bloc> {
    vec![
        gaz_w8_bt::w8_4_0_ss_g(),  gaz_w8_bt::w8_4_0_ts_g(),  gaz_w8_bt::w8_4_0_vg_g(),
        gaz_w8_bsc::w8_4_0_rt_g(), gaz_w8_bsc::w8_4_0_tw_g(), gaz_w8_bsc::w8_4_0_cf_g(),
    ]
}

fn w10_blocs() -> Vec<Bloc> {
    vec![
        gaz_w10_bt::w10_5_0_ss_g(),  gaz_w10_bt::w10_5_0_ts_g(),  gaz_w10_bt::w10_5_0_vg_g(),
        gaz_w10_bsc::w10_5_0_rt_g(), gaz_w10_bsc::w10_5_0_tw_g(), gaz_w10_bsc::w10_5_0_cf_g(),
    ]
}

fn w12_blocs() -> Vec<Bloc> {
    vec![
        gaz_w12_bt::w12_6_0_ss_g(),  gaz_w12_bt::w12_6_0_ts_g(),  gaz_w12_bt::w12_6_0_vg_g(),
        gaz_w12_bsc::w12_6_0_rt_g(), gaz_w12_bsc::w12_6_0_tw_g(), gaz_w12_bsc::w12_6_0_cf_g(),
    ]
}

fn w16_blocs() -> Vec<Bloc> {
    vec![
        gaz_w16_bt::w16_8_0_ss_g(),  gaz_w16_bt::w16_8_0_ts_g(),  gaz_w16_bt::w16_8_0_vg_g(),
        gaz_w16_bsc::w16_8_0_rt_g(), gaz_w16_bsc::w16_8_0_tw_g(), gaz_w16_bsc::w16_8_0_cf_g(),
    ]
}

pub fn sport() -> Vec<IcePowerpack> {
    let mut result = Vec::new();
    
    for engine in expand_engines(v8_blocs()) {
        result.push(build_manual(engine.clone(), sport_6speed_manual()));
        result.push(build_7at(engine.clone(), sport_7speed_at()));
        result.push(build_8dct(engine.clone(), sport_8speed_dct()));
        result.push(build_sequential(engine.clone(), sport_7speed_sequential()));
        result.push(build_sequential(engine, sport_8speed_sequential()));
    }
    
    for engine in expand_engines(v10_blocs()) {
        result.push(build_manual(engine.clone(), sport_6speed_manual()));
        result.push(build_7at(engine.clone(), sport_7speed_at()));
        result.push(build_8dct(engine.clone(), sport_8speed_dct()));
        result.push(build_sequential(engine.clone(), sport_7speed_sequential()));
        result.push(build_sequential(engine, sport_8speed_sequential()));
    }
    
    for engine in expand_engines(v12_blocs()) {
        result.push(build_manual(engine.clone(), sport_6speed_manual()));
        result.push(build_7at(engine.clone(), sport_7speed_at()));
        result.push(build_8dct(engine.clone(), sport_8speed_dct()));
        result.push(build_sequential(engine.clone(), sport_7speed_sequential()));
        result.push(build_sequential(engine, sport_8speed_sequential()));
    }
    
    for engine in expand_engines(v16_blocs()) {
        result.push(build_manual(engine.clone(), sport_6speed_manual()));
        result.push(build_7at(engine.clone(), sport_7speed_at()));
        result.push(build_8dct(engine.clone(), sport_8speed_dct()));
        result.push(build_sequential(engine.clone(), sport_7speed_sequential()));
        result.push(build_sequential(engine, sport_8speed_sequential()));
    }
    
    for engine in expand_engines(w6_blocs()) {
        result.push(build_manual(engine.clone(), sport_6speed_manual()));
        result.push(build_7at(engine.clone(), sport_7speed_at()));
        result.push(build_8dct(engine.clone(), sport_8speed_dct()));
        result.push(build_sequential(engine.clone(), sport_7speed_sequential()));
        result.push(build_sequential(engine, sport_8speed_sequential()));
    }
    
    for engine in expand_engines(w8_blocs()) {
        result.push(build_manual(engine.clone(), sport_6speed_manual()));
        result.push(build_7at(engine.clone(), sport_7speed_at()));
        result.push(build_8dct(engine.clone(), sport_8speed_dct()));
        result.push(build_sequential(engine.clone(), sport_7speed_sequential()));
        result.push(build_sequential(engine, sport_8speed_sequential()));
    }
    
    for engine in expand_engines(w10_blocs()) {
        result.push(build_manual(engine.clone(), sport_6speed_manual()));
        result.push(build_7at(engine.clone(), sport_7speed_at()));
        result.push(build_8dct(engine.clone(), sport_8speed_dct()));
        result.push(build_sequential(engine.clone(), sport_7speed_sequential()));
        result.push(build_sequential(engine, sport_8speed_sequential()));
    }
    
    for engine in expand_engines(w12_blocs()) {
        result.push(build_manual(engine.clone(), sport_6speed_manual()));
        result.push(build_7at(engine.clone(), sport_7speed_at()));
        result.push(build_8dct(engine.clone(), sport_8speed_dct()));
        result.push(build_sequential(engine.clone(), sport_7speed_sequential()));
        result.push(build_sequential(engine, sport_8speed_sequential()));
    }
    
    for engine in expand_engines(w16_blocs()) {
        result.push(build_manual(engine.clone(), sport_6speed_manual()));
        result.push(build_7at(engine.clone(), sport_7speed_at()));
        result.push(build_8dct(engine.clone(), sport_8speed_dct()));
        result.push(build_sequential(engine.clone(), sport_7speed_sequential()));
        result.push(build_sequential(engine, sport_8speed_sequential()));
    }
    
    result
}

pub fn all() -> Vec<IcePowerpack> {
    sport()
}
