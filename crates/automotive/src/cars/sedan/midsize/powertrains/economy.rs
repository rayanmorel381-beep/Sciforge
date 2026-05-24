use crate::cars::blocs::{
    Bloc, Engine, Fuel,
    DrivelineSpec, EngineOrientation,
    IcePowerpack,
    Cvt,
    WheelBoltPattern,
    Fwd, Longitudinal, Transverse,
    DriveLayout,
    engines::{
        gazolines::{
            i3::{na as gaz_i3_na, turbo as gaz_i3_turbo},
            i4::{na as gaz_i4_na, turbo as gaz_i4_turbo},
            i5::{na as gaz_i5_na, turbo as gaz_i5_turbo},
            v6::{na as gaz_v6_na, turbo as gaz_v6_turbo},
            v8::{na as gaz_v8_na, turbo as gaz_v8_turbo},
        },
        diesels::{
            v6::turbo as diesel_v6_turbo,
            v8::turbo as diesel_v8_turbo,
        },
    },
};

const FRONT_TRACK_MM: f64 = 1555.0;
const REAR_TRACK_MM: f64 = 1545.0;
const WHEELBASE_MM: f64 = 2700.0;
const GEARBOX_TO_REAR_AXLE_MM: f64 = 2050.0;
const MAX_AXIAL_LOAD_N: f64 = 7500.0;
const MAX_RADIAL_LOAD_N: f64 = 11000.0;

const FRONT_PINION: u16 = 11;
const FRONT_RING: u16 = 38;
const SPIDER_COUNT: u8 = 4;
const SPIDER_TEETH: u16 = 14;
const SIDE_TEETH: u16 = 18;

#[derive(Debug, Clone)]
pub struct PowertrainKit {
    pub powertrains: Vec<IcePowerpack>,
}

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
        front_drive_pinion_teeth: FRONT_PINION,
        front_ring_gear_teeth: FRONT_RING,
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
        motorpropulsor_position: 1.25,
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
        front_drive_pinion_teeth: FRONT_PINION,
        front_ring_gear_teeth: FRONT_RING,
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

fn economy_cvt() -> Cvt {
    Cvt::belt(0.45, 2.60)
}

fn build_ice_fwd(engine: Engine, gearbox: Cvt) -> IcePowerpack {
    let input_torque = gearbox.max_input_torque_nm;
    let max_speed = engine.rpm_max;
    let spec = fwd_transverse_spec(input_torque, max_speed);
    IcePowerpack::new_cvt(engine, None, gearbox).with_driveline(&spec)
}

fn build_ice_fwd_longitudinal(engine: Engine, gearbox: Cvt) -> IcePowerpack {
    let input_torque = gearbox.max_input_torque_nm;
    let max_speed = engine.rpm_max;
    let spec = fwd_longitudinal_spec(input_torque, max_speed);
    IcePowerpack::new_cvt(engine, None, gearbox).with_driveline(&spec)
}

fn expand_engines(blocs: Vec<Bloc>, fuel: Fuel) -> Vec<Engine> {
    blocs
        .into_iter()
        .flat_map(|bloc| Engine::assemble_all_variants(&bloc, fuel))
        .collect()
}

fn gaz_i3_na_blocs() -> Vec<Bloc> {
    vec![
        gaz_i3_na::i3_0_6_std_g(), gaz_i3_na::i3_0_6_hc_g(),
        gaz_i3_na::i3_0_7_std_g(), gaz_i3_na::i3_0_7_hc_g(),
        gaz_i3_na::i3_0_8_std_g(), gaz_i3_na::i3_0_8_hc_g(),
        gaz_i3_na::i3_0_9_std_g(), gaz_i3_na::i3_0_9_hc_g(),
        gaz_i3_na::i3_1_0_std_g(), gaz_i3_na::i3_1_0_hc_g(),
    ]
}

fn gaz_i4_na_blocs() -> Vec<Bloc> {
    vec![
        gaz_i4_na::i4_1_0_std_g(), gaz_i4_na::i4_1_0_hc_g(),
        gaz_i4_na::i4_1_1_std_g(), gaz_i4_na::i4_1_1_hc_g(),
        gaz_i4_na::i4_1_2_std_g(), gaz_i4_na::i4_1_2_hc_g(),
    ]
}

fn gaz_i5_na_blocs() -> Vec<Bloc> {
    vec![
        gaz_i5_na::i5_2_0_std_g(), gaz_i5_na::i5_2_0_hc_g(),
        gaz_i5_na::i5_2_1_std_g(), gaz_i5_na::i5_2_1_hc_g(),
    ]
}

