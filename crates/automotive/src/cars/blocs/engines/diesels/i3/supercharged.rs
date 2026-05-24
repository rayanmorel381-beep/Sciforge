use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 3;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64, boost: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Supercharged, variant,
        compression_ratio: cr, max_boost_bar: boost,
        conrod_ratio_lambda: 0.3,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::sohc_2v_1p_pre_chamber(N), Culasse::dohc_4v_1p_bowl(N), Culasse::dohc_4v_2p_bowl(N)],
    }
}

pub fn i3_0_6_rt_d() -> Bloc { make(cc::_0_6::CC, 61.6, 67.0, "RT_d", 20.0, 0.5) }
pub fn i3_0_6_tw_d() -> Bloc { make(cc::_0_6::CC, 61.6, 67.0, "TW_d", 19.0, 0.9) }
pub fn i3_0_6_cf_d() -> Bloc { make(cc::_0_6::CC, 61.6, 67.0, "CF_d", 18.5, 1.2) }
pub fn i3_0_7_rt_d() -> Bloc { make(cc::_0_7::CC, 64.9, 70.5, "RT_d", 20.0, 0.5) }
pub fn i3_0_7_tw_d() -> Bloc { make(cc::_0_7::CC, 64.9, 70.5, "TW_d", 19.0, 0.9) }
pub fn i3_0_7_cf_d() -> Bloc { make(cc::_0_7::CC, 64.9, 70.5, "CF_d", 18.5, 1.2) }
pub fn i3_0_8_rt_d() -> Bloc { make(cc::_0_8::CC, 67.9, 73.8, "RT_d", 20.0, 0.5) }
pub fn i3_0_8_tw_d() -> Bloc { make(cc::_0_8::CC, 67.9, 73.8, "TW_d", 19.0, 0.9) }
pub fn i3_0_8_cf_d() -> Bloc { make(cc::_0_8::CC, 67.9, 73.8, "CF_d", 18.5, 1.2) }
pub fn i3_0_9_rt_d() -> Bloc { make(cc::_0_9::CC, 70.6, 76.7, "RT_d", 20.0, 0.5) }
pub fn i3_0_9_tw_d() -> Bloc { make(cc::_0_9::CC, 70.6, 76.7, "TW_d", 19.0, 0.9) }
pub fn i3_0_9_cf_d() -> Bloc { make(cc::_0_9::CC, 70.6, 76.7, "CF_d", 18.5, 1.2) }
pub fn i3_1_0_rt_d() -> Bloc { make(cc::_1_0::CC, 73.1, 79.5, "RT_d", 20.0, 0.5) }
pub fn i3_1_0_tw_d() -> Bloc { make(cc::_1_0::CC, 73.1, 79.5, "TW_d", 19.0, 0.9) }
pub fn i3_1_0_cf_d() -> Bloc { make(cc::_1_0::CC, 73.1, 79.5, "CF_d", 18.5, 1.2) }
