use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 3;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64, boost: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Turbo(1), variant,
        compression_ratio: cr, max_boost_bar: boost,
        conrod_ratio_lambda: 0.3,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::sohc_2v_1p_pre_chamber(N), Culasse::dohc_4v_1p_bowl(N), Culasse::dohc_4v_2p_bowl(N)],
    }
}

pub fn i3_0_6_ss_d() -> Bloc { make(cc::_0_6::CC, 61.6, 67.0, "SS_d", 17.5, 1.5) }
pub fn i3_0_6_ts_d() -> Bloc { make(cc::_0_6::CC, 61.6, 67.0, "TS_d", 16.5, 1.9) }
pub fn i3_0_6_vg_d() -> Bloc { make(cc::_0_6::CC, 61.6, 67.0, "VG_d", 16.0, 2.2) }
pub fn i3_0_7_ss_d() -> Bloc { make(cc::_0_7::CC, 64.9, 70.5, "SS_d", 17.5, 1.5) }
pub fn i3_0_7_ts_d() -> Bloc { make(cc::_0_7::CC, 64.9, 70.5, "TS_d", 16.5, 1.9) }
pub fn i3_0_7_vg_d() -> Bloc { make(cc::_0_7::CC, 64.9, 70.5, "VG_d", 16.0, 2.2) }
pub fn i3_0_8_ss_d() -> Bloc { make(cc::_0_8::CC, 67.9, 73.8, "SS_d", 17.5, 1.5) }
pub fn i3_0_8_ts_d() -> Bloc { make(cc::_0_8::CC, 67.9, 73.8, "TS_d", 16.5, 1.9) }
pub fn i3_0_8_vg_d() -> Bloc { make(cc::_0_8::CC, 67.9, 73.8, "VG_d", 16.0, 2.2) }
pub fn i3_0_9_ss_d() -> Bloc { make(cc::_0_9::CC, 70.6, 76.7, "SS_d", 17.5, 1.5) }
pub fn i3_0_9_ts_d() -> Bloc { make(cc::_0_9::CC, 70.6, 76.7, "TS_d", 16.5, 1.9) }
pub fn i3_0_9_vg_d() -> Bloc { make(cc::_0_9::CC, 70.6, 76.7, "VG_d", 16.0, 2.2) }
pub fn i3_1_0_ss_d() -> Bloc { make(cc::_1_0::CC, 73.1, 79.5, "SS_d", 17.5, 1.5) }
pub fn i3_1_0_ts_d() -> Bloc { make(cc::_1_0::CC, 73.1, 79.5, "TS_d", 16.5, 1.9) }
pub fn i3_1_0_vg_d() -> Bloc { make(cc::_1_0::CC, 73.1, 79.5, "VG_d", 16.0, 2.2) }