fn gaz_i3_blocs() -> Vec<Bloc> {
    vec![
        gaz_i3_turbo::i3_0_6_ss_g(), gaz_i3_turbo::i3_0_6_ts_g(), gaz_i3_turbo::i3_0_6_vg_g(),
        gaz_i3_turbo::i3_0_7_ss_g(), gaz_i3_turbo::i3_0_7_ts_g(), gaz_i3_turbo::i3_0_7_vg_g(),
        gaz_i3_turbo::i3_0_8_ss_g(), gaz_i3_turbo::i3_0_8_ts_g(), gaz_i3_turbo::i3_0_8_vg_g(),
        gaz_i3_turbo::i3_0_9_ss_g(), gaz_i3_turbo::i3_0_9_ts_g(), gaz_i3_turbo::i3_0_9_vg_g(),
        gaz_i3_turbo::i3_1_0_ss_g(), gaz_i3_turbo::i3_1_0_ts_g(), gaz_i3_turbo::i3_1_0_vg_g(),
    ]
}

fn gaz_i4_blocs() -> Vec<Bloc> {
    vec![
        gaz_i4_turbo::i4_1_0_ss_g(), gaz_i4_turbo::i4_1_0_ts_g(), gaz_i4_turbo::i4_1_0_vg_g(),
        gaz_i4_turbo::i4_1_1_ss_g(), gaz_i4_turbo::i4_1_1_ts_g(), gaz_i4_turbo::i4_1_1_vg_g(),
        gaz_i4_turbo::i4_1_2_ss_g(), gaz_i4_turbo::i4_1_2_ts_g(), gaz_i4_turbo::i4_1_2_vg_g(),
    ]
}

fn gaz_i5_blocs() -> Vec<Bloc> {
    vec![
        gaz_i5_turbo::i5_2_0_ss_g(), gaz_i5_turbo::i5_2_0_ts_g(), gaz_i5_turbo::i5_2_0_vg_g(),
        gaz_i5_turbo::i5_2_1_ss_g(), gaz_i5_turbo::i5_2_1_ts_g(), gaz_i5_turbo::i5_2_1_vg_g(),
    ]
}

fn gaz_v6_na_blocs() -> Vec<Bloc> {
    vec![
        gaz_v6_na::v6_2_0_std_g(), gaz_v6_na::v6_2_0_hc_g(),
    ]
}

fn gaz_v8_na_blocs() -> Vec<Bloc> {
    vec![
        gaz_v8_na::v8_3_5_std_g(), gaz_v8_na::v8_3_5_hc_g(),
    ]
}

fn gaz_v6_blocs() -> Vec<Bloc> {
    vec![
        gaz_v6_turbo::v6_2_0_ss_g(), gaz_v6_turbo::v6_2_0_ts_g(), gaz_v6_turbo::v6_2_0_vg_g(),
    ]
}

fn gaz_v8_blocs() -> Vec<Bloc> {
    vec![
        gaz_v8_turbo::v8_3_5_ss_g(), gaz_v8_turbo::v8_3_5_ts_g(), gaz_v8_turbo::v8_3_5_vg_g(),
    ]
}

fn diesel_v6_blocs() -> Vec<Bloc> {
    vec![
        diesel_v6_turbo::v6_2_0_ss_d(), diesel_v6_turbo::v6_2_0_ts_d(), diesel_v6_turbo::v6_2_0_vg_d(),
    ]
}

fn diesel_v8_blocs() -> Vec<Bloc> {
    vec![
        diesel_v8_turbo::v8_3_5_ss_d(), diesel_v8_turbo::v8_3_5_ts_d(), diesel_v8_turbo::v8_3_5_vg_d(),
    ]
}

pub fn gasoline() -> Vec<IcePowerpack> {
    let inline_blocs: Vec<Bloc> = gaz_i3_na_blocs()
        .into_iter()
        .chain(gaz_i4_na_blocs())
        .chain(gaz_i5_na_blocs())
        .chain(gaz_i3_blocs())
        .chain(gaz_i4_blocs())
        .chain(gaz_i5_blocs())
        .collect();
    let v_blocs: Vec<Bloc> = gaz_v6_na_blocs()
        .into_iter()
        .chain(gaz_v8_na_blocs())
        .chain(gaz_v6_blocs())
        .chain(gaz_v8_blocs())
        .collect();
    let mut result: Vec<IcePowerpack> = expand_engines(inline_blocs, Fuel::Gasoline)
        .into_iter()
        .map(|engine| build_ice_fwd(engine, economy_cvt()))
        .collect();
    result.extend(
        expand_engines(v_blocs, Fuel::Gasoline)
            .into_iter()
            .map(|engine| build_ice_fwd_longitudinal(engine, economy_cvt())),
    );
    result
}

pub fn diesel() -> Vec<IcePowerpack> {
    let v_blocs: Vec<Bloc> = diesel_v6_blocs()
        .into_iter()
        .chain(diesel_v8_blocs())
        .collect();
    expand_engines(v_blocs, Fuel::Diesel)
        .into_iter()
        .map(|engine| build_ice_fwd_longitudinal(engine, economy_cvt()))
        .collect()
}

pub fn all() -> Vec<IcePowerpack> {
    let mut v = gasoline();
    v.extend(diesel());
    v
}
