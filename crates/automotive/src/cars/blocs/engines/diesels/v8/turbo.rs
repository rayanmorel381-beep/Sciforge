use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 8;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64, boost: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Turbo(1), variant,
        compression_ratio: cr, max_boost_bar: boost,
        conrod_ratio_lambda: 0.3,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::ohv_2v_1p_bowl(N), Culasse::sohc_2v_1p_pre_chamber(N), Culasse::dohc_4v_1p_bowl(N), Culasse::dohc_4v_2p_bowl(N)],
    }
}

pub fn v8_3_5_ss_d() -> Bloc { make(cc::_3_5::CC, 84.4, 78.1, "SS_d", 17.5, 1.5) }
pub fn v8_3_5_ts_d() -> Bloc { make(cc::_3_5::CC, 84.4, 78.1, "TS_d", 16.5, 1.9) }
pub fn v8_3_5_vg_d() -> Bloc { make(cc::_3_5::CC, 84.4, 78.1, "VG_d", 16.0, 2.2) }
pub fn v8_3_9_ss_d() -> Bloc { make(cc::_3_9::CC, 87.5, 81.0, "SS_d", 17.5, 1.5) }
pub fn v8_3_9_ts_d() -> Bloc { make(cc::_3_9::CC, 87.5, 81.0, "TS_d", 16.5, 1.9) }
pub fn v8_3_9_vg_d() -> Bloc { make(cc::_3_9::CC, 87.5, 81.0, "VG_d", 16.0, 2.2) }
pub fn v8_4_0_ss_d() -> Bloc { make(cc::_4_0::CC, 88.3, 81.8, "SS_d", 17.5, 1.5) }
pub fn v8_4_0_ts_d() -> Bloc { make(cc::_4_0::CC, 88.3, 81.8, "TS_d", 16.5, 1.9) }
pub fn v8_4_0_vg_d() -> Bloc { make(cc::_4_0::CC, 88.3, 81.8, "VG_d", 16.0, 2.2) }
pub fn v8_4_4_ss_d() -> Bloc { make(cc::_4_4::CC, 91.1, 84.4, "SS_d", 17.5, 1.5) }
pub fn v8_4_4_ts_d() -> Bloc { make(cc::_4_4::CC, 91.1, 84.4, "TS_d", 16.5, 1.9) }
pub fn v8_4_4_vg_d() -> Bloc { make(cc::_4_4::CC, 91.1, 84.4, "VG_d", 16.0, 2.2) }
pub fn v8_4_5_ss_d() -> Bloc { make(cc::_4_5::CC, 91.8, 85.0, "SS_d", 17.5, 1.5) }
pub fn v8_4_5_ts_d() -> Bloc { make(cc::_4_5::CC, 91.8, 85.0, "TS_d", 16.5, 1.9) }
pub fn v8_4_5_vg_d() -> Bloc { make(cc::_4_5::CC, 91.8, 85.0, "VG_d", 16.0, 2.2) }
pub fn v8_4_6_ss_d() -> Bloc { make(cc::_4_6::CC, 92.5, 85.6, "SS_d", 17.5, 1.5) }
pub fn v8_4_6_ts_d() -> Bloc { make(cc::_4_6::CC, 92.5, 85.6, "TS_d", 16.5, 1.9) }
pub fn v8_4_6_vg_d() -> Bloc { make(cc::_4_6::CC, 92.5, 85.6, "VG_d", 16.0, 2.2) }
pub fn v8_4_8_ss_d() -> Bloc { make(cc::_4_8::CC, 93.8, 86.9, "SS_d", 17.5, 1.5) }
pub fn v8_4_8_ts_d() -> Bloc { make(cc::_4_8::CC, 93.8, 86.9, "TS_d", 16.5, 1.9) }
pub fn v8_4_8_vg_d() -> Bloc { make(cc::_4_8::CC, 93.8, 86.9, "VG_d", 16.0, 2.2) }
pub fn v8_5_0_ss_d() -> Bloc { make(cc::_5_0::CC, 95.1, 88.1, "SS_d", 17.5, 1.5) }
pub fn v8_5_0_ts_d() -> Bloc { make(cc::_5_0::CC, 95.1, 88.1, "TS_d", 16.5, 1.9) }
pub fn v8_5_0_vg_d() -> Bloc { make(cc::_5_0::CC, 95.1, 88.1, "VG_d", 16.0, 2.2) }
pub fn v8_5_3_ss_d() -> Bloc { make(cc::_5_3::CC, 96.9, 89.7, "SS_d", 17.5, 1.5) }
pub fn v8_5_3_ts_d() -> Bloc { make(cc::_5_3::CC, 96.9, 89.7, "TS_d", 16.5, 1.9) }
pub fn v8_5_3_vg_d() -> Bloc { make(cc::_5_3::CC, 96.9, 89.7, "VG_d", 16.0, 2.2) }
pub fn v8_5_5_ss_d() -> Bloc { make(cc::_5_5::CC, 98.1, 90.8, "SS_d", 17.5, 1.5) }
pub fn v8_5_5_ts_d() -> Bloc { make(cc::_5_5::CC, 98.1, 90.8, "TS_d", 16.5, 1.9) }
pub fn v8_5_5_vg_d() -> Bloc { make(cc::_5_5::CC, 98.1, 90.8, "VG_d", 16.0, 2.2) }
pub fn v8_5_7_ss_d() -> Bloc { make(cc::_5_7::CC, 99.3, 91.9, "SS_d", 17.5, 1.5) }
pub fn v8_5_7_ts_d() -> Bloc { make(cc::_5_7::CC, 99.3, 91.9, "TS_d", 16.5, 1.9) }
pub fn v8_5_7_vg_d() -> Bloc { make(cc::_5_7::CC, 99.3, 91.9, "VG_d", 16.0, 2.2) }
pub fn v8_6_0_ss_d() -> Bloc { make(cc::_6_0::CC, 101.0, 93.5, "SS_d", 17.5, 1.5) }
pub fn v8_6_0_ts_d() -> Bloc { make(cc::_6_0::CC, 101.0, 93.5, "TS_d", 16.5, 1.9) }
pub fn v8_6_0_vg_d() -> Bloc { make(cc::_6_0::CC, 101.0, 93.5, "VG_d", 16.0, 2.2) }
pub fn v8_6_2_ss_d() -> Bloc { make(cc::_6_2::CC, 102.1, 94.5, "SS_d", 17.5, 1.5) }
pub fn v8_6_2_ts_d() -> Bloc { make(cc::_6_2::CC, 102.1, 94.5, "TS_d", 16.5, 1.9) }
pub fn v8_6_2_vg_d() -> Bloc { make(cc::_6_2::CC, 102.1, 94.5, "VG_d", 16.0, 2.2) }
pub fn v8_6_4_ss_d() -> Bloc { make(cc::_6_4::CC, 103.2, 95.6, "SS_d", 17.5, 1.5) }
pub fn v8_6_4_ts_d() -> Bloc { make(cc::_6_4::CC, 103.2, 95.6, "TS_d", 16.5, 1.9) }
pub fn v8_6_4_vg_d() -> Bloc { make(cc::_6_4::CC, 103.2, 95.6, "VG_d", 16.0, 2.2) }
pub fn v8_7_0_ss_d() -> Bloc { make(cc::_7_0::CC, 106.4, 98.5, "SS_d", 17.5, 1.5) }
pub fn v8_7_0_ts_d() -> Bloc { make(cc::_7_0::CC, 106.4, 98.5, "TS_d", 16.5, 1.9) }
pub fn v8_7_0_vg_d() -> Bloc { make(cc::_7_0::CC, 106.4, 98.5, "VG_d", 16.0, 2.2) }